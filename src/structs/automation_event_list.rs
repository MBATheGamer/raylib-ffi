use crate::structs::AutomationEvent;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AutomationEventList {
  pub capacity: u32,            // Events max entries (MAX_AUTOMATION_EVENTS)
  pub count: u32,               // Events entries count
  events: *mut AutomationEvent, // Events entries
}
