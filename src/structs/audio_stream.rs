#[repr(C)]
struct AudioBuffer;

#[repr(C)]
struct AudioProcessor;

#[repr(C)]
pub struct AudioStream {
  buffer: *mut AudioBuffer, // Pointer to internal data used by the audio system
  processor: *mut AudioProcessor, // Pointer to internal data processor, useful for audio effects

  sample_rate: u32, // Frequency (samples per second)
  sample_size: u32, // Bit depth (bits per sample): 8, 16, 32 (24 not supported)
  channels: u32,    // Number of channels (1-mono, 2-stereo, ...)
}
