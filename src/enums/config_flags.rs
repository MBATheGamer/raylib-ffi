use std::ops::BitOr;

#[derive(Clone, Copy)]
pub enum ConfigFlags {
  VsyncHint = 0x00000040,              // Set to try enabling V-Sync on GPU
  FullscreenMode = 0x00000002,         // Set to run program in fullscreen
  WindowResizable = 0x00000004,        // Set to allow resizable window
  WindowUndecorated = 0x00000008,      // Set to disable window decoration (frame and buttons)
  WindowHidden = 0x00000080,           // Set to hide window
  WindowMinimized = 0x00000200,        // Set to minimize window (iconify)
  WindowMaximized = 0x00000400,        // Set to maximize window (expanded to monitor)
  WindowUnfocused = 0x00000800,        // Set to window non focused
  WindowTopmost = 0x00001000,          // Set to window always on top
  WindowAlwaysRun = 0x00000100,        // Set to allow windows running while minimized
  WindowTransparent = 0x00000010,      // Set to allow transparent framebuffer
  WindowHighDPI = 0x00002000,          // Set to support HighDPI
  WindowMousePassthrough = 0x00004000, // Set to support mouse passthrough, only supported when FLAG_WINDOW_UNDECORATED
  BorderlessWindowedMode = 0x00008000, // Set to run program in borderless windowed mode
  MSAA4xHint = 0x00000020,             // Set to try enabling MSAA 4X
  InterlacedHint = 0x00010000,         // Set to try enabling interlaced video format (for V3D)
}

impl BitOr for ConfigFlags {
  type Output = u32;

  fn bitor(self, rhs: Self) -> u32 {
    return self as u32 | rhs as u32;
  }
}

impl BitOr<u32> for ConfigFlags {
  type Output = u32;

  fn bitor(self, rhs: u32) -> u32 {
    return self as u32 | rhs;
  }
}
