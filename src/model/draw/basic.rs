use crate::{
  model::ffi::{DrawCube, DrawCubeWires},
  structs::{Color, Vector3},
};

pub fn draw_cube(position: Vector3, width: f32, height: f32, length: f32, color: Color) {
  unsafe { DrawCube(position, width, height, length, color) };
}

pub fn draw_cube_wires(position: Vector3, width: f32, height: f32, length: f32, color: Color) {
  unsafe { DrawCubeWires(position, width, height, length, color) };
}
