use crate::structs::{Color, Font, Image, NPatchInfo, Rectangle, RenderTexture, Texture, Vector2};

unsafe extern "C" {
  // Image loading functions
  pub fn LoadImage(file_name: *const i8) -> Image;
  pub fn LoadImageRaw(
    file_name: *const i8,
    width: i32,
    height: i32,
    format: i32,
    header_size: i32,
  ) -> Image;
  pub fn LoadImageAnim(file_name: *const i8, frames: *mut i32) -> Image;
  pub fn LoadImageFromTexture(texture: Texture) -> Image;
  pub fn IsImageValid(image: Image) -> bool;
  pub fn UnloadImage(image: Image);
  pub fn ExportImage(image: Image, file_name: *const i8) -> bool;

  // Image generation functions
  pub fn GenImageGradientLinear(
    width: i32,
    height: i32,
    direction: i32,
    start: Color,
    end: Color,
  ) -> Image;
  pub fn GenImageGradientRadial(
    width: i32,
    height: i32,
    density: f32,
    inner: Color,
    outer: Color,
  ) -> Image;
  pub fn GenImageGradientSquare(
    width: i32,
    height: i32,
    density: f32,
    inner: Color,
    outer: Color,
  ) -> Image;
  pub fn GenImageChecked(
    width: i32,
    height: i32,
    checks_x: i32,
    checks_y: i32,
    col1: Color,
    col2: Color,
  ) -> Image;
  pub fn GenImageWhiteNoise(width: i32, height: i32, factor: f32) -> Image;
  pub fn GenImagePerlinNoise(
    width: i32,
    height: i32,
    offset_x: i32,
    offset_y: i32,
    scale: f32,
  ) -> Image;
  pub fn GenImageCellular(width: i32, height: i32, tile_size: i32) -> Image;

  // Image manipulation functions
  pub fn ImageCopy(image: Image) -> Image;
  pub fn ImageFromChannel(image: Image, selected_channel: i32) -> Image;
  pub fn ImageFormat(image: *mut Image, new_format: i32);
  pub fn ImageCrop(image: *mut Image, crop: Rectangle);
  pub fn ImageBlurGaussian(image: *mut Image, blur_size: i32);
  pub fn ImageKernelConvolution(image: *mut Image, kernel: *const f32, kernel_size: i32);
  pub fn ImageResize(image: *mut Image, new_width: i32, new_height: i32);
  pub fn ImageFlipVertical(image: *mut Image);
  pub fn ImageFlipHorizontal(image: *mut Image);
  pub fn ImageColorTint(image: *mut Image, color: Color);
  pub fn ImageColorInvert(image: *mut Image);
  pub fn ImageColorGrayscale(image: *mut Image);
  pub fn ImageColorContrast(image: *mut Image, contrast: f32);
  pub fn ImageColorBrightness(image: *mut Image, brightness: i32);
  pub fn LoadImageColors(image: Image) -> *mut Color;
  pub fn UnloadImageColors(colors: *mut Color);

  // Image drawing functions
  pub fn ImageDrawPixel(dst: *mut Image, pos_x: i32, pos_y: i32, color: Color);
  pub fn ImageDrawCircleLines(
    dst: *mut Image,
    center_x: i32,
    center_y: i32,
    radius: i32,
    color: Color,
  );
  pub fn ImageDrawRectangle(
    dst: *mut Image,
    pos_x: i32,
    pos_y: i32,
    width: i32,
    height: i32,
    color: Color,
  );
  pub fn ImageDraw(
    dst: *mut Image,
    src: Image,
    src_rec: Rectangle,
    dst_rec: Rectangle,
    tint: Color,
  );
  pub fn ImageDrawTextEx(
    dst: *mut Image,
    font: Font,
    text: *const i8,
    position: Vector2,
    font_size: f32,
    spacing: f32,
    tint: Color,
  );

  // Texture loading functions
  pub fn LoadTexture(filename: *const i8) -> Texture;
  pub fn LoadTextureFromImage(image: Image) -> Texture;
  pub fn LoadRenderTexture(width: i32, height: i32) -> RenderTexture;
  pub fn UnloadTexture(texture: Texture);
  pub fn UnloadRenderTexture(target: RenderTexture);
  pub fn UpdateTexture(texture: Texture, pixels: *const Color);

  // Texture configuration functions
  pub fn SetTextureFilter(texture: Texture, filter: i32);

  // Texture drawing functions
  pub fn DrawTexture(texture: Texture, pos_x: i32, pos_y: i32, tint: Color);
  pub fn DrawTextureV(texture: Texture, position: Vector2, tint: Color);
  pub fn DrawTextureEx(texture: Texture, position: Vector2, rotation: f32, scale: f32, tint: Color);
  pub fn DrawTextureRec(texture: Texture, source: Rectangle, position: Vector2, tint: Color);
  pub fn DrawTexturePro(
    texture: Texture,
    source: Rectangle,
    dest: Rectangle,
    origin: Vector2,
    rotation: f32,
    tint: Color,
  );
  pub fn DrawTextureNPatch(
    texture: Texture,
    n_patch_info: NPatchInfo,
    dest: Rectangle,
    origin: Vector2,
    rotation: f32,
    tint: Color,
  );

  // Color/pixel related functions
  pub fn Fade(color: Color, alpha: f32) -> Color;
  pub fn ColorFromHSV(hue: f32, saturation: f32, value: f32) -> Color;
  pub fn ColorAlpha(color: Color, alpha: f32) -> Color;
  pub fn ColorLerp(color1: Color, color2: Color, factor: f32) -> Color;
  pub fn GetColor(hex_value: u32) -> Color;
}
