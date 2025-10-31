use std::ffi::CString;

use crate::{core::ffi::LoadShader, structs::Shader};

#[inline]
pub fn load_shader(vs_file_name: &str, fs_file_name: &str) -> Shader {
  let vs_file_name = CString::new(vs_file_name).expect("");
  let fs_file_name = CString::new(fs_file_name).expect("");

  return unsafe { LoadShader(vs_file_name.as_ptr(), fs_file_name.as_ptr()) };
}
