use crate::{
  structs::{Image, Rectangle},
  texture::ffi::ImageCrop,
};

#[inline]
pub fn image_crop(image: &mut Image, crop: Rectangle) {
  unsafe { ImageCrop(image, crop) };
}
