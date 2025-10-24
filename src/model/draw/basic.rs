use crate::{
  model::ffi::{DrawCube, DrawCubeWires, DrawGrid, DrawPlane, DrawRay},
  structs::{Color, Ray, Vector2, Vector3},
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
pub fn draw_plane(center_pos: Vector3, size: Vector2, color: Color) {
  unsafe { DrawPlane(center_pos, size, color) };
}

#[inline]
pub fn draw_ray(ray: Ray, color: Color) {
  unsafe { DrawRay(ray, color) };
}

#[inline]
pub fn draw_grid(slices: i32, spacing: f32) {
  unsafe { DrawGrid(slices, spacing) };
}
