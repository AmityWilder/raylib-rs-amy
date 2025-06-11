use std::ffi::{CStr, CString, NulError};
use std::marker::PhantomData;
use std::mem::{ManuallyDrop, MaybeUninit};
use std::num::NonZero;
use std::ops::{Deref, DerefMut};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr::{null, NonNull};
use std::time::Duration;

/// Direct bindings to Raylib C
pub mod sys;

// Window-related functions

/// Initialize window and OpenGL context
#[inline]
pub fn init_window(
    width: u32,
    height: u32,
    title: &CStr,
) {
    unsafe {
        sys::InitWindow(
            width.try_into().unwrap(),
            height.try_into().unwrap(),
            title.as_ptr(),
        );
    }
}

/// Close window and unload OpenGL context
#[inline]
pub fn close_window() {
    unsafe {
        sys::CloseWindow();
    }
}

/// Check if application should close ([`sys::KeyboardKey::KEY_ESCAPE`] pressed or windows close icon clicked)
#[inline]
pub fn window_should_close() -> bool {
    unsafe {
        sys::WindowShouldClose()
    }
}

/// Check if window has been initialized successfully
#[inline]
pub fn is_window_ready() -> bool {
    unsafe {
        sys::IsWindowReady()
    }
}

/// Check if window is currently fullscreen
#[inline]
pub fn is_window_fullscreen() -> bool {
    unsafe {
        sys::IsWindowFullscreen()
    }
}

/// Check if window is currently hidden
#[inline]
pub fn is_window_hidden() -> bool {
    unsafe {
        sys::IsWindowHidden()
    }
}

/// Check if window is currently minimized
#[inline]
pub fn is_window_minimized() -> bool {
    unsafe {
        sys::IsWindowMinimized()
    }
}

/// Check if window is currently maximized
#[inline]
pub fn is_window_maximized() -> bool {
    unsafe {
        sys::IsWindowMaximized()
    }
}

/// Check if window is currently focused
#[inline]
pub fn is_window_focused() -> bool {
    unsafe {
        sys::IsWindowFocused()
    }
}

/// Check if window has been resized last frame
#[inline]
pub fn is_window_resized() -> bool {
    unsafe {
        sys::IsWindowResized()
    }
}

/// Check if one specific window flag is enabled
#[inline]
pub fn is_window_state(
    flag: sys::ConfigFlags,
) -> bool {
    unsafe {
        sys::IsWindowState(
            flag.0.try_into().unwrap(),
        )
    }
}

/// Set window configuration state using flags
#[inline]
pub fn set_window_state(
    flags: sys::ConfigFlags,
) {
    unsafe {
        sys::SetWindowState(
            flags.0.try_into().unwrap(),
        );
    }
}

/// Clear window configuration state flags
#[inline]
pub fn clear_window_state(
    flags: sys::ConfigFlags,
) {
    unsafe {
        sys::ClearWindowState(
            flags.0.try_into().unwrap(),
        );
    }
}

/// Toggle window state: fullscreen/windowed, resizes monitor to match window resolution
#[inline]
pub fn toggle_fullscreen() {
    unsafe {
        sys::ToggleFullscreen();
    }
}

/// Toggle window state: borderless windowed, resizes window to match monitor resolution
#[inline]
pub fn toggle_borderless_windowed() {
    unsafe {
        sys::ToggleBorderlessWindowed();
    }
}

/// Set window state: maximized, if resizable
#[inline]
pub fn maximize_window() {
    unsafe {
        sys::MaximizeWindow();
    }
}

/// Set window state: minimized, if resizable
#[inline]
pub fn minimize_window() {
    unsafe {
        sys::MinimizeWindow();
    }
}

/// Restore window from being minimized/maximized
#[inline]
pub fn restore_window() {
    unsafe {
        sys::RestoreWindow();
    }
}

/// Set icon for window (single image, RGBA 32bit)
#[inline]
pub fn set_window_icon(
    image: sys::Image,
) {
    unsafe {
        sys::SetWindowIcon(
            image,
        );
    }
}

/// Set icon for window (multiple images, RGBA 32bit)
#[inline]
pub fn set_window_icons(
    images: &[sys::Image],
) {
    unsafe {
        sys::SetWindowIcons(
            // Casting to mut is safe because images are never modified.
            // Mutable pointers are just the default in C.
            images.as_ptr().cast_mut(),
            images.len().try_into().unwrap(),
        );
    }
}

/// Set title for window
#[inline]
pub fn set_window_title(
    title: &CStr,
) {
    unsafe {
        sys::SetWindowTitle(
            title.as_ptr(),
        );
    }
}

/// Set window position on screen
#[inline]
pub fn set_window_position(
    x: i32,
    y: i32,
) {
    unsafe {
        sys::SetWindowPosition(
            x.try_into().unwrap(),
            y.try_into().unwrap(),
        );
    }
}

/// Set monitor for the current window
#[inline]
pub fn set_window_monitor(
    monitor: usize,
) {
    unsafe {
        sys::SetWindowMonitor(
            monitor.try_into().unwrap(),
        );
    }
}

/// Set window minimum dimensions (for [`sys::ConfigFlags::FLAG_WINDOW_RESIZABLE`])
#[inline]
pub fn set_window_min_size(
    width: u32,
    height: u32,
) {
    unsafe {
        sys::SetWindowMinSize(
            width.try_into().unwrap(),
            height.try_into().unwrap(),
        );
    }
}

/// Set window maximum dimensions (for [`sys::ConfigFlags::FLAG_WINDOW_RESIZABLE`])
#[inline]
pub fn set_window_max_size(
    width: u32,
    height: u32,
) {
    unsafe {
        sys::SetWindowMaxSize(
            width.try_into().unwrap(),
            height.try_into().unwrap(),
        );
    }
}

/// Set window dimensions
#[inline]
pub fn set_window_size(
    width: u32,
    height: u32,
) {
    unsafe {
        sys::SetWindowSize(
            width.try_into().unwrap(),
            height.try_into().unwrap(),
        );
    }
}

/// Set window opacity [0.0..=1.0]
#[inline]
pub fn set_window_opacity(
    opacity: f32,
) {
    assert!(0.0 <= opacity && opacity <= 1.0,
        "opacity out of range\n expect: [0.0..=1.0]\n actual: {opacity}",
    );
    unsafe {
        sys::SetWindowOpacity(
            opacity.try_into().unwrap(),
        );
    }
}

/// Set window focused
#[inline]
pub fn set_window_focused() {
    unsafe {
        sys::SetWindowFocused();
    }
}

/// An opaque, platform-specific window handle
pub struct NativeWindowHandle<'a> {
    ptr: NonNull<c_void>,
    _marker: PhantomData<&'a [c_void]>,
}

impl<'a> NativeWindowHandle<'a> {
    #[inline]
    pub const fn as_ptr(&self) -> *const c_void {
        self.ptr.as_ptr().cast_const()
    }

    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut c_void {
        self.ptr.as_ptr()
    }
}

/// Get native window handle
#[inline]
pub fn get_window_handle<'a>() -> Option<NativeWindowHandle<'a>> {
    NonNull::new(unsafe {
        sys::GetWindowHandle()
    })
        .map(|ptr| NativeWindowHandle {
            ptr,
            _marker: PhantomData
        })
}

/// Get current screen width
#[inline]
pub fn get_screen_width() -> u32 {
    unsafe {
        sys::GetScreenWidth()
    }.try_into().unwrap()
}

/// Get current screen height
#[inline]
pub fn get_screen_height() -> u32 {
    unsafe {
        sys::GetScreenHeight()
    }.try_into().unwrap()
}

/// Get current render width (it considers HiDPI)
#[inline]
pub fn get_render_width() -> u32 {
    unsafe {
        sys::GetRenderWidth()
    }.try_into().unwrap()
}

/// Get current render height (it considers HiDPI)
#[inline]
pub fn get_render_height() -> u32 {
    unsafe {
        sys::GetRenderHeight()
    }.try_into().unwrap()
}

/// Get number of connected monitors
#[inline]
pub fn get_monitor_count() -> usize {
    unsafe {
        sys::GetMonitorCount()
    }.try_into().unwrap()
}

/// Get current monitor where window is placed
#[inline]
pub fn get_current_monitor() -> Result<usize, usize> {
    let idx = unsafe {
        sys::GetCurrentMonitor()
    };
    if idx >= 0 {
        Ok(idx.try_into().unwrap())
    } else {
        Err(idx.unsigned_abs().try_into().unwrap())
    }
}

/// Get specified monitor position
#[inline]
pub fn get_monitor_position(
    monitor: usize,
) -> sys::Vector2 {
    unsafe {
        sys::GetMonitorPosition(
            monitor.try_into().unwrap(),
        )
    }
}

/// Get specified monitor width (current video mode used by monitor)
#[inline]
pub fn get_monitor_width(
    monitor: usize,
) -> u32 {
    unsafe {
        sys::GetMonitorWidth(
            monitor.try_into().unwrap(),
        )
    }.try_into().unwrap()
}

/// Get specified monitor height (current video mode used by monitor)
#[inline]
pub fn get_monitor_height(
    monitor: usize,
) -> u32 {
    unsafe {
        sys::GetMonitorHeight(
            monitor.try_into().unwrap(),
        )
    }.try_into().unwrap()
}

/// Get specified monitor physical width in millimetres
#[inline]
pub fn get_monitor_physical_width(
    monitor: usize,
) -> u32 {
    unsafe {
        sys::GetMonitorPhysicalWidth(
            monitor.try_into().unwrap(),
        )
    }.try_into().unwrap()
}

/// Get specified monitor physical height in millimetres
#[inline]
pub fn get_monitor_physical_height(
    monitor: usize,
) -> u32 {
    unsafe {
        sys::GetMonitorPhysicalHeight(
            monitor.try_into().unwrap(),
        )
    }.try_into().unwrap()
}

/// Get specified monitor refresh rate
#[inline]
pub fn get_monitor_refresh_rate(
    monitor: usize,
) -> u32 {
    unsafe {
        sys::GetMonitorRefreshRate(
            monitor.try_into().unwrap(),
        )
    }.try_into().unwrap()
}

/// Get window position XY on monitor
#[inline]
pub fn get_window_position() -> sys::Vector2 {
    unsafe {
        sys::GetWindowPosition()
    }
}

/// Get window scale DPI factor
#[inline]
pub fn get_window_scale_dpi() -> sys::Vector2 {
    unsafe {
        sys::GetWindowScaleDPI()
    }
}

