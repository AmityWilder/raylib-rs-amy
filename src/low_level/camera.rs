use super::sys;

/// Update camera position for selected mode
#[inline]
pub fn update_camera(
    camera: &mut sys::Camera,
    mode: sys::CameraMode,
) {
    unsafe {
        sys::UpdateCamera(
            std::ptr::from_mut(
    camera),
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
pub fn update_camera_pro(
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
pub fn GetCameraForward(
    camera: *mut Camera,
) -> Vector3;

/// Returns the cameras up vector (normalized)
///
/// Note: The up vector might not be perpendicular to the forward vector
#[inline]
pub fn GetCameraUp(
    camera: *mut Camera,
) -> Vector3;

/// Returns the cameras right vector (normalized)
#[inline]
pub fn GetCameraRight(
    camera: *mut Camera,
) -> Vector3;

/// Moves the camera in its forward direction
#[inline]
pub fn CameraMoveForward(
    camera: *mut Camera,
    distance: f32,
    moveInWorldPlane: bool,
);

/// Moves the camera in its up direction
#[inline]
pub fn CameraMoveUp(
    camera: *mut Camera,
    distance: f32,
);

/// Moves the camera target in its current right direction
#[inline]
pub fn CameraMoveRight(
    camera: *mut Camera,
    distance: f32,
    moveInWorldPlane: bool,
);

/// Moves the camera position closer/farther to/from the camera target
#[inline]
pub fn CameraMoveToTarget(
    camera: *mut Camera,
    delta: f32,
);

/// Rotates the camera around its up vector
///
/// Yaw is "looking left and right"
///
/// If `rotateAroundTarget` is false, the camera rotates around its position
///
/// Note: angle must be provided in radians
#[inline]
pub fn CameraYaw(
    camera: *mut Camera,
    angle: f32,
    rotateAroundTarget: bool,
);

/// Rotates the camera around its right vector, pitch is "looking up and down"
///  - lockView prevents camera overrotation (aka "somersaults")
///  - rotateAroundTarget defines if rotation is around target or around its position
///  - rotateUp rotates the up direction as well (typically only usefull in CAMERA_FREE)
///
/// NOTE: angle must be provided in radians
#[inline]
pub fn CameraPitch(
    camera: *mut Camera,
    angle: f32,
    lockView: bool,
    rotateAroundTarget: bool,
    rotateUp: bool,
);

/// Rotates the camera around its forward vector
///
/// Roll is "turning your head sideways to the left or right"
///
/// Note: angle must be provided in radians
#[inline]
pub fn CameraRoll(
    camera: *mut Camera,
    angle: f32,
);

/// Returns the camera view matrix
#[inline]
pub fn GetCameraViewMatrix(
    camera: *mut Camera,
) -> Matrix;

/// Returns the camera projection matrix
#[inline]
pub fn GetCameraProjectionMatrix(
    camera: *mut Camera,
    aspect: f32,
) -> Matrix;
