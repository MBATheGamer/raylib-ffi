#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FilePathList {
  pub capacity: u32,
  pub count: u32,
  pub paths: *mut *mut i8,
}
