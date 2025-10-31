use std::ffi::{CString, c_void};

use crate::{
  core::ffi::{GetShaderLocation, LoadShader, SetShaderValue, UnloadShader},
  enums::ShaderUniformType,
  structs::Shader,
};

#[inline]
pub fn load_shader(vs_file_name: &str, fs_file_name: &str) -> Shader {
  let vs_file_name = CString::new(vs_file_name).expect("");
  let fs_file_name = CString::new(fs_file_name).expect("");

  return unsafe { LoadShader(vs_file_name.as_ptr(), fs_file_name.as_ptr()) };
}

#[inline]
pub fn get_shader_location(shader: Shader, uniform_name: &str) -> i32 {
  let uniform_name = CString::new(uniform_name).expect("");

  return unsafe { GetShaderLocation(shader, uniform_name.as_ptr()) };
}

#[inline]
pub fn set_shader_value(
  shader: Shader,
  loc_index: i32,
  value: &[f32],
  uniform_type: ShaderUniformType,
) {
  unsafe {
    SetShaderValue(
      shader,
      loc_index,
      value.as_ptr() as *const c_void,
      uniform_type as i32,
    )
  };
}

#[inline]
pub fn unload_shader(shader: Shader) {
  unsafe { UnloadShader(shader) };
}
