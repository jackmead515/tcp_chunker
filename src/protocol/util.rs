use byteorder::ReadBytesExt;
use std::io::Cursor;
use byteorder::{LittleEndian};
use std::fs;
use std::fs::{OpenOptions, File};

use crate::protocol::errors::Errors;

/// tests if a bit is set in a byte
pub fn bit_at(byte: u8, n: u8) -> bool {
  if n < 32 {
    return byte & (1 << n) != 0;
  } else {
      return false;
  }
}

pub fn to_vec_u32(bytes: Vec<u8>) -> Vec<u32> {
  let mut nums: Vec<u32> = Vec::new();

  let mut index = 0;
  while index <= bytes.len() {
    let mut c = [0u8; 4];
    c.clone_from_slice(&bytes[index..index+4]);
    nums.push(u32::from_le_bytes(c));
    index += 4;
  }

  return nums;
}

pub fn read_u32(bytes: Vec<u8>) -> Result<u32, Errors> {
  return match Cursor::new(&bytes).read_u32::<LittleEndian>() {
    Ok(c) => Ok(c),
    Err(e) => Err(Errors::ParseError)
  };
}

pub fn file_exists(path: &String) -> bool {
  return fs::metadata(path).is_ok();
}

pub fn create_file(path: &String) -> Result<(), Errors> {
  match File::create(path) {
    Ok(f) => return Ok(()),
    Err(e) => return Err(Errors::FileIOError)
  }
}

pub fn open_r_file(path: &String) -> Result<File, Errors> {
  match OpenOptions::new().read(true).write(false).open(path) {
    Ok(f) => return Ok(f),
    Err(e) => return Err(Errors::FileIOError)
  };
}

pub fn open_w_file(path: &String) -> Result<File, Errors> {
  match OpenOptions::new().read(false).write(true).open(path) {
    Ok(f) => return Ok(f),
    Err(e) => return Err(Errors::FileIOError)
  };
}

pub fn open_rw_file(path: &String) -> Result<File, Errors> {
  match OpenOptions::new().read(true).write(true).open(path) {
    Ok(f) => return Ok(f),
    Err(e) => return Err(Errors::FileIOError)
  };
}