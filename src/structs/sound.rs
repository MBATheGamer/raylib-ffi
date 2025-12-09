use crate::structs::AudioStream;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Sound {
  stream: AudioStream, // Audio stream
  frame_count: u32,    // Total number of frames (considering channels)
}
