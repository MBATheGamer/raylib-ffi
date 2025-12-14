unsafe extern "C" {
  pub fn rlPushMatrix();
  pub fn rlPopMatrix();
  pub fn rlRotatef(angle: f32, x: f32, y: f32, z: f32);
  pub fn rlTranslatef(x: f32, y: f32, z: f32);
}
