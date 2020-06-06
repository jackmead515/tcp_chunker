use byteorder::ReadBytesExt;
use std::net::{TcpStream};
use std::io::Cursor;

use crate::protocol::read::read;
use crate::protocol::errors::Errors;

use byteorder::{LittleEndian};

pub const HEADERS_SIZE: usize = 128 + 32 + 32 + 32;

pub struct Headers {
  uuid: String,
  chunk: usize,
  chunk_length: usize,
  chunk_amount: usize,
}

pub fn headers(client: &mut TcpStream) -> Result<Headers, Errors> {
  let data = match read(client, HEADERS_SIZE) {
    Ok(d) => d,
    Err(e) => {
      return Err(Errors::ReadError);
    }
  };

  let uuid = String::from_utf8_lossy(&data[0..128]).to_string();
  let chunk = match Cursor::new(&data[128..160]).read_u32::<LittleEndian>() {
    Ok(c) => c as usize,
    Err(e) => {
      return Err(Errors::ParseError);
    }
  };
  let chunk_length = match Cursor::new(&data[160..192]).read_u32::<LittleEndian>() {
    Ok(c) => c as usize,
    Err(e) => {
      return Err(Errors::ParseError);
    }
  };
  let chunk_amount = match Cursor::new(&data[192..224]).read_u32::<LittleEndian>() {
    Ok(c) => c as usize,
    Err(e) => {
      return Err(Errors::ParseError);
    }
  };

  return Ok(Headers {
    uuid,
    chunk,
    chunk_length,
    chunk_amount
  });
}