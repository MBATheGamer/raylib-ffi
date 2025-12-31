#[repr(C)]
#[derive(Clone, Copy)]
pub struct AutomationEvent {
  pub frame: u32,       // Event frame
  pub event_type: u32,  // Event type (AutomationEventType)
  pub params: [i32; 4], // Event parameters (if required)
}
