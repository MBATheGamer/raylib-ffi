use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, get_random_value, init_window,
    keyboard::{is_key_down, is_key_pressed},
    set_target_fps, window_should_close,
  },
  enums::KeyboardKey,
  shape::{draw_line, draw_rectangle, draw_rectangle_lines, draw_rectangle_rec},
  structs::{Color, Rectangle, Vector2},
  text::draw_text,
};

#[derive(Clone, Copy, Default, PartialEq)]
struct Point {
  x: i32,
  y: i32,
}

#[derive(Clone, Copy, Default, PartialEq)]
struct PlayerState {
  cell: Point,
  color: Color,
}

const MAX_UNDO_STATES: i32 = 26;

const GRID_CELL_SIZE: i32 = 24;
const MAX_GRID_CELLS_X: i32 = 30;
const MAX_GRID_CELLS_Y: i32 = 13;

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - undo redo",
  );

  let mut current_undo_index = 0;
  let mut first_undo_index = 0;
  let mut last_undo_index = 0;
  let mut undo_frame_counter = 0;
  let undo_info_pos = Vector2 { x: 110.0, y: 400.0 };

  let mut player = PlayerState {
    cell: Point { x: 10, y: 10 },
    color: colors::RED,
  };

  let mut states: Vec<PlayerState> = vec![];

  for _ in 0..MAX_UNDO_STATES {
    states.push(player);
  }

  let grid_position = Vector2 { x: 40.0, y: 60.0 };

  set_target_fps(60);

  while !window_should_close() {
    if is_key_pressed(KeyboardKey::KeyRight) {
      player.cell.x += 1;
    } else if is_key_pressed(KeyboardKey::KeyLeft) {
      player.cell.x -= 1;
    } else if is_key_pressed(KeyboardKey::KeyUp) {
      player.cell.y -= 1;
    } else if is_key_pressed(KeyboardKey::KeyDown) {
      player.cell.y += 1;
    }

    if player.cell.x < 0 {
      player.cell.x = 0;
    } else if player.cell.x >= MAX_GRID_CELLS_X {
      player.cell.x = MAX_GRID_CELLS_X - 1;
    }
    if player.cell.y < 0 {
      player.cell.y = 0;
    } else if player.cell.y >= MAX_GRID_CELLS_Y {
      player.cell.y = MAX_GRID_CELLS_Y - 1;
    }

    if is_key_pressed(KeyboardKey::KeySpace) {
      player.color.red = get_random_value(20, 255) as u8;
      player.color.green = get_random_value(20, 220) as u8;
      player.color.blue = get_random_value(20, 240) as u8;
    }

    undo_frame_counter += 1;

    if undo_frame_counter >= 2 {
      // if memcmp (&states[currentUndoIndex], &player, sizeof(PlayerState)) != 0 {
      if !states.contains(&player) {
        current_undo_index += 1;
        if current_undo_index >= MAX_UNDO_STATES {
          current_undo_index = 0;
        }
        if current_undo_index == first_undo_index {
          first_undo_index += 1;
        }
        if first_undo_index >= MAX_UNDO_STATES {
          first_undo_index = 0;
        }
        // memcpy(&states[currentUndoIndex], &player, sizeof(PlayerState));
        states[current_undo_index as usize] = player;
        last_undo_index = current_undo_index;
      }

      undo_frame_counter = 0;
    }

    if is_key_down(KeyboardKey::KeyLeftControl) && is_key_pressed(KeyboardKey::KeyZ) {
      if current_undo_index != first_undo_index {
        current_undo_index -= 1;
        if current_undo_index < 0 {
          current_undo_index = MAX_UNDO_STATES - 1;
        }
        // if (memcmp(&states[currentUndoIndex], &player, sizeof(PlayerState)) !=0) {
        if states[current_undo_index as usize] != player {
          // memcpy(&player, &states[currentUndoIndex], sizeof(PlayerState));
          player = states[current_undo_index as usize];
        }
      }
    }

    if is_key_down(KeyboardKey::KeyLeftControl) && is_key_pressed(KeyboardKey::KeyY) {
      if current_undo_index != last_undo_index {
        let mut next_undo_index = current_undo_index + 1;
        if next_undo_index >= MAX_UNDO_STATES {
          next_undo_index = 0;
        }

        if next_undo_index != first_undo_index {
          current_undo_index = next_undo_index;

          // if memcmp(&states[current_undo_index], &player, sizeof(PlayerState)) != 0 {
          if states[current_undo_index as usize] != player {
            // memcpy(&player, &states[current_undo_index], sizeof(PlayerState));
            player = states[current_undo_index as usize];
          }
        }
      }
    }

    begin_drawing();
    clear_background(colors::RAYWHITE);

    draw_text(
      "[ARROWS] MOVE PLAYER - [SPACE] CHANGE PLAYER COLOR",
      40,
      20,
      20,
      colors::DARKGRAY,
    );

    if last_undo_index > first_undo_index {
      for i in first_undo_index..current_undo_index {
        draw_rectangle_rec(
          Rectangle {
            x: grid_position.x + (states[i as usize].cell.x * GRID_CELL_SIZE) as f32,
            y: grid_position.y + (states[i as usize].cell.y * GRID_CELL_SIZE) as f32,
            width: GRID_CELL_SIZE as f32,
            height: GRID_CELL_SIZE as f32,
          },
          colors::LIGHTGRAY,
        );
      }
    } else if first_undo_index > last_undo_index {
      if current_undo_index < MAX_UNDO_STATES && current_undo_index > last_undo_index {
        for i in first_undo_index..current_undo_index {
          draw_rectangle_rec(
            Rectangle {
              x: grid_position.x + (states[i as usize].cell.x * GRID_CELL_SIZE) as f32,
              y: grid_position.y + (states[i as usize].cell.y * GRID_CELL_SIZE) as f32,
              width: GRID_CELL_SIZE as f32,
              height: GRID_CELL_SIZE as f32,
            },
            colors::LIGHTGRAY,
          );
        }
      } else {
        for i in first_undo_index..MAX_UNDO_STATES {
          draw_rectangle(
            grid_position.x as i32 + states[i as usize].cell.x * GRID_CELL_SIZE,
            grid_position.y as i32 + states[i as usize].cell.y * GRID_CELL_SIZE,
            GRID_CELL_SIZE,
            GRID_CELL_SIZE,
            colors::LIGHTGRAY,
          );
        }
        for i in 0..current_undo_index {
          draw_rectangle(
            grid_position.x as i32 + states[i as usize].cell.x * GRID_CELL_SIZE,
            grid_position.y as i32 + states[i as usize].cell.y * GRID_CELL_SIZE,
            GRID_CELL_SIZE,
            GRID_CELL_SIZE,
            colors::LIGHTGRAY,
          );
        }
      }
    }

    for y in 0..MAX_GRID_CELLS_Y + 1 {
      draw_line(
        grid_position.x as i32,
        grid_position.y as i32 + y * GRID_CELL_SIZE,
        grid_position.x as i32 + MAX_GRID_CELLS_X * GRID_CELL_SIZE,
        grid_position.y as i32 + y * GRID_CELL_SIZE,
        colors::GRAY,
      );
    }
    for x in 0..MAX_GRID_CELLS_X + 1 {
      draw_line(
        grid_position.x as i32 + x * GRID_CELL_SIZE,
        grid_position.y as i32,
        grid_position.x as i32 + x * GRID_CELL_SIZE,
        grid_position.y as i32 + MAX_GRID_CELLS_Y * GRID_CELL_SIZE,
        colors::GRAY,
      );
    }
    draw_rectangle(
      grid_position.x as i32 + player.cell.x * GRID_CELL_SIZE,
      grid_position.y as i32 + player.cell.y * GRID_CELL_SIZE,
      GRID_CELL_SIZE + 1,
      GRID_CELL_SIZE + 1,
      player.color,
    );

    draw_text(
      "UNDO STATES:",
      undo_info_pos.x as i32 - 85,
      undo_info_pos.y as i32 + 9,
      10,
      colors::DARKGRAY,
    );
    draw_undo_buffer(
      undo_info_pos,
      first_undo_index,
      last_undo_index,
      current_undo_index,
      24,
    );

    end_drawing();
  }

  // RL_FREE(states);

  close_window();
}