/// Get the human-readable, UTF-8 encoded name of the specified monitor
#[inline]
pub fn get_monitor_name<'a>(
    monitor: usize,
) -> Option<&'a CStr> {
    let ptr = unsafe {
        sys::GetMonitorName(
            monitor.try_into().unwrap(),
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

/// Set clipboard text content
#[inline]
pub fn set_clipboard_text(
    text: &CStr,
) {
    unsafe {
        sys::SetClipboardText(
            text.as_ptr(),
        );
    }
}

/// Get clipboard text content
#[inline]
pub fn get_clipboard_text<'a>() -> Option<&'a CStr> {
    let ptr = unsafe {
        sys::GetClipboardText()
    };
    if !ptr.is_null() {
        Some(unsafe {
            CStr::from_ptr(ptr)
        })
    } else {
        None
    }
}

/// Get clipboard image content
#[inline]
pub fn get_clipboard_image() -> sys::Image {
    unsafe {
        sys::GetClipboardImage()
    }
}

/// Enable waiting for events on EndDrawing(), no automatic event polling
#[inline]
pub fn enable_event_waiting() {
    unsafe {
        sys::EnableEventWaiting();
    }
}

/// Disable waiting for events on EndDrawing(), automatic events polling
#[inline]
pub fn disable_event_waiting() {
    unsafe {
        sys::DisableEventWaiting();
    }
}

// Cursor-related functions

/// Shows cursor
#[inline]
pub fn show_cursor() {
    unsafe {
        sys::ShowCursor();
    }
}

/// Hides cursor
#[inline]
pub fn hide_cursor() {
    unsafe {
        sys::HideCursor();
    }
}

/// Check if cursor is not visible
#[inline]
pub fn is_cursor_hidden() -> bool {
    unsafe {
        sys::IsCursorHidden()
    }
}

/// Enables cursor (unlock cursor)
#[inline]
pub fn enable_cursor() {
    unsafe {
        sys::EnableCursor();
    }
}

/// Disables cursor (lock cursor)
#[inline]
pub fn disable_cursor() {
    unsafe {
        sys::DisableCursor();
    }
}

/// Check if cursor is on the screen
#[inline]
pub fn is_cursor_on_screen() -> bool {
    unsafe {
        sys::IsCursorOnScreen()
    }
}

// Drawing-related functions

/// Set background color (framebuffer clear color)
#[inline]
pub fn clear_background(
    color: sys::Color,
) {
    unsafe {
        sys::ClearBackground(
            color,
        );
    }
}

/// Setup canvas (framebuffer) to start drawing
#[inline]
pub fn begin_drawing() {
    unsafe {
        sys::BeginDrawing();
    }
}

/// End canvas drawing and swap buffers (double buffering)
#[inline]
pub fn end_drawing() {
    unsafe {
        sys::EndDrawing();
    }
}

/// Begin 2D mode with custom camera (2D)
#[allow(non_snake_case)]
#[inline]
pub fn begin_mode2D(
    camera: sys::Camera2D,
) {
    unsafe {
        sys::BeginMode2D(
            camera,
        );
    }
}

/// Ends 2D mode with custom camera
#[allow(non_snake_case)]
#[inline]
pub fn end_mode2D() {
    unsafe {
        sys::EndMode2D();
    }
}

/// Begin 3D mode with custom camera (3D)
#[allow(non_snake_case)]
#[inline]
pub fn begin_mode3D(
    camera: sys::Camera3D,
) {
    unsafe {
        sys::BeginMode3D(
            camera,
        );
    }
}

/// Ends 3D mode and returns to default 2D orthographic mode
#[allow(non_snake_case)]
#[inline]
pub fn end_mode3D() {
    unsafe {
        sys::EndMode3D();
    }
}

/// Begin drawing to render texture
#[inline]
pub fn begin_texture_mode(
    target: sys::RenderTexture2D,
) {
    unsafe {
        sys::BeginTextureMode(
            target,
        );
    }
}

/// Ends drawing to render texture
#[inline]
pub fn end_texture_mode() {
    unsafe {
        sys::EndTextureMode();
    }
}

/// Begin custom shader drawing
#[inline]
pub fn begin_shader_mode(
    shader: sys::Shader,
) {
    unsafe {
        sys::BeginShaderMode(
            shader,
        );
    }
}

/// End custom shader drawing (use default shader)
#[inline]
pub fn end_shader_mode() {
    unsafe {
        sys::EndShaderMode();
    }
}

/// Begin blending mode (alpha, additive, multiplied, subtract, custom)
#[inline]
pub fn begin_blend_mode(
    mode: sys::BlendMode,
) {
    unsafe {
        sys::BeginBlendMode(
            (mode as i32).try_into().unwrap(),
        );
    }
}

/// End blending mode (reset to default: alpha blending)
#[inline]
pub fn end_blend_mode() {
    unsafe {
        sys::EndBlendMode();
    }
}

/// Begin scissor mode (define screen area for following drawing)
#[inline]
pub fn begin_scissor_mode(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) {
    unsafe {
        sys::BeginScissorMode(
            x,
            y,
            width,
            height,
        );
    }
}

/// End scissor mode
#[inline]
pub fn end_scissor_mode() {
    unsafe {
        sys::EndScissorMode();
    }
}

/// Begin stereo rendering (requires VR simulator)
#[inline]
pub fn begin_vr_stereo_mode(
    config: sys::VrStereoConfig,
) {
    unsafe {
        sys::BeginVrStereoMode(
            config,
        );
    }
}

/// End stereo rendering (requires VR simulator)
#[inline]
pub fn end_vr_stereo_mode() {
    unsafe {
        sys::EndVrStereoMode();
    }
}

// VR stereo config functions for VR simulator

/// Load VR stereo config for VR simulator device parameters
#[inline]
pub fn load_vr_stereo_config(
    device: sys::VrDeviceInfo,
) -> sys::VrStereoConfig {
    unsafe {
        sys::LoadVrStereoConfig(
            device,
        )
    }
}

/// Unload VR stereo config
#[inline]
pub fn unload_vr_stereo_config(
    config: sys::VrStereoConfig,
) {
    unsafe {
        sys::UnloadVrStereoConfig(
            config,
        );
    }
}

// Shader management functions
// NOTE: Shader functionality is not available on OpenGL 1.1

/// Load shader from files and bind default locations
#[inline]
pub fn load_shader(
    vs_file_name: Option<&CStr>,
    fs_file_name: Option<&CStr>,
) -> sys::Shader {
    unsafe {
        sys::LoadShader(
            vs_file_name.map_or_else(null, CStr::as_ptr),
            fs_file_name.map_or_else(null, CStr::as_ptr),
        )
    }
}

/// Load shader from code strings and bind default locations
#[inline]
pub fn load_shader_from_memory(
    vs_code: Option<&CStr>,
    fs_code: Option<&CStr>,
) -> sys::Shader {
    unsafe {
        sys::LoadShaderFromMemory(
            vs_code.map_or_else(null, CStr::as_ptr),
            fs_code.map_or_else(null, CStr::as_ptr),
        )
    }
}

/// Check if a shader is valid (loaded on GPU)
#[inline]
pub fn is_shader_valid(
    shader: sys::Shader,
) -> bool {
    unsafe {
        sys::IsShaderValid(
            shader,
        )
    }
}

/// Location of a shader uniform
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UniformLoc(pub c_int);

/// Get shader uniform location
#[inline]
pub fn get_shader_location(
    shader: sys::Shader,
    uniform_name: &CStr,
) -> UniformLoc {
    UniformLoc(unsafe {
        sys::GetShaderLocation(
            shader,
            uniform_name.as_ptr(),
        )
    })
}

/// Location of a shader attribute
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AttribLoc(pub c_int);

/// Get shader attribute location
#[inline]
pub fn get_shader_location_attrib(
    shader: sys::Shader,
    attrib_name: &CStr,
) -> AttribLoc {
    AttribLoc(unsafe {
        sys::GetShaderLocationAttrib(
            shader,
            attrib_name.as_ptr(),
        )
    })
}

pub trait UniformType {
    const UNIFORM_TYPE: sys::ShaderUniformDataType;

    #[must_use]
    #[inline]
    unsafe fn as_data(value: &Self) -> *const c_void {
        std::ptr::from_ref(value).cast()
    }

    #[must_use]
    #[inline]
    unsafe fn arr_as_data(value: &[Self]) -> *const c_void where Self: Sized {
       value.as_ptr().cast()
    }
}

macro_rules! define_uniform_types {
    ($(impl $variant:ident for $T:ty $(=> |$value:ident| $convert:expr)? $(, |$arr:ident| $convert_v:expr)?;)*) => {$(
        impl UniformType for $T {
            const UNIFORM_TYPE: sys::ShaderUniformDataType = sys::ShaderUniformDataType::$variant;
            $(#[inline]
            unsafe fn as_data($value: &Self) -> *const c_void {
                $convert
            })?
        }
    )*};
}

define_uniform_types! {
    impl SHADER_UNIFORM_FLOAT for f32;
    impl SHADER_UNIFORM_VEC2 for [f32; 2];
    impl SHADER_UNIFORM_VEC3 for [f32; 3];
    impl SHADER_UNIFORM_VEC4 for [f32; 4];
    impl SHADER_UNIFORM_VEC2 for sys::Vector2;
    impl SHADER_UNIFORM_VEC3 for sys::Vector3;
    impl SHADER_UNIFORM_VEC4 for sys::Vector4;
    impl SHADER_UNIFORM_INT for i32;
    impl SHADER_UNIFORM_IVEC2 for [i32; 2];
    impl SHADER_UNIFORM_IVEC3 for [i32; 3];
    impl SHADER_UNIFORM_IVEC4 for [i32; 4];
    impl SHADER_UNIFORM_UINT for u32;
    impl SHADER_UNIFORM_UIVEC2 for [u32; 2];
    impl SHADER_UNIFORM_UIVEC3 for [u32; 3];
    impl SHADER_UNIFORM_UIVEC4 for [u32; 4];
    impl SHADER_UNIFORM_SAMPLER2D for &[i32];
}

/// Set shader uniform value
#[inline]
pub fn set_shader_value<T: UniformType>(
    shader: sys::Shader,
    loc: UniformLoc,
    value: &T,
) {
    unsafe {
        sys::SetShaderValue(
            shader,
            loc.0,
            T::as_data(value),
            T::UNIFORM_TYPE as c_int,
        );
    }
}

pub trait UniformVType: Sized {
    const UNIFORM_TYPE: sys::ShaderUniformDataType;

    #[must_use]
    #[inline]
    unsafe fn as_data_v(value: &[Self]) -> *const c_void {
        value.as_ptr().cast()
    }
}

macro_rules! define_uniform_types {
    ($(impl $variant:ident for $T:ty $(=> |$value:ident| $convert:expr)? $(, |$arr:ident| $convert_v:expr)?;)*) => {$(
        impl UniformVType for $T {
            const UNIFORM_TYPE: sys::ShaderUniformDataType = sys::ShaderUniformDataType::$variant;
            $(#[inline]
            unsafe fn as_data_v($value: &Self) -> *const c_void {
                $convert
            })?
        }
    )*};
}

define_uniform_types! {
    impl SHADER_UNIFORM_FLOAT for f32;
    impl SHADER_UNIFORM_INT for i32;
    impl SHADER_UNIFORM_UINT for u32;
}

/// Set shader uniform value vector
#[inline]
pub fn set_shader_value_v<T: UniformVType>(
    shader: sys::Shader,
    loc: UniformLoc,
    value: &[T],
) {
    unsafe {
        sys::SetShaderValueV(
            shader,
            loc.0,
            T::as_data_v(value),
            T::UNIFORM_TYPE as c_int,
            value.len().try_into().unwrap(),
        );
    }
}

/// Set shader uniform value (matrix 4x4)
#[inline]
pub fn set_shader_value_matrix(shader: sys::Shader, loc: UniformLoc, mat: sys::Matrix) {
    unsafe {
        sys::SetShaderValueMatrix(
            shader,
            loc.0,
            mat,
        );
    }
}

/// Set shader uniform value and bind the texture (sampler2d)
#[inline]
pub fn set_shader_value_texture(
    shader: sys::Shader,
    loc: UniformLoc,
    texture: sys::Texture2D,
) {
    unsafe {
        sys::SetShaderValueTexture(
            shader,
            loc.0,
            texture,
        );
    }
}

/// Unload shader from GPU memory (VRAM)
#[inline]
pub fn unload_shader(shader: sys::Shader) {
    unsafe {
        sys::UnloadShader(
            shader,
        );
    }
}

// Screen-space-related functions

/// Get a ray trace from screen position (i.e mouse)
#[inline]
pub fn get_screen_to_world_ray(
    position: sys::Vector2,
    camera: sys::Camera,
) -> sys::Ray {
    unsafe {
        sys::GetScreenToWorldRay(
            position,
            camera,
        )
    }
}

/// Get a ray trace from screen position (i.e mouse) in a viewport
#[inline]
pub fn get_screen_to_world_ray_ex(
    position: sys::Vector2,
    camera: sys::Camera,
    width: i32,
    height: i32,
) -> sys::Ray {
    unsafe {
        sys::GetScreenToWorldRayEx(
            position,
            camera,
            width,
            height,
        )
    }
}

/// Get the screen space position for a 3d world space position
#[inline]
pub fn get_world_to_screen(
    position: sys::Vector3,
    camera: sys::Camera,
) -> sys::Vector2 {
    unsafe {
        sys::GetWorldToScreen(
            position,
            camera,
        )
    }
}

/// Get size position for a 3d world space position
#[inline]
pub fn get_world_to_screen_ex(
    position: sys::Vector3,
    camera: sys::Camera,
    width: i32,
    height: i32,
) -> sys::Vector2 {
    unsafe {
        sys::GetWorldToScreenEx(
            position,
            camera,
            width,
            height,
        )
    }
}

/// Get the screen space position for a 2d camera world space position
#[allow(non_snake_case)]
#[inline]
pub fn get_world_to_screen2D(
    position: sys::Vector2,
    camera: sys::Camera2D,
) -> sys::Vector2 {
    unsafe {
        sys::GetWorldToScreen2D(
            position,
            camera,
        )
    }
}

/// Get the world space position for a 2d camera screen space position
#[allow(non_snake_case)]
#[inline]
pub fn get_screen_to_world2D(
    position: sys::Vector2,
    camera: sys::Camera2D,
) -> sys::Vector2 {
    unsafe {
        sys::GetScreenToWorld2D(
            position,
            camera,
        )
    }
}

/// Get camera transform matrix (view matrix)
#[inline]
pub fn get_camera_matrix(
    camera: sys::Camera,
) -> sys::Matrix {
    unsafe {
        sys::GetCameraMatrix(
            camera,
        )
    }
}

/// Get camera 2d transform matrix
#[allow(non_snake_case)]
#[inline]
pub fn get_camera_matrix2D(
    camera: sys::Camera2D,
) -> sys::Matrix {
    unsafe {
        sys::GetCameraMatrix2D(
            camera,
        )
    }
}

// Timing-related functions

/// Set target FPS (maximum)
#[inline]
pub fn set_target_fps(
    fps: u32,
) {
    unsafe {
        sys::SetTargetFPS(
            fps.try_into().unwrap(),
        );
    }
}

/// Get time in seconds for last frame drawn (delta time)
#[inline]
pub fn get_frame_time() -> f32 {
    unsafe {
        sys::GetFrameTime()
    }
}

/// Get duration for last frame drawn (delta time)
#[inline]
pub fn get_frame_duration() -> Duration {
    Duration::from_secs_f32(get_frame_time())
}

/// Get elapsed time in seconds since [`init_window()`]
#[inline]
pub fn get_time() -> f64 {
    unsafe {
        sys::GetTime()
    }
}

/// Get elapsed duration since [`init_window()`]
#[inline]
pub fn get_elapsed() -> Duration {
    Duration::from_secs_f64(get_time())
}

/// Get current FPS
#[inline]
pub fn get_fps() -> u32 {
    unsafe {
        sys::GetFPS()
    }.try_into().unwrap()
}

#[cfg(feature = "custom_frame_control")]
pub mod custom_frame_control {
    //! Custom frame control functions
    //! NOTE: Those functions are intended for advanced users that want full control over the frame processing
    //! By default [`end_drawing()`] does this job: draws everything + [`swap_screen_buffer()`] + manage frame timing + [`poll_input_events()`]
    //! To avoid that behaviour and control frame processes manually, enable the feature: `custom_frame_control`
    use super::*;

    /// Swap back buffer with front buffer (screen drawing)
    #[inline]
    pub fn swap_screen_buffer() {
        unsafe {
            sys::SwapScreenBuffer();
        }
    }

    /// Register all input events
    #[inline]
    pub fn poll_input_events() {
        unsafe {
            sys::PollInputEvents();
        }
    }

    /// Wait for some time (halt program execution)
    #[inline]
    pub fn wait_time(
        seconds: f64,
    ) {
        unsafe {
            sys::WaitTime(
                seconds,
            );
        }
    }

    /// Wait for some duration (halt program execution)
    #[inline]
    pub fn wait_duration(
        duration: Duration,
    ) {
        wait_time(duration.as_secs_f64());
    }
}
#[cfg(feature = "custom_frame_control")]
pub use custom_frame_control::*;

// Random values generation functions

/// Set the seed for the random number generator
#[inline]
pub fn set_random_seed(
    seed: u32,
) {
    unsafe {
        sys::SetRandomSeed(
            seed,
        );
    }
}

/// Get a random value between min and max (both included)
#[inline]
pub fn get_random_value(
    min: i32,
    max: i32,
) -> i32 {
    unsafe {
        sys::GetRandomValue(
            min,
            max,
        )
    }
}

/// An owned slice of data that must be deallocated manually using [`unload_random_sequence()`]
#[must_use]
pub struct RandomSequence {
    data: NonNull<i32>,
    len: usize,
}

impl Deref for RandomSequence {
    type Target = [i32];

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe {
            std::slice::from_raw_parts(
                self.data.as_ptr().cast_const(),
                self.len,
            )
        }
    }
}

impl DerefMut for RandomSequence {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.data.as_ptr(),
                self.len,
            )
        }
    }
}

/// Load random values sequence, no values repeated
#[inline]
pub fn load_random_sequence(
    len: usize,
    min: i32,
    max: i32,
) -> Option<RandomSequence> {
    let ptr = unsafe {
        sys::LoadRandomSequence(
            len.try_into().unwrap(),
            min,
            max,
        )
    };
    NonNull::new(ptr)
        .map(|data| RandomSequence {
            data,
            len,
        })
}

/// Unload random values sequence
#[inline]
pub fn unload_random_sequence(
    sequence: RandomSequence,
) {
    unsafe {
        sys::UnloadRandomSequence(
            sequence.data.as_ptr(),
        );
    }
}

// Misc. functions

/// Takes a screenshot of current screen (filename extension defines format)
#[inline]
pub fn take_screenshot(
    file_name: &CStr,
) {
    unsafe {
        sys::TakeScreenshot(
            file_name.as_ptr(),
        );
    }
}

/// Setup init configuration flags (view FLAGS)
#[inline]
pub fn set_config_flags(
    flags: sys::ConfigFlags,
) {
    unsafe {
        sys::SetConfigFlags(
            flags.0.try_into().unwrap(),
        )
    }
}

/// Open URL with default system browser (if available)
#[inline]
pub fn open_url(
    url: &CStr,
) {
    unsafe {
        sys::OpenURL(
            url.as_ptr(),
        );
    }
}

// NOTE: Following functions implemented in module [utils]
//------------------------------------------------------------------

/// Show trace log messages (LOG_DEBUG, LOG_INFO, LOG_WARNING, LOG_ERROR...)
#[inline]
pub fn trace_log(
    log_level: sys::TraceLogLevel,
    args: std::fmt::Arguments<'_>,
) -> Result<(), NulError> {
    let text = CString::new(args.to_string())?;
    unsafe {
        sys::TraceLog(
            (log_level as i32).try_into().unwrap(),
            text.as_c_str().as_ptr(),
        );
    }
    Ok(())
}

#[macro_export]
macro_rules! trace_log {
    ($log_level:expr, $(args:tt)+) => {
        $crate::low_level::trace_log(
            $log_level,
            ::std::format_args!($(args)+),
        )
    };
}

/// Set the current threshold (minimum) log level
#[inline]
pub fn set_trace_log_level(
    log_level: sys::TraceLogLevel,
) {
    unsafe {
        sys::SetTraceLogLevel(
            (log_level as i32).try_into().unwrap(),
        )
    }
}

/// Internal memory allocator
#[inline]
pub fn mem_alloc(
    size: usize,
) -> Option<NonNull<c_void>> {
    NonNull::new(unsafe {
        sys::MemAlloc(
            size.try_into().unwrap(),
        )
    })
}

/// Internal memory reallocator
#[inline]
pub fn mem_realloc(
    ptr: NonNull<c_void>,
    size: usize,
) -> Option<NonNull<c_void>> {
    NonNull::new(unsafe {
        sys::MemRealloc(
            ptr.as_ptr(),
            size.try_into().unwrap(),
        )
    })
}

/// Internal memory free
#[inline]
pub fn mem_free(
    ptr: NonNull<c_void>,
) {
    unsafe {
        sys::MemFree(
            ptr.as_ptr(),
        );
    }
}

// Set custom callbacks
// WARNING: Callbacks setup is intended for advanced users

/// Set custom trace log
#[inline]
pub fn set_trace_log_callback(
    callback: sys::TraceLogCallback,
) {
    unsafe {
        sys::SetTraceLogCallback(
            callback,
        );
    }
}

/// Set custom file binary data loader
#[inline]
pub fn set_load_file_data_callback(
    callback: sys::LoadFileDataCallback,
) {
    unsafe {
        sys::SetLoadFileDataCallback(
            callback,
        );
    }
}

/// Set custom file binary data saver
#[inline]
pub fn set_save_file_data_callback(
    callback: sys::SaveFileDataCallback,
) {
    unsafe {
        sys::SetSaveFileDataCallback(
            callback,
        );
    }
}

/// Set custom file text data loader
#[inline]
pub fn set_load_file_text_callback(
    callback: sys::LoadFileTextCallback,
) {
    unsafe {
        sys::SetLoadFileTextCallback(
            callback,
        );
    }
}

/// Set custom file text data saver
#[inline]
pub fn set_save_file_text_callback(
    callback: sys::SaveFileTextCallback,
) {
    unsafe {
        sys::SetSaveFileTextCallback(
            callback,
        );
    }
}

// Files management functions

/// Data loaded with [`load_file_data()`] that must be unloaded manually with [`unload_file_data()`]
pub struct FileData {
    data: NonNull<u8>,
    len: usize,
}

impl Deref for FileData {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe {
            std::slice::from_raw_parts(
                self.data.as_ptr().cast_const(),
                self.len,
            )
        }
    }
}

impl DerefMut for FileData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.data.as_ptr(),
                self.len,
            )
        }
    }
}

/// Load file data as byte array (read)
#[inline]
pub fn load_file_data(
    file_name: &CStr,
) -> Option<FileData> {
    let mut len = MaybeUninit::uninit();
    let ptr = unsafe {
        sys::LoadFileData(
            file_name.as_ptr(),
            len.as_mut_ptr(),
        )
    };
    NonNull::new(ptr)
        .map(|data| FileData {
            data,
            len: unsafe {
                len.assume_init()
            }.try_into().unwrap(),
        })
}

/// Unload file data allocated by [`load_file_data()`]
#[inline]
pub fn unload_file_data(
    data: FileData,
) {
    unsafe {
        sys::UnloadFileData(
            data.data.as_ptr(),
        );
    }
}

/// Save data to file from byte array (write)
#[inline]
pub fn save_file_data(
    file_name: &CStr,
    data: &mut [u8],
) -> Result<(), ()> {
    match unsafe {
        sys::SaveFileData(
            file_name.as_ptr(),
            // This can't simply be cast_mut() because if there is a custom callback, it may mutate the data.
            // See [`SaveFileDataCallback`] signature
            data.as_mut_ptr().cast(),
            data.len().try_into().unwrap(),
        )
    } {
        true => Ok(()),
        false => Err(()),
    }
}

/// Export data to code (.h)
#[inline]
pub fn export_data_as_code(
    data: &[u8],
    file_name: &CStr,
) -> Result<(), ()> {
    match unsafe {
        sys::ExportDataAsCode(
            data.as_ptr(),
            data.len().try_into().unwrap(),
            file_name.as_ptr(),
        )
    } {
        true => Ok(()),
        false => Err(()),
    }
}

/// An owned cstring returned by [`load_file_text()`] that must be manually unloaded with [`unload_file_text()`]
pub struct RaylibCString(NonNull<c_char>);

