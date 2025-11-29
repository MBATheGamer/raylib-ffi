use crate::{
  structs::{Image, Rectangle},
  texture::ffi::{ImageCrop, ImageFlipHorizontal, ImageResize},
};

#[inline]
pub fn image_crop(image: &mut Image, crop: Rectangle) {
  unsafe { ImageCrop(image, crop) };
}

#[inline]
pub fn image_resize(image: *mut Image, new_width: i32, new_height: i32) {
  unsafe { ImageResize(image, new_width, new_height) };
}

#[inline]
pub fn image_flip_horizontal(image: &mut Image) {
  unsafe { ImageFlipHorizontal(image) }
}
