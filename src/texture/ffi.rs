use crate::structs::{Color, Font, Image, Rectangle, RenderTexture, Texture, Vector2};

unsafe extern "C" {
  // Image loading functions
  pub unsafe fn LoadImage(file_name: *const i8) -> Image;
  pub unsafe fn IsImageValid(image: Image) -> bool;
  pub unsafe fn UnloadImage(image: Image);

  // Image generation functions
  pub unsafe fn GenImageGradientLinear(
    width: i32,
    height: i32,
    direction: i32,
    start: Color,
    end: Color,
  ) -> Image;
  pub unsafe fn GenImageGradientRadial(
    width: i32,
    height: i32,
    density: f32,
    inner: Color,
    outer: Color,
  ) -> Image;
  pub unsafe fn GenImageGradientSquare(
    width: i32,
    height: i32,
    density: f32,
    inner: Color,
    outer: Color,
  ) -> Image;
  pub unsafe fn GenImageChecked(
    width: i32,
    height: i32,
    checks_x: i32,
    checks_y: i32,
    col1: Color,
    col2: Color,
  ) -> Image;
  pub unsafe fn GenImageWhiteNoise(width: i32, height: i32, factor: f32) -> Image;
  pub unsafe fn GenImagePerlinNoise(
    width: i32,
    height: i32,
    offset_x: i32,
    offset_y: i32,
    scale: f32,
  ) -> Image;
  pub unsafe fn GenImageCellular(width: i32, height: i32, tile_size: i32) -> Image;

  // Image manipulation functions
  pub unsafe fn ImageCopy(image: Image) -> Image;
  pub unsafe fn ImageFormat(image: *mut Image, new_format: i32);
  pub unsafe fn ImageCrop(image: *mut Image, crop: Rectangle);
  pub unsafe fn ImageBlurGaussian(image: *mut Image, blur_size: i32);
  pub unsafe fn ImageResize(image: *mut Image, new_width: i32, new_height: i32);
  pub unsafe fn ImageFlipVertical(image: *mut Image);
  pub unsafe fn ImageFlipHorizontal(image: *mut Image);
  pub unsafe fn ImageColorTint(image: *mut Image, color: Color);
  pub unsafe fn ImageColorInvert(image: *mut Image);
  pub unsafe fn ImageColorGrayscale(image: *mut Image);
  pub unsafe fn ImageColorContrast(image: *mut Image, contrast: f32);
  pub unsafe fn ImageColorBrightness(image: *mut Image, brightness: i32);
  pub unsafe fn LoadImageColors(image: Image) -> *mut Color;
  pub unsafe fn UnloadImageColors(colors: *mut Color);

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
  pub unsafe fn ImageDrawTextEx(
    dst: *mut Image,
    font: Font,
    text: *const i8,
    position: Vector2,
    font_size: f32,
    spacing: f32,
    tint: Color,
  );

  // Texture loading functions
  pub unsafe fn LoadTexture(filename: *const i8) -> Texture;
  pub unsafe fn LoadTextureFromImage(image: Image) -> Texture;
  pub unsafe fn LoadRenderTexture(width: i32, height: i32) -> RenderTexture;
  pub unsafe fn UnloadTexture(texture: Texture);
  pub unsafe fn UnloadRenderTexture(target: RenderTexture);
  pub unsafe fn UpdateTexture(texture: Texture, pixels: *const Color);

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
