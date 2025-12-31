use crate::{core::ffi::PlayAutomationEvent, structs::AutomationEvent};

#[inline]
pub fn play_automation_event(event: AutomationEvent) {
  unsafe { PlayAutomationEvent(event) };
}
