use crate::{
  core::ffi::{
    BeginDrawing, BeginMode2D, BeginMode3D, BeginScissorMode, BeginShaderMode, BeginTextureMode,
    BeginVrStereoMode, ClearBackground, EndDrawing, EndMode2D, EndMode3D, EndScissorMode,
    EndShaderMode, EndTextureMode, EndVrStereoMode,
  },
  structs::{Camera2D, Camera3D, Color, RenderTexture, Shader, VrStereoConfig},
};

#[inline]
pub fn clear_background(color: Color) {
  unsafe { ClearBackground(color) };
}

#[inline]
pub fn begin_drawing() {
  unsafe { BeginDrawing() };
}

#[inline]
pub fn end_drawing() {
  unsafe { EndDrawing() };
}

#[inline]
pub fn begin_mode_2d(camera: Camera2D) {
  unsafe { BeginMode2D(camera) };
}

#[inline]
pub fn end_mode_2d() {
  unsafe { EndMode2D() };
}

#[inline]
pub fn begin_mode_3d(camera: Camera3D) {
  unsafe { BeginMode3D(camera) };
}

#[inline]
pub fn end_mode_3d() {
  unsafe { EndMode3D() };
}

#[inline]
pub fn begin_texture_mode(target: RenderTexture) {
  unsafe { BeginTextureMode(target) };
}

#[inline]
pub fn end_texture_mode() {
  unsafe { EndTextureMode() };
}

#[inline]
pub fn begin_shader_mode(shader: Shader) {
  unsafe { BeginShaderMode(shader) };
}

#[inline]
pub fn end_shader_mode() {
  unsafe { EndShaderMode() };
}

#[inline]
pub fn begin_scissor_mode(x: i32, y: i32, width: i32, height: i32) {
  unsafe { BeginScissorMode(x, y, width, height) };
}

#[inline]

pub fn end_scissor_mode() {
  unsafe { EndScissorMode() };
}

#[inline]
pub fn begin_vr_stereo_mode(config: VrStereoConfig) {
  unsafe { BeginVrStereoMode(config) };
}

#[inline]
pub fn end_vr_stereo_mode() {
  unsafe { EndVrStereoMode() };
}
