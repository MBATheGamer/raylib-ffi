use crate::{
  enums::PixelFormat,
  structs::{Color, Image, Rectangle},
  texture::ffi::{
    ImageColorGrayscale, ImageColorTint, ImageCopy, ImageCrop, ImageFlipHorizontal, ImageFormat,
    ImageResize,
  },
};

#[inline]
pub fn image_copy(image: Image) -> Image {
  return unsafe { ImageCopy(image) };
}

#[inline]
pub fn image_format(image: &mut Image, new_format: PixelFormat) {
  unsafe { ImageFormat(image, new_format as i32) };
}

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

#[inline]
pub fn image_color_tint(image: &mut Image, color: Color) {
  unsafe { ImageColorTint(image, color) };
}

#[inline]
pub fn image_color_grayscale(image: &mut Image) {
  unsafe { ImageColorGrayscale(image) };
}
