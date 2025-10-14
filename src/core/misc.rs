use crate::{core::ffi::SetConfigFlags, enums::ConfigFlags};

#[inline]
pub fn set_config_flags(flags: ConfigFlags) {
  unsafe {
    SetConfigFlags(flags as u32);
  };
}
