use crate::{
  core::ffi::{IsFileDropped, LoadDroppedFiles},
  structs::FilePathList,
};

pub fn is_file_dropped() -> bool {
  return unsafe { IsFileDropped() };
}

pub fn load_dropped_files() -> FilePathList {
  return unsafe { LoadDroppedFiles() };
}
