use std::ffi::CString;

use crate::{
  core::ffi::{
    ExportAutomationEventList, LoadAutomationEventList, PlayAutomationEvent,
    SetAutomationEventBaseFrame, SetAutomationEventList, StartAutomationEventRecording,
    UnloadAutomationEventList,
  },
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
pub fn export_automation_event_list(list: AutomationEventList, file_name: &str) -> bool {
  let file_name = CString::new(file_name).expect("[Error] Expecting a valid file name");

  return unsafe { ExportAutomationEventList(list, file_name.as_ptr()) };
}

#[inline]
pub fn set_automation_event_list(list: &mut AutomationEventList) {
  unsafe { SetAutomationEventList(list) };
}

#[inline]
pub fn set_automation_event_base_frame(frame: i32) {
  unsafe { SetAutomationEventBaseFrame(frame) };
}

#[inline]
pub fn start_automation_event_recording() {
  unsafe { StartAutomationEventRecording() };
}

#[inline]
pub fn play_automation_event(event: AutomationEvent) {
  unsafe { PlayAutomationEvent(event) };
}
