use std::ffi::CString;

use crate::{
  core::ffi::{LoadAutomationEventList, PlayAutomationEvent, UnloadAutomationEventList},
  structs::{AutomationEvent, AutomationEventList},
};

#[inline]
pub fn load_automation_event_list(file_name: &str) -> AutomationEventList {
  let file_name = CString::new(file_name).expect("[Error] Expecting a valid file name");

  return unsafe { LoadAutomationEventList(file_name.as_ptr()) };
}

#[inline]
pub fn unload_automation_event_list(list: AutomationEventList) {
  unsafe { UnloadAutomationEventList(list) };
}

#[inline]
pub fn play_automation_event(event: AutomationEvent) {
  unsafe { PlayAutomationEvent(event) };
}
