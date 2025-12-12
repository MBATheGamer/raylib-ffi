use crate::structs::{BoundingBox, Color, Ray, RayCollision, Vector2, Vector3};

unsafe extern "C" {
  // Basic geometric 3D shapes drawing functions
  pub unsafe fn DrawCube(position: Vector3, width: f32, height: f32, length: f32, color: Color);
  pub unsafe fn DrawCubeV(position: Vector3, size: Vector3, color: Color);
  pub unsafe fn DrawCubeWires(
    position: Vector3,
    width: f32,
    height: f32,
    length: f32,
    color: Color,
  );
  pub unsafe fn DrawCubeWiresV(position: Vector3, size: Vector3, color: Color);
  pub unsafe fn DrawPlane(center_pos: Vector3, size: Vector2, color: Color);
  pub unsafe fn DrawRay(ray: Ray, color: Color);
  pub unsafe fn DrawGrid(slices: i32, spacing: f32);

  // Collision detection functions
  pub unsafe fn GetRayCollisionBox(ray: Ray, bounding_box: BoundingBox) -> RayCollision;
}
