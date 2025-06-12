use super::*;

/// Update camera position for selected mode
#[inline]
pub fn update_camera(
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

/// Update camera movement/rotation
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

pub fn GetCameraForward(camera: *mut Camera) -> Vector3;
pub fn GetCameraUp(camera: *mut Camera) -> Vector3;
pub fn GetCameraRight(camera: *mut Camera) -> Vector3;
pub fn CameraMoveForward(
    camera: *mut Camera,
    distance: ::std::os::raw::c_float,
    moveInWorldPlane: bool,
);
pub fn CameraMoveUp(camera: *mut Camera, distance: ::std::os::raw::c_float);
pub fn CameraMoveRight(
    camera: *mut Camera,
    distance: ::std::os::raw::c_float,
    moveInWorldPlane: bool,
);
pub fn CameraMoveToTarget(camera: *mut Camera, delta: ::std::os::raw::c_float);
pub fn CameraYaw(camera: *mut Camera, angle: ::std::os::raw::c_float, rotateAroundTarget: bool);
pub fn CameraPitch(
    camera: *mut Camera,
    angle: ::std::os::raw::c_float,
    lockView: bool,
    rotateAroundTarget: bool,
    rotateUp: bool,
);
pub fn CameraRoll(camera: *mut Camera, angle: ::std::os::raw::c_float);
pub fn GetCameraViewMatrix(camera: *mut Camera) -> Matrix;
pub fn GetCameraProjectionMatrix(
    camera: *mut Camera,
    aspect: ::std::os::raw::c_float,
) -> Matrix;