impl Deref for RaylibCString {
    type Target = CStr;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe {
            CStr::from_ptr(
                self.0.as_ptr(),
            )
        }
    }
}

/// Load text data from file (read)
#[inline]
pub fn load_file_text(
    file_name: &CStr,
) -> Option<RaylibCString> {
    NonNull::new(unsafe {
        sys::LoadFileText(
            file_name.as_ptr(),
        )
    }).map(RaylibCString)
}

/// Unload file text data allocated by [`load_file_text()`]
#[inline]
pub fn unload_file_text(
    text: RaylibCString,
) {
    unsafe {
        sys::UnloadFileText(
            text.0.as_ptr(),
        );
    }
}

/// Save text data to file (write), string must be '\0' terminated, returns true on success
#[inline]
pub fn save_file_text(
    file_name: &CStr,
    text: &CStr,
) -> Result<(), ()> {
    match unsafe {
        sys::SaveFileText(
            file_name.as_ptr(),
            text.as_ptr(),
        )
    } {
        true => Ok(()),
        false => Err(()),
    }
}
//------------------------------------------------------------------

// File system functions

/// Check if file exists
#[inline]
pub fn file_exists(
    file_name: &CStr,
) -> bool {
    unsafe {
        sys::FileExists(
            file_name.as_ptr(),
        )
    }
}

/// Check if a directory path exists
#[inline]
pub fn directory_exists(
    dir_path: &CStr,
) -> bool {
    unsafe {
        sys::DirectoryExists(
            dir_path.as_ptr(),
        )
    }
}

/// Check file extension (including point: .png, .wav)
#[inline]
pub fn is_file_extension(
    file_name: &CStr,
    ext: &CStr,
) -> bool {
    unsafe {
        sys::IsFileExtension(
            file_name.as_ptr(),
            ext.as_ptr(),
        )
    }
}

/// Get file length in bytes (NOTE: `GetFileSize()` conflicts with windows.h)
#[inline]
pub fn get_file_length(
    file_name: &CStr,
) -> Result<usize, usize> {
    let len = unsafe {
        sys::GetFileLength(
            file_name.as_ptr(),
        )
    };
    if len >= 0 {
        Ok(len.try_into().unwrap())
    } else {
        Err(len.unsigned_abs().try_into().unwrap())
    }
}

/// Get pointer to extension for a filename string (includes dot: '.png')
#[inline]
pub fn get_file_extension(
    file_name: &CStr,
) -> Option<&CStr> {
    let ptr = unsafe {
        sys::GetFileExtension(
            file_name.as_ptr(),
        )
    };
    if !ptr.is_null() {
        Some(unsafe {
            // returns a slice of the input string
            CStr::from_ptr(ptr)
        })
    } else {
        None
    }
}

/// Get pointer to filename for a path string
#[inline]
pub fn get_file_name(
    file_path: &CStr,
) -> Option<&CStr> {
    let ptr = unsafe {
        sys::GetFileName(
            file_path.as_ptr(),
        )
    };
    if !ptr.is_null() {
        Some(unsafe {
            // returns a slice of the input string
            CStr::from_ptr(ptr)
        })
    } else {
        None
    }
}

/// Get filename string without extension
#[inline]
pub fn get_file_name_without_ext<'a>(
    file_path: &CStr, // 'a is NOT the lifetime of the `file_path`
) -> Option<&'a CStr> {
    let ptr = unsafe {
        sys::GetFileNameWithoutExt(
            file_path.as_ptr(),
        )
    };
    if !ptr.is_null() {
        Some(unsafe {
            // returns a reference to a static buffer that gets
            // overwritten when this function is called
            CStr::from_ptr(ptr)
        })
    } else {
        None
    }
}

/// Get full path for a given fileName with path
#[inline]
pub fn get_directory_path<'a>(
    file_path: &CStr, // 'a is NOT the lifetime of the `file_path`
) -> Option<&'a CStr> {
    let ptr = unsafe {
        sys::GetDirectoryPath(
            file_path.as_ptr(),
        )
    };
    if !ptr.is_null() {
        Some(unsafe {
            // returns a reference to a static buffer that gets
            // overwritten when this function is called
            CStr::from_ptr(ptr)
        })
    } else {
        None
    }
}

/// Get previous directory path for a given path
#[inline]
pub fn get_prev_directory_path<'a>(
    dir_path: &CStr, // 'a is NOT the lifetime of the `file_path`
) -> Option<&'a CStr> {
    let ptr = unsafe {
        sys::GetPrevDirectoryPath(
            dir_path.as_ptr(),
        )
    };
    if !ptr.is_null() {
        Some(unsafe {
            // returns a reference to a static buffer that gets
            // overwritten when this function is called
            CStr::from_ptr(ptr)
        })
    } else {
        None
    }
}

/// Get current working directory (uses static string)
#[inline]
pub fn get_working_directory<'a>() -> Option<&'a CStr> {
    let ptr = unsafe {
        sys::GetWorkingDirectory()
    };
    if !ptr.is_null() {
        Some(unsafe {
            // returns a reference to a static buffer that gets
            // overwritten when this function is called
            CStr::from_ptr(ptr)
        })
    } else {
        None
    }
}

/// Get the directory of the running application (uses static string)
#[inline]
pub fn get_application_directory<'a>() -> Option<&'a CStr> {
    let ptr = unsafe {
        sys::GetApplicationDirectory()
    };
    if !ptr.is_null() {
        Some(unsafe {
            // returns a reference to a static buffer that gets
            // overwritten when this function is called
            CStr::from_ptr(ptr)
        })
    } else {
        None
    }
}

/// Create directories (including full path requested)
#[inline]
pub fn make_directory(
    dir_path: &CStr,
) -> Result<(), NonZero<c_int>> {
    let result = unsafe {
        sys::MakeDirectory(
            dir_path.as_ptr(),
        )
    };
    match NonZero::new(result) {
        None => Ok(()),
        Some(value) => Err(value),
    }
}

/// Change working directory, return true on success
#[inline]
pub fn change_directory(
    dir: &CStr,
) -> Result<(), ()> {
    match unsafe {
        sys::ChangeDirectory(
            dir.as_ptr(),
        )
    } {
        true => Ok(()),
        false => Err(()),
    }
}

/// Check if a given path is a file or a directory
#[inline]
pub fn is_path_file(
    path: &CStr,
) -> bool {
    unsafe {
        sys::IsPathFile(
            path.as_ptr(),
        )
    }
}

/// Check if fileName is valid for the platform/OS
#[inline]
pub fn is_file_name_valid(
    file_name: &CStr,
) -> bool {
    unsafe {
        sys::IsFileNameValid(
            file_name.as_ptr(),
        )
    }
}

/// Load directory filepaths
#[inline]
pub fn load_directory_files(
    dir_path: &CStr,
) -> sys::FilePathList {
    unsafe {
        sys::LoadDirectoryFiles(
            dir_path.as_ptr(),
        )
    }
}

/// Load directory filepaths with extension filtering and recursive directory scan. Use 'DIR' in the filter string to include directories in the result
#[inline]
pub fn load_directory_files_ex(
    base_path: Option<&CStr>,
    filter: Option<&CStr>,
    scan_subdirs: bool,
) -> sys::FilePathList {
    unsafe {
        sys::LoadDirectoryFilesEx(
            base_path.map_or_else(null, CStr::as_ptr),
            filter.map_or_else(null, CStr::as_ptr),
            scan_subdirs,
        )
    }
}

/// Unload filepaths
#[inline]
pub fn unload_directory_files(
    files: sys::FilePathList,
) {
    unsafe {
        sys::UnloadDirectoryFiles(
            files,
        );
    }
}

/// Check if a file has been dropped into window
#[inline]
pub fn is_file_dropped() -> bool {
    unsafe {
        sys::IsFileDropped()
    }
}

/// Load dropped filepaths
#[inline]
pub fn load_dropped_files() -> sys::FilePathList {
    unsafe {
        sys::LoadDroppedFiles()
    }
}

/// Unload dropped filepaths
#[inline]
pub fn unload_dropped_files(
    files: sys::FilePathList,
) {
    unsafe {
        sys::UnloadDroppedFiles(
            files,
        );
    }
}

/// Get file modification time (last write time)
#[inline]
pub fn get_file_mod_time(
    file_name: &CStr,
) -> i64 {
    unsafe {
        sys::GetFileModTime(
            file_name.as_ptr(),
        ).into()
    }
}

