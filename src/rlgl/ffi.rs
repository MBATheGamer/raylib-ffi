unsafe extern "C" {
  pub unsafe fn rlPushMatrix();
  pub unsafe fn rlRotatef(angle: f32, x: f32, y: f32, z: f32);
  pub unsafe fn rlTranslatef(x: f32, y: f32, z: f32);
}
