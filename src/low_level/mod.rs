use std::ffi::{CStr, CString, NulError};
use std::marker::PhantomData;
use std::mem::{MaybeUninit, transmute};
use std::num::NonZero;
use std::ops::{Deref, DerefMut};
use std::os::raw::{c_int, c_void, c_char};
use std::ptr::{null, null_mut, NonNull};
use std::sync::Once;
use std::time::Duration;

macro_rules! define_buffer_handle {
    ($func:ident() -> $Type:ident) => {
        pub struct $Type(());

        #[inline]
        pub fn $func() -> Option<$Type> {
            static SINGLETON: Once = Once::new();

            let mut result = None;
            SINGLETON.call_once(|| result = Some($Type(())));
            result
        }
    };
}

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
            flag.0,
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
            flags.0,
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
            flags.0,
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
pub struct RandomSequence(NonNull<[i32]>);

impl RandomSequence {
    unsafe fn new(data: *mut i32, len: usize) -> Option<Self> {
        if !data.is_null() {
            Some(Self(unsafe {
                NonNull::new_unchecked(
                    std::ptr::slice_from_raw_parts_mut(
                        data,
                        len,
                    )
                )
            }))
        } else {
            None
        }
    }
}

impl Deref for RandomSequence {
    type Target = [i32];

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe {
            self.0.as_ref()
        }
    }
}

impl DerefMut for RandomSequence {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            self.0.as_mut()
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
    unsafe {
        let data = sys::LoadRandomSequence(
            len.try_into().unwrap(),
            min,
            max,
        );
        RandomSequence::new(data, len)
    }
}

/// Unload random values sequence
#[inline]
pub fn unload_random_sequence(
    mut sequence: RandomSequence,
) {
    unsafe {
        sys::UnloadRandomSequence(
            sequence.as_mut_ptr(),
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
            flags.0,
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

pub mod utils;

//------------------------------------------------------------------

pub mod fs;

// Compression/Encoding functionality

/// Owned bytes that use Raylib memory functions
pub struct RlBytes(NonNull<[u8]>);

impl Drop for RlBytes {
    fn drop(&mut self) {
        utils::mem_free(self.0.cast());
    }
}

impl RlBytes {
    unsafe fn new(ptr: *mut u8, len: MaybeUninit<c_int>) -> Option<Self> {
        if !ptr.is_null() {
            Some(RlBytes(unsafe {
                NonNull::new_unchecked(
                    std::slice::from_raw_parts_mut(
                        ptr,
                        len.assume_init().try_into().unwrap(),
                    )
                )
            }))
        } else {
            None
        }
    }
}

impl Deref for RlBytes {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe {
            self.0.as_ref()
        }
    }
}

impl DerefMut for RlBytes {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            self.0.as_mut()
        }
    }
}

/// Compress data (DEFLATE algorithm), memory must be MemFree()
#[inline]
pub fn compress_data(
    data: &[u8],
) -> Option<RlBytes> {
    let mut len = MaybeUninit::uninit();
    let ptr = unsafe {
        sys::CompressData(
            data.as_ptr(),
            data.len().try_into().unwrap(),
            len.as_mut_ptr(),
        )
    };
    unsafe {
        RlBytes::new(
            ptr,
            len,
        )
    }
}

/// Decompress data (DEFLATE algorithm), memory must be MemFree()
#[inline]
pub fn decompress_data(
    comp_data: &[u8],
) -> Option<RlBytes> {
    let mut len = MaybeUninit::uninit();
    let ptr = unsafe {
        sys::DecompressData(
            comp_data.as_ptr(),
            comp_data.len().try_into().unwrap(),
            len.as_mut_ptr(),
        )
    };
    unsafe {
        RlBytes::new(
            ptr,
            len,
        )
    }
}

pub struct RlCString(NonNull<CStr>);

impl Drop for RlCString {
    fn drop(&mut self) {
        utils::mem_free(self.0.cast());
    }
}

impl RlCString {
    unsafe fn new(ptr: *mut c_char) -> Option<Self> {
        if !ptr.is_null() {
            unsafe {
                Some(Self(
                    NonNull::new_unchecked(
                        std::ptr::from_ref(CStr::from_ptr(ptr)).cast_mut()
                    )
                ))
            }
        } else {
            None
        }
    }

    unsafe fn with_len(ptr: *mut u8, len: c_int) -> Option<Self> {
        if !ptr.is_null() {
            unsafe {
                let bytes = std::slice::from_raw_parts_mut(
                    ptr,
                    len.try_into().unwrap(),
                );
                Some(Self(
                    NonNull::new_unchecked(
                        std::ptr::from_ref(CStr::from_bytes_with_nul_unchecked(bytes)).cast_mut()
                    )
                ))
            }
        } else {
            None
        }
    }
}

/// Encode data to Base64 string (includes NULL terminator), memory must be MemFree()
#[inline]
pub fn encode_data_base64(
    data: &[u8],
) -> Option<RlCString> {
    let mut len = MaybeUninit::uninit();
    unsafe {
        let ptr = sys::EncodeDataBase64(
            data.as_ptr(),
            data.len().try_into().unwrap(),
            len.as_mut_ptr(),
        );
        RlCString::with_len(ptr.cast(), len.assume_init())
    }
}

