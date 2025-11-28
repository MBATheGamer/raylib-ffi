use crate::{
  structs::{Image, Rectangle},
  texture::ffi::{ImageCrop, ImageFlipHorizontal},
};

#[inline]
pub fn image_crop(image: &mut Image, crop: Rectangle) {
  unsafe { ImageCrop(image, crop) };
}

#[inline]
pub fn image_flip_horizontal(image: &mut Image) {
  unsafe { ImageFlipHorizontal(image) }
}
