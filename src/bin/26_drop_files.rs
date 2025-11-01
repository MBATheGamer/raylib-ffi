use std::ffi::CStr;

use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, init_window, is_file_dropped,
    load_dropped_files, set_target_fps, unload_dropped_files, window_should_close,
  },
  shape::draw_rectangle,
  text::draw_text,
  texture::fade,
};

fn main() {
  const MAX_FILEPATH_RECORDED: i32 = 4096;

  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - drop files",
  );

  let mut file_path_counter = 0;
  let mut file_paths: Vec<String> = vec![];

  for _ in 0..MAX_FILEPATH_RECORDED {
    file_paths.push(String::new());
  }

  set_target_fps(60);

  while !window_should_close() {
    if is_file_dropped() {
      let dropped_files = load_dropped_files();

      let dropped_paths =
        unsafe { std::slice::from_raw_parts(dropped_files.paths, dropped_files.count as usize) };

      let offset = file_path_counter as usize;
      for (i, &ptr) in dropped_paths.iter().enumerate() {
        if file_path_counter < (MAX_FILEPATH_RECORDED - 1) {
          let path = unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() };
          file_paths[i + offset] = path;
          file_path_counter += 1;
        }
      }

      unload_dropped_files(dropped_files);
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    if file_path_counter == 0 {
      draw_text(
        "Drop your files to this window!",
        100,
        40,
        20,
        colors::DARKGRAY,
      );
    } else {
      draw_text("Dropped files:", 100, 40, 20, colors::DARKGRAY);

      for i in 0..file_path_counter {
        if i % 2 == 0 {
          draw_rectangle(
            0,
            85 + 40 * i,
            SCREEN_WIDTH,
            40,
            fade(colors::LIGHTGRAY, 0.5),
          );
        } else {
          draw_rectangle(
            0,
            85 + 40 * i,
            SCREEN_WIDTH,
            40,
            fade(colors::LIGHTGRAY, 0.3),
          );
        }
        draw_text(
          &file_paths[i as usize],
          120,
          100 + 40 * i,
          10,
          colors::BLACK,
        );
      }

      draw_text(
        "Drop new files...",
        100,
        110 + 40 * file_path_counter,
        20,
        colors::DARKGRAY,
      );
    }

    end_drawing();
  }

  close_window();
}
