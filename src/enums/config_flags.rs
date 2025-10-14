pub enum ConfigFlags {
  FlagVsyncHint = 0x00000040,         // Set to try enabling V-Sync on GPU
  FlagFullscreenMode = 0x00000002,    // Set to run program in fullscreen
  FlagWindowResizable = 0x00000004,   // Set to allow resizable window
  FlagWindowUndecorated = 0x00000008, // Set to disable window decoration (frame and buttons)
  FlagWindowHidden = 0x00000080,      // Set to hide window
  FlagWindowMinimized = 0x00000200,   // Set to minimize window (iconify)
  FlagWindowMaximized = 0x00000400,   // Set to maximize window (expanded to monitor)
  FlagWindowUnfocused = 0x00000800,   // Set to window non focused
  FlagWindowTopmost = 0x00001000,     // Set to window always on top
  FlagWindowAlwaysRun = 0x00000100,   // Set to allow windows running while minimized
  FlagWindowTransparent = 0x00000010, // Set to allow transparent framebuffer
  FlagWindowHighdpi = 0x00002000,     // Set to support HighDPI
  FlagWindowMousePassthrough = 0x00004000, // Set to support mouse passthrough, only supported when FLAG_WINDOW_UNDECORATED
  FlagBorderlessWindowedMode = 0x00008000, // Set to run program in borderless windowed mode
  FlagMSAA4xHint = 0x00000020,             // Set to try enabling MSAA 4X
  FlagInterlacedHint = 0x00010000,         // Set to try enabling interlaced video format (for V3D)
}
