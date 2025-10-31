#[repr(i32)]
pub enum ShaderUniformType {
  Float = 0, // Shader uniform type: float
  Vec2,      // Shader uniform type: vec2 (2 float)
  Vec3,      // Shader uniform type: vec3 (3 float)
  Vec4,      // Shader uniform type: vec4 (4 float)
  Int,       // Shader uniform type: int
  IVec2,     // Shader uniform type: ivec2 (2 int)
  IVec3,     // Shader uniform type: ivec3 (3 int)
  IVec4,     // Shader uniform type: ivec4 (4 int)
  UInt,      // Shader uniform type: unsigned int
  UIVec2,    // Shader uniform type: uivec2 (2 unsigned int)
  UIVec3,    // Shader uniform type: uivec3 (3 unsigned int)
  UIVec4,    // Shader uniform type: uivec4 (4 unsigned int)
  Sampler2d, // Shader uniform type: sampler2d
}
