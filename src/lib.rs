#[macro_use]
extern crate napi_derive;

use napi::{Error, Result, Status, Env};
use shared_memory::{Shmem, ShmemConf};
use std::fs::{metadata, remove_file};

pub enum ShareMemoryError {
  NapiError(Error<Status>),
  UnExpectedError,
  Panic(String),
  SizeOverflow,
  Uninitialized,
  RepeatInitialized,
}
impl AsRef<str> for ShareMemoryError {
  fn as_ref(&self) -> &str {
    match self {
      ShareMemoryError::UnExpectedError => "UnexpectedError",
      ShareMemoryError::NapiError(e) => e.status.as_ref(),
      ShareMemoryError::Panic(desc) => desc,
      ShareMemoryError::SizeOverflow => "data length exceeds over maxium length",
      ShareMemoryError::Uninitialized => "memory id is uninitialized",
      ShareMemoryError::RepeatInitialized => "memory id has been initialized",
    }
  }
}
impl From<ShareMemoryError> for Error {
  fn from(err: ShareMemoryError) -> Self {
    Error::new(napi::Status::Unknown, format!("{}", err.as_ref()))
  }
}

#[napi]
unsafe fn get_string(mem_id: String) -> Result<String> {
  let shmem = ShmemConf::new()
    .flink(mem_id)
    .open()
    .map_err(|_| ShareMemoryError::Uninitialized)?;
  let len_slice = std::slice::from_raw_parts(shmem.as_ptr(), 4);
  let len = u32::from_ne_bytes(len_slice.try_into().unwrap());

  let slice = std::slice::from_raw_parts(shmem.as_ptr().offset(4), len as usize);
  let str = std::str::from_utf8_unchecked(slice).to_string();
  Ok(str)
}

#[napi]
unsafe fn set_string(mem_id: String, js_string: String) -> Result<()> {
  let shmem = ShmemConf::new()
    .flink(&mem_id)
    .open()
    .map_err(|_| ShareMemoryError::Uninitialized)?;

  let string_len = js_string.len() as u32;
  if string_len + 4 > shmem.len() as u32 {
    return Err(ShareMemoryError::SizeOverflow.into());
  }
  let string_len_byte = string_len.to_ne_bytes();
  std::ptr::copy(string_len_byte.as_ptr(), shmem.as_ptr(), 4);
  std::ptr::copy(
    js_string.as_ptr(),
    shmem.as_ptr().offset(4),
    string_len as usize,
  );
  Ok(())
}

#[napi]
unsafe fn init(mem_id: String, max_size: u32) -> Result<()> {
  if metadata(&mem_id).is_ok() {
    return Err(ShareMemoryError::RepeatInitialized.into());
  }
  let shmem = ShmemConf::new()
    .size(max_size as usize)
    .flink(&mem_id)
    .create()
    .unwrap();
  Box::into_raw(Box::new(shmem));
  Ok(())
}

#[napi]
unsafe fn clear(mem_id: String) {
  if metadata(&mem_id).is_ok() {
    remove_file(mem_id).unwrap();
  }
}

#[napi]
unsafe fn set_buffer(mem_id: String, js_buffer: napi::JsBuffer) -> Result<()> {
  let shmem = ShmemConf::new()
    .flink(&mem_id)
    .open()
    .map_err(|_| ShareMemoryError::Uninitialized)?;
  let js_buffer_value = js_buffer.into_value()?;
  let buffer_len = js_buffer_value.len() as u32;
  if buffer_len + 4 > shmem.len() as u32 {
    return Err(ShareMemoryError::SizeOverflow.into());
  }
  let buffer_len_byte = buffer_len.to_ne_bytes();
  std::ptr::copy(buffer_len_byte.as_ptr(), shmem.as_ptr(), 4);
  std::ptr::copy(
    js_buffer_value.as_ptr(),
    shmem.as_ptr().offset(4),
    buffer_len as usize,
  );
  Ok(())
}

#[napi]
unsafe fn get_buffer(env: Env, mem_id: String) -> Result<napi::JsBuffer> {
  let shmem = ShmemConf::new()
    .flink(mem_id)
    .open()
    .map_err(|_| ShareMemoryError::Uninitialized)?;
  let len_slice = std::slice::from_raw_parts(shmem.as_ptr(), 4);
  let len = u32::from_ne_bytes(len_slice.try_into().unwrap());

  let slice = std::slice::from_raw_parts(shmem.as_ptr().offset(4), len as usize);
  let vec_buffer = slice.to_vec();
  let buffer = env.create_buffer_with_data(vec_buffer)?;
  Ok(buffer.into_raw())
}
