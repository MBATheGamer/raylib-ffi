use crate::{
  core::ffi::{IsFileDropped, LoadDroppedFiles, UnloadDroppedFiles},
  structs::FilePathList,
};

pub fn is_file_dropped() -> bool {
  return unsafe { IsFileDropped() };
}

pub fn load_dropped_files() -> FilePathList {
  return unsafe { LoadDroppedFiles() };
}

pub fn unload_dropped_files(files: FilePathList) {
  unsafe { UnloadDroppedFiles(files) };
}
