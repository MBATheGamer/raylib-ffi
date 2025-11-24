#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VrDeviceInfo {
  pub h_resolution: i32,                // Horizontal resolution in pixels
  pub v_resolution: i32,                // Vertical resolution in pixels
  pub h_screen_size: f32,               // Horizontal size in meters
  pub v_screen_size: f32,               // Vertical size in meters
  pub eye_to_screen_distance: f32,      // Distance between eye and display in meters
  pub lens_separation_distance: f32,    // Lens separation distance in meters
  pub interpupillary_distance: f32,     // IPD (distance between pupils) in meters
  pub lens_distortion_values: [f32; 4], // Lens distortion constant parameters
  pub chroma_ab_correction: [f32; 4],   // Chromatic aberration correction parameters
}
