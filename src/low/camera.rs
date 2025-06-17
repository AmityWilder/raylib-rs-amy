use super::sys;

/// Update camera position for selected mode
#[inline]
pub unsafe fn update_camera(
    camera: &mut sys::Camera,
    mode: sys::CameraMode,
) {
    unsafe {
        sys::UpdateCamera(
            std::ptr::from_mut(camera),
            mode as i32,
        );
    }
}

/// Update camera movement, movement/rotation values should be provided by user
///
/// **Required values:**
/// - `movement.x` - Move forward/backward
/// - `movement.y` - Move right/left
/// - `movement.z` - Move up/down
/// - `rotation.x` - yaw
/// - `rotation.y` - pitch
/// - `rotation.z` - roll
/// - `zoom` - Move towards target
#[inline]
pub unsafe fn update_camera_pro(
    camera: &mut sys::Camera,
    movement: sys::Vector3,
    rotation: sys::Vector3,
    zoom: f32,
) {
    unsafe {
        sys::UpdateCameraPro(
            camera,
            movement,
            rotation,
            zoom,
        );
    }
}

/// Returns the cameras forward vector (normalized)
#[inline]
pub unsafe fn get_camera_forward(
    camera: &sys::Camera,
) -> sys::Vector3 {
    unsafe {
        sys::GetCameraForward(
            std::ptr::from_ref(camera).cast_mut(),
        )
    }
}

/// Returns the cameras up vector (normalized)
///
/// Note: The up vector might not be perpendicular to the forward vector
#[inline]
pub unsafe fn get_camera_up(
    camera: &sys::Camera,
) -> sys::Vector3 {
    unsafe {
        sys::GetCameraUp(
            std::ptr::from_ref(camera).cast_mut(),
        )
    }
}

/// Returns the cameras right vector (normalized)
#[inline]
pub unsafe fn get_camera_right(
    camera: &sys::Camera,
) -> sys::Vector3 {
    unsafe {
        sys::GetCameraRight(
            std::ptr::from_ref(camera).cast_mut(),
        )
    }
}

/// Moves the camera in its forward direction
#[inline]
pub unsafe fn camera_move_forward(
    camera: &mut sys::Camera,
    distance: f32,
    move_in_world_plane: bool,
) {
    unsafe {
        sys::CameraMoveForward(
            camera,
            distance,
            move_in_world_plane,
        );
    }
}

/// Moves the camera in its up direction
#[inline]
pub unsafe fn camera_move_up(
    camera: &mut sys::Camera,
    distance: f32,
) {
    unsafe {
        sys::CameraMoveUp(
            camera,
            distance,
        );
    }
}

/// Moves the camera target in its current right direction
#[inline]
pub unsafe fn camera_move_right(
    camera: &mut sys::Camera,
    distance: f32,
    move_in_world_plane: bool,
) {
    unsafe {
        sys::CameraMoveRight(
            camera,
            distance,
            move_in_world_plane,
        );
    }
}

/// Moves the camera position closer/farther to/from the camera target
#[inline]
pub unsafe fn camera_move_to_target(
    camera: &mut sys::Camera,
    delta: f32,
) {
    unsafe {
        sys::CameraMoveToTarget(
            camera,
            delta,
        );
    }
}

/// Rotates the camera around its up vector
///
/// Yaw is "looking left and right"
///
/// If `rotateAroundTarget` is false, the camera rotates around its position
///
/// Note: angle must be provided in radians
#[inline]
pub unsafe fn camera_yaw(
    camera: &mut sys::Camera,
    angle: f32,
    rotate_around_target: bool,
) {
    unsafe {
        sys::CameraYaw(
            camera,
            angle,
            rotate_around_target,
        );
    }
}

/// Rotates the camera around its right vector, pitch is "looking up and down"
///  - lockView prevents camera overrotation (aka "somersaults")
///  - rotateAroundTarget defines if rotation is around target or around its position
///  - rotateUp rotates the up direction as well (typically only usefull in CAMERA_FREE)
///
/// NOTE: angle must be provided in radians
#[inline]
pub unsafe fn camera_pitch(
    camera: &mut sys::Camera,
    angle: f32,
    lock_view: bool,
    rotate_around_target: bool,
    rotate_up: bool,
) {
    unsafe {
        sys::CameraPitch(
            camera,
            angle,
            lock_view,
            rotate_around_target,
            rotate_up,
        )
    }
}

/// Rotates the camera around its forward vector
///
/// Roll is "turning your head sideways to the left or right"
///
/// Note: angle must be provided in radians
#[inline]
pub unsafe fn camera_roll(
    camera: &mut sys::Camera,
    angle: f32,
) {
    unsafe {
        sys::CameraRoll(
            camera,
            angle,
        )
    }
}

/// Returns the camera view matrix
#[inline]
pub unsafe fn get_camera_view_matrix(
    camera: &sys::Camera,
) -> sys::Matrix {
    unsafe {
        sys::GetCameraViewMatrix(
            std::ptr::from_ref(camera).cast_mut(),
        )
    }
}

/// Returns the camera projection matrix
#[inline]
pub unsafe fn get_camera_projection_matrix(
    camera: &sys::Camera,
    aspect: f32,
) -> sys::Matrix {
    unsafe {
        sys::GetCameraProjectionMatrix(
            std::ptr::from_ref(camera).cast_mut(),
            aspect,
        )
    }
}
