use std::ffi::CString;

use crate::{
  core::ffi::{IsFileDropped, IsFileExtension, LoadDroppedFiles, UnloadDroppedFiles},
  structs::FilePathList,
};

#[inline]
pub fn is_file_extension(file_name: &str, extension: &str) -> bool {
  let file_name = CString::new(file_name).expect("[ERROR] Expecting valid file name");
  let extension = CString::new(extension).expect("[ERROR] Expecting valid extensions");

  return unsafe { IsFileExtension(file_name.as_ptr(), extension.as_ptr()) };
}

#[inline]
pub fn is_file_dropped() -> bool {
  return unsafe { IsFileDropped() };
}

#[inline]
pub fn load_dropped_files() -> FilePathList {
  return unsafe { LoadDroppedFiles() };
}

#[inline]
pub fn unload_dropped_files(files: FilePathList) {
  unsafe { UnloadDroppedFiles(files) };
}
