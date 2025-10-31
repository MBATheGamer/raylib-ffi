use crate::{
  core::ffi::{LoadVrStereoConfig, UnloadVrStereoConfig},
  structs::{VrDeviceInfo, VrStereoConfig},
};

#[inline]
pub fn load_vr_stereo_config(device: VrDeviceInfo) -> VrStereoConfig {
  return unsafe { LoadVrStereoConfig(device) };
}

#[inline]
pub fn unload_vr_stereo_config(config: VrStereoConfig) {
  unsafe { UnloadVrStereoConfig(config) };
}
