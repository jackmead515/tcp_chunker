use std::net::TcpStream;

use crate::protocol::read;
use crate::protocol::errors::Errors;
use crate::protocol::util;

pub const UUID_BYTES: usize = 128;
pub const UUID_POS: u8 = 0;

pub const CHUNK_BYTES: usize = 32;
pub const CHUNK_POS: u8 = 1;

pub const CHUNK_LENGTH_BYTES: usize = 32;
pub const CHUNK_LENGTH_POS: u8 = 2;

pub const CHUNK_AMOUNT_BYTES: usize = 32;
pub const CHUNK_AMOUNT_POS: u8 = 3;

pub const FILE_NAME_BYTES: usize = 128;
pub const FILE_NAME_POS: u8 = 4;

pub struct Headers {
  pub uuid: Option<String>,
  pub chunk_num: Option<u32>,
  pub chunk_length: Option<u32>,
  pub chunk_amount: Option<u32>,
  pub file_name: Option<String>,
}

pub struct NewRequestHeaders {
  pub chunk_length: u32,
  pub chunk_amount: u32,
  pub file_name: String
}

pub struct ChunkHeaders {
  pub uuid: String,
  pub chunk_num: u32,
}

impl Headers { 
  pub fn new() -> Self {
    return Headers {
      uuid: None,
      file_name: None,
      chunk_num: None,
      chunk_length: None,
      chunk_amount: None
    };
  }

  /// Reads and sets the headers from the tcp stream
  pub fn read(&mut self, client: &mut TcpStream) -> Result<(), Errors> {
    let params = read::pluck(client, 1)?[0];

    if util::bit_at(params, UUID_POS) {
      let data = read::pluck(client, UUID_BYTES)?;
      self.uuid = Some(String::from_utf8_lossy(&data).to_string());
    }
  
    if util::bit_at(params, CHUNK_POS) {
      let data = read::pluck(client, CHUNK_BYTES)?;
      self.chunk_num = Some(util::read_u32(data)?);
    }
  
    if util::bit_at(params, CHUNK_LENGTH_POS) {
      let data = read::pluck(client, CHUNK_LENGTH_BYTES)?;
      self.chunk_length = Some(util::read_u32(data)?);
    }
  
    if util::bit_at(params, CHUNK_AMOUNT_POS) {
      let data = read::pluck(client, CHUNK_AMOUNT_BYTES)?;
      self.chunk_amount = Some(util::read_u32(data)?);
    }

    if util::bit_at(params, FILE_NAME_POS) {
      let data = read::pluck(client, FILE_NAME_BYTES)?;
      self.file_name = Some(String::from_utf8_lossy(&data).to_string());
    }

    return Ok(());
  }

  pub fn is_new_request(&self) -> bool {
    return self.uuid.is_none()
      && self.chunk_num.is_none()
      && self.chunk_length.is_some()
      && self.chunk_amount.is_some()
      && self.file_name.is_some();
  }

  pub fn get_new_request_headers(&self) -> NewRequestHeaders {
    return NewRequestHeaders {
      chunk_length: self.chunk_length.unwrap(),
      chunk_amount: self.chunk_amount.unwrap(),
      file_name: self.file_name.as_ref().unwrap().clone()
    };
  }

  pub fn is_chunk_request(&self) -> bool {
    return self.uuid.is_some()
      && self.chunk_num.is_some()
      && self.chunk_length.is_none()
      && self.chunk_amount.is_none()
      && self.file_name.is_none();
  }

  pub fn get_chunk_headers(&self) -> ChunkHeaders {
    return ChunkHeaders {
      uuid: self.uuid.as_ref().unwrap().clone(),
      chunk_num: self.chunk_num.unwrap()
    };
  }

  pub fn is_valid(&self) -> bool {
    return self.is_chunk_request() || self.is_new_request();
  }
}