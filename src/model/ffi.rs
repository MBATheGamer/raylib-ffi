use crate::structs::{BoundingBox, Color, Ray, RayCollision, Vector2, Vector3};

unsafe extern "C" {
  // Basic geometric 3D shapes drawing functions
  pub fn DrawCube(position: Vector3, width: f32, height: f32, length: f32, color: Color);
  pub fn DrawCubeV(position: Vector3, size: Vector3, color: Color);
  pub fn DrawCubeWires(position: Vector3, width: f32, height: f32, length: f32, color: Color);
  pub fn DrawCubeWiresV(position: Vector3, size: Vector3, color: Color);
  pub fn DrawSphere(center_pos: Vector3, radius: f32, color: Color);
  pub fn DrawPlane(center_pos: Vector3, size: Vector2, color: Color);
  pub fn DrawRay(ray: Ray, color: Color);
  pub fn DrawGrid(slices: i32, spacing: f32);

  // Collision detection functions
  pub fn GetRayCollisionBox(ray: Ray, bounding_box: BoundingBox) -> RayCollision;
}
