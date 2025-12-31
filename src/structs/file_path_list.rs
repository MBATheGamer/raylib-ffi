use std::ffi::CStr;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FilePathList {
  pub capacity: u32,
  pub count: u32,
  paths: *mut *mut i8,
}

impl FilePathList {
  pub fn get(&self, index: usize) -> Option<&str> {
    if index >= self.count as usize {
      return None;
    }

    let ptr = unsafe { *self.paths.add(index) };

    if ptr.is_null() {
      return None;
    }

    return match unsafe { CStr::from_ptr(ptr).to_str() } {
      Ok(path) => Some(path),
      Err(_) => None,
    };
  }
}
