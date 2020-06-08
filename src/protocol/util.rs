use byteorder::ReadBytesExt;
use std::io::Cursor;
use byteorder::{LittleEndian};

use crate::protocol::errors::Errors;

/// tests if a bit is set in a byte
pub fn bit_at(byte: u8, n: u8) -> bool {
  if n < 32 {
    return byte & (1 << n) != 0;
  } else {
      return false;
  }
}

pub fn read_u32(bytes: Vec<u8>) -> Result<u32, Errors> {
  return match Cursor::new(&bytes).read_u32::<LittleEndian>() {
    Ok(c) => Ok(c),
    Err(e) => Err(Errors::ParseError)
  };
}