use crate::{
  model::ffi::{DrawCube, DrawCubeWires, DrawGrid},
  structs::{Color, Vector3},
};

#[inline]
pub fn draw_cube(position: Vector3, width: f32, height: f32, length: f32, color: Color) {
  unsafe { DrawCube(position, width, height, length, color) };
}

#[inline]
pub fn draw_cube_wires(position: Vector3, width: f32, height: f32, length: f32, color: Color) {
  unsafe { DrawCubeWires(position, width, height, length, color) };
}

#[inline]
pub fn draw_grid(slices: i32, spacing: f32) {
  unsafe { DrawGrid(slices, spacing) };
}
