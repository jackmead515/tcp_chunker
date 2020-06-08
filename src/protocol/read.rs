use std::io::Read;
use std::net::{TcpStream};

use crate::protocol::errors::Errors;

pub fn read(client: &mut TcpStream, read_length: usize, chunk: usize) -> Result<Vec<u8>, Errors> {
  let mut data = Vec::with_capacity(read_length);
  let mut read_bytes = 0;

  while read_bytes < read_length {
    let mut buffer = Vec::with_capacity(chunk);
    let length = match client.read(&mut buffer) {
      Ok(l) => l,
      Err(e) => {
        return Err(Errors::ReadError);
      }
    };
    read_bytes += length;

    data.append(&mut buffer.to_vec());

    if length <= 0 {
      break;
    }
  }

  let slice = &data[0..read_length];

  return Ok(slice.to_vec());
}

/// Reads no more, but potentially less data than the read_length from the stream.
pub fn pluck(client: &mut TcpStream, read_length: usize) -> Result<Vec<u8>, Errors> {
  let mut data = Vec::with_capacity(read_length);

  let length = match client.read(&mut data) {
    Ok(l) => l,
    Err(e) => {
      return Err(Errors::ReadError);
    }
  };

  return Ok(data);
}

