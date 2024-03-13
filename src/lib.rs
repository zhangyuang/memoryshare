#[macro_use]
extern crate napi_derive;

use std::collections::HashMap;

use napi::{Error, Result, Status};
use shared_memory::{Shmem, ShmemConf};

pub struct SHMEM {
  shmem: Shmem,
  data_len: usize,
  max_size: u32,
}
pub enum ShareMemoryError {
  NapiError(Error<Status>),
  UnExpectedError,
  Panic(String),
  SizeOverflow,
  Uninitialized,
}
impl AsRef<str> for ShareMemoryError {
  fn as_ref(&self) -> &str {
    match self {
      ShareMemoryError::UnExpectedError => "UnexpectedError",
      ShareMemoryError::NapiError(e) => e.status.as_ref(),
      ShareMemoryError::Panic(desc) => desc,
      ShareMemoryError::SizeOverflow => "data length exceeds over maxium length",
      ShareMemoryError::Uninitialized => "memory id is uninitialized",
    }
  }
}
impl From<ShareMemoryError> for Error {
  fn from(err: ShareMemoryError) -> Self {
    Error::new(napi::Status::Unknown, format!("{}", err.as_ref()))
  }
}

static mut GLOBAL_SHMEM: Option<HashMap<String, SHMEM>> = None;

#[napi]
unsafe fn get_string(mem_id: String) -> Result<String> {
  let shmem = ShmemConf::new().flink(mem_id).open().unwrap();

  let slice = std::slice::from_raw_parts(shmem.as_ptr(), 4096);
  let str = std::str::from_utf8_unchecked(slice).to_string();
  Ok(str)
}

#[napi]
unsafe fn set_string(mem_id: String, js_string: String) -> Result<()> {
  let shmem = ShmemConf::new().flink(mem_id).open().unwrap();
  let ptr = shmem.as_ptr();
  let string_len = js_string.len();
  if string_len > shmem.len() as usize {
    return Err(ShareMemoryError::SizeOverflow.into());
  }
  std::ptr::copy(js_string.as_ptr(), ptr, string_len);
  Ok(())
}

#[napi]
unsafe fn init(mem_id: String, max_size: u32) {
  let shmem = ShmemConf::new()
    .size(max_size as usize)
    .flink(&mem_id)
    .create()
    .unwrap();
  Box::into_raw(Box::new(shmem));
}

#[napi]
unsafe fn clear(mem_id: String) {
  let global_shmem = GLOBAL_SHMEM.as_mut().unwrap();
  global_shmem.remove(&mem_id);
}
