use crate::structs::AudioStream;

pub struct Sound {
  stream: AudioStream, // Audio stream
  frame_count: u32,    // Total number of frames (considering channels)
}
