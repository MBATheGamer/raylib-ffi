pub enum GamepadButton {
  Unknown = 0,    // Unknown button, just for error checking
  LeftFaceUp,     // Gamepad left DPAD up button
  LeftFaceRight,  // Gamepad left DPAD right button
  LeftFaceDown,   // Gamepad left DPAD down button
  LeftFaceLeft,   // Gamepad left DPAD left button
  RightFaceUp,    // Gamepad right button up (i.e. PS3: Triangle, Xbox: Y)
  RightFaceRight, // Gamepad right button right (i.e. PS3: Circle, Xbox: B)
  RightFaceDown,  // Gamepad right button down (i.e. PS3: Cross, Xbox: A)
  RightFaceLeft,  // Gamepad right button left (i.e. PS3: Square, Xbox: X)
  LeftTrigger1,   // Gamepad top/back trigger left (first), it could be a trailing button
  LeftTrigger2,   // Gamepad top/back trigger left (second), it could be a trailing button
  RightTrigger1,  // Gamepad top/back trigger right (first), it could be a trailing button
  RightTrigger2,  // Gamepad top/back trigger right (second), it could be a trailing button
  MiddleLeft,     // Gamepad center buttons, left one (i.e. PS3: Select)
  Middle,         // Gamepad center buttons, middle one (i.e. PS3: PS, Xbox: XBOX)
  MiddleRight,    // Gamepad center buttons, right one (i.e. PS3: Start)
  LeftThumb,      // Gamepad joystick pressed button left
  RightThumb,     // Gamepad joystick pressed button right
}
