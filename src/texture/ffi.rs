use crate::structs::{Color, Image, Rectangle, RenderTexture, Texture, Vector2};

unsafe extern "C" {
  // Image loading functions
  pub unsafe fn LoadImage(file_name: *const i8) -> Image;
  pub unsafe fn IsImageValid(image: Image) -> bool;
  pub unsafe fn UnloadImage(image: Image);

  // Image manipulation functions
  pub unsafe fn ImageCrop(image: *mut Image, crop: Rectangle);
  pub unsafe fn ImageResize(image: *mut Image, new_width: i32, new_height: i32);
  pub unsafe fn ImageFlipHorizontal(image: *mut Image);

  // Image drawing functions
  pub unsafe fn ImageDrawPixel(dst: *mut Image, pos_x: i32, pos_y: i32, color: Color);
  pub unsafe fn ImageDrawCircleLines(
    dst: *mut Image,
    center_x: i32,
    center_y: i32,
    radius: i32,
    color: Color,
  );
  pub unsafe fn ImageDrawRectangle(
    dst: *mut Image,
    pos_x: i32,
    pos_y: i32,
    width: i32,
    height: i32,
    color: Color,
  );
  pub unsafe fn ImageDraw(
    dst: *mut Image,
    src: Image,
    src_rec: Rectangle,
    dst_rec: Rectangle,
    tint: Color,
  );

  // Texture loading functions
  pub unsafe fn LoadTexture(filename: *const i8) -> Texture;
  pub unsafe fn LoadRenderTexture(width: i32, height: i32) -> RenderTexture;
  pub unsafe fn UnloadTexture(texture: Texture);
  pub unsafe fn UnloadRenderTexture(target: RenderTexture);

  // Texture configuration functions
  pub unsafe fn SetTextureFilter(texture: Texture, filter: i32);

  // Texture drawing functions
  pub unsafe fn DrawTexture(texture: Texture, pos_x: i32, pos_y: i32, tint: Color);
  pub unsafe fn DrawTextureRec(texture: Texture, source: Rectangle, position: Vector2, tint: Color);
  pub unsafe fn DrawTexturePro(
    texture: Texture,
    source: Rectangle,
    dest: Rectangle,
    origin: Vector2,
    rotation: f32,
    tint: Color,
  );

  // Color/pixel related functions
  pub unsafe fn Fade(color: Color, alpha: f32) -> Color;
  pub unsafe fn ColorAlpha(color: Color, alpha: f32) -> Color;
}
