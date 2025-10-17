#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum Gesture {
  None = 0,        // No gesture
  Tap = 1,         // Tap gesture
  DoubleTap = 2,   // Double tap gesture
  Hold = 4,        // Hold gesture
  Drag = 8,        // Drag gesture
  SwipeRight = 16, // Swipe right gesture
  SwipeLeft = 32,  // Swipe left gesture
  SwipeUp = 64,    // Swipe up gesture
  SwipeDown = 128, // Swipe down gesture
  PinchIn = 256,   // Pinch in gesture
  PinchOut = 512,  // Pinch out gesture
}
