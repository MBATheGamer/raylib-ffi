#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Shader {
  pub id: u32,        // Shader program id
  pub locs: *mut i32, // Shader locations array (RL_MAX_SHADER_LOCATIONS)
}
