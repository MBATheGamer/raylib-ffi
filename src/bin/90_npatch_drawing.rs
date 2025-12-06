fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [textures] example - npatch drawing",
  );

  let n_patch_texture = load_texture("resources/ninepatch_button.png");

  let origin = Vector2::default();

  let dst_rec1 = Rectangle {
    x: 480.0,
    y: 160.0,
    width: 32.0,
    height: 32.0,
  };
  let dst_rec2 = Rectangle {
    x: 160.0,
    y: 160.0,
    width: 32.0,
    height: 32.0,
  };
  let dst_rec_h = Rectangle {
    x: 160.0,
    y: 93.0,
    width: 32.0,
    height: 32.0,
  };
  let dst_rec_v = Rectangle {
    x: 92.0,
    y: 160.0,
    width: 32.0,
    height: 32.0,
  };

  let nine_patch_info1 = NPatchInfo {
    source: Rectangle {
      x: 0.0,
      y: 0.0,
      width: 64.0,
      height: 64.0,
    },
    left: 12,
    top: 40,
    right: 12,
    bottom: 12,
    layout: NPatchLayout::NinePatch,
  };
  let nine_patch_info2 = NPatchInfo {
    source: Rectangle {
      x: 0.0,
      y: 128.0,
      width: 64.0,
      height: 64.0,
    },
    left: 16,
    top: 16,
    right: 16,
    bottom: 16,
    layout: NPatchLayout::NinePatch,
  };

  let h3_patch_info = NPatchInfo {
    source: Rectangle {
      x: 0.0,
      y: 64.0,
      width: 64.0,
      height: 64.0,
    },
    left: 8,
    top: 8,
    right: 8,
    bottom: 8,
    layout: NPatchLayout::ThreePatchHorizontal,
  };

  let v3_patch_info = NPatchInfo {
    source: Rectangle {
      x: 0.0,
      y: 192.0,
      width: 64.0,
      height: 64.0,
    },
    left: 6,
    top: 6,
    right: 6,
    bottom: 6,
    layout: NPatchLayout::ThreePatchVertical,
  };

  set_target_fps(60);

  while !window_should_close() {
    let mouse_position = get_mouse_position();

    dst_rec1.width = mouse_position.x - dst_rec1.x;
    dst_rec1.height = mouse_position.y - dst_rec1.y;
    dst_rec2.width = mouse_position.x - dst_rec2.x;
    dst_rec2.height = mouse_position.y - dst_rec2.y;
    dst_rec_h.width = mouse_position.x - dst_rec_h.x;
    dst_rec_v.height = mouse_position.y - dst_rec_v.y;

    if dst_rec1.width < 1.0 {
      dst_rec1.width = 1.0;
    }
    if dst_rec1.width > 300.0 {
      dst_rec1.width = 300.0;
    }
    if dst_rec1.height < 1.0 {
      dst_rec1.height = 1.0;
    }
    if dst_rec2.width < 1.0 {
      dst_rec2.width = 1.0;
    }
    if dst_rec2.width > 300.0 {
      dst_rec2.width = 300.0;
    }
    if dst_rec2.height < 1.0 {
      dst_rec2.height = 1.0;
    }
    if dst_rec_h.width < 1.0 {
      dst_rec_h.width = 1.0;
    }
    if dst_rec_v.height < 1.0 {
      dst_rec_v.height = 1.0;
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    draw_texture_n_patch(
      n_patch_texture,
      nine_patch_info2,
      dst_rec2,
      origin,
      0.0,
      colors::WHITE,
    );
    draw_texture_n_patch(
      n_patch_texture,
      nine_patch_info1,
      dst_rec1,
      origin,
      0.0,
      colors::WHITE,
    );
    draw_texture_n_patch(
      n_patch_texture,
      h3_patch_info,
      dst_rec_h,
      origin,
      0.0,
      colors::WHITE,
    );
    draw_texture_n_patch(
      n_patch_texture,
      v3_patch_info,
      dst_rec_v,
      origin,
      0.0,
      colors::WHITE,
    );

    draw_rectangle_lines(5, 88, 74, 266, colors::BLUE);
    draw_texture(n_patch_texture, 10, 93, colors::WHITE);
    draw_text("TEXTURE", 15, 360, 10, colors::DARKGRAY);

    draw_text(
      "Move the mouse to stretch or shrink the n-patches",
      10,
      20,
      20,
      colors::DARKGRAY,
    );

    end_drawing();
  }

  unload_texture(n_patch_texture);

  close_window();
}
