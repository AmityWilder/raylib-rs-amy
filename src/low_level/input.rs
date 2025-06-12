use super::*;
// Input-related functions: keyboard

/// Check if a key has been pressed once
#[inline]
pub fn is_key_pressed(
    key: sys::KeyboardKey,
) -> bool {
    unsafe {
        sys::IsKeyPressed(
            key as i32,
        )
    }
}

/// Check if a key has been pressed again
#[inline]
pub fn is_key_pressed_repeat(
    key: sys::KeyboardKey,
) -> bool {
    unsafe {
        sys::IsKeyPressedRepeat(
            key as i32,
        )
    }
}

/// Check if a key is being pressed
#[inline]
pub fn is_key_down(
    key: sys::KeyboardKey,
) -> bool {
    unsafe {
        sys::IsKeyDown(
            key as i32,
        )
    }
}

/// Check if a key has been released once
#[inline]
pub fn is_key_released(
    key: sys::KeyboardKey,
) -> bool {
    unsafe {
        sys::IsKeyReleased(
            key as i32,
        )
    }
}

/// Check if a key is NOT being pressed
#[inline]
pub fn is_key_up(
    key: sys::KeyboardKey,
) -> bool {
    unsafe {
        sys::IsKeyUp(
            key as i32,
        )
    }
}

/// Get key pressed (keycode), call it multiple times for keys queued, returns [`None`] when the queue is empty
#[inline]
pub fn get_key_pressed() -> Option<sys::KeyboardKey> {
    unsafe {
        let key = sys::GetKeyPressed();
        if key != 0 {
            Some(transmute(key))
        } else {
            None
        }
    }
}

/// Get char pressed (unicode), call it multiple times for chars queued, returns [`None`] when the queue is empty
#[inline]
pub fn get_char_pressed() -> Option<char> {
    unsafe {
        char::from_u32(sys::GetCharPressed().try_into().unwrap())
            .filter(|ch| *ch != '\0')
    }
}

/// Get name of a QWERTY key on the current keyboard layout (eg returns string 'q' for KEY_A on an AZERTY keyboard)
#[inline]
pub fn get_key_name(
    key: sys::KeyboardKey,
) -> Option<&'static CStr> {
    let ptr = unsafe {
        sys::GetKeyName(
            key as i32,
        )
    };
    if !ptr.is_null() {
        Some(unsafe {
            CStr::from_ptr(ptr)
        })
    } else {
        None
    }
}

/// Set a custom key to exit program (default is ESC)
#[inline]
pub fn set_exit_key(
    key: sys::KeyboardKey,
) {
    unsafe {
        sys::SetExitKey(
            key as i32,
        )
    }
}

// Input-related functions: gamepads

/// Check if a gamepad is available
#[inline]
pub fn is_gamepad_available(
    gamepad: i32,
) -> bool {
    unsafe {
        sys::IsGamepadAvailable(
            gamepad
        )
    }
}

/// Get gamepad internal name id
#[inline]
pub fn get_gamepad_name(
    gamepad: i32,
) -> Option<&'static CStr> {
    let ptr = unsafe {
        sys::GetGamepadName(
            gamepad,
        )
    };
    if !ptr.is_null() {
        Some(unsafe {
            CStr::from_ptr(ptr)
        })
    } else {
        None
    }
}

/// Check if a gamepad button has been pressed once
#[inline]
pub fn is_gamepad_button_pressed(
    gamepad: i32,
    button: sys::GamepadButton,
) -> bool {
    unsafe {
        sys::IsGamepadButtonPressed(
            gamepad,
            button as i32,
        )
    }
}

/// Check if a gamepad button is being pressed
#[inline]
pub fn is_gamepad_button_down(
    gamepad: i32,
    button: sys::GamepadButton,
) -> bool {
    unsafe {
        sys::IsGamepadButtonDown(
            gamepad,
            button as i32,
        )
    }
}

/// Check if a gamepad button has been released once
#[inline]
pub fn is_gamepad_button_released(
    gamepad: i32,
    button: sys::GamepadButton,
) -> bool {
    unsafe {
        sys::IsGamepadButtonReleased(
            gamepad,
            button as i32,
        )
    }
}

/// Check if a gamepad button is NOT being pressed
#[inline]
pub fn is_gamepad_button_up(
    gamepad: i32,
    button: sys::GamepadButton,
) -> bool {
    unsafe {
        sys::IsGamepadButtonUp(
            gamepad,
            button as i32,
        )
    }
}

/// Get the last gamepad button pressed
#[inline]
pub fn get_gamepad_button_pressed() -> sys::GamepadButton {
    unsafe {
        transmute(sys::GetGamepadButtonPressed())
    }
}

/// Get axis count for a gamepad
#[inline]
pub fn get_gamepad_axis_count(
    gamepad: i32,
) -> usize {
    unsafe {
        sys::GetGamepadAxisCount(
            gamepad,
        )
    }.try_into().unwrap()
}

/// Get movement value for a gamepad axis
#[inline]
pub fn get_gamepad_axis_movement(
    gamepad: i32,
    axis: sys::GamepadAxis,
) -> f32 {
    unsafe {
        sys::GetGamepadAxisMovement(
            gamepad,
            axis as i32,
        )
    }
}