fn draw_undo_buffer(
  position: Vector2,
  first_undo_index: i32,
  last_undo_index: i32,
  current_undo_index: i32,
  slot_size: i32,
) {
  draw_rectangle(
    position.x as i32 + 8 + slot_size * current_undo_index,
    position.y as i32 - 10,
    8,
    8,
    colors::RED,
  );
  draw_rectangle_lines(
    position.x as i32 + 2 + slot_size * first_undo_index,
    position.y as i32 + 27,
    8,
    8,
    colors::BLACK,
  );
  draw_rectangle(
    position.x as i32 + 14 + slot_size * last_undo_index,
    position.y as i32 + 27,
    8,
    8,
    colors::BLACK,
  );

  for i in 0..MAX_UNDO_STATES {
    draw_rectangle(
      position.x as i32 + slot_size * i,
      position.y as i32,
      slot_size,
      slot_size,
      colors::LIGHTGRAY,
    );
    draw_rectangle_lines(
      position.x as i32 + slot_size * i,
      position.y as i32,
      slot_size,
      slot_size,
      colors::GRAY,
    );
  }

  if first_undo_index <= last_undo_index {
    for i in first_undo_index..last_undo_index + 1 {
      draw_rectangle(
        position.x as i32 + slot_size * i,
        position.y as i32,
        slot_size,
        slot_size,
        colors::SKYBLUE,
      );
      draw_rectangle_lines(
        position.x as i32 + slot_size * i,
        position.y as i32,
        slot_size,
        slot_size,
        colors::BLUE,
      );
    }
  } else if last_undo_index < first_undo_index {
    for i in first_undo_index..MAX_UNDO_STATES {
      draw_rectangle(
        position.x as i32 + slot_size * i,
        position.y as i32,
        slot_size,
        slot_size,
        colors::SKYBLUE,
      );
      draw_rectangle_lines(
        position.x as i32 + slot_size * i,
        position.y as i32,
        slot_size,
        slot_size,
        colors::BLUE,
      );
    }

    for i in 0..last_undo_index + 1 {
      draw_rectangle(
        position.x as i32 + slot_size * i,
        position.y as i32,
        slot_size,
        slot_size,
        colors::SKYBLUE,
      );
      draw_rectangle_lines(
        position.x as i32 + slot_size * i,
        position.y as i32,
        slot_size,
        slot_size,
        colors::BLUE,
      );
    }
  }

  if first_undo_index < current_undo_index {
    for i in first_undo_index..current_undo_index {
      draw_rectangle(
        position.x as i32 + slot_size * i,
        position.y as i32,
        slot_size,
        slot_size,
        colors::GREEN,
      );
      draw_rectangle_lines(
        position.x as i32 + slot_size * i,
        position.y as i32,
        slot_size,
        slot_size,
        colors::LIME,
      );
    }
  } else if current_undo_index < first_undo_index {
    for i in first_undo_index..MAX_UNDO_STATES {
      draw_rectangle(
        position.x as i32 + slot_size * i,
        position.y as i32,
        slot_size,
        slot_size,
        colors::GREEN,
      );
      draw_rectangle_lines(
        position.x as i32 + slot_size * i,
        position.y as i32,
        slot_size,
        slot_size,
        colors::LIME,
      );
    }

    for i in 0..current_undo_index {
      draw_rectangle(
        position.x as i32 + slot_size * i,
        position.y as i32,
        slot_size,
        slot_size,
        colors::GREEN,
      );
      draw_rectangle_lines(
        position.x as i32 + slot_size * i,
        position.y as i32,
        slot_size,
        slot_size,
        colors::LIME,
      );
    }
  }

  draw_rectangle(
    position.x as i32 + slot_size * current_undo_index,
    position.y as i32,
    slot_size,
    slot_size,
    colors::GOLD,
  );
  draw_rectangle_lines(
    position.x as i32 + slot_size * current_undo_index,
    position.y as i32,
    slot_size,
    slot_size,
    colors::ORANGE,
  );
}