/// Decode Base64 string (expected NULL terminated), memory must be MemFree()
#[inline]
pub fn decode_data_base64(
    text: &CStr,
) -> Option<RlCString> {
    let mut len = MaybeUninit::uninit();
    unsafe {
        let ptr = sys::DecodeDataBase64(
            text.as_ptr(),
            len.as_mut_ptr(),
        );
        RlCString::with_len(ptr, len.assume_init())
    }
}

/// Compute CRC32 hash code
#[inline]
pub fn compute_crc32(
    data: &mut [u8],
) -> u32 {
    unsafe {
        sys::ComputeCRC32(
            data.as_mut_ptr(),
            data.len().try_into().unwrap(),
        )
    }
}

define_buffer_handle!(compute_md5_handle() -> ComputeMD5Handle);

/// Compute MD5 hash code, returns static int[4] (16 bytes)
#[inline]
pub fn compute_md5<'a>(
    _marker: &'a mut ComputeMD5Handle,
    data: &[u8],
) -> &'a mut [u32; 4] {
    unsafe {
        let ptr = sys::ComputeMD5(
            data.as_ptr().cast_mut(),
            data.len().try_into().unwrap(),
        );
        assert!(!ptr.is_null(), "ComputeMD5 should always return its static buffer, never null, not even if there is an error");
        &mut *ptr.cast::<[u32; 4]>()
    }
}

define_buffer_handle!(compute_sha1_handle() -> ComputeSHA1Handle);

/// Compute SHA1 hash code, returns static int[5] (20 bytes)
#[inline]
pub fn compute_sha1<'a>(
    _marker: &'a mut ComputeSHA1Handle,
    data: &[u8],
) -> &'a mut [u32; 5] {
    unsafe {
        let ptr = sys::ComputeSHA1(
            data.as_ptr().cast_mut(),
            data.len().try_into().unwrap(),
        );
        assert!(!ptr.is_null(), "ComputeSHA1 should always return its static buffer, never null, not even if there is an error");
        &mut *ptr.cast::<[u32; 5]>()
    }
}

// Automation events functionality

/// Load automation events list from file, NULL for empty list, capacity = MAX_AUTOMATION_EVENTS
#[inline]
pub fn load_automation_event_list(
    file_name: &CStr,
) -> sys::AutomationEventList {
    unsafe {
        sys::LoadAutomationEventList(
            file_name.as_ptr(),
        )
    }
}

/// Unload automation events list from file
#[inline]
pub fn unload_automation_event_list(
    list: sys::AutomationEventList,
) {
    unsafe {
        sys::UnloadAutomationEventList(
            list,
        );
    }
}

/// Export automation events list as text file
#[inline]
pub fn export_automation_event_list(
    list: sys::AutomationEventList,
    file_name: &CStr,
) -> bool {
    unsafe {
        sys::ExportAutomationEventList(
            list,
            file_name.as_ptr(),
        )
    }
}

/// Set automation event list to record to
#[inline]
pub fn set_automation_event_list(
    list: Option<&mut sys::AutomationEventList>,
) {
    unsafe {
        sys::SetAutomationEventList(
            list.map_or_else(null_mut, std::ptr::from_mut),
        )
    }
}

/// Set automation event internal base frame to start recording
#[inline]
pub fn set_automation_event_base_frame(
    frame: i32,
) {
    unsafe {
        sys::SetAutomationEventBaseFrame(
            frame,
        )
    }
}

/// Start recording automation events ([`sys::AutomationEventList`] must be set)
#[inline]
pub fn start_automation_event_recording() {
    unsafe {
        sys::StartAutomationEventRecording();
    }
}

/// Stop recording automation events
#[inline]
pub fn stop_automation_event_recording() {
    unsafe {
        sys::StopAutomationEventRecording();
    }
}

/// Play a recorded automation event
#[inline]
pub fn play_automation_event(
    event: sys::AutomationEvent,
) {
    unsafe {
        sys::PlayAutomationEvent(
            event,
        );
    }
}

//------------------------------------------------------------------------------------
// Input Handling Functions (Module: core)
//------------------------------------------------------------------------------------

pub mod input;

//------------------------------------------------------------------------------------
// Gestures and Touch Handling Functions (Module: rgestures)
//------------------------------------------------------------------------------------

pub mod gestures;

//------------------------------------------------------------------------------------
// Camera System Functions (Module: rcamera)
//------------------------------------------------------------------------------------

pub mod camera;

//------------------------------------------------------------------------------------
// Basic Shapes Drawing Functions (Module: shapes)
//------------------------------------------------------------------------------------

pub mod shapes;

//------------------------------------------------------------------------------------
// Texture Loading and Drawing Functions (Module: textures)
//------------------------------------------------------------------------------------

pub mod textures;

//------------------------------------------------------------------------------------
// Font Loading and Text Drawing Functions (Module: text)
//------------------------------------------------------------------------------------

pub mod text;

//------------------------------------------------------------------------------------
// Basic 3d Shapes Drawing Functions (Module: models)
//------------------------------------------------------------------------------------

pub mod models;

//------------------------------------------------------------------------------------
// Audio Loading and Playing Functions (Module: audio)
//------------------------------------------------------------------------------------

pub mod audio;

pub mod rlgl;
