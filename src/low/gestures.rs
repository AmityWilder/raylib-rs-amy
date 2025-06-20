use super::*;

/// Enable only desired gestures to be detected
#[inline]
pub unsafe fn set_gestures_enabled(
    flags: sys::Gesture,
) {
    unsafe {
        sys::SetGesturesEnabled(
            #[allow(clippy::unnecessary_cast, reason = "bindgen generates bitfields as i32 on some systems and u32 on others")]
            { flags.0 as u32 },
        );
    }
}

/// Check if a gesture have been detected
#[inline]
pub unsafe fn is_gesture_detected(
    gesture: sys::Gesture,
) -> bool {
    unsafe {
        sys::IsGestureDetected(
            #[allow(clippy::unnecessary_cast, reason = "bindgen generates bitfields as i32 on some systems and u32 on others")]
            { gesture.0 as u32 },
        )
    }
}

/// Get latest detected gesture
#[inline]
pub unsafe fn get_gesture_detected() -> sys::Gesture {
    unsafe {
        sys::Gesture(
            #[allow(clippy::unnecessary_cast, reason = "bindgen generates bitfields as i32 on some systems and u32 on others")]
            { sys::GetGestureDetected() as i32 },
        )
    }
}

/// Get gesture hold time in seconds
///
/// NOTE: time is calculated on current gesture HOLD
#[inline]
pub unsafe fn get_gesture_hold_duration() -> f32 {
    unsafe {
        sys::GetGestureHoldDuration()
    }
}

/// Get gesture hold time
///
/// NOTE: time is calculated on current gesture HOLD
#[inline]
pub fn get_gesture_hold_time() -> Duration {
    Duration::from_secs_f32(unsafe { get_gesture_hold_duration() })
}

/// Get gesture drag vector (between initial touch point to current)
///
/// NOTE: drag vector is calculated on one touch points `TOUCH_ACTION_MOVE`
#[inline]
pub unsafe fn get_gesture_drag_vector() -> sys::Vector2 {
    unsafe {
        sys::GetGestureDragVector()
    }
}

/// Get gesture drag angle
///
/// NOTE: Angle in degrees, horizontal-right is 0, counterclockwise
///
/// NOTE: drag angle is calculated on one touch points `TOUCH_ACTION_UP`
#[inline]
pub unsafe fn get_gesture_drag_angle() -> f32 {
    unsafe {
        sys::GetGestureDragAngle()
    }
}

/// Get gesture pinch delta; distance between two pinch points
///
/// NOTE: Pinch distance is calculated on two touch points `TOUCH_ACTION_MOVE`
#[inline]
pub unsafe fn get_gesture_pinch_vector() -> sys::Vector2 {
    unsafe {
        sys::GetGesturePinchVector()
    }
}

/// Get gesture pinch angle
///
/// NOTE: Angle in degrees, horizontal-right is 0, counterclockwise
///
/// NOTE: pinch angle is calculated on two touch points `TOUCH_ACTION_MOVE`
#[inline]
pub unsafe fn get_gesture_pinch_angle() -> f32 {
    unsafe {
        sys::GetGesturePinchAngle()
    }
}

/// Process gesture event and translate it into gestures
#[inline]
pub unsafe fn process_gesture_event(
    event: sys::GestureEvent,
) {
    unsafe {
        sys::ProcessGestureEvent(
            event,
        );
    }
}

/// Update gestures detected (must be called every frame)
///
/// NOTE: Gestures are processed through system callbacks on touch events
#[inline]
pub unsafe fn update_gestures() {
    unsafe {
        sys::UpdateGestures();
    }
}
