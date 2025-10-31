use crate::{
  core::ffi::LoadVrStereoConfig,
  structs::{VrDeviceInfo, VrStereoConfig},
};

#[inline]
pub fn load_vr_stereo_config(device: VrDeviceInfo) -> VrStereoConfig {
  return unsafe { LoadVrStereoConfig(device) };
}