/// Set internal gamepad mappings (SDL_GameControllerDB)
#[inline]
pub fn set_gamepad_mappings(
    mappings: Option<&CStr>,
) -> i32 {
    unsafe {
        sys::SetGamepadMappings(
            mappings.map_or_else(null, CStr::as_ptr),
        )
    }
}

/// Set gamepad vibration for both motors (duration in seconds)
#[inline]
pub fn set_gamepad_vibration(
    gamepad: i32,
    left_motor: f32,
    right_motor: f32,
    duration: f32,
) {
    unsafe {
        sys::SetGamepadVibration(
            gamepad,
            left_motor,
            right_motor,
            duration,
        );
    }
}
/// Set gamepad vibration for both motors
#[inline]
pub fn set_gamepad_vibration_time(
    gamepad: i32,
    left_motor: f32,
    right_motor: f32,
    duration: Duration,
) {
    set_gamepad_vibration(
        gamepad,
        left_motor,
        right_motor,
        duration.as_secs_f32(),
    );
}

// Input-related functions: mouse

/// Check if a mouse button has been pressed once
#[inline]
pub fn is_mouse_button_pressed(
    button: sys::MouseButton,
) -> bool {
    unsafe {
        sys::IsMouseButtonPressed(
            button as i32,
        )
    }
}

/// Check if a mouse button is being pressed
#[inline]
pub fn is_mouse_button_down(
    button: sys::MouseButton,
) -> bool {
    unsafe {
        sys::IsMouseButtonDown(
            button as i32,
        )
    }
}

/// Check if a mouse button has been released once
#[inline]
pub fn is_mouse_button_released(
    button: sys::MouseButton,
) -> bool {
    unsafe {
        sys::IsMouseButtonReleased(
            button as i32,
        )
    }
}

/// Check if a mouse button is NOT being pressed
#[inline]
pub fn is_mouse_button_up(
    button: sys::MouseButton,
) -> bool {
    unsafe {
        sys::IsMouseButtonUp(
            button as i32,
        )
    }
}

/// Get mouse position X
#[inline]
pub fn get_mouse_x() -> i32 {
    unsafe {
        sys::GetMouseX()
    }
}

/// Get mouse position Y
#[inline]
pub fn get_mouse_y() -> i32 {
    unsafe {
        sys::GetMouseY()
    }
}

/// Get mouse position XY
#[inline]
pub fn get_mouse_position() -> sys::Vector2 {
    unsafe {
        sys::GetMousePosition()
    }
}

/// Get mouse delta between frames
#[inline]
pub fn get_mouse_delta() -> sys::Vector2 {
    unsafe {
        sys::GetMouseDelta()
    }
}

/// Set mouse position XY
#[inline]
pub fn set_mouse_position(
    x: i32,
    y: i32,
) {
    unsafe {
        sys::SetMousePosition(
            x,
            y,
        );
    }
}

/// Set mouse offset
#[inline]
pub fn set_mouse_offset(
    offset_x: i32,
    offset_y: i32,
) {
    unsafe {
        sys::SetMouseOffset(
            offset_x,
            offset_y,
        );
    }
}

/// Set mouse scaling
#[inline]
pub fn set_mouse_scale(
    scale_x: f32,
    scale_y: f32,
) {
    unsafe {
        sys::SetMouseScale(
            scale_x,
            scale_y,
        );
    }
}

/// Get mouse wheel movement for X or Y, whichever is larger
#[inline]
pub fn get_mouse_wheel_move() -> f32 {
    unsafe {
        sys::GetMouseWheelMove()
    }
}

/// Get mouse wheel movement for both X and Y
#[inline]
pub fn get_mouse_wheel_move_v() -> sys::Vector2 {
    unsafe {
        sys::GetMouseWheelMoveV()
    }
}

/// Set mouse cursor
#[inline]
pub fn set_mouse_cursor(
    cursor: sys::MouseCursor,
) {
    unsafe {
        sys::SetMouseCursor(
            cursor as i32,
        );
    }
}

// Input-related functions: touch

/// Get touch position X for touch point 0 (relative to screen size)
#[inline]
pub fn get_touch_x() -> i32 {
    unsafe {
        sys::GetTouchX()
    }
}

/// Get touch position Y for touch point 0 (relative to screen size)
#[inline]
pub fn get_touch_y() -> i32 {
    unsafe {
        sys::GetTouchY()
    }
}

/// Get touch position XY for a touch point index (relative to screen size)
#[inline]
pub fn get_touch_position(
    index: usize,
) -> sys::Vector2 {
    unsafe {
        sys::GetTouchPosition(
            index.try_into().unwrap(),
        )
    }
}

/// Get touch point identifier for given index
#[inline]
pub fn get_touch_point_id(
    index: usize,
) -> Option<u32> {
    let id = unsafe {
        sys::GetTouchPointId(
            index.try_into().unwrap(),
        )
    };
    if id != -1 {
        Some(id.try_into().unwrap())
    } else {
        None
    }
}

/// Get number of touch points
#[inline]
pub fn get_touch_point_count() -> usize {
    unsafe {
        sys::GetTouchPointCount()
    }.try_into().unwrap()
}
