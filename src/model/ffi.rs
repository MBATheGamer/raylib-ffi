use crate::structs::{Color, Vector3};

unsafe extern "C" {
  // Basic geometric 3D shapes drawing functions
  pub unsafe fn DrawCube(position: Vector3, width: f32, height: f32, length: f32, color: Color);
  pub unsafe fn DrawCubeWires(
    position: Vector3,
    width: f32,
    height: f32,
    length: f32,
    color: Color,
  );
  pub unsafe fn DrawGrid(slices: i32, spacing: f32);
}
