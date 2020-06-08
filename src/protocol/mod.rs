pub mod headers;
pub mod read;
pub mod errors;
pub mod util;

use dirs;
use uuid::Uuid;

use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::io::prelude::*;

pub fn get_db_path() -> String {
  let mut path = dirs::home_dir().unwrap().to_str().unwrap().to_string();
  path.push_str("/.chunker/");
  return path;
}

#[derive(Clone)]
pub struct Request {
  uuid: String,
  chunk_num: u32,
  chunk_length: u32,
  chunk_amount: u32,
  chunk_pos: Vec<u32>,
  file_name: String
}

pub struct Pipeline {
  client: TcpStream,
  cache: Arc<AppCache>
}

pub struct AppCache {
  requests: Mutex<HashMap<String, Request>>
}

impl AppCache {

  pub fn new() -> Self {
    return AppCache {
      requests: Mutex::new(HashMap::new())
    };
  }

  pub fn save_request(&self, uuid: String, request: Request) {
    let mut requests = self.requests.lock().unwrap();
    requests.insert(uuid, request);
  }

  pub fn get_request(&self, uuid: &String) -> Option<Request> {
    let mut requests = self.requests.lock().unwrap();

    return match requests.get_mut(uuid) {
      Some(request) => Some(request.clone()),
      None => None,
    };
  }

  pub fn drop_request(&self, uuid: &String) {
    let mut requests = self.requests.lock().unwrap();

    requests.remove(uuid);
  }

  pub fn active_requests(&self) -> usize {
    let requests = self.requests.lock().unwrap();
    return requests.keys().len();
  }

}

impl Pipeline {

  pub fn new(client: TcpStream, cache: Arc<AppCache>) -> Self {
    return Pipeline {
      client,
      cache
    };
  }

  pub async fn run(&mut self) -> Result<(), errors::Errors> {
    let mut headers = headers::Headers::new();
    headers.read(&mut self.client)?;

    if headers.is_valid() {
      self.save_if_new_request(&headers);
      self.update_request(headers)?;
      self.reply_to_client();

      return Ok(());
    }
  
    return Err(errors::Errors::InvalidRequest);
  }

  /// Reads the chunk, updates the request, saves the file. Stuff like that.
  fn update_request(&mut self, headers: headers::Headers) -> Result<(), errors::Errors> {
    if headers.is_chunk_request() {
      let uuid = headers.uuid.as_ref().unwrap();
      let request = self.cache.get_request(uuid);
      if !request.is_some() {
        let mut request = request.unwrap();
        self.ingest_chunk(&request)?;

        let chunk_num = headers.chunk_num.unwrap();
        request.chunk_num = chunk_num;
        request.chunk_pos.push(chunk_num);

        if self.is_last_request(&request) {

          // TODO
          // remove from cache
          // and then rearrange file

          self.cache.drop_request(uuid);
        } else {
          self.cache.save_request(uuid.clone(), request);
        }
      }
    }

    return Ok(());
  }

  /// Reads the chunk from the TcpStream and writes to the file
  fn ingest_chunk(&mut self, request: &Request) -> Result<(), errors::Errors> {
    let chunk_data = read::read(&mut self.client, request.chunk_length as usize, 1024)?;

    let mut file_location = String::from(get_db_path());
    file_location.push_str(&request.uuid);
    file_location.push_str("_");
    file_location.push_str(&request.file_name);

    if !util::file_exists(&file_location) {
      util::create_file(&file_location)?;
    }

    let mut file = util::open_w_file(&file_location)?;

    return match file.write_all(&chunk_data[..]) {
      Ok(()) => Ok(()),
      Err(e) => return Err(errors::Errors::FileIOError)
    };
  }

  /// Tests if the request is the last request in the sequence
  fn is_last_request(&self, request: &Request) -> bool {
    if request.chunk_pos.len() == request.chunk_amount as usize {
      return true;
    }

    return false;
  }

  /// Saves a new request from the headers
  fn save_if_new_request(&mut self, headers: &headers::Headers) {
    if headers.is_new_request() {
      let headers = headers.get_new_request_headers();

      let uuid = Uuid::new_v4().to_string();
      let file_name = headers.file_name.clone();
      let request = Request {
        uuid: uuid.clone(),
        file_name,
        chunk_num: 0,
        chunk_pos: Vec::new(),
        chunk_length: headers.chunk_length,
        chunk_amount: headers.chunk_amount,
      };

      self.cache.save_request(uuid, request);
    }
  }
}

