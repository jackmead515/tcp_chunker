use std::io::Read;
use std::io::Error;
use std::net::{TcpStream};

pub fn read(client: &mut TcpStream, read_length: usize) -> Result<Vec<u8>, Error> {
  let mut data = Vec::with_capacity(read_length);
  let mut read_bytes = 0;

  while read_bytes < read_length {
    
    let mut buffer: [u8; 1024] = [0; 1024];
    let length = client.read(&mut buffer)?;
    read_bytes += length;

    data.append(&mut buffer.to_vec());

    if length <= 0 {
      break;
    }
  }

  let slice = &data[0..read_length];

  return Ok(slice.to_vec());
}

