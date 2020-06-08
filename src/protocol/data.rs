use std::net::TcpStream;

use crate::protocol::Request;
use crate::protocol::read;
use crate::protocol::errors::Errors;

pub struct DataHandler {}

impl DataHandler {

  pub fn new() -> Self {
    return DataHandler {};
  }

  pub fn read_chunk(&self, client: &mut TcpStream, request: Request) -> Result<Vec<u8>, Errors> {
    return read::read(client, request.chunk_length as usize, 1024);
  }

}