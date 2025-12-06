use crate::{enums::NPatchLayout, structs::Rectangle};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NPatchInfo {
  pub source: Rectangle,    // Texture source rectangle
  pub left: i32,            // Left border offset
  pub top: i32,             // Top border offset
  pub right: i32,           // Right border offset
  pub bottom: i32,          // Bottom border offset
  pub layout: NPatchLayout, // Layout of the n-patch: 3x3, 1x3 or 3x1
}
