use super::*;

/// Enable a set of gestures using flags
#[inline]
pub fn set_gestures_enabled(
    flags: sys::Gesture,
) {
    unsafe {
        sys::SetGesturesEnabled(
            i32::cast_unsigned(flags.0),
        );
    }
}

/// Check if a gesture have been detected
#[inline]
pub fn is_gesture_detected(
    gesture: sys::Gesture,
) -> bool {
    unsafe {
        sys::IsGestureDetected(
            i32::cast_unsigned(gesture.0),
        )
    }
}

/// Get latest detected gesture
#[inline]
pub fn get_gesture_detected() -> sys::Gesture {
    unsafe {
        sys::Gesture(sys::GetGestureDetected())
    }
}

/// Get gesture hold time in seconds
#[inline]
pub fn get_gesture_hold_duration() -> f32 {
    unsafe {
        sys::GetGestureHoldDuration()
    }
}

/// Get gesture hold time
#[inline]
pub fn get_gesture_hold_time() -> Duration {
    Duration::from_secs_f32(get_gesture_hold_duration())
}

/// Get gesture drag vector
#[inline]
pub fn get_gesture_drag_vector() -> sys::Vector2 {
    unsafe {
        sys::GetGestureDragVector()
    }
}

/// Get gesture drag angle
#[inline]
pub fn get_gesture_drag_angle() -> f32 {
    unsafe {
        sys::GetGestureDragAngle()
    }
}

/// Get gesture pinch delta
#[inline]
pub fn get_gesture_pinch_vector() -> sys::Vector2 {
    unsafe {
        sys::GetGesturePinchVector()
    }
}

/// Get gesture pinch angle
#[inline]
pub fn get_gesture_pinch_angle() -> f32 {
    unsafe {
        sys::GetGesturePinchAngle()
    }
}

pub fn ProcessGestureEvent(event: GestureEvent);
pub fn UpdateGestures();
