use crate::{core::ffi::SetConfigFlags, enums::ConfigFlags};

#[inline]
pub fn set_config_flags(flags: &[ConfigFlags]) {
  let mut sum = 0u32;
  for flag in flags {
    sum = *flag | sum;
  }

  unsafe { SetConfigFlags(sum) };
}
