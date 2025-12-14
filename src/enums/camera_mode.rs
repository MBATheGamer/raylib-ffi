#[repr(i32)]
#[derive(Clone, Copy, PartialEq)]
pub enum CameraMode {
  Custom = 0,  // Camera custom, controlled by user (UpdateCamera() does nothing)
  Free,        // Camera free mode
  Orbital,     // Camera orbital, around target, zoom supported
  FirstPerson, // Camera first person
  ThirdPerson, // Camera third person
}
