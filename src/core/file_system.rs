use crate::core::ffi::IsFileDropped;

pub fn is_file_dropped() -> bool {
  return unsafe { IsFileDropped() };
}