// pub fn CompressData(
//     data: *const ::std::os::raw::c_uchar,
//     dataSize: ::std::os::raw::c_int,
//     compDataSize: *mut ::std::os::raw::c_int,
// ) -> *mut ::std::os::raw::c_uchar;
// pub fn DecompressData(
//     compData: *const ::std::os::raw::c_uchar,
//     compDataSize: ::std::os::raw::c_int,
//     dataSize: *mut ::std::os::raw::c_int,
// ) -> *mut ::std::os::raw::c_uchar;
// pub fn EncodeDataBase64(
//     data: *const ::std::os::raw::c_uchar,
//     dataSize: ::std::os::raw::c_int,
//     outputSize: *mut ::std::os::raw::c_int,
// ) -> *mut ::std::os::raw::c_char;
// pub fn DecodeDataBase64(
//     text: *const ::std::os::raw::c_char,
//     outputSize: *mut ::std::os::raw::c_int,
// ) -> *mut ::std::os::raw::c_uchar;
// pub fn ComputeCRC32(
//     data: *mut ::std::os::raw::c_uchar,
//     dataSize: ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_uint;
// pub fn ComputeMD5(
//     data: *mut ::std::os::raw::c_uchar,
//     dataSize: ::std::os::raw::c_int,
// ) -> *mut ::std::os::raw::c_uint;
// pub fn ComputeSHA1(
//     data: *mut ::std::os::raw::c_uchar,
//     dataSize: ::std::os::raw::c_int,
// ) -> *mut ::std::os::raw::c_uint;
// pub fn LoadAutomationEventList(fileName: *const ::std::os::raw::c_char) -> AutomationEventList;
// pub fn UnloadAutomationEventList(list: AutomationEventList);
// pub fn ExportAutomationEventList(
//     list: AutomationEventList,
//     fileName: *const ::std::os::raw::c_char,
// ) -> bool;
// pub fn SetAutomationEventList(list: *mut AutomationEventList);
// pub fn SetAutomationEventBaseFrame(frame: ::std::os::raw::c_int);
// pub fn StartAutomationEventRecording();
// pub fn StopAutomationEventRecording();
// pub fn PlayAutomationEvent(event: AutomationEvent);
// pub fn IsKeyPressed(key: ::std::os::raw::c_int) -> bool;
// pub fn IsKeyPressedRepeat(key: ::std::os::raw::c_int) -> bool;
// pub fn IsKeyDown(key: ::std::os::raw::c_int) -> bool;
// pub fn IsKeyReleased(key: ::std::os::raw::c_int) -> bool;
// pub fn IsKeyUp(key: ::std::os::raw::c_int) -> bool;
// pub fn GetKeyPressed() -> ::std::os::raw::c_int;
// pub fn GetCharPressed() -> ::std::os::raw::c_int;
// pub fn GetKeyName(key: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
// pub fn SetExitKey(key: ::std::os::raw::c_int);
// pub fn IsGamepadAvailable(gamepad: ::std::os::raw::c_int) -> bool;
// pub fn GetGamepadName(gamepad: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
// pub fn IsGamepadButtonPressed(
//     gamepad: ::std::os::raw::c_int,
//     button: ::std::os::raw::c_int,
// ) -> bool;
// pub fn IsGamepadButtonDown(
//     gamepad: ::std::os::raw::c_int,
//     button: ::std::os::raw::c_int,
// ) -> bool;
// pub fn IsGamepadButtonReleased(
//     gamepad: ::std::os::raw::c_int,
//     button: ::std::os::raw::c_int,
// ) -> bool;
// pub fn IsGamepadButtonUp(gamepad: ::std::os::raw::c_int, button: ::std::os::raw::c_int)
// -> bool;
// pub fn GetGamepadButtonPressed() -> ::std::os::raw::c_int;
// pub fn GetGamepadAxisCount(gamepad: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
// pub fn GetGamepadAxisMovement(
//     gamepad: ::std::os::raw::c_int,
//     axis: ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_float;
// pub fn SetGamepadMappings(mappings: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
// pub fn SetGamepadVibration(
//     gamepad: ::std::os::raw::c_int,
//     leftMotor: ::std::os::raw::c_float,
//     rightMotor: ::std::os::raw::c_float,
//     duration: ::std::os::raw::c_float,
// );
// pub fn IsMouseButtonPressed(button: ::std::os::raw::c_int) -> bool;
// pub fn IsMouseButtonDown(button: ::std::os::raw::c_int) -> bool;
// pub fn IsMouseButtonReleased(button: ::std::os::raw::c_int) -> bool;
// pub fn IsMouseButtonUp(button: ::std::os::raw::c_int) -> bool;
// pub fn GetMouseX() -> ::std::os::raw::c_int;
// pub fn GetMouseY() -> ::std::os::raw::c_int;
// pub fn GetMousePosition() -> Vector2;
// pub fn GetMouseDelta() -> Vector2;
// pub fn SetMousePosition(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int);
// pub fn SetMouseOffset(offsetX: ::std::os::raw::c_int, offsetY: ::std::os::raw::c_int);
// pub fn SetMouseScale(scaleX: ::std::os::raw::c_float, scaleY: ::std::os::raw::c_float);
// pub fn GetMouseWheelMove() -> ::std::os::raw::c_float;
// pub fn GetMouseWheelMoveV() -> Vector2;
// pub fn SetMouseCursor(cursor: ::std::os::raw::c_int);
// pub fn GetTouchX() -> ::std::os::raw::c_int;
// pub fn GetTouchY() -> ::std::os::raw::c_int;
// pub fn GetTouchPosition(index: ::std::os::raw::c_int) -> Vector2;
// pub fn GetTouchPointId(index: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
// pub fn GetTouchPointCount() -> ::std::os::raw::c_int;
// pub fn SetGesturesEnabled(flags: ::std::os::raw::c_uint);
// pub fn IsGestureDetected(gesture: ::std::os::raw::c_uint) -> bool;
// pub fn GetGestureDetected() -> ::std::os::raw::c_int;
// pub fn GetGestureHoldDuration() -> ::std::os::raw::c_float;
// pub fn GetGestureDragVector() -> Vector2;
// pub fn GetGestureDragAngle() -> ::std::os::raw::c_float;
// pub fn GetGesturePinchVector() -> Vector2;
// pub fn GetGesturePinchAngle() -> ::std::os::raw::c_float;
// pub fn UpdateCamera(camera: *mut Camera, mode: ::std::os::raw::c_int);
// pub fn UpdateCameraPro(
//     camera: *mut Camera,
//     movement: Vector3,
//     rotation: Vector3,
//     zoom: ::std::os::raw::c_float,
// );
// pub fn SetShapesTexture(texture: Texture2D, source: Rectangle);
// pub fn GetShapesTexture() -> Texture2D;
// pub fn GetShapesTextureRectangle() -> Rectangle;
// pub fn DrawPixel(posX: ::std::os::raw::c_int, posY: ::std::os::raw::c_int, color: Color);
// pub fn DrawPixelV(position: Vector2, color: Color);
// pub fn DrawLine(
//     startPosX: ::std::os::raw::c_int,
//     startPosY: ::std::os::raw::c_int,
//     endPosX: ::std::os::raw::c_int,
//     endPosY: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn DrawLineV(startPos: Vector2, endPos: Vector2, color: Color);
// pub fn DrawLineEx(
//     startPos: Vector2,
//     endPos: Vector2,
//     thick: ::std::os::raw::c_float,
//     color: Color,
// );
// pub fn DrawLineStrip(points: *const Vector2, pointCount: ::std::os::raw::c_int, color: Color);
// pub fn DrawLineBezier(
//     startPos: Vector2,
//     endPos: Vector2,
//     thick: ::std::os::raw::c_float,
//     color: Color,
// );
// pub fn DrawCircle(
//     centerX: ::std::os::raw::c_int,
//     centerY: ::std::os::raw::c_int,
//     radius: ::std::os::raw::c_float,
//     color: Color,
// );
// pub fn DrawCircleSector(
//     center: Vector2,
//     radius: ::std::os::raw::c_float,
//     startAngle: ::std::os::raw::c_float,
//     endAngle: ::std::os::raw::c_float,
//     segments: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn DrawCircleSectorLines(
//     center: Vector2,
//     radius: ::std::os::raw::c_float,
//     startAngle: ::std::os::raw::c_float,
//     endAngle: ::std::os::raw::c_float,
//     segments: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn DrawCircleGradient(
//     centerX: ::std::os::raw::c_int,
//     centerY: ::std::os::raw::c_int,
//     radius: ::std::os::raw::c_float,
//     inner: Color,
//     outer: Color,
// );
// pub fn DrawCircleV(center: Vector2, radius: ::std::os::raw::c_float, color: Color);
// pub fn DrawCircleLines(
//     centerX: ::std::os::raw::c_int,
//     centerY: ::std::os::raw::c_int,
//     radius: ::std::os::raw::c_float,
//     color: Color,
// );
// pub fn DrawCircleLinesV(center: Vector2, radius: ::std::os::raw::c_float, color: Color);
// pub fn DrawEllipse(
//     centerX: ::std::os::raw::c_int,
//     centerY: ::std::os::raw::c_int,
//     radiusH: ::std::os::raw::c_float,
//     radiusV: ::std::os::raw::c_float,
//     color: Color,
// );
// pub fn DrawEllipseV(
//     center: Vector2,
//     radiusH: ::std::os::raw::c_float,
//     radiusV: ::std::os::raw::c_float,
//     color: Color,
// );
// pub fn DrawEllipseLines(
//     centerX: ::std::os::raw::c_int,
//     centerY: ::std::os::raw::c_int,
//     radiusH: ::std::os::raw::c_float,
//     radiusV: ::std::os::raw::c_float,
//     color: Color,
// );
// pub fn DrawEllipseLinesV(
//     center: Vector2,
//     radiusH: ::std::os::raw::c_float,
//     radiusV: ::std::os::raw::c_float,
//     color: Color,
// );
// pub fn DrawRing(
//     center: Vector2,
//     innerRadius: ::std::os::raw::c_float,
//     outerRadius: ::std::os::raw::c_float,
//     startAngle: ::std::os::raw::c_float,
//     endAngle: ::std::os::raw::c_float,
//     segments: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn DrawRingLines(
//     center: Vector2,
//     innerRadius: ::std::os::raw::c_float,
//     outerRadius: ::std::os::raw::c_float,
//     startAngle: ::std::os::raw::c_float,
//     endAngle: ::std::os::raw::c_float,
//     segments: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn DrawRectangle(
//     posX: ::std::os::raw::c_int,
//     posY: ::std::os::raw::c_int,
//     width: ::std::os::raw::c_int,
//     height: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn DrawRectangleV(position: Vector2, size: Vector2, color: Color);
// pub fn DrawRectangleRec(rec: Rectangle, color: Color);
// pub fn DrawRectanglePro(
//     rec: Rectangle,
//     origin: Vector2,
//     rotation: ::std::os::raw::c_float,
//     color: Color,
// );
// pub fn DrawRectangleGradientV(
//     posX: ::std::os::raw::c_int,
//     posY: ::std::os::raw::c_int,
//     width: ::std::os::raw::c_int,
//     height: ::std::os::raw::c_int,
//     top: Color,
//     bottom: Color,
// );
// pub fn DrawRectangleGradientH(
//     posX: ::std::os::raw::c_int,
//     posY: ::std::os::raw::c_int,
//     width: ::std::os::raw::c_int,
//     height: ::std::os::raw::c_int,
//     left: Color,
//     right: Color,
// );
// pub fn DrawRectangleGradientEx(
//     rec: Rectangle,
//     topLeft: Color,
//     bottomLeft: Color,
//     bottomRight: Color,
//     topRight: Color,
// );
// pub fn DrawRectangleLines(
//     posX: ::std::os::raw::c_int,
//     posY: ::std::os::raw::c_int,
//     width: ::std::os::raw::c_int,
//     height: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn DrawRectangleLinesEx(rec: Rectangle, lineThick: ::std::os::raw::c_float, color: Color);
// pub fn DrawRectangleRounded(
//     rec: Rectangle,
//     roundness: ::std::os::raw::c_float,
//     segments: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn DrawRectangleRoundedLines(
//     rec: Rectangle,
//     roundness: ::std::os::raw::c_float,
//     segments: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn DrawRectangleRoundedLinesEx(
//     rec: Rectangle,
//     roundness: ::std::os::raw::c_float,
//     segments: ::std::os::raw::c_int,
//     lineThick: ::std::os::raw::c_float,
//     color: Color,
// );
// pub fn DrawTriangle(v1: Vector2, v2: Vector2, v3: Vector2, color: Color);
// pub fn DrawTriangleLines(v1: Vector2, v2: Vector2, v3: Vector2, color: Color);
// pub fn DrawTriangleFan(points: *const Vector2, pointCount: ::std::os::raw::c_int, color: Color);
// pub fn DrawTriangleStrip(
//     points: *const Vector2,
//     pointCount: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn DrawPoly(
//     center: Vector2,
//     sides: ::std::os::raw::c_int,
//     radius: ::std::os::raw::c_float,
//     rotation: ::std::os::raw::c_float,
//     color: Color,
// );
// pub fn DrawPolyLines(
//     center: Vector2,
//     sides: ::std::os::raw::c_int,
//     radius: ::std::os::raw::c_float,
//     rotation: ::std::os::raw::c_float,
//     color: Color,
// );
// pub fn DrawPolyLinesEx(
//     center: Vector2,
//     sides: ::std::os::raw::c_int,
//     radius: ::std::os::raw::c_float,
//     rotation: ::std::os::raw::c_float,
//     lineThick: ::std::os::raw::c_float,
//     color: Color,
// );
// pub fn DrawSplineLinear(
//     points: *const Vector2,
//     pointCount: ::std::os::raw::c_int,
//     thick: ::std::os::raw::c_float,
//     color: Color,
// );
// pub fn DrawSplineBasis(
//     points: *const Vector2,
//     pointCount: ::std::os::raw::c_int,
//     thick: ::std::os::raw::c_float,
//     color: Color,
// );
// pub fn DrawSplineCatmullRom(
//     points: *const Vector2,
//     pointCount: ::std::os::raw::c_int,
//     thick: ::std::os::raw::c_float,
//     color: Color,
// );
// pub fn DrawSplineBezierQuadratic(
//     points: *const Vector2,
//     pointCount: ::std::os::raw::c_int,
//     thick: ::std::os::raw::c_float,
//     color: Color,
// );
// pub fn DrawSplineBezierCubic(
//     points: *const Vector2,
//     pointCount: ::std::os::raw::c_int,
//     thick: ::std::os::raw::c_float,
//     color: Color,
// );
// pub fn DrawSplineSegmentLinear(
//     p1: Vector2,
//     p2: Vector2,
//     thick: ::std::os::raw::c_float,
//     color: Color,
// );
// pub fn DrawSplineSegmentBasis(
//     p1: Vector2,
//     p2: Vector2,
//     p3: Vector2,
//     p4: Vector2,
//     thick: ::std::os::raw::c_float,
//     color: Color,
// );
// pub fn DrawSplineSegmentCatmullRom(
//     p1: Vector2,
//     p2: Vector2,
//     p3: Vector2,
//     p4: Vector2,
//     thick: ::std::os::raw::c_float,
//     color: Color,
// );
// pub fn DrawSplineSegmentBezierQuadratic(
//     p1: Vector2,
//     c2: Vector2,
//     p3: Vector2,
//     thick: ::std::os::raw::c_float,
//     color: Color,
// );
// pub fn DrawSplineSegmentBezierCubic(
//     p1: Vector2,
//     c2: Vector2,
//     c3: Vector2,
//     p4: Vector2,
//     thick: ::std::os::raw::c_float,
//     color: Color,
// );
// pub fn GetSplinePointLinear(
//     startPos: Vector2,
//     endPos: Vector2,
//     t: ::std::os::raw::c_float,
// ) -> Vector2;
// pub fn GetSplinePointBasis(
//     p1: Vector2,
//     p2: Vector2,
//     p3: Vector2,
//     p4: Vector2,
//     t: ::std::os::raw::c_float,
// ) -> Vector2;
// pub fn GetSplinePointCatmullRom(
//     p1: Vector2,
//     p2: Vector2,
//     p3: Vector2,
//     p4: Vector2,
//     t: ::std::os::raw::c_float,
// ) -> Vector2;
// pub fn GetSplinePointBezierQuad(
//     p1: Vector2,
//     c2: Vector2,
//     p3: Vector2,
//     t: ::std::os::raw::c_float,
// ) -> Vector2;
// pub fn GetSplinePointBezierCubic(
//     p1: Vector2,
//     c2: Vector2,
//     c3: Vector2,
//     p4: Vector2,
//     t: ::std::os::raw::c_float,
// ) -> Vector2;
// pub fn CheckCollisionRecs(rec1: Rectangle, rec2: Rectangle) -> bool;
// pub fn CheckCollisionCircles(
//     center1: Vector2,
//     radius1: ::std::os::raw::c_float,
//     center2: Vector2,
//     radius2: ::std::os::raw::c_float,
// ) -> bool;
// pub fn CheckCollisionCircleRec(
//     center: Vector2,
//     radius: ::std::os::raw::c_float,
//     rec: Rectangle,
// ) -> bool;
// pub fn CheckCollisionCircleLine(
//     center: Vector2,
//     radius: ::std::os::raw::c_float,
//     p1: Vector2,
//     p2: Vector2,
// ) -> bool;
// pub fn CheckCollisionPointRec(point: Vector2, rec: Rectangle) -> bool;
// pub fn CheckCollisionPointCircle(
//     point: Vector2,
//     center: Vector2,
//     radius: ::std::os::raw::c_float,
// ) -> bool;
// pub fn CheckCollisionPointTriangle(
//     point: Vector2,
//     p1: Vector2,
//     p2: Vector2,
//     p3: Vector2,
// ) -> bool;
// pub fn CheckCollisionPointLine(
//     point: Vector2,
//     p1: Vector2,
//     p2: Vector2,
//     threshold: ::std::os::raw::c_int,
// ) -> bool;
// pub fn CheckCollisionPointPoly(
//     point: Vector2,
//     points: *const Vector2,
//     pointCount: ::std::os::raw::c_int,
// ) -> bool;
// pub fn CheckCollisionLines(
//     startPos1: Vector2,
//     endPos1: Vector2,
//     startPos2: Vector2,
//     endPos2: Vector2,
//     collisionPoint: *mut Vector2,
// ) -> bool;
// pub fn GetCollisionRec(rec1: Rectangle, rec2: Rectangle) -> Rectangle;
// pub fn LoadImage(fileName: *const ::std::os::raw::c_char) -> Image;
// pub fn LoadImageRaw(
//     fileName: *const ::std::os::raw::c_char,
//     width: ::std::os::raw::c_int,
//     height: ::std::os::raw::c_int,
//     format: ::std::os::raw::c_int,
//     headerSize: ::std::os::raw::c_int,
// ) -> Image;
// pub fn LoadImageAnim(
//     fileName: *const ::std::os::raw::c_char,
//     frames: *mut ::std::os::raw::c_int,
// ) -> Image;
// pub fn LoadImageAnimFromMemory(
//     fileType: *const ::std::os::raw::c_char,
//     fileData: *const ::std::os::raw::c_uchar,
//     dataSize: ::std::os::raw::c_int,
//     frames: *mut ::std::os::raw::c_int,
// ) -> Image;
// pub fn LoadImageFromMemory(
//     fileType: *const ::std::os::raw::c_char,
//     fileData: *const ::std::os::raw::c_uchar,
//     dataSize: ::std::os::raw::c_int,
// ) -> Image;
// pub fn LoadImageFromTexture(texture: Texture2D) -> Image;
// pub fn LoadImageFromScreen() -> Image;
// pub fn IsImageValid(image: Image) -> bool;
// pub fn UnloadImage(image: Image);
// pub fn ExportImage(image: Image, fileName: *const ::std::os::raw::c_char) -> bool;
// pub fn ExportImageToMemory(
//     image: Image,
//     fileType: *const ::std::os::raw::c_char,
//     fileSize: *mut ::std::os::raw::c_int,
// ) -> *mut ::std::os::raw::c_uchar;
// pub fn ExportImageAsCode(image: Image, fileName: *const ::std::os::raw::c_char) -> bool;
// pub fn GenImageColor(
//     width: ::std::os::raw::c_int,
//     height: ::std::os::raw::c_int,
//     color: Color,
// ) -> Image;
// pub fn GenImageGradientLinear(
//     width: ::std::os::raw::c_int,
//     height: ::std::os::raw::c_int,
//     direction: ::std::os::raw::c_int,
//     start: Color,
//     end: Color,
// ) -> Image;
// pub fn GenImageGradientRadial(
//     width: ::std::os::raw::c_int,
//     height: ::std::os::raw::c_int,
//     density: ::std::os::raw::c_float,
//     inner: Color,
//     outer: Color,
// ) -> Image;
// pub fn GenImageGradientSquare(
//     width: ::std::os::raw::c_int,
//     height: ::std::os::raw::c_int,
//     density: ::std::os::raw::c_float,
//     inner: Color,
//     outer: Color,
// ) -> Image;
// pub fn GenImageChecked(
//     width: ::std::os::raw::c_int,
//     height: ::std::os::raw::c_int,
//     checksX: ::std::os::raw::c_int,
//     checksY: ::std::os::raw::c_int,
//     col1: Color,
//     col2: Color,
// ) -> Image;
// pub fn GenImageWhiteNoise(
//     width: ::std::os::raw::c_int,
//     height: ::std::os::raw::c_int,
//     factor: ::std::os::raw::c_float,
// ) -> Image;
// pub fn GenImagePerlinNoise(
//     width: ::std::os::raw::c_int,
//     height: ::std::os::raw::c_int,
//     offsetX: ::std::os::raw::c_int,
//     offsetY: ::std::os::raw::c_int,
//     scale: ::std::os::raw::c_float,
// ) -> Image;
// pub fn GenImageCellular(
//     width: ::std::os::raw::c_int,
//     height: ::std::os::raw::c_int,
//     tileSize: ::std::os::raw::c_int,
// ) -> Image;
// pub fn GenImageText(
//     width: ::std::os::raw::c_int,
//     height: ::std::os::raw::c_int,
//     text: *const ::std::os::raw::c_char,
// ) -> Image;
// pub fn ImageCopy(image: Image) -> Image;
// pub fn ImageFromImage(image: Image, rec: Rectangle) -> Image;
// pub fn ImageFromChannel(image: Image, selectedChannel: ::std::os::raw::c_int) -> Image;
// pub fn ImageText(
//     text: *const ::std::os::raw::c_char,
//     fontSize: ::std::os::raw::c_int,
//     color: Color,
// ) -> Image;
// pub fn ImageTextEx(
//     font: Font,
//     text: *const ::std::os::raw::c_char,
//     fontSize: ::std::os::raw::c_float,
//     spacing: ::std::os::raw::c_float,
//     tint: Color,
// ) -> Image;
// pub fn ImageFormat(image: *mut Image, newFormat: ::std::os::raw::c_int);
// pub fn ImageToPOT(image: *mut Image, fill: Color);
// pub fn ImageCrop(image: *mut Image, crop: Rectangle);
// pub fn ImageAlphaCrop(image: *mut Image, threshold: ::std::os::raw::c_float);
// pub fn ImageAlphaClear(image: *mut Image, color: Color, threshold: ::std::os::raw::c_float);
// pub fn ImageAlphaMask(image: *mut Image, alphaMask: Image);
// pub fn ImageAlphaPremultiply(image: *mut Image);
// pub fn ImageBlurGaussian(image: *mut Image, blurSize: ::std::os::raw::c_int);
// pub fn ImageKernelConvolution(
//     image: *mut Image,
//     kernel: *const ::std::os::raw::c_float,
//     kernelSize: ::std::os::raw::c_int,
// );
// pub fn ImageResize(
//     image: *mut Image,
//     newWidth: ::std::os::raw::c_int,
//     newHeight: ::std::os::raw::c_int,
// );
// pub fn ImageResizeNN(
//     image: *mut Image,
//     newWidth: ::std::os::raw::c_int,
//     newHeight: ::std::os::raw::c_int,
// );
// pub fn ImageResizeCanvas(
//     image: *mut Image,
//     newWidth: ::std::os::raw::c_int,
//     newHeight: ::std::os::raw::c_int,
//     offsetX: ::std::os::raw::c_int,
//     offsetY: ::std::os::raw::c_int,
//     fill: Color,
// );
// pub fn ImageMipmaps(image: *mut Image);
// pub fn ImageDither(
//     image: *mut Image,
//     rBpp: ::std::os::raw::c_int,
//     gBpp: ::std::os::raw::c_int,
//     bBpp: ::std::os::raw::c_int,
//     aBpp: ::std::os::raw::c_int,
// );
// pub fn ImageFlipVertical(image: *mut Image);
// pub fn ImageFlipHorizontal(image: *mut Image);
// pub fn ImageRotate(image: *mut Image, degrees: ::std::os::raw::c_int);
// pub fn ImageRotateCW(image: *mut Image);
// pub fn ImageRotateCCW(image: *mut Image);
// pub fn ImageColorTint(image: *mut Image, color: Color);
// pub fn ImageColorInvert(image: *mut Image);
// pub fn ImageColorGrayscale(image: *mut Image);
// pub fn ImageColorContrast(image: *mut Image, contrast: ::std::os::raw::c_float);
// pub fn ImageColorBrightness(image: *mut Image, brightness: ::std::os::raw::c_int);
// pub fn ImageColorReplace(image: *mut Image, color: Color, replace: Color);
// pub fn LoadImageColors(image: Image) -> *mut Color;
// pub fn LoadImagePalette(
//     image: Image,
//     maxPaletteSize: ::std::os::raw::c_int,
//     colorCount: *mut ::std::os::raw::c_int,
// ) -> *mut Color;
// pub fn UnloadImageColors(colors: *mut Color);
// pub fn UnloadImagePalette(colors: *mut Color);
// pub fn GetImageAlphaBorder(image: Image, threshold: ::std::os::raw::c_float) -> Rectangle;
// pub fn GetImageColor(image: Image, x: ::std::os::raw::c_int, y: ::std::os::raw::c_int)
// -> Color;
// pub fn ImageClearBackground(dst: *mut Image, color: Color);
// pub fn ImageDrawPixel(
//     dst: *mut Image,
//     posX: ::std::os::raw::c_int,
//     posY: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn ImageDrawPixelV(dst: *mut Image, position: Vector2, color: Color);
// pub fn ImageDrawLine(
//     dst: *mut Image,
//     startPosX: ::std::os::raw::c_int,
//     startPosY: ::std::os::raw::c_int,
//     endPosX: ::std::os::raw::c_int,
//     endPosY: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn ImageDrawLineV(dst: *mut Image, start: Vector2, end: Vector2, color: Color);
// pub fn ImageDrawLineEx(
//     dst: *mut Image,
//     start: Vector2,
//     end: Vector2,
//     thick: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn ImageDrawCircle(
//     dst: *mut Image,
//     centerX: ::std::os::raw::c_int,
//     centerY: ::std::os::raw::c_int,
//     radius: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn ImageDrawCircleV(
//     dst: *mut Image,
//     center: Vector2,
//     radius: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn ImageDrawCircleLines(
//     dst: *mut Image,
//     centerX: ::std::os::raw::c_int,
//     centerY: ::std::os::raw::c_int,
//     radius: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn ImageDrawCircleLinesV(
//     dst: *mut Image,
//     center: Vector2,
//     radius: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn ImageDrawRectangle(
//     dst: *mut Image,
//     posX: ::std::os::raw::c_int,
//     posY: ::std::os::raw::c_int,
//     width: ::std::os::raw::c_int,
//     height: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn ImageDrawRectangleV(dst: *mut Image, position: Vector2, size: Vector2, color: Color);
// pub fn ImageDrawRectangleRec(dst: *mut Image, rec: Rectangle, color: Color);
// pub fn ImageDrawRectangleLines(
//     dst: *mut Image,
//     rec: Rectangle,
//     thick: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn ImageDrawTriangle(dst: *mut Image, v1: Vector2, v2: Vector2, v3: Vector2, color: Color);
// pub fn ImageDrawTriangleEx(
//     dst: *mut Image,
//     v1: Vector2,
//     v2: Vector2,
//     v3: Vector2,
//     c1: Color,
//     c2: Color,
//     c3: Color,
// );
// pub fn ImageDrawTriangleLines(
//     dst: *mut Image,
//     v1: Vector2,
//     v2: Vector2,
//     v3: Vector2,
//     color: Color,
// );
// pub fn ImageDrawTriangleFan(
//     dst: *mut Image,
//     points: *const Vector2,
//     pointCount: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn ImageDrawTriangleStrip(
//     dst: *mut Image,
//     points: *const Vector2,
//     pointCount: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn ImageDraw(
//     dst: *mut Image,
//     src: Image,
//     srcRec: Rectangle,
//     dstRec: Rectangle,
//     tint: Color,
// );
// pub fn ImageDrawText(
//     dst: *mut Image,
//     text: *const ::std::os::raw::c_char,
//     posX: ::std::os::raw::c_int,
//     posY: ::std::os::raw::c_int,
//     fontSize: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn ImageDrawTextEx(
//     dst: *mut Image,
//     font: Font,
//     text: *const ::std::os::raw::c_char,
//     position: Vector2,
//     fontSize: ::std::os::raw::c_float,
//     spacing: ::std::os::raw::c_float,
//     tint: Color,
// );
// pub fn LoadTexture(fileName: *const ::std::os::raw::c_char) -> Texture2D;
// pub fn LoadTextureFromImage(image: Image) -> Texture2D;
// pub fn LoadTextureCubemap(image: Image, layout: ::std::os::raw::c_int) -> TextureCubemap;
// pub fn LoadRenderTexture(
//     width: ::std::os::raw::c_int,
//     height: ::std::os::raw::c_int,
// ) -> RenderTexture2D;
// pub fn IsTextureValid(texture: Texture2D) -> bool;
// pub fn UnloadTexture(texture: Texture2D);
// pub fn IsRenderTextureValid(target: RenderTexture2D) -> bool;
// pub fn UnloadRenderTexture(target: RenderTexture2D);
// pub fn UpdateTexture(texture: Texture2D, pixels: *const ::std::os::raw::c_void);
// pub fn UpdateTextureRec(
//     texture: Texture2D,
//     rec: Rectangle,
//     pixels: *const ::std::os::raw::c_void,
// );
// pub fn GenTextureMipmaps(texture: *mut Texture2D);
// pub fn SetTextureFilter(texture: Texture2D, filter: ::std::os::raw::c_int);
// pub fn SetTextureWrap(texture: Texture2D, wrap: ::std::os::raw::c_int);
// pub fn DrawTexture(
//     texture: Texture2D,
//     posX: ::std::os::raw::c_int,
//     posY: ::std::os::raw::c_int,
//     tint: Color,
// );
// pub fn DrawTextureV(texture: Texture2D, position: Vector2, tint: Color);
// pub fn DrawTextureEx(
//     texture: Texture2D,
//     position: Vector2,
//     rotation: ::std::os::raw::c_float,
//     scale: ::std::os::raw::c_float,
//     tint: Color,
// );
// pub fn DrawTextureRec(texture: Texture2D, source: Rectangle, position: Vector2, tint: Color);
// pub fn DrawTexturePro(
//     texture: Texture2D,
//     source: Rectangle,
//     dest: Rectangle,
//     origin: Vector2,
//     rotation: ::std::os::raw::c_float,
//     tint: Color,
// );
// pub fn DrawTextureNPatch(
//     texture: Texture2D,
//     nPatchInfo: NPatchInfo,
//     dest: Rectangle,
//     origin: Vector2,
//     rotation: ::std::os::raw::c_float,
//     tint: Color,
// );
// pub fn ColorIsEqual(col1: Color, col2: Color) -> bool;
// pub fn Fade(color: Color, alpha: ::std::os::raw::c_float) -> Color;
// pub fn ColorToInt(color: Color) -> ::std::os::raw::c_int;
// pub fn ColorNormalize(color: Color) -> Vector4;
// pub fn ColorFromNormalized(normalized: Vector4) -> Color;
// pub fn ColorToHSV(color: Color) -> Vector3;
// pub fn ColorFromHSV(
//     hue: ::std::os::raw::c_float,
//     saturation: ::std::os::raw::c_float,
//     value: ::std::os::raw::c_float,
// ) -> Color;
// pub fn ColorTint(color: Color, tint: Color) -> Color;
// pub fn ColorBrightness(color: Color, factor: ::std::os::raw::c_float) -> Color;
// pub fn ColorContrast(color: Color, contrast: ::std::os::raw::c_float) -> Color;
// pub fn ColorAlpha(color: Color, alpha: ::std::os::raw::c_float) -> Color;
// pub fn ColorAlphaBlend(dst: Color, src: Color, tint: Color) -> Color;
// pub fn ColorLerp(color1: Color, color2: Color, factor: ::std::os::raw::c_float) -> Color;
// pub fn GetColor(hexValue: ::std::os::raw::c_uint) -> Color;
// pub fn GetPixelColor(
//     srcPtr: *mut ::std::os::raw::c_void,
//     format: ::std::os::raw::c_int,
// ) -> Color;
// pub fn SetPixelColor(
//     dstPtr: *mut ::std::os::raw::c_void,
//     color: Color,
//     format: ::std::os::raw::c_int,
// );
// pub fn GetPixelDataSize(
//     width: ::std::os::raw::c_int,
//     height: ::std::os::raw::c_int,
//     format: ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_int;
// pub fn GetFontDefault() -> Font;
// pub fn LoadFont(fileName: *const ::std::os::raw::c_char) -> Font;
// pub fn LoadFontEx(
//     fileName: *const ::std::os::raw::c_char,
//     fontSize: ::std::os::raw::c_int,
//     codepoints: *mut ::std::os::raw::c_int,
//     codepointCount: ::std::os::raw::c_int,
// ) -> Font;
// pub fn LoadFontFromImage(image: Image, key: Color, firstChar: ::std::os::raw::c_int) -> Font;
// pub fn LoadFontFromMemory(
//     fileType: *const ::std::os::raw::c_char,
//     fileData: *const ::std::os::raw::c_uchar,
//     dataSize: ::std::os::raw::c_int,
//     fontSize: ::std::os::raw::c_int,
//     codepoints: *mut ::std::os::raw::c_int,
//     codepointCount: ::std::os::raw::c_int,
// ) -> Font;
// pub fn IsFontValid(font: Font) -> bool;
// pub fn LoadFontData(
//     fileData: *const ::std::os::raw::c_uchar,
//     dataSize: ::std::os::raw::c_int,
//     fontSize: ::std::os::raw::c_int,
//     codepoints: *mut ::std::os::raw::c_int,
//     codepointCount: ::std::os::raw::c_int,
//     type_: ::std::os::raw::c_int,
// ) -> *mut GlyphInfo;
// pub fn GenImageFontAtlas(
//     glyphs: *const GlyphInfo,
//     glyphRecs: *mut *mut Rectangle,
//     glyphCount: ::std::os::raw::c_int,
//     fontSize: ::std::os::raw::c_int,
//     padding: ::std::os::raw::c_int,
//     packMethod: ::std::os::raw::c_int,
// ) -> Image;
// pub fn UnloadFontData(glyphs: *mut GlyphInfo, glyphCount: ::std::os::raw::c_int);
// pub fn UnloadFont(font: Font);
// pub fn ExportFontAsCode(font: Font, fileName: *const ::std::os::raw::c_char) -> bool;
// pub fn DrawFPS(posX: ::std::os::raw::c_int, posY: ::std::os::raw::c_int);
// pub fn DrawText(
//     text: *const ::std::os::raw::c_char,
//     posX: ::std::os::raw::c_int,
//     posY: ::std::os::raw::c_int,
//     fontSize: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn DrawTextEx(
//     font: Font,
//     text: *const ::std::os::raw::c_char,
//     position: Vector2,
//     fontSize: ::std::os::raw::c_float,
//     spacing: ::std::os::raw::c_float,
//     tint: Color,
// );
// pub fn DrawTextPro(
//     font: Font,
//     text: *const ::std::os::raw::c_char,
//     position: Vector2,
//     origin: Vector2,
//     rotation: ::std::os::raw::c_float,
//     fontSize: ::std::os::raw::c_float,
//     spacing: ::std::os::raw::c_float,
//     tint: Color,
// );
// pub fn DrawTextCodepoint(
//     font: Font,
//     codepoint: ::std::os::raw::c_int,
//     position: Vector2,
//     fontSize: ::std::os::raw::c_float,
//     tint: Color,
// );
// pub fn DrawTextCodepoints(
//     font: Font,
//     codepoints: *const ::std::os::raw::c_int,
//     codepointCount: ::std::os::raw::c_int,
//     position: Vector2,
//     fontSize: ::std::os::raw::c_float,
//     spacing: ::std::os::raw::c_float,
//     tint: Color,
// );
// pub fn SetTextLineSpacing(spacing: ::std::os::raw::c_int);
// pub fn MeasureText(
//     text: *const ::std::os::raw::c_char,
//     fontSize: ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_int;
// pub fn MeasureTextEx(
//     font: Font,
//     text: *const ::std::os::raw::c_char,
//     fontSize: ::std::os::raw::c_float,
//     spacing: ::std::os::raw::c_float,
// ) -> Vector2;
// pub fn GetGlyphIndex(font: Font, codepoint: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
// pub fn GetGlyphInfo(font: Font, codepoint: ::std::os::raw::c_int) -> GlyphInfo;
// pub fn GetGlyphAtlasRec(font: Font, codepoint: ::std::os::raw::c_int) -> Rectangle;
// pub fn LoadUTF8(
//     codepoints: *const ::std::os::raw::c_int,
//     length: ::std::os::raw::c_int,
// ) -> *mut ::std::os::raw::c_char;
// pub fn UnloadUTF8(text: *mut ::std::os::raw::c_char);
// pub fn LoadCodepoints(
//     text: *const ::std::os::raw::c_char,
//     count: *mut ::std::os::raw::c_int,
// ) -> *mut ::std::os::raw::c_int;
// pub fn UnloadCodepoints(codepoints: *mut ::std::os::raw::c_int);
// pub fn GetCodepointCount(text: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
// pub fn GetCodepoint(
//     text: *const ::std::os::raw::c_char,
//     codepointSize: *mut ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_int;
// pub fn GetCodepointNext(
//     text: *const ::std::os::raw::c_char,
//     codepointSize: *mut ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_int;
// pub fn GetCodepointPrevious(
//     text: *const ::std::os::raw::c_char,
//     codepointSize: *mut ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_int;
// pub fn CodepointToUTF8(
//     codepoint: ::std::os::raw::c_int,
//     utf8Size: *mut ::std::os::raw::c_int,
// ) -> *const ::std::os::raw::c_char;
// pub fn TextCopy(
//     dst: *mut ::std::os::raw::c_char,
//     src: *const ::std::os::raw::c_char,
// ) -> ::std::os::raw::c_int;
// pub fn TextIsEqual(
//     text1: *const ::std::os::raw::c_char,
//     text2: *const ::std::os::raw::c_char,
// ) -> bool;
// pub fn TextLength(text: *const ::std::os::raw::c_char) -> ::std::os::raw::c_uint;
// pub fn TextFormat(text: *const ::std::os::raw::c_char, ...) -> *const ::std::os::raw::c_char;
// pub fn TextSubtext(
//     text: *const ::std::os::raw::c_char,
//     position: ::std::os::raw::c_int,
//     length: ::std::os::raw::c_int,
// ) -> *const ::std::os::raw::c_char;
// pub fn TextReplace(
//     text: *const ::std::os::raw::c_char,
//     replace: *const ::std::os::raw::c_char,
//     by: *const ::std::os::raw::c_char,
// ) -> *mut ::std::os::raw::c_char;
// pub fn TextInsert(
//     text: *const ::std::os::raw::c_char,
//     insert: *const ::std::os::raw::c_char,
//     position: ::std::os::raw::c_int,
// ) -> *mut ::std::os::raw::c_char;
// pub fn TextJoin(
//     textList: *mut *mut ::std::os::raw::c_char,
//     count: ::std::os::raw::c_int,
//     delimiter: *const ::std::os::raw::c_char,
// ) -> *mut ::std::os::raw::c_char;
// pub fn TextSplit(
//     text: *const ::std::os::raw::c_char,
//     delimiter: ::std::os::raw::c_char,
//     count: *mut ::std::os::raw::c_int,
// ) -> *mut *mut ::std::os::raw::c_char;
// pub fn TextAppend(
//     text: *mut ::std::os::raw::c_char,
//     append: *const ::std::os::raw::c_char,
//     position: *mut ::std::os::raw::c_int,
// );
// pub fn TextFindIndex(
//     text: *const ::std::os::raw::c_char,
//     find: *const ::std::os::raw::c_char,
// ) -> ::std::os::raw::c_int;
// pub fn TextToUpper(text: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
// pub fn TextToLower(text: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
// pub fn TextToPascal(text: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
// pub fn TextToSnake(text: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
// pub fn TextToCamel(text: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
// pub fn TextToInteger(text: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
// pub fn TextToFloat(text: *const ::std::os::raw::c_char) -> ::std::os::raw::c_float;
// pub fn DrawLine3D(startPos: Vector3, endPos: Vector3, color: Color);
// pub fn DrawPoint3D(position: Vector3, color: Color);
// pub fn DrawCircle3D(
//     center: Vector3,
//     radius: ::std::os::raw::c_float,
//     rotationAxis: Vector3,
//     rotationAngle: ::std::os::raw::c_float,
//     color: Color,
// );
// pub fn DrawTriangle3D(v1: Vector3, v2: Vector3, v3: Vector3, color: Color);
// pub fn DrawTriangleStrip3D(
//     points: *const Vector3,
//     pointCount: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn DrawCube(
//     position: Vector3,
//     width: ::std::os::raw::c_float,
//     height: ::std::os::raw::c_float,
//     length: ::std::os::raw::c_float,
//     color: Color,
// );
// pub fn DrawCubeV(position: Vector3, size: Vector3, color: Color);
// pub fn DrawCubeWires(
//     position: Vector3,
//     width: ::std::os::raw::c_float,
//     height: ::std::os::raw::c_float,
//     length: ::std::os::raw::c_float,
//     color: Color,
// );
// pub fn DrawCubeWiresV(position: Vector3, size: Vector3, color: Color);
// pub fn DrawSphere(centerPos: Vector3, radius: ::std::os::raw::c_float, color: Color);
// pub fn DrawSphereEx(
//     centerPos: Vector3,
//     radius: ::std::os::raw::c_float,
//     rings: ::std::os::raw::c_int,
//     slices: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn DrawSphereWires(
//     centerPos: Vector3,
//     radius: ::std::os::raw::c_float,
//     rings: ::std::os::raw::c_int,
//     slices: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn DrawCylinder(
//     position: Vector3,
//     radiusTop: ::std::os::raw::c_float,
//     radiusBottom: ::std::os::raw::c_float,
//     height: ::std::os::raw::c_float,
//     slices: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn DrawCylinderEx(
//     startPos: Vector3,
//     endPos: Vector3,
//     startRadius: ::std::os::raw::c_float,
//     endRadius: ::std::os::raw::c_float,
//     sides: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn DrawCylinderWires(
//     position: Vector3,
//     radiusTop: ::std::os::raw::c_float,
//     radiusBottom: ::std::os::raw::c_float,
//     height: ::std::os::raw::c_float,
//     slices: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn DrawCylinderWiresEx(
//     startPos: Vector3,
//     endPos: Vector3,
//     startRadius: ::std::os::raw::c_float,
//     endRadius: ::std::os::raw::c_float,
//     sides: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn DrawCapsule(
//     startPos: Vector3,
//     endPos: Vector3,
//     radius: ::std::os::raw::c_float,
//     slices: ::std::os::raw::c_int,
//     rings: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn DrawCapsuleWires(
//     startPos: Vector3,
//     endPos: Vector3,
//     radius: ::std::os::raw::c_float,
//     slices: ::std::os::raw::c_int,
//     rings: ::std::os::raw::c_int,
//     color: Color,
// );
// pub fn DrawPlane(centerPos: Vector3, size: Vector2, color: Color);
// pub fn DrawRay(ray: Ray, color: Color);
// pub fn DrawGrid(slices: ::std::os::raw::c_int, spacing: ::std::os::raw::c_float);
// pub fn LoadModel(fileName: *const ::std::os::raw::c_char) -> Model;
// pub fn LoadModelFromMesh(mesh: Mesh) -> Model;
// pub fn IsModelValid(model: Model) -> bool;
// pub fn UnloadModel(model: Model);
// pub fn GetModelBoundingBox(model: Model) -> BoundingBox;
// pub fn DrawModel(model: Model, position: Vector3, scale: ::std::os::raw::c_float, tint: Color);
// pub fn DrawModelEx(
//     model: Model,
//     position: Vector3,
//     rotationAxis: Vector3,
//     rotationAngle: ::std::os::raw::c_float,
//     scale: Vector3,
//     tint: Color,
// );
// pub fn DrawModelWires(
//     model: Model,
//     position: Vector3,
//     scale: ::std::os::raw::c_float,
//     tint: Color,
// );
// pub fn DrawModelWiresEx(
//     model: Model,
//     position: Vector3,
//     rotationAxis: Vector3,
//     rotationAngle: ::std::os::raw::c_float,
//     scale: Vector3,
//     tint: Color,
// );
// pub fn DrawModelPoints(
//     model: Model,
//     position: Vector3,
//     scale: ::std::os::raw::c_float,
//     tint: Color,
// );
// pub fn DrawModelPointsEx(
//     model: Model,
//     position: Vector3,
//     rotationAxis: Vector3,
//     rotationAngle: ::std::os::raw::c_float,
//     scale: Vector3,
//     tint: Color,
// );
// pub fn DrawBoundingBox(box_: BoundingBox, color: Color);
// pub fn DrawBillboard(
//     camera: Camera,
//     texture: Texture2D,
//     position: Vector3,
//     scale: ::std::os::raw::c_float,
//     tint: Color,
// );
// pub fn DrawBillboardRec(
//     camera: Camera,
//     texture: Texture2D,
//     source: Rectangle,
//     position: Vector3,
//     size: Vector2,
//     tint: Color,
// );
// pub fn DrawBillboardPro(
//     camera: Camera,
//     texture: Texture2D,
//     source: Rectangle,
//     position: Vector3,
//     up: Vector3,
//     size: Vector2,
//     origin: Vector2,
//     rotation: ::std::os::raw::c_float,
//     tint: Color,
// );
// pub fn UploadMesh(mesh: *mut Mesh, dynamic: bool);
// pub fn UpdateMeshBuffer(
//     mesh: Mesh,
//     index: ::std::os::raw::c_int,
//     data: *const ::std::os::raw::c_void,
//     dataSize: ::std::os::raw::c_int,
//     offset: ::std::os::raw::c_int,
// );
// pub fn UnloadMesh(mesh: Mesh);
// pub fn DrawMesh(mesh: Mesh, material: Material, transform: Matrix);
// pub fn DrawMeshInstanced(
//     mesh: Mesh,
//     material: Material,
//     transforms: *const Matrix,
//     instances: ::std::os::raw::c_int,
// );
// pub fn GetMeshBoundingBox(mesh: Mesh) -> BoundingBox;
// pub fn GenMeshTangents(mesh: *mut Mesh);
// pub fn ExportMesh(mesh: Mesh, fileName: *const ::std::os::raw::c_char) -> bool;
// pub fn ExportMeshAsCode(mesh: Mesh, fileName: *const ::std::os::raw::c_char) -> bool;
// pub fn GenMeshPoly(sides: ::std::os::raw::c_int, radius: ::std::os::raw::c_float) -> Mesh;
// pub fn GenMeshPlane(
//     width: ::std::os::raw::c_float,
//     length: ::std::os::raw::c_float,
//     resX: ::std::os::raw::c_int,
//     resZ: ::std::os::raw::c_int,
// ) -> Mesh;
// pub fn GenMeshCube(
//     width: ::std::os::raw::c_float,
//     height: ::std::os::raw::c_float,
//     length: ::std::os::raw::c_float,
// ) -> Mesh;
// pub fn GenMeshSphere(
//     radius: ::std::os::raw::c_float,
//     rings: ::std::os::raw::c_int,
//     slices: ::std::os::raw::c_int,
// ) -> Mesh;
// pub fn GenMeshHemiSphere(
//     radius: ::std::os::raw::c_float,
//     rings: ::std::os::raw::c_int,
//     slices: ::std::os::raw::c_int,
// ) -> Mesh;
// pub fn GenMeshCylinder(
//     radius: ::std::os::raw::c_float,
//     height: ::std::os::raw::c_float,
//     slices: ::std::os::raw::c_int,
// ) -> Mesh;
// pub fn GenMeshCone(
//     radius: ::std::os::raw::c_float,
//     height: ::std::os::raw::c_float,
//     slices: ::std::os::raw::c_int,
// ) -> Mesh;
// pub fn GenMeshTorus(
//     radius: ::std::os::raw::c_float,
//     size: ::std::os::raw::c_float,
//     radSeg: ::std::os::raw::c_int,
//     sides: ::std::os::raw::c_int,
// ) -> Mesh;
// pub fn GenMeshKnot(
//     radius: ::std::os::raw::c_float,
//     size: ::std::os::raw::c_float,
//     radSeg: ::std::os::raw::c_int,
//     sides: ::std::os::raw::c_int,
// ) -> Mesh;
// pub fn GenMeshHeightmap(heightmap: Image, size: Vector3) -> Mesh;
// pub fn GenMeshCubicmap(cubicmap: Image, cubeSize: Vector3) -> Mesh;
// pub fn LoadMaterials(
//     fileName: *const ::std::os::raw::c_char,
//     materialCount: *mut ::std::os::raw::c_int,
// ) -> *mut Material;
// pub fn LoadMaterialDefault() -> Material;
// pub fn IsMaterialValid(material: Material) -> bool;
// pub fn UnloadMaterial(material: Material);
// pub fn SetMaterialTexture(
//     material: *mut Material,
//     mapType: ::std::os::raw::c_int,
//     texture: Texture2D,
// );
// pub fn SetModelMeshMaterial(
//     model: *mut Model,
//     meshId: ::std::os::raw::c_int,
//     materialId: ::std::os::raw::c_int,
// );
// pub fn LoadModelAnimations(
//     fileName: *const ::std::os::raw::c_char,
//     animCount: *mut ::std::os::raw::c_int,
// ) -> *mut ModelAnimation;
// pub fn UpdateModelAnimation(model: Model, anim: ModelAnimation, frame: ::std::os::raw::c_int);
// pub fn UpdateModelAnimationBones(
//     model: Model,
//     anim: ModelAnimation,
//     frame: ::std::os::raw::c_int,
// );
// pub fn UnloadModelAnimation(anim: ModelAnimation);
// pub fn UnloadModelAnimations(animations: *mut ModelAnimation, animCount: ::std::os::raw::c_int);
// pub fn IsModelAnimationValid(model: Model, anim: ModelAnimation) -> bool;
// pub fn CheckCollisionSpheres(
//     center1: Vector3,
//     radius1: ::std::os::raw::c_float,
//     center2: Vector3,
//     radius2: ::std::os::raw::c_float,
// ) -> bool;
// pub fn CheckCollisionBoxes(box1: BoundingBox, box2: BoundingBox) -> bool;
// pub fn CheckCollisionBoxSphere(
//     box_: BoundingBox,
//     center: Vector3,
//     radius: ::std::os::raw::c_float,
// ) -> bool;
// pub fn GetRayCollisionSphere(
//     ray: Ray,
//     center: Vector3,
//     radius: ::std::os::raw::c_float,
// ) -> RayCollision;
// pub fn GetRayCollisionBox(ray: Ray, box_: BoundingBox) -> RayCollision;
// pub fn GetRayCollisionMesh(ray: Ray, mesh: Mesh, transform: Matrix) -> RayCollision;
// pub fn GetRayCollisionTriangle(ray: Ray, p1: Vector3, p2: Vector3, p3: Vector3)
// -> RayCollision;
// pub fn GetRayCollisionQuad(
//     ray: Ray,
//     p1: Vector3,
//     p2: Vector3,
//     p3: Vector3,
//     p4: Vector3,
// ) -> RayCollision;
// pub fn InitAudioDevice();
// pub fn CloseAudioDevice();
// pub fn IsAudioDeviceReady() -> bool;
// pub fn SetMasterVolume(volume: ::std::os::raw::c_float);
// pub fn GetMasterVolume() -> ::std::os::raw::c_float;
// pub fn LoadWave(fileName: *const ::std::os::raw::c_char) -> Wave;
// pub fn LoadWaveFromMemory(
//     fileType: *const ::std::os::raw::c_char,
//     fileData: *const ::std::os::raw::c_uchar,
//     dataSize: ::std::os::raw::c_int,
// ) -> Wave;
// pub fn IsWaveValid(wave: Wave) -> bool;
// pub fn LoadSound(fileName: *const ::std::os::raw::c_char) -> Sound;
// pub fn LoadSoundFromWave(wave: Wave) -> Sound;
// pub fn LoadSoundAlias(source: Sound) -> Sound;
// pub fn IsSoundValid(sound: Sound) -> bool;
// pub fn UpdateSound(
//     sound: Sound,
//     data: *const ::std::os::raw::c_void,
//     sampleCount: ::std::os::raw::c_int,
// );
// pub fn UnloadWave(wave: Wave);
// pub fn UnloadSound(sound: Sound);
// pub fn UnloadSoundAlias(alias: Sound);
// pub fn ExportWave(wave: Wave, fileName: *const ::std::os::raw::c_char) -> bool;
// pub fn ExportWaveAsCode(wave: Wave, fileName: *const ::std::os::raw::c_char) -> bool;
// pub fn PlaySound(sound: Sound);
// pub fn StopSound(sound: Sound);
// pub fn PauseSound(sound: Sound);
// pub fn ResumeSound(sound: Sound);
// pub fn IsSoundPlaying(sound: Sound) -> bool;
// pub fn SetSoundVolume(sound: Sound, volume: ::std::os::raw::c_float);
// pub fn SetSoundPitch(sound: Sound, pitch: ::std::os::raw::c_float);
// pub fn SetSoundPan(sound: Sound, pan: ::std::os::raw::c_float);
// pub fn WaveCopy(wave: Wave) -> Wave;
// pub fn WaveCrop(
//     wave: *mut Wave,
//     initFrame: ::std::os::raw::c_int,
//     finalFrame: ::std::os::raw::c_int,
// );
// pub fn WaveFormat(
//     wave: *mut Wave,
//     sampleRate: ::std::os::raw::c_int,
//     sampleSize: ::std::os::raw::c_int,
//     channels: ::std::os::raw::c_int,
// );
// pub fn LoadWaveSamples(wave: Wave) -> *mut ::std::os::raw::c_float;
// pub fn UnloadWaveSamples(samples: *mut ::std::os::raw::c_float);
// pub fn LoadMusicStream(fileName: *const ::std::os::raw::c_char) -> Music;
// pub fn LoadMusicStreamFromMemory(
//     fileType: *const ::std::os::raw::c_char,
//     data: *const ::std::os::raw::c_uchar,
//     dataSize: ::std::os::raw::c_int,
// ) -> Music;
// pub fn IsMusicValid(music: Music) -> bool;
// pub fn UnloadMusicStream(music: Music);
// pub fn PlayMusicStream(music: Music);
// pub fn IsMusicStreamPlaying(music: Music) -> bool;
// pub fn UpdateMusicStream(music: Music);
// pub fn StopMusicStream(music: Music);
// pub fn PauseMusicStream(music: Music);
// pub fn ResumeMusicStream(music: Music);
// pub fn SeekMusicStream(music: Music, position: ::std::os::raw::c_float);
// pub fn SetMusicVolume(music: Music, volume: ::std::os::raw::c_float);
// pub fn SetMusicPitch(music: Music, pitch: ::std::os::raw::c_float);
// pub fn SetMusicPan(music: Music, pan: ::std::os::raw::c_float);
// pub fn GetMusicTimeLength(music: Music) -> ::std::os::raw::c_float;
// pub fn GetMusicTimePlayed(music: Music) -> ::std::os::raw::c_float;
// pub fn LoadAudioStream(
//     sampleRate: ::std::os::raw::c_uint,
//     sampleSize: ::std::os::raw::c_uint,
//     channels: ::std::os::raw::c_uint,
// ) -> AudioStream;
// pub fn IsAudioStreamValid(stream: AudioStream) -> bool;
// pub fn UnloadAudioStream(stream: AudioStream);
// pub fn UpdateAudioStream(
//     stream: AudioStream,
//     data: *const ::std::os::raw::c_void,
//     frameCount: ::std::os::raw::c_int,
// );
// pub fn IsAudioStreamProcessed(stream: AudioStream) -> bool;
// pub fn PlayAudioStream(stream: AudioStream);
// pub fn PauseAudioStream(stream: AudioStream);
// pub fn ResumeAudioStream(stream: AudioStream);
// pub fn IsAudioStreamPlaying(stream: AudioStream) -> bool;
// pub fn StopAudioStream(stream: AudioStream);
// pub fn SetAudioStreamVolume(stream: AudioStream, volume: ::std::os::raw::c_float);
// pub fn SetAudioStreamPitch(stream: AudioStream, pitch: ::std::os::raw::c_float);
// pub fn SetAudioStreamPan(stream: AudioStream, pan: ::std::os::raw::c_float);
// pub fn SetAudioStreamBufferSizeDefault(size: ::std::os::raw::c_int);
// pub fn SetAudioStreamCallback(stream: AudioStream, callback: AudioCallback);
// pub fn AttachAudioStreamProcessor(stream: AudioStream, processor: AudioCallback);
// pub fn DetachAudioStreamProcessor(stream: AudioStream, processor: AudioCallback);
// pub fn AttachAudioMixedProcessor(processor: AudioCallback);
// pub fn DetachAudioMixedProcessor(processor: AudioCallback);
// pub fn __va_start(arg1: *mut *mut ::std::os::raw::c_char, ...);
// pub fn __security_init_cookie();
// pub fn __security_check_cookie(_StackCookie: usize);
// pub fn __report_gsfailure(_StackCookie: usize) -> !;
// pub static mut __security_cookie: usize;
// pub fn _invalid_parameter_noinfo();
// pub fn _invalid_parameter_noinfo_noreturn() -> !;
// pub fn _invoke_watson(
//     _Expression: *const wchar_t,
//     _FunctionName: *const wchar_t,
//     _FileName: *const wchar_t,
//     _LineNo: ::std::os::raw::c_uint,
//     _Reserved: usize,
// ) -> !;
// pub static _HUGE: ::std::os::raw::c_double;
// pub fn _fperrraise(_Except: ::std::os::raw::c_int);
// pub fn _dclass(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_short;
// pub fn _ldclass(_X: f64) -> ::std::os::raw::c_short;
// pub fn _fdclass(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_short;
// pub fn _dsign(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_int;
// pub fn _ldsign(_X: f64) -> ::std::os::raw::c_int;
// pub fn _fdsign(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_int;
// pub fn _dpcomp(
//     _X: ::std::os::raw::c_double,
//     _Y: ::std::os::raw::c_double,
// ) -> ::std::os::raw::c_int;
// pub fn _ldpcomp(_X: f64, _Y: f64) -> ::std::os::raw::c_int;
// pub fn _fdpcomp(
//     _X: ::std::os::raw::c_float,
//     _Y: ::std::os::raw::c_float,
// ) -> ::std::os::raw::c_int;
// pub fn _dtest(_Px: *mut ::std::os::raw::c_double) -> ::std::os::raw::c_short;
// pub fn _ldtest(_Px: *mut f64) -> ::std::os::raw::c_short;
// pub fn _fdtest(_Px: *mut ::std::os::raw::c_float) -> ::std::os::raw::c_short;
// pub fn _d_int(
//     _Px: *mut ::std::os::raw::c_double,
//     _Xexp: ::std::os::raw::c_short,
// ) -> ::std::os::raw::c_short;
// pub fn _ld_int(_Px: *mut f64, _Xexp: ::std::os::raw::c_short) -> ::std::os::raw::c_short;
// pub fn _fd_int(
//     _Px: *mut ::std::os::raw::c_float,
//     _Xexp: ::std::os::raw::c_short,
// ) -> ::std::os::raw::c_short;
// pub fn _dscale(
//     _Px: *mut ::std::os::raw::c_double,
//     _Lexp: ::std::os::raw::c_long,
// ) -> ::std::os::raw::c_short;
// pub fn _ldscale(_Px: *mut f64, _Lexp: ::std::os::raw::c_long) -> ::std::os::raw::c_short;
// pub fn _fdscale(
//     _Px: *mut ::std::os::raw::c_float,
//     _Lexp: ::std::os::raw::c_long,
// ) -> ::std::os::raw::c_short;
// pub fn _dunscale(
//     _Pex: *mut ::std::os::raw::c_short,
//     _Px: *mut ::std::os::raw::c_double,
// ) -> ::std::os::raw::c_short;
// pub fn _ldunscale(_Pex: *mut ::std::os::raw::c_short, _Px: *mut f64)
// -> ::std::os::raw::c_short;
// pub fn _fdunscale(
//     _Pex: *mut ::std::os::raw::c_short,
//     _Px: *mut ::std::os::raw::c_float,
// ) -> ::std::os::raw::c_short;
// pub fn _dexp(
//     _Px: *mut ::std::os::raw::c_double,
//     _Y: ::std::os::raw::c_double,
//     _Eoff: ::std::os::raw::c_long,
// ) -> ::std::os::raw::c_short;
// pub fn _ldexp(_Px: *mut f64, _Y: f64, _Eoff: ::std::os::raw::c_long)
// -> ::std::os::raw::c_short;
// pub fn _fdexp(
//     _Px: *mut ::std::os::raw::c_float,
//     _Y: ::std::os::raw::c_float,
//     _Eoff: ::std::os::raw::c_long,
// ) -> ::std::os::raw::c_short;
// pub fn _dnorm(_Ps: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_short;
// pub fn _fdnorm(_Ps: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_short;
// pub fn _dpoly(
//     _X: ::std::os::raw::c_double,
//     _Tab: *const ::std::os::raw::c_double,
//     _N: ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_double;
// pub fn _ldpoly(_X: f64, _Tab: *const f64, _N: ::std::os::raw::c_int) -> f64;
// pub fn _fdpoly(
//     _X: ::std::os::raw::c_float,
//     _Tab: *const ::std::os::raw::c_float,
//     _N: ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_float;
// pub fn _dlog(
//     _X: ::std::os::raw::c_double,
//     _Baseflag: ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_double;
// pub fn _ldlog(_X: f64, _Baseflag: ::std::os::raw::c_int) -> f64;
// pub fn _fdlog(
//     _X: ::std::os::raw::c_float,
//     _Baseflag: ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_float;
// pub fn _dsin(
//     _X: ::std::os::raw::c_double,
//     _Qoff: ::std::os::raw::c_uint,
// ) -> ::std::os::raw::c_double;
// pub fn _ldsin(_X: f64, _Qoff: ::std::os::raw::c_uint) -> f64;
// pub fn _fdsin(
//     _X: ::std::os::raw::c_float,
//     _Qoff: ::std::os::raw::c_uint,
// ) -> ::std::os::raw::c_float;
// pub static _Denorm_C: _float_const;
// pub static _Inf_C: _float_const;
// pub static _Nan_C: _float_const;
// pub static _Snan_C: _float_const;
// pub static _Hugeval_C: _float_const;
// pub static _FDenorm_C: _float_const;
// pub static _FInf_C: _float_const;
// pub static _FNan_C: _float_const;
// pub static _FSnan_C: _float_const;
// pub static _LDenorm_C: _float_const;
// pub static _LInf_C: _float_const;
// pub static _LNan_C: _float_const;
// pub static _LSnan_C: _float_const;
// pub static _Eps_C: _float_const;
// pub static _Rteps_C: _float_const;
// pub static _FEps_C: _float_const;
// pub static _FRteps_C: _float_const;
// pub static _LEps_C: _float_const;
// pub static _LRteps_C: _float_const;
// pub static _Zero_C: ::std::os::raw::c_double;
// pub static _Xbig_C: ::std::os::raw::c_double;
// pub static _FZero_C: ::std::os::raw::c_float;
// pub static _FXbig_C: ::std::os::raw::c_float;
// pub static _LZero_C: f64;
// pub static _LXbig_C: f64;
// pub fn abs(_X: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
// pub fn labs(_X: ::std::os::raw::c_long) -> ::std::os::raw::c_long;
// pub fn llabs(_X: ::std::os::raw::c_longlong) -> ::std::os::raw::c_longlong;
// pub fn acos(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn asin(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn atan(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn atan2(
//     _Y: ::std::os::raw::c_double,
//     _X: ::std::os::raw::c_double,
// ) -> ::std::os::raw::c_double;
// pub fn cos(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn cosh(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn exp(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn fabs(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn fmod(
//     _X: ::std::os::raw::c_double,
//     _Y: ::std::os::raw::c_double,
// ) -> ::std::os::raw::c_double;
// pub fn log(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn log10(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn pow(
//     _X: ::std::os::raw::c_double,
//     _Y: ::std::os::raw::c_double,
// ) -> ::std::os::raw::c_double;
// pub fn sin(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn sinh(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn sqrt(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn tan(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn tanh(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn acosh(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn asinh(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn atanh(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn atof(_String: *const ::std::os::raw::c_char) -> ::std::os::raw::c_double;
// pub fn _atof_l(
//     _String: *const ::std::os::raw::c_char,
//     _Locale: _locale_t,
// ) -> ::std::os::raw::c_double;
// pub fn _cabs(_Complex_value: _complex) -> ::std::os::raw::c_double;
// pub fn cbrt(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn ceil(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn _chgsign(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn copysign(
//     _Number: ::std::os::raw::c_double,
//     _Sign: ::std::os::raw::c_double,
// ) -> ::std::os::raw::c_double;
// pub fn _copysign(
//     _Number: ::std::os::raw::c_double,
//     _Sign: ::std::os::raw::c_double,
// ) -> ::std::os::raw::c_double;
// pub fn erf(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn erfc(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn exp2(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn expm1(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn fdim(
//     _X: ::std::os::raw::c_double,
//     _Y: ::std::os::raw::c_double,
// ) -> ::std::os::raw::c_double;
// pub fn floor(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn fma(
//     _X: ::std::os::raw::c_double,
//     _Y: ::std::os::raw::c_double,
//     _Z: ::std::os::raw::c_double,
// ) -> ::std::os::raw::c_double;
// pub fn fmax(
//     _X: ::std::os::raw::c_double,
//     _Y: ::std::os::raw::c_double,
// ) -> ::std::os::raw::c_double;
// pub fn fmin(
//     _X: ::std::os::raw::c_double,
//     _Y: ::std::os::raw::c_double,
// ) -> ::std::os::raw::c_double;
// pub fn frexp(
//     _X: ::std::os::raw::c_double,
//     _Y: *mut ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_double;
// pub fn hypot(
//     _X: ::std::os::raw::c_double,
//     _Y: ::std::os::raw::c_double,
// ) -> ::std::os::raw::c_double;
// pub fn _hypot(
//     _X: ::std::os::raw::c_double,
//     _Y: ::std::os::raw::c_double,
// ) -> ::std::os::raw::c_double;
// pub fn ilogb(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_int;
// pub fn ldexp(
//     _X: ::std::os::raw::c_double,
//     _Y: ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_double;
// pub fn lgamma(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn llrint(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_longlong;
// pub fn llround(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_longlong;
// pub fn log1p(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn log2(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn logb(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn lrint(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_long;
// pub fn lround(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_long;
// pub fn _matherr(_Except: *mut _exception) -> ::std::os::raw::c_int;
// pub fn modf(
//     _X: ::std::os::raw::c_double,
//     _Y: *mut ::std::os::raw::c_double,
// ) -> ::std::os::raw::c_double;
// pub fn nan(_X: *const ::std::os::raw::c_char) -> ::std::os::raw::c_double;
// pub fn nearbyint(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn nextafter(
//     _X: ::std::os::raw::c_double,
//     _Y: ::std::os::raw::c_double,
// ) -> ::std::os::raw::c_double;
// pub fn nexttoward(_X: ::std::os::raw::c_double, _Y: f64) -> ::std::os::raw::c_double;
// pub fn remainder(
//     _X: ::std::os::raw::c_double,
//     _Y: ::std::os::raw::c_double,
// ) -> ::std::os::raw::c_double;
// pub fn remquo(
//     _X: ::std::os::raw::c_double,
//     _Y: ::std::os::raw::c_double,
//     _Z: *mut ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_double;
// pub fn rint(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn round(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn scalbln(
//     _X: ::std::os::raw::c_double,
//     _Y: ::std::os::raw::c_long,
// ) -> ::std::os::raw::c_double;
// pub fn scalbn(
//     _X: ::std::os::raw::c_double,
//     _Y: ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_double;
// pub fn tgamma(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn trunc(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn _j0(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn _j1(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn _jn(_X: ::std::os::raw::c_int, _Y: ::std::os::raw::c_double)
// -> ::std::os::raw::c_double;
// pub fn _y0(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn _y1(_X: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
// pub fn _yn(_X: ::std::os::raw::c_int, _Y: ::std::os::raw::c_double)
// -> ::std::os::raw::c_double;
// pub fn acoshf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn asinhf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn atanhf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn cbrtf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn _chgsignf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn copysignf(
//     _Number: ::std::os::raw::c_float,
//     _Sign: ::std::os::raw::c_float,
// ) -> ::std::os::raw::c_float;
// pub fn _copysignf(
//     _Number: ::std::os::raw::c_float,
//     _Sign: ::std::os::raw::c_float,
// ) -> ::std::os::raw::c_float;
// pub fn erff(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn erfcf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn expm1f(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn exp2f(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn fdimf(
//     _X: ::std::os::raw::c_float,
//     _Y: ::std::os::raw::c_float,
// ) -> ::std::os::raw::c_float;
// pub fn fmaf(
//     _X: ::std::os::raw::c_float,
//     _Y: ::std::os::raw::c_float,
//     _Z: ::std::os::raw::c_float,
// ) -> ::std::os::raw::c_float;
// pub fn fmaxf(
//     _X: ::std::os::raw::c_float,
//     _Y: ::std::os::raw::c_float,
// ) -> ::std::os::raw::c_float;
// pub fn fminf(
//     _X: ::std::os::raw::c_float,
//     _Y: ::std::os::raw::c_float,
// ) -> ::std::os::raw::c_float;
// pub fn _hypotf(
//     _X: ::std::os::raw::c_float,
//     _Y: ::std::os::raw::c_float,
// ) -> ::std::os::raw::c_float;
// pub fn ilogbf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_int;
// pub fn lgammaf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn llrintf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_longlong;
// pub fn llroundf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_longlong;
// pub fn log1pf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn log2f(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn logbf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn lrintf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_long;
// pub fn lroundf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_long;
// pub fn nanf(_X: *const ::std::os::raw::c_char) -> ::std::os::raw::c_float;
// pub fn nearbyintf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn nextafterf(
//     _X: ::std::os::raw::c_float,
//     _Y: ::std::os::raw::c_float,
// ) -> ::std::os::raw::c_float;
// pub fn nexttowardf(_X: ::std::os::raw::c_float, _Y: f64) -> ::std::os::raw::c_float;
// pub fn remainderf(
//     _X: ::std::os::raw::c_float,
//     _Y: ::std::os::raw::c_float,
// ) -> ::std::os::raw::c_float;
// pub fn remquof(
//     _X: ::std::os::raw::c_float,
//     _Y: ::std::os::raw::c_float,
//     _Z: *mut ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_float;
// pub fn rintf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn roundf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn scalblnf(
//     _X: ::std::os::raw::c_float,
//     _Y: ::std::os::raw::c_long,
// ) -> ::std::os::raw::c_float;
// pub fn scalbnf(
//     _X: ::std::os::raw::c_float,
//     _Y: ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_float;
// pub fn tgammaf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn truncf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn _logbf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn _nextafterf(
//     _X: ::std::os::raw::c_float,
//     _Y: ::std::os::raw::c_float,
// ) -> ::std::os::raw::c_float;
// pub fn _finitef(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_int;
// pub fn _isnanf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_int;
// pub fn _fpclassf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_int;
// pub fn _set_FMA3_enable(_Flag: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
// pub fn _get_FMA3_enable() -> ::std::os::raw::c_int;
// pub fn acosf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn asinf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn atan2f(
//     _Y: ::std::os::raw::c_float,
//     _X: ::std::os::raw::c_float,
// ) -> ::std::os::raw::c_float;
// pub fn atanf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn ceilf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn cosf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn coshf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn expf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn floorf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn fmodf(
//     _X: ::std::os::raw::c_float,
//     _Y: ::std::os::raw::c_float,
// ) -> ::std::os::raw::c_float;
// pub fn log10f(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn logf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn modff(
//     _X: ::std::os::raw::c_float,
//     _Y: *mut ::std::os::raw::c_float,
// ) -> ::std::os::raw::c_float;
// pub fn powf(
//     _X: ::std::os::raw::c_float,
//     _Y: ::std::os::raw::c_float,
// ) -> ::std::os::raw::c_float;
// pub fn sinf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn sinhf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn sqrtf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn tanf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn tanhf(_X: ::std::os::raw::c_float) -> ::std::os::raw::c_float;
// pub fn acoshl(_X: f64) -> f64;
// pub fn asinhl(_X: f64) -> f64;
// pub fn atanhl(_X: f64) -> f64;
// pub fn cbrtl(_X: f64) -> f64;
// pub fn copysignl(_Number: f64, _Sign: f64) -> f64;
// pub fn erfl(_X: f64) -> f64;
// pub fn erfcl(_X: f64) -> f64;
// pub fn exp2l(_X: f64) -> f64;
// pub fn expm1l(_X: f64) -> f64;
// pub fn fdiml(_X: f64, _Y: f64) -> f64;
// pub fn fmal(_X: f64, _Y: f64, _Z: f64) -> f64;
// pub fn fmaxl(_X: f64, _Y: f64) -> f64;
// pub fn fminl(_X: f64, _Y: f64) -> f64;
// pub fn ilogbl(_X: f64) -> ::std::os::raw::c_int;
// pub fn lgammal(_X: f64) -> f64;
// pub fn llrintl(_X: f64) -> ::std::os::raw::c_longlong;
// pub fn llroundl(_X: f64) -> ::std::os::raw::c_longlong;
// pub fn log1pl(_X: f64) -> f64;
// pub fn log2l(_X: f64) -> f64;
// pub fn logbl(_X: f64) -> f64;
// pub fn lrintl(_X: f64) -> ::std::os::raw::c_long;
// pub fn lroundl(_X: f64) -> ::std::os::raw::c_long;
// pub fn nanl(_X: *const ::std::os::raw::c_char) -> f64;
// pub fn nearbyintl(_X: f64) -> f64;
// pub fn nextafterl(_X: f64, _Y: f64) -> f64;
// pub fn nexttowardl(_X: f64, _Y: f64) -> f64;
// pub fn remainderl(_X: f64, _Y: f64) -> f64;
// pub fn remquol(_X: f64, _Y: f64, _Z: *mut ::std::os::raw::c_int) -> f64;
// pub fn rintl(_X: f64) -> f64;
// pub fn roundl(_X: f64) -> f64;
// pub fn scalblnl(_X: f64, _Y: ::std::os::raw::c_long) -> f64;
// pub fn scalbnl(_X: f64, _Y: ::std::os::raw::c_int) -> f64;
// pub fn tgammal(_X: f64) -> f64;
// pub fn truncl(_X: f64) -> f64;
// pub fn GetCameraForward(camera: *mut Camera) -> Vector3;
// pub fn GetCameraUp(camera: *mut Camera) -> Vector3;
// pub fn GetCameraRight(camera: *mut Camera) -> Vector3;
// pub fn CameraMoveForward(
//     camera: *mut Camera,
//     distance: ::std::os::raw::c_float,
//     moveInWorldPlane: bool,
// );
// pub fn CameraMoveUp(camera: *mut Camera, distance: ::std::os::raw::c_float);
// pub fn CameraMoveRight(
//     camera: *mut Camera,
//     distance: ::std::os::raw::c_float,
//     moveInWorldPlane: bool,
// );
// pub fn CameraMoveToTarget(camera: *mut Camera, delta: ::std::os::raw::c_float);
// pub fn CameraYaw(camera: *mut Camera, angle: ::std::os::raw::c_float, rotateAroundTarget: bool);
// pub fn CameraPitch(
//     camera: *mut Camera,
//     angle: ::std::os::raw::c_float,
//     lockView: bool,
//     rotateAroundTarget: bool,
//     rotateUp: bool,
// );
// pub fn CameraRoll(camera: *mut Camera, angle: ::std::os::raw::c_float);
// pub fn GetCameraViewMatrix(camera: *mut Camera) -> Matrix;
// pub fn GetCameraProjectionMatrix(
//     camera: *mut Camera,
//     aspect: ::std::os::raw::c_float,
// ) -> Matrix;
// pub fn ProcessGestureEvent(event: GestureEvent);
// pub fn UpdateGestures();
// pub fn rlMatrixMode(mode: ::std::os::raw::c_int);
// pub fn rlPushMatrix();
// pub fn rlPopMatrix();
// pub fn rlLoadIdentity();
// pub fn rlTranslatef(
//     x: ::std::os::raw::c_float,
//     y: ::std::os::raw::c_float,
//     z: ::std::os::raw::c_float,
// );
// pub fn rlRotatef(
//     angle: ::std::os::raw::c_float,
//     x: ::std::os::raw::c_float,
//     y: ::std::os::raw::c_float,
//     z: ::std::os::raw::c_float,
// );
// pub fn rlScalef(
//     x: ::std::os::raw::c_float,
//     y: ::std::os::raw::c_float,
//     z: ::std::os::raw::c_float,
// );
// pub fn rlMultMatrixf(matf: *const ::std::os::raw::c_float);
// pub fn rlFrustum(
//     left: ::std::os::raw::c_double,
//     right: ::std::os::raw::c_double,
//     bottom: ::std::os::raw::c_double,
//     top: ::std::os::raw::c_double,
//     znear: ::std::os::raw::c_double,
//     zfar: ::std::os::raw::c_double,
// );
// pub fn rlOrtho(
//     left: ::std::os::raw::c_double,
//     right: ::std::os::raw::c_double,
//     bottom: ::std::os::raw::c_double,
//     top: ::std::os::raw::c_double,
//     znear: ::std::os::raw::c_double,
//     zfar: ::std::os::raw::c_double,
// );
// pub fn rlViewport(
//     x: ::std::os::raw::c_int,
//     y: ::std::os::raw::c_int,
//     width: ::std::os::raw::c_int,
//     height: ::std::os::raw::c_int,
// );
// pub fn rlSetClipPlanes(nearPlane: ::std::os::raw::c_double, farPlane: ::std::os::raw::c_double);
// pub fn rlGetCullDistanceNear() -> ::std::os::raw::c_double;
// pub fn rlGetCullDistanceFar() -> ::std::os::raw::c_double;
// pub fn rlBegin(mode: ::std::os::raw::c_int);
// pub fn rlEnd();
// pub fn rlVertex2i(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int);
// pub fn rlVertex2f(x: ::std::os::raw::c_float, y: ::std::os::raw::c_float);
// pub fn rlVertex3f(
//     x: ::std::os::raw::c_float,
//     y: ::std::os::raw::c_float,
//     z: ::std::os::raw::c_float,
// );
// pub fn rlTexCoord2f(x: ::std::os::raw::c_float, y: ::std::os::raw::c_float);
// pub fn rlNormal3f(
//     x: ::std::os::raw::c_float,
//     y: ::std::os::raw::c_float,
//     z: ::std::os::raw::c_float,
// );
// pub fn rlColor4ub(
//     r: ::std::os::raw::c_uchar,
//     g: ::std::os::raw::c_uchar,
//     b: ::std::os::raw::c_uchar,
//     a: ::std::os::raw::c_uchar,
// );
// pub fn rlColor3f(
//     x: ::std::os::raw::c_float,
//     y: ::std::os::raw::c_float,
//     z: ::std::os::raw::c_float,
// );
// pub fn rlColor4f(
//     x: ::std::os::raw::c_float,
//     y: ::std::os::raw::c_float,
//     z: ::std::os::raw::c_float,
//     w: ::std::os::raw::c_float,
// );
// pub fn rlEnableVertexArray(vaoId: ::std::os::raw::c_uint) -> bool;
// pub fn rlDisableVertexArray();
// pub fn rlEnableVertexBuffer(id: ::std::os::raw::c_uint);
// pub fn rlDisableVertexBuffer();
// pub fn rlEnableVertexBufferElement(id: ::std::os::raw::c_uint);
// pub fn rlDisableVertexBufferElement();
// pub fn rlEnableVertexAttribute(index: ::std::os::raw::c_uint);
// pub fn rlDisableVertexAttribute(index: ::std::os::raw::c_uint);
// pub fn rlActiveTextureSlot(slot: ::std::os::raw::c_int);
// pub fn rlEnableTexture(id: ::std::os::raw::c_uint);
// pub fn rlDisableTexture();
// pub fn rlEnableTextureCubemap(id: ::std::os::raw::c_uint);
// pub fn rlDisableTextureCubemap();
// pub fn rlTextureParameters(
//     id: ::std::os::raw::c_uint,
//     param: ::std::os::raw::c_int,
//     value: ::std::os::raw::c_int,
// );
// pub fn rlCubemapParameters(
//     id: ::std::os::raw::c_uint,
//     param: ::std::os::raw::c_int,
//     value: ::std::os::raw::c_int,
// );
// pub fn rlEnableShader(id: ::std::os::raw::c_uint);
// pub fn rlDisableShader();
// pub fn rlEnableFramebuffer(id: ::std::os::raw::c_uint);
// pub fn rlDisableFramebuffer();
// pub fn rlGetActiveFramebuffer() -> ::std::os::raw::c_uint;
// pub fn rlActiveDrawBuffers(count: ::std::os::raw::c_int);
// pub fn rlBlitFramebuffer(
//     srcX: ::std::os::raw::c_int,
//     srcY: ::std::os::raw::c_int,
//     srcWidth: ::std::os::raw::c_int,
//     srcHeight: ::std::os::raw::c_int,
//     dstX: ::std::os::raw::c_int,
//     dstY: ::std::os::raw::c_int,
//     dstWidth: ::std::os::raw::c_int,
//     dstHeight: ::std::os::raw::c_int,
//     bufferMask: ::std::os::raw::c_int,
// );
// pub fn rlBindFramebuffer(target: ::std::os::raw::c_uint, framebuffer: ::std::os::raw::c_uint);
// pub fn rlEnableColorBlend();
// pub fn rlDisableColorBlend();
// pub fn rlEnableDepthTest();
// pub fn rlDisableDepthTest();
// pub fn rlEnableDepthMask();
// pub fn rlDisableDepthMask();
// pub fn rlEnableBackfaceCulling();
// pub fn rlDisableBackfaceCulling();
// pub fn rlColorMask(r: bool, g: bool, b: bool, a: bool);
// pub fn rlSetCullFace(mode: ::std::os::raw::c_int);
// pub fn rlEnableScissorTest();
// pub fn rlDisableScissorTest();
// pub fn rlScissor(
//     x: ::std::os::raw::c_int,
//     y: ::std::os::raw::c_int,
//     width: ::std::os::raw::c_int,
//     height: ::std::os::raw::c_int,
// );
// pub fn rlEnablePointMode();
// pub fn rlDisablePointMode();
// pub fn rlEnableWireMode();
// pub fn rlDisableWireMode();
// pub fn rlSetLineWidth(width: ::std::os::raw::c_float);
// pub fn rlGetLineWidth() -> ::std::os::raw::c_float;
// pub fn rlEnableSmoothLines();
// pub fn rlDisableSmoothLines();
// pub fn rlEnableStereoRender();
// pub fn rlDisableStereoRender();
// pub fn rlIsStereoRenderEnabled() -> bool;
// pub fn rlClearColor(
//     r: ::std::os::raw::c_uchar,
//     g: ::std::os::raw::c_uchar,
//     b: ::std::os::raw::c_uchar,
//     a: ::std::os::raw::c_uchar,
// );
// pub fn rlClearScreenBuffers();
// pub fn rlCheckErrors();
// pub fn rlSetBlendMode(mode: ::std::os::raw::c_int);
// pub fn rlSetBlendFactors(
//     glSrcFactor: ::std::os::raw::c_int,
//     glDstFactor: ::std::os::raw::c_int,
//     glEquation: ::std::os::raw::c_int,
// );
// pub fn rlSetBlendFactorsSeparate(
//     glSrcRGB: ::std::os::raw::c_int,
//     glDstRGB: ::std::os::raw::c_int,
//     glSrcAlpha: ::std::os::raw::c_int,
//     glDstAlpha: ::std::os::raw::c_int,
//     glEqRGB: ::std::os::raw::c_int,
//     glEqAlpha: ::std::os::raw::c_int,
// );
// pub fn rlglInit(width: ::std::os::raw::c_int, height: ::std::os::raw::c_int);
// pub fn rlglClose();
// pub fn rlLoadExtensions(loader: *mut ::std::os::raw::c_void);
// pub fn rlGetVersion() -> ::std::os::raw::c_int;
// pub fn rlSetFramebufferWidth(width: ::std::os::raw::c_int);
// pub fn rlGetFramebufferWidth() -> ::std::os::raw::c_int;
// pub fn rlSetFramebufferHeight(height: ::std::os::raw::c_int);
// pub fn rlGetFramebufferHeight() -> ::std::os::raw::c_int;
// pub fn rlGetTextureIdDefault() -> ::std::os::raw::c_uint;
// pub fn rlGetShaderIdDefault() -> ::std::os::raw::c_uint;
// pub fn rlGetShaderLocsDefault() -> *mut ::std::os::raw::c_int;
// pub fn rlLoadRenderBatch(
//     numBuffers: ::std::os::raw::c_int,
//     bufferElements: ::std::os::raw::c_int,
// ) -> rlRenderBatch;
// pub fn rlUnloadRenderBatch(batch: rlRenderBatch);
// pub fn rlDrawRenderBatch(batch: *mut rlRenderBatch);
// pub fn rlSetRenderBatchActive(batch: *mut rlRenderBatch);
// pub fn rlDrawRenderBatchActive();
// pub fn rlCheckRenderBatchLimit(vCount: ::std::os::raw::c_int) -> bool;
// pub fn rlSetTexture(id: ::std::os::raw::c_uint);
// pub fn rlLoadVertexArray() -> ::std::os::raw::c_uint;
// pub fn rlLoadVertexBuffer(
//     buffer: *const ::std::os::raw::c_void,
//     size: ::std::os::raw::c_int,
//     dynamic: bool,
// ) -> ::std::os::raw::c_uint;
// pub fn rlLoadVertexBufferElement(
//     buffer: *const ::std::os::raw::c_void,
//     size: ::std::os::raw::c_int,
//     dynamic: bool,
// ) -> ::std::os::raw::c_uint;
// pub fn rlUpdateVertexBuffer(
//     bufferId: ::std::os::raw::c_uint,
//     data: *const ::std::os::raw::c_void,
//     dataSize: ::std::os::raw::c_int,
//     offset: ::std::os::raw::c_int,
// );
// pub fn rlUpdateVertexBufferElements(
//     id: ::std::os::raw::c_uint,
//     data: *const ::std::os::raw::c_void,
//     dataSize: ::std::os::raw::c_int,
//     offset: ::std::os::raw::c_int,
// );
// pub fn rlUnloadVertexArray(vaoId: ::std::os::raw::c_uint);
// pub fn rlUnloadVertexBuffer(vboId: ::std::os::raw::c_uint);
// pub fn rlSetVertexAttribute(
//     index: ::std::os::raw::c_uint,
//     compSize: ::std::os::raw::c_int,
//     type_: ::std::os::raw::c_int,
//     normalized: bool,
//     stride: ::std::os::raw::c_int,
//     offset: ::std::os::raw::c_int,
// );
// pub fn rlSetVertexAttributeDivisor(
//     index: ::std::os::raw::c_uint,
//     divisor: ::std::os::raw::c_int,
// );
// pub fn rlSetVertexAttributeDefault(
//     locIndex: ::std::os::raw::c_int,
//     value: *const ::std::os::raw::c_void,
//     attribType: ::std::os::raw::c_int,
//     count: ::std::os::raw::c_int,
// );
// pub fn rlDrawVertexArray(offset: ::std::os::raw::c_int, count: ::std::os::raw::c_int);
// pub fn rlDrawVertexArrayElements(
//     offset: ::std::os::raw::c_int,
//     count: ::std::os::raw::c_int,
//     buffer: *const ::std::os::raw::c_void,
// );
// pub fn rlDrawVertexArrayInstanced(
//     offset: ::std::os::raw::c_int,
//     count: ::std::os::raw::c_int,
//     instances: ::std::os::raw::c_int,
// );
// pub fn rlDrawVertexArrayElementsInstanced(
//     offset: ::std::os::raw::c_int,
//     count: ::std::os::raw::c_int,
//     buffer: *const ::std::os::raw::c_void,
//     instances: ::std::os::raw::c_int,
// );
// pub fn rlLoadTexture(
//     data: *const ::std::os::raw::c_void,
//     width: ::std::os::raw::c_int,
//     height: ::std::os::raw::c_int,
//     format: ::std::os::raw::c_int,
//     mipmapCount: ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_uint;
// pub fn rlLoadTextureDepth(
//     width: ::std::os::raw::c_int,
//     height: ::std::os::raw::c_int,
//     useRenderBuffer: bool,
// ) -> ::std::os::raw::c_uint;
// pub fn rlLoadTextureCubemap(
//     data: *const ::std::os::raw::c_void,
//     size: ::std::os::raw::c_int,
//     format: ::std::os::raw::c_int,
//     mipmapCount: ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_uint;
// pub fn rlUpdateTexture(
//     id: ::std::os::raw::c_uint,
//     offsetX: ::std::os::raw::c_int,
//     offsetY: ::std::os::raw::c_int,
//     width: ::std::os::raw::c_int,
//     height: ::std::os::raw::c_int,
//     format: ::std::os::raw::c_int,
//     data: *const ::std::os::raw::c_void,
// );
// pub fn rlGetGlTextureFormats(
//     format: ::std::os::raw::c_int,
//     glInternalFormat: *mut ::std::os::raw::c_uint,
//     glFormat: *mut ::std::os::raw::c_uint,
//     glType: *mut ::std::os::raw::c_uint,
// );
// pub fn rlGetPixelFormatName(format: ::std::os::raw::c_uint) -> *const ::std::os::raw::c_char;
// pub fn rlUnloadTexture(id: ::std::os::raw::c_uint);
// pub fn rlGenTextureMipmaps(
//     id: ::std::os::raw::c_uint,
//     width: ::std::os::raw::c_int,
//     height: ::std::os::raw::c_int,
//     format: ::std::os::raw::c_int,
//     mipmaps: *mut ::std::os::raw::c_int,
// );
// pub fn rlReadTexturePixels(
//     id: ::std::os::raw::c_uint,
//     width: ::std::os::raw::c_int,
//     height: ::std::os::raw::c_int,
//     format: ::std::os::raw::c_int,
// ) -> *mut ::std::os::raw::c_void;
// pub fn rlReadScreenPixels(
//     width: ::std::os::raw::c_int,
//     height: ::std::os::raw::c_int,
// ) -> *mut ::std::os::raw::c_uchar;
// pub fn rlLoadFramebuffer() -> ::std::os::raw::c_uint;
// pub fn rlFramebufferAttach(
//     fboId: ::std::os::raw::c_uint,
//     texId: ::std::os::raw::c_uint,
//     attachType: ::std::os::raw::c_int,
//     texType: ::std::os::raw::c_int,
//     mipLevel: ::std::os::raw::c_int,
// );
// pub fn rlFramebufferComplete(id: ::std::os::raw::c_uint) -> bool;
// pub fn rlUnloadFramebuffer(id: ::std::os::raw::c_uint);
// pub fn rlLoadShaderCode(
//     vsCode: *const ::std::os::raw::c_char,
//     fsCode: *const ::std::os::raw::c_char,
// ) -> ::std::os::raw::c_uint;
// pub fn rlCompileShader(
//     shaderCode: *const ::std::os::raw::c_char,
//     type_: ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_uint;
// pub fn rlLoadShaderProgram(
//     vShaderId: ::std::os::raw::c_uint,
//     fShaderId: ::std::os::raw::c_uint,
// ) -> ::std::os::raw::c_uint;
// pub fn rlUnloadShaderProgram(id: ::std::os::raw::c_uint);
// pub fn rlGetLocationUniform(
//     shaderId: ::std::os::raw::c_uint,
//     uniformName: *const ::std::os::raw::c_char,
// ) -> ::std::os::raw::c_int;
// pub fn rlGetLocationAttrib(
//     shaderId: ::std::os::raw::c_uint,
//     attribName: *const ::std::os::raw::c_char,
// ) -> ::std::os::raw::c_int;
// pub fn rlSetUniform(
//     locIndex: ::std::os::raw::c_int,
//     value: *const ::std::os::raw::c_void,
//     uniformType: ::std::os::raw::c_int,
//     count: ::std::os::raw::c_int,
// );
// pub fn rlSetUniformMatrix(locIndex: ::std::os::raw::c_int, mat: Matrix);
// pub fn rlSetUniformMatrices(
//     locIndex: ::std::os::raw::c_int,
//     mat: *const Matrix,
//     count: ::std::os::raw::c_int,
// );
// pub fn rlSetUniformSampler(locIndex: ::std::os::raw::c_int, textureId: ::std::os::raw::c_uint);
// pub fn rlSetShader(id: ::std::os::raw::c_uint, locs: *mut ::std::os::raw::c_int);
// pub fn rlLoadComputeShaderProgram(shaderId: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;
// pub fn rlComputeShaderDispatch(
//     groupX: ::std::os::raw::c_uint,
//     groupY: ::std::os::raw::c_uint,
//     groupZ: ::std::os::raw::c_uint,
// );
// pub fn rlLoadShaderBuffer(
//     size: ::std::os::raw::c_uint,
//     data: *const ::std::os::raw::c_void,
//     usageHint: ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_uint;
// pub fn rlUnloadShaderBuffer(ssboId: ::std::os::raw::c_uint);
// pub fn rlUpdateShaderBuffer(
//     id: ::std::os::raw::c_uint,
//     data: *const ::std::os::raw::c_void,
//     dataSize: ::std::os::raw::c_uint,
//     offset: ::std::os::raw::c_uint,
// );
// pub fn rlBindShaderBuffer(id: ::std::os::raw::c_uint, index: ::std::os::raw::c_uint);
// pub fn rlReadShaderBuffer(
//     id: ::std::os::raw::c_uint,
//     dest: *mut ::std::os::raw::c_void,
//     count: ::std::os::raw::c_uint,
//     offset: ::std::os::raw::c_uint,
// );
// pub fn rlCopyShaderBuffer(
//     destId: ::std::os::raw::c_uint,
//     srcId: ::std::os::raw::c_uint,
//     destOffset: ::std::os::raw::c_uint,
//     srcOffset: ::std::os::raw::c_uint,
//     count: ::std::os::raw::c_uint,
// );
// pub fn rlGetShaderBufferSize(id: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;
// pub fn rlBindImageTexture(
//     id: ::std::os::raw::c_uint,
//     index: ::std::os::raw::c_uint,
//     format: ::std::os::raw::c_int,
//     readonly: bool,
// );
// pub fn rlGetMatrixModelview() -> Matrix;
// pub fn rlGetMatrixProjection() -> Matrix;
// pub fn rlGetMatrixTransform() -> Matrix;
// pub fn rlGetMatrixProjectionStereo(eye: ::std::os::raw::c_int) -> Matrix;
// pub fn rlGetMatrixViewOffsetStereo(eye: ::std::os::raw::c_int) -> Matrix;
// pub fn rlSetMatrixProjection(proj: Matrix);
// pub fn rlSetMatrixModelview(view: Matrix);
// pub fn rlSetMatrixProjectionStereo(right: Matrix, left: Matrix);
// pub fn rlSetMatrixViewOffsetStereo(right: Matrix, left: Matrix);
// pub fn rlLoadDrawCube();
// pub fn rlLoadDrawQuad();
