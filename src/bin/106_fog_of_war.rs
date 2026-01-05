use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, begin_texture_mode, clear_background, close_window, end_drawing,
    end_texture_mode, get_random_value, init_window, keyboard::is_key_down, set_target_fps,
    window_should_close,
  },
  enums::{KeyboardKey, TextureFilter},
  shape::{draw_rectangle, draw_rectangle_lines, draw_rectangle_v},
  structs::{Rectangle, Vector2},
  text::draw_text,
  texture::{
    draw_texture_pro, fade, load_render_texture, set_texture_filter, unload_render_texture,
  },
};

const MAP_TILE_SIZE: u32 = 32;
const PLAYER_SIZE: u32 = 16;
const PLAYER_TILE_VISIBILITY: u32 = 2;

struct Map {
  tiles_x: u32,
  tiles_y: u32,
  tile_ids: Vec<u8>,
  tile_fog: Vec<u8>,
}

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [textures] example - fog of war",
  );

  let mut map = Map {
    tiles_x: 25,
    tiles_y: 15,
    tile_ids: vec![],
    tile_fog: vec![],
  };

  for _ in 0..(map.tiles_y * map.tiles_x) {
    map.tile_ids.push(get_random_value(0, 1) as u8);
    map.tile_fog.push(0);
  }

  let mut player_position = Vector2::new(180.0, 130.0);

  let fog_of_war = load_render_texture(map.tiles_x as i32, map.tiles_y as i32);
  set_texture_filter(fog_of_war.texture, TextureFilter::Bilinear);

  set_target_fps(60);

  while !window_should_close() {
    if is_key_down(KeyboardKey::KeyRight) {
      player_position.x += 5.0;
    }
    if is_key_down(KeyboardKey::KeyLeft) {
      player_position.x -= 5.0;
    }
    if is_key_down(KeyboardKey::KeyDown) {
      player_position.y += 5.0;
    }
    if is_key_down(KeyboardKey::KeyUp) {
      player_position.y -= 5.0;
    }

    if player_position.x <= 0.0 {
      player_position.x = 0.0;
    } else if (player_position.x + PLAYER_SIZE as f32) > (map.tiles_x * MAP_TILE_SIZE) as f32 {
      player_position.x = (map.tiles_x * MAP_TILE_SIZE - PLAYER_SIZE) as f32;
    }
    if player_position.y <= 0.0 {
      player_position.y = 0.0;
    } else if (player_position.y + PLAYER_SIZE as f32) > (map.tiles_y * MAP_TILE_SIZE) as f32 {
      player_position.y = (map.tiles_y * MAP_TILE_SIZE - PLAYER_SIZE) as f32;
    }

    for i in 0..(map.tiles_x * map.tiles_y) as usize {
      if map.tile_fog[i] == 1 {
        map.tile_fog[i] = 2;
      }
    }

    let player_tile_x = ((player_position.x as u32 + MAP_TILE_SIZE / 2) / MAP_TILE_SIZE) as i32;
    let player_tile_y = ((player_position.y as u32 + MAP_TILE_SIZE / 2) / MAP_TILE_SIZE) as i32;

    for y in (player_tile_y - PLAYER_TILE_VISIBILITY as i32)
      ..(player_tile_y + PLAYER_TILE_VISIBILITY as i32)
    {
      for x in (player_tile_x - PLAYER_TILE_VISIBILITY as i32)
        ..(player_tile_x + PLAYER_TILE_VISIBILITY as i32)
      {
        if x >= 0 && x < map.tiles_x as i32 && y >= 0 && y < map.tiles_y as i32 {
          map.tile_fog[(y * map.tiles_x as i32 + x) as usize] = 1;
        }
      }
    }

    begin_texture_mode(fog_of_war);
    clear_background(colors::BLANK);
    for y in 0..map.tiles_y {
      for x in 0..map.tiles_x {
        if map.tile_fog[(y * map.tiles_x + x) as usize] == 0 {
          draw_rectangle(x as i32, y as i32, 1, 1, colors::BLACK);
        } else if map.tile_fog[(y * map.tiles_x + x) as usize] == 2 {
          draw_rectangle(x as i32, y as i32, 1, 1, fade(colors::BLACK, 0.8));
        }
      }
    }
    end_texture_mode();

    begin_drawing();

    clear_background(colors::RAYWHITE);

    for y in 0..map.tiles_y {
      for x in 0..map.tiles_x {
        draw_rectangle(
          (x * MAP_TILE_SIZE) as i32,
          (y * MAP_TILE_SIZE) as i32,
          MAP_TILE_SIZE as i32,
          MAP_TILE_SIZE as i32,
          if map.tile_ids[(y * map.tiles_x + x) as usize] == 0 {
            colors::BLUE
          } else {
            fade(colors::BLUE, 0.9)
          },
        );
        draw_rectangle_lines(
          (x * MAP_TILE_SIZE) as i32,
          (y * MAP_TILE_SIZE) as i32,
          MAP_TILE_SIZE as i32,
          MAP_TILE_SIZE as i32,
          fade(colors::DARKBLUE, 0.5),
        );
      }
    }

    draw_rectangle_v(
      player_position,
      Vector2::new(PLAYER_SIZE as f32, PLAYER_SIZE as f32),
      colors::RED,
    );

    draw_texture_pro(
      fog_of_war.texture,
      Rectangle::new(
        0.0,
        0.0,
        fog_of_war.texture.width as f32,
        -fog_of_war.texture.height as f32,
      ),
      Rectangle::new(
        0.0,
        0.0,
        (map.tiles_x * MAP_TILE_SIZE) as f32,
        (map.tiles_y * MAP_TILE_SIZE) as f32,
      ),
      Vector2::new(0.0, 0.0),
      0.0,
      colors::WHITE,
    );

    draw_text(
      &format!("Current tile: [{},{}]", player_tile_x, player_tile_y),
      10,
      10,
      20,
      colors::RAYWHITE,
    );
    draw_text(
      "ARROW KEYS to move",
      10,
      SCREEN_HEIGHT - 25,
      20,
      colors::RAYWHITE,
    );

    end_drawing();
  }

  unload_render_texture(fog_of_war);

  close_window();
}
