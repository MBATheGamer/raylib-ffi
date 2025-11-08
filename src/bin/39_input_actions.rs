#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum ActionType {
  NoAction = 0,
  ActionUp,
  ActionDown,
  ActionLeft,
  ActionRight,
  ActionFire,
  MaxAction,
}

impl Default for ActionType {
  fn default() -> Self {
    Self::NoAction
  }
}

#[derive(Clone, Copy)]
struct ActionInput {
  key: KeyboardKey,
  button: GamepadButton,
}

static GAMEPAD_INDEX: i32 = 0;
static ACTION_INPUTS: [ActionInput; ActionType::MaxAction as usize] = [ActionInput {
  key: KeyboardKey::KeyNull,
  button: GamepadButton::Unknown,
};
  ActionType::MaxAction as usize];

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - input actions",
  );

  let action_set: i8 = 0;
  set_actions_default();

  let position = Vector2 { x: 400.0, y: 200.0 };
  let size = Vector2 { x: 40.0, y: 40.0 };

  set_target_fps(60);

  while !window_should_close() {
    {
      let gamepad_index = GAMEPAD_INDEX;
      *gamepad_index = 0;
    }
    if is_action_down(ActionType::ActionUp) {
      position.y -= 2.0;
    }
    if is_action_down(ActionType::ActionDown) {
      position.y += 2.0;
    }
    if is_action_down(ActionType::ActionLeft) {
      position.x -= 2.0;
    }
    if is_action_down(ActionType::ActionRight) {
      position.x += 2.0;
    }
    if is_action_pressed(ActionType::ActionFire) {
      position.x = (SCREEN_WIDTH as f32 - size.x) / 2.0;
      position.y = (SCREEN_HEIGHT as f32 - size.y) / 2.0;
    }

    if is_key_pressed(KeyboardKey::KeyTab) {
      action_set = !action_set;
      if action_set == 0 {
        set_actions_default();
      } else {
        set_actions_cursor();
      }
    }

    begin_drawing();

    clear_background(colors::GRAY);

    draw_rectangle_v(position, size, colors::RED);

    draw_text(
      if action_set == 0 {
        "Current input set: WASD (default)"
      } else {
        "Current input set: Cursor"
      },
      10,
      10,
      20,
      colors::WHITE,
    );
    draw_text(
      "Use TAB key to toggles Actions keyset",
      10,
      50,
      20,
      colors::GREEN,
    );

    end_drawing();
  }

  close_window();
}

fn is_action_pressed(action: ActionType) -> bool {
  let action_inputs = ACTION_INPUTS;
  let gamepad_index = GAMEPAD_INDEX;

  if action < ActionType::MaxAction {
    return is_key_pressed(action_inputs[action as usize].key)
      || is_gamepad_button_pressed(*gamepad_index, action_inputs[action as usize].button);
  }

  return false;
}

fn is_action_down(action: ActionType) -> bool {
  let result = false;
  let action_inputs = ACTION_INPUTS;
  let gamepad_index = GAMEPAD_INDEX;

  if action < ActionType::MaxAction {
    result = is_key_down(action_inputs[action as usize].key)
      || is_gamepad_button_down(*gamepad_index, action_inputs[action as usize].button);
  }

  return result;
}

fn set_actions_default() {
  let action_inputs = ACTION_INPUTS;

  action_inputs[ActionType::ActionUp as usize].key = KeyboardKey::KeyW;
  action_inputs[ActionType::ActionDown as usize].key = KeyboardKey::KeyS;
  action_inputs[ActionType::ActionLeft as usize].key = KeyboardKey::KeyA;
  action_inputs[ActionType::ActionRight as usize].key = KeyboardKey::KeyD;
  action_inputs[ActionType::ActionFire as usize].key = KeyboardKey::KeySpace;

  action_inputs[ActionType::ActionUp as usize].button = GamepadButton::LeftFaceUp;
  action_inputs[ActionType::ActionDown as usize].button = GamepadButton::LeftFaceDown;
  action_inputs[ActionType::ActionLeft as usize].button = GamepadButton::LeftFaceLeft;
  action_inputs[ActionType::ActionRight as usize].button = GamepadButton::LeftFaceRight;
  action_inputs[ActionType::ActionFire as usize].button = GamepadButton::RightFaceDown;
}

fn set_actions_cursor() {
  let action_inputs = ACTION_INPUTS;

  action_inputs[ActionType::ActionUp as usize].key = KeyboardKey::KeyUp;
  action_inputs[ActionType::ActionDown as usize].key = KeyboardKey::KeyDown;
  action_inputs[ActionType::ActionLeft as usize].key = KeyboardKey::KeyLeft;
  action_inputs[ActionType::ActionRight as usize].key = KeyboardKey::KeyRight;
  action_inputs[ActionType::ActionFire as usize].key = KeyboardKey::KeySpace;

  action_inputs[ActionType::ActionUp as usize].button = GamepadButton::RightFaceUp;
  action_inputs[ActionType::ActionDown as usize].button = GamepadButton::RightFaceDown;
  action_inputs[ActionType::ActionLeft as usize].button = GamepadButton::RightFaceLeft;
  action_inputs[ActionType::ActionRight as usize].button = GamepadButton::RightFaceRight;
  action_inputs[ActionType::ActionFire as usize].button = GamepadButton::LeftFaceDown;
}
