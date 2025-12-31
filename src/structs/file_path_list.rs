use std::ffi::CStr;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FilePathList {
  pub capacity: u32,
  pub count: u32,
  paths: *mut *mut i8,
}

impl FilePathList {
  #[inline]
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

  #[inline]
  pub fn get_paths(&self) -> Vec<String> {
    let mut out = Vec::with_capacity(self.count as usize);

    for i in 0..self.count as usize {
      let c_ptr = unsafe { *self.paths.add(i) };

      if !c_ptr.is_null() {
        if let Ok(s) = unsafe { CStr::from_ptr(c_ptr).to_str() } {
          out.push(s.to_string());
        }
      }
    }

    return out;
  }
}
