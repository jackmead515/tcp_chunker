pub mod headers;
pub mod read;
pub mod data;
pub mod errors;
pub mod util;

use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Request {
  uuid: String,
  chunks: u32,
  chunk_length: u32,
  chunk_amount: u32,
  file_name: String,
  file_location: String,
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

    // next request
    if headers.is_new_request() {
      self.save_new_request(headers);
    
    // more data from existing request
    } else if headers.is_next_chunk() {
      self.ingest_chunk(&mut headers)?;
      self.update_request(&mut headers)?;
    
    // headers are incorrect
    } else {
      return Err(errors::Errors::InvalidRequest);
    }
    
    return Ok(());
  }

  fn ingest_chunk(&mut self, headers: &mut headers::Headers) -> Result<(), errors::Errors> {
    let uuid = headers.uuid.as_ref().unwrap();
    let request = self.cache.get_request(&uuid);

    match request {
      Some(request) => {
        let handler = data::DataHandler::new();
        handler.read_chunk(&mut self.client, request)?;

        return Ok(());
      },
      None => {
        return Err(errors::Errors::InvalidRequest);
      }
    };
  }

  fn update_request(&mut self, headers: &mut headers::Headers) -> Result<(), errors::Errors> {
    let chunk_number = headers.chunk.unwrap();
    let uuid = headers.uuid.as_ref().unwrap();
    let request = self.cache.get_request(&uuid);

    match request {
      Some(mut request) => {
        request.chunks = chunk_number;
        self.cache.save_request(uuid.clone(), request);

        return Ok(());
      },
      None => {
        return Err(errors::Errors::InvalidRequest);
      }
    }
  }

  fn save_new_request(&mut self, headers: headers::Headers) {
    let uuid = headers.uuid.unwrap();
    let chunk_length = headers.chunk_length.unwrap();
    let chunk_amount = headers.chunk_amount.unwrap();
    let file_name = headers.file_name.unwrap();
    let request = Request {
      uuid: uuid.clone(),
      chunks: 0,
      chunk_length,
      chunk_amount,
      file_name,
      file_location: "".to_string()
    };

    self.cache.save_request(uuid, request);
  }
}

