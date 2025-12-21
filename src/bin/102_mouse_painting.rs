use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, begin_texture_mode, clear_background, close_window, end_drawing,
    end_texture_mode,
    gesture::get_gesture_detected,
    get_screen_height, get_screen_width, init_window,
    keyboard::is_key_pressed,
    mouse::{
      get_mouse_position, get_mouse_wheel_move, get_mouse_x, get_mouse_y, is_mouse_button_down,
      is_mouse_button_pressed, is_mouse_button_released,
    },
    set_target_fps, window_should_close,
  },
  enums::{Gesture, KeyboardKey, MouseButton},
  shape::{
    check_collision_point_rec, draw_circle, draw_circle_lines, draw_line, draw_rectangle,
    draw_rectangle_lines, draw_rectangle_lines_ex, draw_rectangle_rec,
  },
  structs::{Color, Rectangle, Vector2},
  text::draw_text,
  texture::{
    draw_texture_rec, export_image, fade, image_flip_vertical, load_image_from_texture,
    load_render_texture, unload_image, unload_render_texture,
  },
};

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [textures] example - mouse painting",
  );

  let colors: Vec<Color> = vec![
    colors::RAYWHITE,
    colors::YELLOW,
    colors::GOLD,
    colors::ORANGE,
    colors::PINK,
    colors::RED,
    colors::MAROON,
    colors::GREEN,
    colors::LIME,
    colors::DARKGREEN,
    colors::SKYBLUE,
    colors::BLUE,
    colors::DARKBLUE,
    colors::PURPLE,
    colors::VIOLET,
    colors::DARKPURPLE,
    colors::BEIGE,
    colors::BROWN,
    colors::DARKBROWN,
    colors::LIGHTGRAY,
    colors::GRAY,
    colors::DARKGRAY,
    colors::BLACK,
  ];

  let mut colors_recs: Vec<Rectangle> = vec![];

  for i in 0..colors.len() {
    colors_recs.push(Rectangle {
      x: 10.0 + 30.0 * i as f32 + 2.0 * i as f32,
      y: 10.0,
      width: 30.0,
      height: 30.0,
    });
  }

  let mut color_selected = 0;
  let mut color_selected_prev = color_selected;
  let mut color_mouse_hover = 0;
  let mut brush_size = 20.0;
  let mut mouse_was_pressed = false;

  let btn_save_rec = Rectangle {
    x: 750.0,
    y: 10.0,
    width: 40.0,
    height: 30.0,
  };
  let mut show_save_message = false;
  let mut save_message_counter = 0;

  let target = load_render_texture(SCREEN_WIDTH, SCREEN_HEIGHT);

  begin_texture_mode(target);
  clear_background(colors[0]);
  end_texture_mode();

  set_target_fps(120);

  while !window_should_close() {
    let mouse_pos = get_mouse_position();

    if is_key_pressed(KeyboardKey::KeyRight) {
      color_selected += 1;
    } else if is_key_pressed(KeyboardKey::KeyLeft) {
      color_selected -= 1;
    }

    if color_selected >= colors.len() as i32 {
      color_selected = colors.len() as i32 - 1;
    } else if color_selected < 0 {
      color_selected = 0;
    }

    for i in 0..colors.len() {
      if check_collision_point_rec(mouse_pos, colors_recs[i]) {
        color_mouse_hover = i as i32;
        break;
      } else {
        color_mouse_hover = -1;
      }
    }

    if color_mouse_hover >= 0 && is_mouse_button_pressed(MouseButton::Left) {
      color_selected = color_mouse_hover;
      color_selected_prev = color_selected;
    }

    brush_size += get_mouse_wheel_move() * 5.0;
    if brush_size < 2.0 {
      brush_size = 2.0;
    }
    if brush_size > 50.0 {
      brush_size = 50.0;
    }

    if is_key_pressed(KeyboardKey::KeyC) {
      begin_texture_mode(target);
      clear_background(colors[0]);
      end_texture_mode();
    }

    if is_mouse_button_down(MouseButton::Left) || get_gesture_detected() == Gesture::Drag {
      begin_texture_mode(target);
      if mouse_pos.y > 50.0 {
        draw_circle(
          mouse_pos.x as i32,
          mouse_pos.y as i32,
          brush_size,
          colors[color_selected as usize],
        );
      }
      end_texture_mode();
    }

    if is_mouse_button_down(MouseButton::Right) {
      if !mouse_was_pressed {
        color_selected_prev = color_selected;
        color_selected = 0;
      }

      mouse_was_pressed = true;

      begin_texture_mode(target);
      if mouse_pos.y > 50.0 {
        draw_circle(
          mouse_pos.x as i32,
          mouse_pos.y as i32,
          brush_size,
          colors[0],
        );
      }
      end_texture_mode();
    } else if is_mouse_button_released(MouseButton::Right) && mouse_was_pressed {
      color_selected = color_selected_prev;
      mouse_was_pressed = false;
    }

    let btn_save_mouse_hover = check_collision_point_rec(mouse_pos, btn_save_rec);

    if btn_save_mouse_hover && is_mouse_button_released(MouseButton::Left)
      || is_key_pressed(KeyboardKey::KeyS)
    {
      let mut image = load_image_from_texture(target.texture);
      image_flip_vertical(&mut image);
      export_image(image, "my_amazing_texture_painting.png");
      unload_image(image);
      show_save_message = true;
    }

    if show_save_message {
      save_message_counter += 1;
      if save_message_counter > 240 {
        show_save_message = false;
        save_message_counter = 0;
      }
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    draw_texture_rec(
      target.texture,
      Rectangle {
        x: 0.0,
        y: 0.0,
        width: target.texture.width as f32,
        height: -target.texture.height as f32,
      },
      Vector2 { x: 0.0, y: 0.0 },
      colors::WHITE,
    );

    if mouse_pos.y > 50.0 {
      if is_mouse_button_down(MouseButton::Right) {
        draw_circle_lines(
          mouse_pos.x as i32,
          mouse_pos.y as i32,
          brush_size,
          colors::GRAY,
        );
      } else {
        draw_circle(
          get_mouse_x(),
          get_mouse_y(),
          brush_size,
          colors[color_selected as usize],
        );
      }
    }

    draw_rectangle(0, 0, get_screen_width(), 50, colors::RAYWHITE);
    draw_line(0, 50, get_screen_width(), 50, colors::LIGHTGRAY);

    for i in 0..colors.len() {
      draw_rectangle_rec(colors_recs[i], colors[i]);
    }
    draw_rectangle_lines(10, 10, 30, 30, colors::LIGHTGRAY);

    if color_mouse_hover >= 0 {
      draw_rectangle_rec(
        colors_recs[color_mouse_hover as usize],
        fade(colors::WHITE, 0.6),
      );
    }

    draw_rectangle_lines_ex(
      Rectangle {
        x: colors_recs[color_selected as usize].x - 2.0,
        y: colors_recs[color_selected as usize].y - 2.0,
        width: colors_recs[color_selected as usize].width + 4.0,
        height: colors_recs[color_selected as usize].height + 4.0,
      },
      2.0,
      colors::BLACK,
    );

    draw_rectangle_lines_ex(
      btn_save_rec,
      2.0,
      if btn_save_mouse_hover {
        colors::RED
      } else {
        colors::BLACK
      },
    );
    draw_text(
      "SAVE!",
      755,
      20,
      10,
      if btn_save_mouse_hover {
        colors::RED
      } else {
        colors::BLACK
      },
    );

    if show_save_message {
      draw_rectangle(
        0,
        0,
        get_screen_width(),
        get_screen_height(),
        fade(colors::RAYWHITE, 0.8),
      );
      draw_rectangle(0, 150, get_screen_width(), 80, colors::BLACK);
      draw_text("IMAGE SAVED!", 150, 180, 20, colors::RAYWHITE);
    }

    end_drawing();
  }

  unload_render_texture(target);

  close_window();
}
