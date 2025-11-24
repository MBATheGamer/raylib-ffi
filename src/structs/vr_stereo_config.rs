#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VrStereoConfig {
  pub projection: [super::Matrix; 2], // VR projection matrices (per eye)
  pub view_offset: [super::Matrix; 2], // VR view offset matrices (per eye)
  pub left_lens_center: [f32; 2],     // VR left lens center
  pub right_lens_center: [f32; 2],    // VR right lens center
  pub left_screen_center: [f32; 2],   // VR left screen center
  pub right_screen_center: [f32; 2],  // VR right screen center
  pub scale: [f32; 2],                // VR distortion scale
  pub scale_in: [f32; 2],             // VR distortion scale in
}
