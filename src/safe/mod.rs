use std::{marker::PhantomData, time::Duration};
use crate::low_level as low;
use into_cstr::IntoCStr;

pub mod into_cstr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl From<low::sys::Vector2> for Vector2 {
    fn from(value: low::sys::Vector2) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<Vector2> for low::sys::Vector2 {
    fn from(value: Vector2) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<low::sys::Vector3> for Vector3 {
    fn from(value: low::sys::Vector3) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<Vector3> for low::sys::Vector3 {
    fn from(value: Vector3) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl From<low::sys::Vector4> for Vector4 {
    fn from(value: low::sys::Vector4) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<Vector4> for low::sys::Vector4 {
    fn from(value: Vector4) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyboardKey {
    Apostrophe = low::sys::KeyboardKey::KEY_APOSTROPHE as isize,
    Comma = low::sys::KeyboardKey::KEY_COMMA as isize,
    Minus = low::sys::KeyboardKey::KEY_MINUS as isize,
    Period = low::sys::KeyboardKey::KEY_PERIOD as isize,
    Slash = low::sys::KeyboardKey::KEY_SLASH as isize,
    Zero = low::sys::KeyboardKey::KEY_ZERO as isize,
    One = low::sys::KeyboardKey::KEY_ONE as isize,
    Two = low::sys::KeyboardKey::KEY_TWO as isize,
    Three = low::sys::KeyboardKey::KEY_THREE as isize,
    Four = low::sys::KeyboardKey::KEY_FOUR as isize,
    Five = low::sys::KeyboardKey::KEY_FIVE as isize,
    Six = low::sys::KeyboardKey::KEY_SIX as isize,
    Seven = low::sys::KeyboardKey::KEY_SEVEN as isize,
    Eight = low::sys::KeyboardKey::KEY_EIGHT as isize,
    Nine = low::sys::KeyboardKey::KEY_NINE as isize,
    Semicolon = low::sys::KeyboardKey::KEY_SEMICOLON as isize,
    Equal = low::sys::KeyboardKey::KEY_EQUAL as isize,
    A = low::sys::KeyboardKey::KEY_A as isize,
    B = low::sys::KeyboardKey::KEY_B as isize,
    C = low::sys::KeyboardKey::KEY_C as isize,
    D = low::sys::KeyboardKey::KEY_D as isize,
    E = low::sys::KeyboardKey::KEY_E as isize,
    F = low::sys::KeyboardKey::KEY_F as isize,
    G = low::sys::KeyboardKey::KEY_G as isize,
    H = low::sys::KeyboardKey::KEY_H as isize,
    I = low::sys::KeyboardKey::KEY_I as isize,
    J = low::sys::KeyboardKey::KEY_J as isize,
    K = low::sys::KeyboardKey::KEY_K as isize,
    L = low::sys::KeyboardKey::KEY_L as isize,
    M = low::sys::KeyboardKey::KEY_M as isize,
    N = low::sys::KeyboardKey::KEY_N as isize,
    O = low::sys::KeyboardKey::KEY_O as isize,
    P = low::sys::KeyboardKey::KEY_P as isize,
    Q = low::sys::KeyboardKey::KEY_Q as isize,
    R = low::sys::KeyboardKey::KEY_R as isize,
    S = low::sys::KeyboardKey::KEY_S as isize,
    T = low::sys::KeyboardKey::KEY_T as isize,
    U = low::sys::KeyboardKey::KEY_U as isize,
    V = low::sys::KeyboardKey::KEY_V as isize,
    W = low::sys::KeyboardKey::KEY_W as isize,
    X = low::sys::KeyboardKey::KEY_X as isize,
    Y = low::sys::KeyboardKey::KEY_Y as isize,
    Z = low::sys::KeyboardKey::KEY_Z as isize,
    LeftBracket = low::sys::KeyboardKey::KEY_LEFT_BRACKET as isize,
    Backslash = low::sys::KeyboardKey::KEY_BACKSLASH as isize,
    RightBracket = low::sys::KeyboardKey::KEY_RIGHT_BRACKET as isize,
    Grave = low::sys::KeyboardKey::KEY_GRAVE as isize,
    Space = low::sys::KeyboardKey::KEY_SPACE as isize,
    Escape = low::sys::KeyboardKey::KEY_ESCAPE as isize,
    Enter = low::sys::KeyboardKey::KEY_ENTER as isize,
    Tab = low::sys::KeyboardKey::KEY_TAB as isize,
    Backspace = low::sys::KeyboardKey::KEY_BACKSPACE as isize,
    Insert = low::sys::KeyboardKey::KEY_INSERT as isize,
    Delete = low::sys::KeyboardKey::KEY_DELETE as isize,
    Right = low::sys::KeyboardKey::KEY_RIGHT as isize,
    Left = low::sys::KeyboardKey::KEY_LEFT as isize,
    Down = low::sys::KeyboardKey::KEY_DOWN as isize,
    Up = low::sys::KeyboardKey::KEY_UP as isize,
    PageUp = low::sys::KeyboardKey::KEY_PAGE_UP as isize,
    PageDown = low::sys::KeyboardKey::KEY_PAGE_DOWN as isize,
    Home = low::sys::KeyboardKey::KEY_HOME as isize,
    End = low::sys::KeyboardKey::KEY_END as isize,
    CapsLock = low::sys::KeyboardKey::KEY_CAPS_LOCK as isize,
    ScrollLock = low::sys::KeyboardKey::KEY_SCROLL_LOCK as isize,
    NumLock = low::sys::KeyboardKey::KEY_NUM_LOCK as isize,
    PrintScreen = low::sys::KeyboardKey::KEY_PRINT_SCREEN as isize,
    Pause = low::sys::KeyboardKey::KEY_PAUSE as isize,
    F1 = low::sys::KeyboardKey::KEY_F1 as isize,
    F2 = low::sys::KeyboardKey::KEY_F2 as isize,
    F3 = low::sys::KeyboardKey::KEY_F3 as isize,
    F4 = low::sys::KeyboardKey::KEY_F4 as isize,
    F5 = low::sys::KeyboardKey::KEY_F5 as isize,
    F6 = low::sys::KeyboardKey::KEY_F6 as isize,
    F7 = low::sys::KeyboardKey::KEY_F7 as isize,
    F8 = low::sys::KeyboardKey::KEY_F8 as isize,
    F9 = low::sys::KeyboardKey::KEY_F9 as isize,
    F10 = low::sys::KeyboardKey::KEY_F10 as isize,
    F11 = low::sys::KeyboardKey::KEY_F11 as isize,
    F12 = low::sys::KeyboardKey::KEY_F12 as isize,
    LeftShift = low::sys::KeyboardKey::KEY_LEFT_SHIFT as isize,
    LeftControl = low::sys::KeyboardKey::KEY_LEFT_CONTROL as isize,
    LeftAlt = low::sys::KeyboardKey::KEY_LEFT_ALT as isize,
    LeftSuper = low::sys::KeyboardKey::KEY_LEFT_SUPER as isize,
    RightShift = low::sys::KeyboardKey::KEY_RIGHT_SHIFT as isize,
    RightControl = low::sys::KeyboardKey::KEY_RIGHT_CONTROL as isize,
    RightAlt = low::sys::KeyboardKey::KEY_RIGHT_ALT as isize,
    RightSuper = low::sys::KeyboardKey::KEY_RIGHT_SUPER as isize,
    KbMenu = low::sys::KeyboardKey::KEY_KB_MENU as isize,
    Kp0 = low::sys::KeyboardKey::KEY_KP_0 as isize,
    Kp1 = low::sys::KeyboardKey::KEY_KP_1 as isize,
    Kp2 = low::sys::KeyboardKey::KEY_KP_2 as isize,
    Kp3 = low::sys::KeyboardKey::KEY_KP_3 as isize,
    Kp4 = low::sys::KeyboardKey::KEY_KP_4 as isize,
    Kp5 = low::sys::KeyboardKey::KEY_KP_5 as isize,
    Kp6 = low::sys::KeyboardKey::KEY_KP_6 as isize,
    Kp7 = low::sys::KeyboardKey::KEY_KP_7 as isize,
    Kp8 = low::sys::KeyboardKey::KEY_KP_8 as isize,
    Kp9 = low::sys::KeyboardKey::KEY_KP_9 as isize,
    KpDecimal = low::sys::KeyboardKey::KEY_KP_DECIMAL as isize,
    KpDivide = low::sys::KeyboardKey::KEY_KP_DIVIDE as isize,
    KpMultiply = low::sys::KeyboardKey::KEY_KP_MULTIPLY as isize,
    KpSubtract = low::sys::KeyboardKey::KEY_KP_SUBTRACT as isize,
    KpAdd = low::sys::KeyboardKey::KEY_KP_ADD as isize,
    KpEnter = low::sys::KeyboardKey::KEY_KP_ENTER as isize,
    KpEqual = low::sys::KeyboardKey::KEY_KP_EQUAL as isize,
    Back = low::sys::KeyboardKey::KEY_BACK as isize,
    Menu = low::sys::KeyboardKey::KEY_MENU as isize,
    VolumeUp = low::sys::KeyboardKey::KEY_VOLUME_UP as isize,
    VolumeDown = low::sys::KeyboardKey::KEY_VOLUME_DOWN as isize,
}

impl From<KeyboardKey> for low::sys::KeyboardKey {
    #[inline]
    fn from(value: KeyboardKey) -> Self {
        unsafe { std::mem::transmute(value as i32) }
    }
}

impl TryFrom<low::sys::KeyboardKey> for KeyboardKey {
    type Error = ();

    #[inline]
    fn try_from(value: low::sys::KeyboardKey) -> Result<Self, Self::Error> {
        match value {
            low::sys::KeyboardKey::KEY_NULL => Err(()),
            _ => Ok(unsafe { std::mem::transmute(value as i16) }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left = low::sys::MouseButton::MOUSE_BUTTON_LEFT as isize,
    Right = low::sys::MouseButton::MOUSE_BUTTON_RIGHT as isize,
    Middle = low::sys::MouseButton::MOUSE_BUTTON_MIDDLE as isize,
    Side = low::sys::MouseButton::MOUSE_BUTTON_SIDE as isize,
    Extra = low::sys::MouseButton::MOUSE_BUTTON_EXTRA as isize,
    Forward = low::sys::MouseButton::MOUSE_BUTTON_FORWARD as isize,
    Back = low::sys::MouseButton::MOUSE_BUTTON_BACK as isize,
}

impl From<MouseButton> for low::sys::MouseButton {
    #[inline]
    fn from(value: MouseButton) -> Self {
        unsafe { std::mem::transmute(value as i32) }
    }
}

impl From<low::sys::MouseButton> for MouseButton {
    #[inline]
    fn from(value: low::sys::MouseButton) -> Self {
        unsafe { std::mem::transmute(value as i8) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseCursor {
    Default = low::sys::MouseCursor::MOUSE_CURSOR_DEFAULT as isize,
    Arrow = low::sys::MouseCursor::MOUSE_CURSOR_ARROW as isize,
    IBeam = low::sys::MouseCursor::MOUSE_CURSOR_IBEAM as isize,
    Crosshair = low::sys::MouseCursor::MOUSE_CURSOR_CROSSHAIR as isize,
    PointingHand = low::sys::MouseCursor::MOUSE_CURSOR_POINTING_HAND as isize,
    ResizeEW = low::sys::MouseCursor::MOUSE_CURSOR_RESIZE_EW as isize,
    ResizeNS = low::sys::MouseCursor::MOUSE_CURSOR_RESIZE_NS as isize,
    ResizeNWSE = low::sys::MouseCursor::MOUSE_CURSOR_RESIZE_NWSE as isize,
    ResizeNESW = low::sys::MouseCursor::MOUSE_CURSOR_RESIZE_NESW as isize,
    ResizeAll = low::sys::MouseCursor::MOUSE_CURSOR_RESIZE_ALL as isize,
    NotAllowed = low::sys::MouseCursor::MOUSE_CURSOR_NOT_ALLOWED as isize,
}

impl From<MouseCursor> for low::sys::MouseCursor {
    #[inline]
    fn from(value: MouseCursor) -> Self {
        unsafe { std::mem::transmute(value as i32) }
    }
}

impl From<low::sys::MouseCursor> for MouseCursor {
    #[inline]
    fn from(value: low::sys::MouseCursor) -> Self {
        unsafe { std::mem::transmute(value as i8) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadButton {
    LeftFaceUp = low::sys::GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_UP as isize,
    LeftFaceRight = low::sys::GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_RIGHT as isize,
    LeftFaceDown = low::sys::GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_DOWN as isize,
    LeftFaceLeft = low::sys::GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_LEFT as isize,
    RightFaceUp = low::sys::GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_UP as isize,
    RightFaceRight = low::sys::GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_RIGHT as isize,
    RightFaceDown = low::sys::GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN as isize,
    RightFaceLeft = low::sys::GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_LEFT as isize,
    LeftTrigger1 = low::sys::GamepadButton::GAMEPAD_BUTTON_LEFT_TRIGGER_1 as isize,
    LeftTrigger2 = low::sys::GamepadButton::GAMEPAD_BUTTON_LEFT_TRIGGER_2 as isize,
    RightTrigger1 = low::sys::GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_1 as isize,
    RightTrigger2 = low::sys::GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_2 as isize,
    MiddleLeft = low::sys::GamepadButton::GAMEPAD_BUTTON_MIDDLE_LEFT as isize,
    Middle = low::sys::GamepadButton::GAMEPAD_BUTTON_MIDDLE as isize,
    MiddleRight = low::sys::GamepadButton::GAMEPAD_BUTTON_MIDDLE_RIGHT as isize,
    LeftThumb = low::sys::GamepadButton::GAMEPAD_BUTTON_LEFT_THUMB as isize,
    RightThumb = low::sys::GamepadButton::GAMEPAD_BUTTON_RIGHT_THUMB as isize,
}

impl From<GamepadButton> for low::sys::GamepadButton {
    #[inline]
    fn from(value: GamepadButton) -> Self {
        unsafe { std::mem::transmute(value as i32) }
    }
}

impl TryFrom<low::sys::GamepadButton> for GamepadButton {
    type Error = ();

    #[inline]
    fn try_from(value: low::sys::GamepadButton) -> Result<Self, Self::Error> {
        match value {
            low::sys::GamepadButton::GAMEPAD_BUTTON_UNKNOWN => Err(()),
            _ => Ok(unsafe { std::mem::transmute(value as i8) }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadAxis {
    LeftX = low::sys::GamepadAxis::GAMEPAD_AXIS_LEFT_X as isize,
    LeftY = low::sys::GamepadAxis::GAMEPAD_AXIS_LEFT_Y as isize,
    RightX = low::sys::GamepadAxis::GAMEPAD_AXIS_RIGHT_X as isize,
    RightY = low::sys::GamepadAxis::GAMEPAD_AXIS_RIGHT_Y as isize,
    LeftTrigger = low::sys::GamepadAxis::GAMEPAD_AXIS_LEFT_TRIGGER as isize,
    RightTrigger = low::sys::GamepadAxis::GAMEPAD_AXIS_RIGHT_TRIGGER as isize,
}

impl From<GamepadAxis> for low::sys::GamepadAxis {
    #[inline]
    fn from(value: GamepadAxis) -> Self {
        unsafe { std::mem::transmute(value as i32) }
    }
}

impl From<low::sys::GamepadAxis> for GamepadAxis {
    #[inline]
    fn from(value: low::sys::GamepadAxis) -> Self {
        unsafe { std::mem::transmute(value as i8) }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PixelFormat {
    UncompressedGrayscale = low::sys::PixelFormat::PIXELFORMAT_UNCOMPRESSED_GRAYSCALE as isize,
    UncompressedGrayAlpha = low::sys::PixelFormat::PIXELFORMAT_UNCOMPRESSED_GRAY_ALPHA as isize,
    UncompressedR5G6B5 = low::sys::PixelFormat::PIXELFORMAT_UNCOMPRESSED_R5G6B5 as isize,
    UncompressedR8G8B8 = low::sys::PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8 as isize,
    UncompressedR5G5B5A1 = low::sys::PixelFormat::PIXELFORMAT_UNCOMPRESSED_R5G5B5A1 as isize,
    UncompressedR4G4B4A4 = low::sys::PixelFormat::PIXELFORMAT_UNCOMPRESSED_R4G4B4A4 as isize,
    UncompressedR8G8B8A8 = low::sys::PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 as isize,
    UncompressedR32 = low::sys::PixelFormat::PIXELFORMAT_UNCOMPRESSED_R32 as isize,
    UncompressedR32G32B32 = low::sys::PixelFormat::PIXELFORMAT_UNCOMPRESSED_R32G32B32 as isize,
    UncompressedR32G32B32A32 = low::sys::PixelFormat::PIXELFORMAT_UNCOMPRESSED_R32G32B32A32 as isize,
    UncompressedR16 = low::sys::PixelFormat::PIXELFORMAT_UNCOMPRESSED_R16 as isize,
    UncompressedR16G16B16 = low::sys::PixelFormat::PIXELFORMAT_UNCOMPRESSED_R16G16B16 as isize,
    UncompressedR16G16B16A16 = low::sys::PixelFormat::PIXELFORMAT_UNCOMPRESSED_R16G16B16A16 as isize,
    CompressedDxt1RGB = low::sys::PixelFormat::PIXELFORMAT_COMPRESSED_DXT1_RGB as isize,
    CompressedDxt1RGBA = low::sys::PixelFormat::PIXELFORMAT_COMPRESSED_DXT1_RGBA as isize,
    CompressedDxt3RGBA = low::sys::PixelFormat::PIXELFORMAT_COMPRESSED_DXT3_RGBA as isize,
    CompressedDxt5RGBA = low::sys::PixelFormat::PIXELFORMAT_COMPRESSED_DXT5_RGBA as isize,
    CompressedEtc1RGB = low::sys::PixelFormat::PIXELFORMAT_COMPRESSED_ETC1_RGB as isize,
    CompressedEtc2RGB = low::sys::PixelFormat::PIXELFORMAT_COMPRESSED_ETC2_RGB as isize,
    CompressedEtc2EacRGBA = low::sys::PixelFormat::PIXELFORMAT_COMPRESSED_ETC2_EAC_RGBA as isize,
    CompressedPvrtRGB = low::sys::PixelFormat::PIXELFORMAT_COMPRESSED_PVRT_RGB as isize,
    CompressedPvrtRGBA = low::sys::PixelFormat::PIXELFORMAT_COMPRESSED_PVRT_RGBA as isize,
    CompressedAstc4x4RGBA = low::sys::PixelFormat::PIXELFORMAT_COMPRESSED_ASTC_4x4_RGBA as isize,
    CompressedAstc8x8RGBA = low::sys::PixelFormat::PIXELFORMAT_COMPRESSED_ASTC_8x8_RGBA as isize,
}

impl From<PixelFormat> for low::sys::PixelFormat {
    #[inline]
    fn from(value: PixelFormat) -> Self {
        unsafe { std::mem::transmute(value as i32) }
    }
}

impl From<low::sys::PixelFormat> for PixelFormat {
    #[inline]
    fn from(value: low::sys::PixelFormat) -> Self {
        unsafe { std::mem::transmute(value as i8) }
    }
}

#[derive(Debug)]
pub struct Image {
    data: std::ptr::NonNull<[u8]>,
    pub width: u32,
    pub height: u32,
    pub mipmaps: u32,
    pub format: PixelFormat,
}

impl Image {
    #[inline]
    fn from_raw(value: low::sys::Image) -> Option<Self> {
        if !value.data.is_null() {
            assert!((0..=24).contains(&value.format));
            let width = value.width.try_into().unwrap();
            let height = value.height.try_into().unwrap();
            let format = unsafe { std::mem::transmute(value.format) };
            let size = low::textures::get_pixel_data_size(width, height, format);
            Some(Self {
                data: unsafe { std::ptr::NonNull::new_unchecked(std::ptr::slice_from_raw_parts_mut(value.data, size)) },
                width,
                height,
                mipmaps: value.mipmaps.try_into().unwrap(),
                format,
            })
        } else {
            None
        }
    }

    #[inline]
    fn to_raw(&self) -> low::sys::Image {
        low::sys::Image {
            data: unsafe { self.data.as_mut() }.as_mut_ptr(),
            width: self.width.try_into().unwrap(),
            height: self.height.try_into().unwrap(),
            mipmaps: self.mipmaps.try_into().unwrap(),
            format: (self.format as i8).into(),
        }
    }

    pub fn data(&self) -> &[u8] {
        self.data().as_ref()
    }
}

/// Evidence that the window has been initialized
///
/// Closes the window upon exiting scope
pub struct Window(());

impl Drop for Window {
    /// Close window and unload OpenGL context
    #[inline]
    fn drop(&mut self) {
        low::close_window();
    }
}

impl Window {
    /// Initialize window and OpenGL context
    #[inline]
    pub fn init(width: u32, height: u32, title: impl IntoCStr) -> Option<Self> {
        if !low::is_window_ready() {
            let title = title.into_cstr().unwrap();
            low::init_window(width, height, title.as_ref());
            if low::is_window_ready() {
                return Some(Self(()));
            }
        }
        None
    }

    /// Check if application should close ([`low::sys::KeyboardKey::KEY_ESCAPE`] pressed or windows close icon clicked)
    #[inline]
    pub fn should_close(&self) -> bool {
        low::window_should_close()
    }

    /// Check if window is currently fullscreen
    #[inline]
    pub fn is_window_fullscreen(&self) -> bool {
        low::is_window_fullscreen()
    }

    /// Check if window is currently hidden
    #[inline]
    pub fn is_window_hidden(&self) -> bool {
        low::is_window_hidden()
    }

    /// Check if window is currently minimized
    #[inline]
    pub fn is_window_minimized(&self) -> bool {
        low::is_window_minimized()
    }

    /// Check if window is currently maximized
    #[inline]
    pub fn is_window_maximized(&self) -> bool {
        low::is_window_maximized()
    }

    /// Check if window is currently focused
    #[inline]
    pub fn is_window_focused(&self) -> bool {
        low::is_window_focused()
    }

    /// Check if window has been resized last frame
    #[inline]
    pub fn is_window_resized(&self) -> bool {
        low::is_window_resized()
    }

    /// Set window configuration state using flags
    #[inline]
    pub fn set_window_state(&mut self, flags: sys::ConfigFlags) {
        low::set_window_state(flags);
    }

    /// Clear window configuration state flags
    #[inline]
    pub fn clear_window_state(&mut self, flags: sys::ConfigFlags) {
        low::clear_window_state(flags);
    }

    /// Toggle window state: fullscreen/windowed, resizes monitor to match window resolution
    #[inline]
    pub fn toggle_fullscreen(&mut self) {
        low::toggle_fullscreen();
    }

    /// Toggle window state: borderless windowed, resizes window to match monitor resolution
    #[inline]
    pub fn toggle_borderless_windowed(&mut self) {
        low::toggle_borderless_windowed();
    }

    /// Set window state: maximized, if resizable
    #[inline]
    pub fn maximize_window(&mut self) {
        low::maximize_window();
    }

    /// Set window state: minimized, if resizable
    #[inline]
    pub fn minimize_window(&mut self) {
        low::minimize_window();
    }

    /// Restore window from being minimized/maximized
    #[inline]
    pub fn restore_window(&mut self) {
        low::restore_window();
    }

    /// Set icon for window (single image, RGBA 32bit)
    #[inline]
    pub fn set_window_icon(&mut self, image: sys::Image) {
        low::set_window_icon(image);
    }

    /// Set icon for window (multiple images, RGBA 32bit)
    #[inline]
    pub fn set_window_icons(&mut self, images: &[sys::Image]) {
        low::set_window_icons(images);
    }

    /// Set title for window
    #[inline]
    pub fn set_window_title(&mut self, title: impl IntoCStr) {
        let title = title.into_cstr().unwrap();
        low::set_window_title(title.as_ref())
    }

    /// Set window position on screen
    #[inline]
    pub fn set_window_position(&mut self, x: i32, y: i32) {
        low::set_window_position(x, y)
    }

    /// Setup canvas (framebuffer) to start drawing
    #[inline]
    pub fn draw(&mut self, f: impl for<'d> FnOnce(&mut Self, &'d mut Drawing, &'d mut BaseDrawMode)) {
        low::begin_drawing();
        f(self, &mut Drawing(()), &mut BaseDrawMode(()));
    }

    #[inline]
    pub fn texture_mode(&mut self, target: &mut low::sys::RenderTexture2D, f: impl for<'d> FnOnce(&mut Self, &'d mut TextureMode, &'d mut BaseDrawMode)) {
        low::begin_texture_mode(*target);
        f(self, &mut TextureMode(()), &mut BaseDrawMode(()))
    }

    /// Measure string width for default font
    #[inline]
    pub fn measure_text(&self, text: impl IntoCStr, font_size: u32) -> i32 {
        let text = text.into_cstr().unwrap();
        low::text::measure_text(text.as_ref(), font_size)
    }

    /// Measure string size for Font
    #[inline]
    pub fn measure_text_ex(&self, font: low::sys::Font, text: impl IntoCStr, font_size: f32, spacing: f32) -> Vector2 {
        let text = text.into_cstr().unwrap();
        low::text::measure_text_ex(font, text.as_ref(), font_size, spacing).into()
    }

    // Input-related functions: keyboard

    /// Check if a key has been pressed once
    #[inline]
    pub fn is_key_pressed(&self, key: KeyboardKey) -> bool {
        unsafe {
            low::input::is_key_pressed(key.into())
        }
    }

    /// Check if a key has been pressed again
    #[inline]
    pub fn is_key_pressed_repeat(&self, key: KeyboardKey) -> bool {
        unsafe {
            low::input::is_key_pressed_repeat(key.into())
        }
    }

    /// Check if a key is being pressed
    #[inline]
    pub fn is_key_down(&self, key: KeyboardKey) -> bool {
        unsafe {
            low::input::is_key_down(key.into())
        }
    }

    /// Check if a key has been released once
    #[inline]
    pub fn is_key_released(&self, key: KeyboardKey) -> bool {
        unsafe {
            low::input::is_key_released(key.into())
        }
    }

    /// Check if a key is NOT being pressed
    #[inline]
    pub fn is_key_up(&self, key: KeyboardKey) -> bool {
        unsafe {
            low::input::is_key_up(key.into())
        }
    }

    /// Get key pressed (keycode), call it multiple times for keys queued, returns [`None`] when the queue is empty
    #[inline]
    pub fn get_key_pressed(&self) -> Option<KeyboardKey> {
        unsafe {
            low::input::get_key_pressed()
                .and_then(|k| k.try_into().ok())
        }
    }

    /// Get char pressed (unicode), call it multiple times for chars queued, returns [`None`] when the queue is empty
    #[inline]
    pub fn get_char_pressed(&self) -> Option<char> {
        unsafe {
            low::input::get_char_pressed()
        }
    }

    /// Get name of a QWERTY key on the current keyboard layout (eg returns string 'q' for KEY_A on an AZERTY keyboard)
    #[inline]
    pub fn get_key_name(&self, key: KeyboardKey) -> Option<&'static str> {
        unsafe {
            low::input::get_key_name(key.into())
                .map(|s| str::from_utf8(s.to_bytes()).unwrap())
        }
    }

    /// Set a custom key to exit program (default is ESC)
    #[inline]
    pub fn set_exit_key(&mut self, key: Option<KeyboardKey>) {
        unsafe {
            low::input::set_exit_key(key.map_or(low::sys::KeyboardKey::KEY_NULL, KeyboardKey::into));
        }
    }

    // Input-related functions: gamepads

    /// Check if a gamepad is available
    #[inline]
    pub fn is_gamepad_available(&self, gamepad: usize) -> bool {
        unsafe {
            low::input::is_gamepad_available(gamepad)
        }
    }

    /// Get gamepad internal name id
    #[inline]
    pub fn get_gamepad_name(&self, gamepad: usize) -> Option<&'static str> {
        unsafe {
            low::input::get_gamepad_name(gamepad)
                .map(|s| str::from_utf8(s.to_bytes()).unwrap())
        }
    }

    /// Check if a gamepad button has been pressed once
    #[inline]
    pub fn is_gamepad_button_pressed(&self, gamepad: usize, button: GamepadButton) -> bool {
        unsafe {
            low::input::is_gamepad_button_pressed(gamepad, button.into())
        }
    }

    /// Check if a gamepad button is being pressed
    #[inline]
    pub fn is_gamepad_button_down(&self, gamepad: usize, button: GamepadButton) -> bool {
        unsafe {
            low::input::is_gamepad_button_down(gamepad, button.into())
        }
    }

    /// Check if a gamepad button has been released once
    #[inline]
    pub fn is_gamepad_button_released(&self, gamepad: usize, button: GamepadButton) -> bool {
        unsafe {
            low::input::is_gamepad_button_released(gamepad, button.into())
        }
    }

    /// Check if a gamepad button is NOT being pressed
    #[inline]
    pub fn is_gamepad_button_up(&self, gamepad: usize, button: GamepadButton) -> bool {
        unsafe {
            low::input::is_gamepad_button_up(gamepad, button.into())
        }
    }

    /// Get the last gamepad button pressed
    #[inline]
    pub fn get_gamepad_button_pressed(&self) -> Option<GamepadButton> {
        unsafe {
            low::input::get_gamepad_button_pressed().try_into().ok()
        }
    }

    /// Get axis count for a gamepad
    #[inline]
    pub fn get_gamepad_axis_count(&self, gamepad: usize) -> usize {
        unsafe {
            low::input::get_gamepad_axis_count(gamepad)
        }
    }

    /// Get movement value for a gamepad axis
    #[inline]
    pub fn get_gamepad_axis_movement(&self, gamepad: usize, axis: GamepadAxis) -> f32 {
        unsafe {
            low::input::get_gamepad_axis_movement(gamepad, axis.into())
        }
    }

    /// Set internal gamepad mappings (SDL_GameControllerDB)
    #[inline]
    pub fn set_gamepad_mappings(&mut self, mappings: Option<impl IntoCStr>) -> i32 {
        let mappings = mappings.map(|s| s.into_cstr().unwrap());
        unsafe {
            low::input::set_gamepad_mappings(mappings.as_ref().map(|s| s.as_ref()))
        }
    }

    /// Set gamepad vibration for both motors (duration in seconds)
    #[inline]
    pub fn set_gamepad_vibration(&mut self, gamepad: usize, left_motor: f32, right_motor: f32, duration: f32) {
        unsafe {
            low::input::set_gamepad_vibration(gamepad, left_motor, right_motor, duration);
        }
    }

    /// Set gamepad vibration for both motors
    #[inline]
    pub fn set_gamepad_vibration_time(&mut self, gamepad: usize, left_motor: f32, right_motor: f32, duration: Duration) {
        self.set_gamepad_vibration(gamepad, left_motor, right_motor, duration.as_secs_f32());
    }

    // Input-related functions: mouse

    /// Check if a mouse button has been pressed once
    #[inline]
    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        unsafe {
            low::input::is_mouse_button_pressed(button.into())
        }
    }

    /// Check if a mouse button is being pressed
    #[inline]
    pub fn is_mouse_button_down(&self, button: MouseButton) -> bool {
        unsafe {
            low::input::is_mouse_button_down(button.into())
        }
    }

    /// Check if a mouse button has been released once
    #[inline]
    pub fn is_mouse_button_released(&self, button: MouseButton) -> bool {
        unsafe {
            low::input::is_mouse_button_released(button.into())
        }
    }

    /// Check if a mouse button is NOT being pressed
    #[inline]
    pub fn is_mouse_button_up(&self, button: MouseButton) -> bool {
        unsafe {
            low::input::is_mouse_button_up(button.into())
        }
    }

    /// Get mouse position X
    #[inline]
    pub fn get_mouse_x(&self) -> i32 {
        unsafe {
            low::input::get_mouse_x()
        }
    }

    /// Get mouse position Y
    #[inline]
    pub fn get_mouse_y(&self) -> i32 {
        unsafe {
            low::input::get_mouse_y()
        }
    }

    /// Get mouse position XY
    #[inline]
    pub fn get_mouse_position(&self) -> Vector2 {
        unsafe {
            low::input::get_mouse_position().into()
        }
    }

    /// Get mouse delta between frames
    #[inline]
    pub fn get_mouse_delta(&self) -> Vector2 {
        unsafe {
            low::input::get_mouse_delta().into()
        }
    }

    /// Set mouse position XY
    #[inline]
    pub fn set_mouse_position(&mut self, x: i32, y: i32) {
        unsafe {
            low::input::set_mouse_position(x, y);
        }
    }

    /// Set mouse offset
    #[inline]
    pub fn set_mouse_offset(&mut self, offset_x: i32, offset_y: i32) {
        unsafe {
            low::input::set_mouse_offset(offset_x, offset_y);
        }
    }

    /// Set mouse scaling
    #[inline]
    pub fn set_mouse_scale(&mut self, scale_x: f32, scale_y: f32) {
        unsafe {
            low::input::set_mouse_scale(scale_x, scale_y);
        }
    }

    /// Get mouse wheel movement for X or Y, whichever is larger
    #[inline]
    pub fn get_mouse_wheel_move(&self) -> f32 {
        unsafe {
            low::input::get_mouse_wheel_move()
        }
    }

    /// Get mouse wheel movement for both X and Y
    #[inline]
    pub fn get_mouse_wheel_move_v(&self) -> Vector2 {
        unsafe {
            low::input::get_mouse_wheel_move_v().into()
        }
    }

    /// Set mouse cursor
    #[inline]
    pub fn set_mouse_cursor(&mut self, cursor: MouseCursor) {
        unsafe {
            low::input::set_mouse_cursor(cursor.into());
        }
    }

    // Input-related functions: touch

    /// Get touch position X for touch point 0 (relative to screen size)
    #[inline]
    pub fn get_touch_x(&self) -> i32 {
        unsafe {
            low::input::get_touch_x()
        }
    }

    /// Get touch position Y for touch point 0 (relative to screen size)
    #[inline]
    pub fn get_touch_y(&self) -> i32 {
        unsafe {
            low::input::get_touch_y()
        }
    }

    /// Get touch position XY for a touch point index (relative to screen size)
    #[inline]
    pub fn get_touch_position(&self, index: usize) -> Vector2 {
        unsafe {
            low::input::get_touch_position(index).into()
        }
    }

    /// Get touch point identifier for given index
    #[inline]
    pub fn get_touch_point_id(&self, index: usize) -> Option<u32> {
        unsafe {
            low::input::get_touch_point_id(index)
        }
    }

    /// Get number of touch points
    #[inline]
    pub fn get_touch_point_count(&self) -> usize {
        unsafe {
            low::input::get_touch_point_count()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl From<Color> for low::sys::Color {
    #[inline]
    fn from(Color { r, g, b, a }: Color) -> Self {
        Self { r, g, b, a }
    }
}

impl From<low::sys::Color> for Color {
    #[inline]
    fn from(low::sys::Color { r, g, b, a }: low::sys::Color) -> Self {
        Self { r, g, b, a }
    }
}

#[inline]
pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color { r, g, b, a: 255 }
}

#[inline]
pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color { r, g, b, a }
}

impl Color {
    #[inline]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /** Light Gray                 */ pub const LIGHTGRAY:  Color = rgb(200, 200, 200);
    /** Gray                       */ pub const GRAY:       Color = rgb(130, 130, 130);
    /** Dark Gray                  */ pub const DARKGRAY:   Color = rgb( 80,  80,  80);
    /** Yellow                     */ pub const YELLOW:     Color = rgb(253, 249,   0);
    /** Gold                       */ pub const GOLD:       Color = rgb(255, 203,   0);
    /** Orange                     */ pub const ORANGE:     Color = rgb(255, 161,   0);
    /** Pink                       */ pub const PINK:       Color = rgb(255, 109, 194);
    /** Red                        */ pub const RED:        Color = rgb(230,  41,  55);
    /** Maroon                     */ pub const MAROON:     Color = rgb(190,  33,  55);
    /** Green                      */ pub const GREEN:      Color = rgb(  0, 228,  48);
    /** Lime                       */ pub const LIME:       Color = rgb(  0, 158,  47);
    /** Dark Green                 */ pub const DARKGREEN:  Color = rgb(  0, 117,  44);
    /** Sky Blue                   */ pub const SKYBLUE:    Color = rgb(102, 191, 255);
    /** Blue                       */ pub const BLUE:       Color = rgb(  0, 121, 241);
    /** Dark Blue                  */ pub const DARKBLUE:   Color = rgb(  0,  82, 172);
    /** Purple                     */ pub const PURPLE:     Color = rgb(200, 122, 255);
    /** Violet                     */ pub const VIOLET:     Color = rgb(135,  60, 190);
    /** Dark Purple                */ pub const DARKPURPLE: Color = rgb(112,  31, 126);
    /** Beige                      */ pub const BEIGE:      Color = rgb(211, 176, 131);
    /** Brown                      */ pub const BROWN:      Color = rgb(127, 106,  79);
    /** Dark Brown                 */ pub const DARKBROWN:  Color = rgb( 76,  63,  47);

    /** White                      */ pub const WHITE:      Color = rgb(255, 255, 255);
    /** Black                      */ pub const BLACK:      Color = rgb(  0,   0,   0);
    /** Magenta                    */ pub const MAGENTA:    Color = rgb(255,   0, 255);
    /** My own White (raylib logo) */ pub const RAYWHITE:   Color = rgb(245, 245, 245);

    /** Blank (Transparent)        */ pub const BLANK:      Color = rgba(0, 0, 0, 0);
}

pub trait CSSPalette {
    /** #f0f8ffff */ const ALICEBLUE:            Color = rgb(0xf0, 0xf8, 0xff);
    /** #faebd7ff */ const ANTIQUEWHITE:         Color = rgb(0xfa, 0xeb, 0xd7);
    /** #00ffffff */ const AQUA:                 Color = rgb(0x00, 0xff, 0xff);
    /** #7fffd4ff */ const AQUAMARINE:           Color = rgb(0x7f, 0xff, 0xd4);
    /** #f0ffffff */ const AZURE:                Color = rgb(0xf0, 0xff, 0xff);
    /** #f5f5dcff */ const BEIGE:                Color = rgb(0xf5, 0xf5, 0xdc);
    /** #ffe4c4ff */ const BISQUE:               Color = rgb(0xff, 0xe4, 0xc4);
    /** #000000ff */ const BLACK:                Color = rgb(0x00, 0x00, 0x00);
    /** #ffebcdff */ const BLANCHEDALMOND:       Color = rgb(0xff, 0xeb, 0xcd);
    /** #0000ffff */ const BLUE:                 Color = rgb(0x00, 0x00, 0xff);
    /** #8a2be2ff */ const BLUEVIOLET:           Color = rgb(0x8a, 0x2b, 0xe2);
    /** #a52a2aff */ const BROWN:                Color = rgb(0xa5, 0x2a, 0x2a);
    /** #deb887ff */ const BURLYWOOD:            Color = rgb(0xde, 0xb8, 0x87);
    /** #5f9ea0ff */ const CADETBLUE:            Color = rgb(0x5f, 0x9e, 0xa0);
    /** #7fff00ff */ const CHARTREUSE:           Color = rgb(0x7f, 0xff, 0x00);
    /** #d2691eff */ const CHOCOLATE:            Color = rgb(0xd2, 0x69, 0x1e);
    /** #ff7f50ff */ const CORAL:                Color = rgb(0xff, 0x7f, 0x50);
    /** #6495edff */ const CORNFLOWERBLUE:       Color = rgb(0x64, 0x95, 0xed);
    /** #fff8dcff */ const CORNSILK:             Color = rgb(0xff, 0xf8, 0xdc);
    /** #dc143cff */ const CRIMSON:              Color = rgb(0xdc, 0x14, 0x3c);
    /** #00ffffff */ const CYAN:                 Color = Self::AQUA;
    /** #00008bff */ const DARKBLUE:             Color = rgb(0x00, 0x00, 0x8b);
    /** #008b8bff */ const DARKCYAN:             Color = rgb(0x00, 0x8b, 0x8b);
    /** #b8860bff */ const DARKGOLDENROD:        Color = rgb(0xb8, 0x86, 0x0b);
    /** #a9a9a9ff */ const DARKGRAY:             Color = rgb(0xa9, 0xa9, 0xa9);
    /** #006400ff */ const DARKGREEN:            Color = rgb(0x00, 0x64, 0x00);
    /** #a9a9a9ff */ const DARKGREY:             Color = rgb(0xa9, 0xa9, 0xa9);
    /** #bdb76bff */ const DARKKHAKI:            Color = rgb(0xbd, 0xb7, 0x6b);
    /** #8b008bff */ const DARKMAGENTA:          Color = rgb(0x8b, 0x00, 0x8b);
    /** #556b2fff */ const DARKOLIVEGREEN:       Color = rgb(0x55, 0x6b, 0x2f);
    /** #ff8c00ff */ const DARKORANGE:           Color = rgb(0xff, 0x8c, 0x00);
    /** #9932ccff */ const DARKORCHID:           Color = rgb(0x99, 0x32, 0xcc);
    /** #8b0000ff */ const DARKRED:              Color = rgb(0x8b, 0x00, 0x00);
    /** #e9967aff */ const DARKSALMON:           Color = rgb(0xe9, 0x96, 0x7a);
    /** #8fbc8fff */ const DARKSEAGREEN:         Color = rgb(0x8f, 0xbc, 0x8f);
    /** #483d8bff */ const DARKSLATEBLUE:        Color = rgb(0x48, 0x3d, 0x8b);
    /** #2f4f4fff */ const DARKSLATEGRAY:        Color = rgb(0x2f, 0x4f, 0x4f);
    /** #2f4f4fff */ const DARKSLATEGREY:        Color = rgb(0x2f, 0x4f, 0x4f);
    /** #00ced1ff */ const DARKTURQUOISE:        Color = rgb(0x00, 0xce, 0xd1);
    /** #9400d3ff */ const DARKVIOLET:           Color = rgb(0x94, 0x00, 0xd3);
    /** #ff1493ff */ const DEEPPINK:             Color = rgb(0xff, 0x14, 0x93);
    /** #00bfffff */ const DEEPSKYBLUE:          Color = rgb(0x00, 0xbf, 0xff);
    /** #696969ff */ const DIMGRAY:              Color = rgb(0x69, 0x69, 0x69);
    /** #696969ff */ const DIMGREY:              Color = rgb(0x69, 0x69, 0x69);
    /** #1e90ffff */ const DODGERBLUE:           Color = rgb(0x1e, 0x90, 0xff);
    /** #b22222ff */ const FIREBRICK:            Color = rgb(0xb2, 0x22, 0x22);
    /** #fffaf0ff */ const FLORALWHITE:          Color = rgb(0xff, 0xfa, 0xf0);
    /** #228b22ff */ const FORESTGREEN:          Color = rgb(0x22, 0x8b, 0x22);
    /** #ff00ffff */ const FUCHSIA:              Color = rgb(0xff, 0x00, 0xff);
    /** #dcdcdcff */ const GAINSBORO:            Color = rgb(0xdc, 0xdc, 0xdc);
    /** #f8f8ffff */ const GHOSTWHITE:           Color = rgb(0xf8, 0xf8, 0xff);
    /** #ffd700ff */ const GOLD:                 Color = rgb(0xff, 0xd7, 0x00);
    /** #daa520ff */ const GOLDENROD:            Color = rgb(0xda, 0xa5, 0x20);
    /** #808080ff */ const GRAY:                 Color = rgb(0x80, 0x80, 0x80);
    /** #008000ff */ const GREEN:                Color = rgb(0x00, 0x80, 0x00);
    /** #adff2fff */ const GREENYELLOW:          Color = rgb(0xad, 0xff, 0x2f);
    /** #808080ff */ const GREY:                 Color = Self::GRAY;
    /** #f0fff0ff */ const HONEYDEW:             Color = rgb(0xf0, 0xff, 0xf0);
    /** #ff69b4ff */ const HOTPINK:              Color = rgb(0xff, 0x69, 0xb4);
    /** #cd5c5cff */ const INDIANRED:            Color = rgb(0xcd, 0x5c, 0x5c);
    /** #4b0082ff */ const INDIGO:               Color = rgb(0x4b, 0x00, 0x82);
    /** #fffff0ff */ const IVORY:                Color = rgb(0xff, 0xff, 0xf0);
    /** #f0e68cff */ const KHAKI:                Color = rgb(0xf0, 0xe6, 0x8c);
    /** #e6e6faff */ const LAVENDER:             Color = rgb(0xe6, 0xe6, 0xfa);
    /** #fff0f5ff */ const LAVENDERBLUSH:        Color = rgb(0xff, 0xf0, 0xf5);
    /** #7cfc00ff */ const LAWNGREEN:            Color = rgb(0x7c, 0xfc, 0x00);
    /** #fffacdff */ const LEMONCHIFFON:         Color = rgb(0xff, 0xfa, 0xcd);
    /** #add8e6ff */ const LIGHTBLUE:            Color = rgb(0xad, 0xd8, 0xe6);
    /** #f08080ff */ const LIGHTCORAL:           Color = rgb(0xf0, 0x80, 0x80);
    /** #e0ffffff */ const LIGHTCYAN:            Color = rgb(0xe0, 0xff, 0xff);
    /** #fafad2ff */ const LIGHTGOLDENRODYELLOW: Color = rgb(0xfa, 0xfa, 0xd2);
    /** #d3d3d3ff */ const LIGHTGRAY:            Color = rgb(0xd3, 0xd3, 0xd3);
    /** #90ee90ff */ const LIGHTGREEN:           Color = rgb(0x90, 0xee, 0x90);
    /** #d3d3d3ff */ const LIGHTGREY:            Color = rgb(0xd3, 0xd3, 0xd3);
    /** #ffb6c1ff */ const LIGHTPINK:            Color = rgb(0xff, 0xb6, 0xc1);
    /** #ffa07aff */ const LIGHTSALMON:          Color = rgb(0xff, 0xa0, 0x7a);
    /** #20b2aaff */ const LIGHTSEAGREEN:        Color = rgb(0x20, 0xb2, 0xaa);
    /** #87cefaff */ const LIGHTSKYBLUE:         Color = rgb(0x87, 0xce, 0xfa);
    /** #778899ff */ const LIGHTSLATEGRAY:       Color = rgb(0x77, 0x88, 0x99);
    /** #778899ff */ const LIGHTSLATEGREY:       Color = rgb(0x77, 0x88, 0x99);
    /** #b0c4deff */ const LIGHTSTEELBLUE:       Color = rgb(0xb0, 0xc4, 0xde);
    /** #ffffe0ff */ const LIGHTYELLOW:          Color = rgb(0xff, 0xff, 0xe0);
    /** #00ff00ff */ const LIME:                 Color = rgb(0x00, 0xff, 0x00);
    /** #32cd32ff */ const LIMEGREEN:            Color = rgb(0x32, 0xcd, 0x32);
    /** #faf0e6ff */ const LINEN:                Color = rgb(0xfa, 0xf0, 0xe6);
    /** #ff00ffff */ const MAGENTA:              Color = Self::FUCHSIA;
    /** #800000ff */ const MAROON:               Color = rgb(0x80, 0x00, 0x00);
    /** #66cdaaff */ const MEDIUMAQUAMARINE:     Color = rgb(0x66, 0xcd, 0xaa);
    /** #0000cdff */ const MEDIUMBLUE:           Color = rgb(0x00, 0x00, 0xcd);
    /** #ba55d3ff */ const MEDIUMORCHID:         Color = rgb(0xba, 0x55, 0xd3);
    /** #9370dbff */ const MEDIUMPURPLE:         Color = rgb(0x93, 0x70, 0xdb);
    /** #3cb371ff */ const MEDIUMSEAGREEN:       Color = rgb(0x3c, 0xb3, 0x71);
    /** #7b68eeff */ const MEDIUMSLATEBLUE:      Color = rgb(0x7b, 0x68, 0xee);
    /** #00fa9aff */ const MEDIUMSPRINGGREEN:    Color = rgb(0x00, 0xfa, 0x9a);
    /** #48d1ccff */ const MEDIUMTURQUOISE:      Color = rgb(0x48, 0xd1, 0xcc);
    /** #c71585ff */ const MEDIUMVIOLETRED:      Color = rgb(0xc7, 0x15, 0x85);
    /** #191970ff */ const MIDNIGHTBLUE:         Color = rgb(0x19, 0x19, 0x70);
    /** #f5fffaff */ const MINTCREAM:            Color = rgb(0xf5, 0xff, 0xfa);
    /** #ffe4e1ff */ const MISTYROSE:            Color = rgb(0xff, 0xe4, 0xe1);
    /** #ffe4b5ff */ const MOCCASIN:             Color = rgb(0xff, 0xe4, 0xb5);
    /** #ffdeadff */ const NAVAJOWHITE:          Color = rgb(0xff, 0xde, 0xad);
    /** #000080ff */ const NAVY:                 Color = rgb(0x00, 0x00, 0x80);
    /** #fdf5e6ff */ const OLDLACE:              Color = rgb(0xfd, 0xf5, 0xe6);
    /** #808000ff */ const OLIVE:                Color = rgb(0x80, 0x80, 0x00);
    /** #6b8e23ff */ const OLIVEDRAB:            Color = rgb(0x6b, 0x8e, 0x23);
    /** #ffa500ff */ const ORANGE:               Color = rgb(0xff, 0xa5, 0x00);
    /** #ff4500ff */ const ORANGERED:            Color = rgb(0xff, 0x45, 0x00);
    /** #da70d6ff */ const ORCHID:               Color = rgb(0xda, 0x70, 0xd6);
    /** #eee8aaff */ const PALEGOLDENROD:        Color = rgb(0xee, 0xe8, 0xaa);
    /** #98fb98ff */ const PALEGREEN:            Color = rgb(0x98, 0xfb, 0x98);
    /** #afeeeeff */ const PALETURQUOISE:        Color = rgb(0xaf, 0xee, 0xee);
    /** #db7093ff */ const PALEVIOLETRED:        Color = rgb(0xdb, 0x70, 0x93);
    /** #ffefd5ff */ const PAPAYAWHIP:           Color = rgb(0xff, 0xef, 0xd5);
    /** #ffdab9ff */ const PEACHPUFF:            Color = rgb(0xff, 0xda, 0xb9);
    /** #cd853fff */ const PERU:                 Color = rgb(0xcd, 0x85, 0x3f);
    /** #ffc0cbff */ const PINK:                 Color = rgb(0xff, 0xc0, 0xcb);
    /** #dda0ddff */ const PLUM:                 Color = rgb(0xdd, 0xa0, 0xdd);
    /** #b0e0e6ff */ const POWDERBLUE:           Color = rgb(0xb0, 0xe0, 0xe6);
    /** #800080ff */ const PURPLE:               Color = rgb(0x80, 0x00, 0x80);
    /** #663399ff */ const REBECCAPURPLE:        Color = rgb(0x66, 0x33, 0x99);
    /** #ff0000ff */ const RED:                  Color = rgb(0xff, 0x00, 0x00);
    /** #bc8f8fff */ const ROSYBROWN:            Color = rgb(0xbc, 0x8f, 0x8f);
    /** #4169e1ff */ const ROYALBLUE:            Color = rgb(0x41, 0x69, 0xe1);
    /** #8b4513ff */ const SADDLEBROWN:          Color = rgb(0x8b, 0x45, 0x13);
    /** #fa8072ff */ const SALMON:               Color = rgb(0xfa, 0x80, 0x72);
    /** #f4a460ff */ const SANDYBROWN:           Color = rgb(0xf4, 0xa4, 0x60);
    /** #2e8b57ff */ const SEAGREEN:             Color = rgb(0x2e, 0x8b, 0x57);
    /** #fff5eeff */ const SEASHELL:             Color = rgb(0xff, 0xf5, 0xee);
    /** #a0522dff */ const SIENNA:               Color = rgb(0xa0, 0x52, 0x2d);
    /** #c0c0c0ff */ const SILVER:               Color = rgb(0xc0, 0xc0, 0xc0);
    /** #87ceebff */ const SKYBLUE:              Color = rgb(0x87, 0xce, 0xeb);
    /** #6a5acdff */ const SLATEBLUE:            Color = rgb(0x6a, 0x5a, 0xcd);
    /** #708090ff */ const SLATEGRAY:            Color = rgb(0x70, 0x80, 0x90);
    /** #708090ff */ const SLATEGREY:            Color = rgb(0x70, 0x80, 0x90);
    /** #fffafaff */ const SNOW:                 Color = rgb(0xff, 0xfa, 0xfa);
    /** #00ff7fff */ const SPRINGGREEN:          Color = rgb(0x00, 0xff, 0x7f);
    /** #4682b4ff */ const STEELBLUE:            Color = rgb(0x46, 0x82, 0xb4);
    /** #d2b48cff */ const TAN:                  Color = rgb(0xd2, 0xb4, 0x8c);
    /** #008080ff */ const TEAL:                 Color = rgb(0x00, 0x80, 0x80);
    /** #d8bfd8ff */ const THISTLE:              Color = rgb(0xd8, 0xbf, 0xd8);
    /** #00000000 */ const TRANSPARENT:          Color = rgb(0x00, 0x00, 0x00);
    /** #ff6347ff */ const TOMATO:               Color = rgb(0xff, 0x63, 0x47);
    /** #40e0d0ff */ const TURQUOISE:            Color = rgb(0x40, 0xe0, 0xd0);
    /** #ee82eeff */ const VIOLET:               Color = rgb(0xee, 0x82, 0xee);
    /** #f5deb3ff */ const WHEAT:                Color = rgb(0xf5, 0xde, 0xb3);
    /** #ffffffff */ const WHITE:                Color = rgb(0xff, 0xff, 0xff);
    /** #f5f5f5ff */ const WHITESMOKE:           Color = rgb(0xf5, 0xf5, 0xf5);
    /** #ffff00ff */ const YELLOW:               Color = rgb(0xff, 0xff, 0x00);
    /** #9acd32ff */ const YELLOWGREEN:          Color = rgb(0x9a, 0xcd, 0x32);
}

impl CSSPalette for Color {}

pub struct Drawing(());

impl Drop for Drawing {
    /// End canvas drawing and swap buffers (double buffering)
    #[inline]
    fn drop(&mut self) {
        low::end_drawing();
    }
}

pub struct TextureMode(());

impl Drop for TextureMode {
    /// Ends drawing to render texture
    #[inline]
    fn drop(&mut self) {
        low::end_texture_mode();
    }
}

mod internal {
    pub trait Sealed {}
    impl Sealed for super::Drawing {}
    impl Sealed for super::TextureMode {}
}

pub trait Draw: internal::Sealed {
    /// Set background color (framebuffer clear color)
    #[inline]
    fn clear_background(&mut self, color: Color) {
        low::clear_background(color.into());
    }

    /// Draw text (using default font)
    #[inline]
    fn draw_text(&mut self, text: impl IntoCStr, pos_x: i32, pos_y: i32, font_size: u32, color: Color) {
        let text = text.into_cstr().unwrap();
        low::text::draw_text(text.as_ref(), pos_x, pos_y, font_size, color.into());
    }

    /// Draw a color-filled rectangle
    #[inline]
    fn draw_rectangle(&mut self, pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
        low::shapes::draw_rectangle(pos_x, pos_y, width, height, color.into());
    }
}

impl Draw for Drawing {}
impl Draw for TextureMode {}

/// Change the draw mode
///
/// This trait is intentionally separate from the [`Drawing`] handle.
/// It is, on its own, zero-cost if the draw mode stack is statically known.
///
/// Conditional draw modes are possible by using [`DrawModeEnum`].
pub trait DrawMode {
    type Enum: DrawMode;

    /// Convert the draw mode into a [`DrawModeEnum`]
    ///
    /// ## See also
    /// - [`DrawMode::no_mode()`]
    fn into_enum(self) -> Self::Enum;

    /// Used as the "`None`" variant of draw modes
    ///
    /// All other branches should use [`DrawMode::into_enum()`]
    ///
    /// # Example
    /// ```ignore
    /// let m = if condition {
    ///     m.begin_scissor(0, 0, 50, 50).into_enum()
    /// } else {
    ///     m.no_mode()
    /// };
    /// ```
    ///
    /// Necessary to prevent the inner draw mode from popping when you would expect
    /// one of the other branches to drop, by adding a buffer layer
    ///
    /// --------------------------------------------------------------------------------
    /// ### Aside
    ///
    /// No, storing the draw mode in enums is not "zero-cost", it stores a discriminant.
    /// But genuinely *intended*, well-defined behavior of Raylib shouldn't be outright
    /// removed for the sake of having EXCLUSIVELY zero-cost abstractions. Sometimes
    /// runtime conditions need to be done *at runtime* to ensure safety.
    ///
    /// Plus, if you want ZERO cost, you always have the option of calling into the
    /// [`crate::low_level`] module itself, if you trust yourself to call the
    /// corresponding `end_*_mode` methods in the right order.
    #[inline]
    fn no_mode(&mut self) -> DrawModeEnum<'_, Self> {
        DrawModeEnum::Ignore(self)
    }
}

pub trait DrawModeExt<'a, M: ?Sized + DrawMode> {
    /// Convert the draw mode into a [`DrawModeEnum`]
    ///
    /// Alias for [`DrawMode::no_mode()`]
    fn into_enum(self) -> DrawModeEnum<'a, M>;
}

impl<'a, M: ?Sized + DrawMode> DrawModeExt<'a, M> for &'a mut M {
    #[inline]
    fn into_enum(self) -> DrawModeEnum<'a, M> {
        self.no_mode()
    }
}

pub enum DrawModeEnum<'a, M: ?Sized + 'a> {
    Ignore(&'a mut M),
    Base(BaseDrawMode),
    Scissor(ScissorMode<'a, M>),
}

impl<'a, M: ?Sized + 'a> From<&'a mut M> for DrawModeEnum<'a, M> {
    #[inline]
    fn from(value: &'a mut M) -> Self {
        Self::Ignore(value)
    }
}

impl From<BaseDrawMode> for DrawModeEnum<'static, ()> {
    #[inline]
    fn from(value: BaseDrawMode) -> Self {
        Self::Base(value)
    }
}

impl<'a, M: ?Sized + 'a> From<ScissorMode<'a, M>> for DrawModeEnum<'a, M> {
    #[inline]
    fn from(value: ScissorMode<'a, M>) -> Self {
        Self::Scissor(value)
    }
}

impl<M: ?Sized> DrawMode for DrawModeEnum<'_, M> {
    type Enum = Self;

    #[inline]
    fn into_enum(self) -> Self {
        self
    }
}

pub struct BaseDrawMode(());

impl DrawMode for BaseDrawMode {
    type Enum = DrawModeEnum<'static, ()>;

    #[inline]
    fn into_enum(self) -> DrawModeEnum<'static, ()> {
        DrawModeEnum::Base(self)
    }
}

pub struct ScissorMode<'a, M: ?Sized>(PhantomData<&'a mut M>);

impl<M: ?Sized> Drop for ScissorMode<'_, M> {
    /// End scissor mode
    #[inline]
    fn drop(&mut self) {
        low::end_scissor_mode();
    }
}

impl<'a, M: ?Sized> ScissorMode<'a, M> {
    /// Begin scissor mode (define screen area for following drawing)
    #[inline]
    pub fn begin(_m: &'a mut M, x: i32, y: i32, width: i32, height: i32) -> ScissorMode<'a, M> {
        low::begin_scissor_mode(x, y, width, height);
        Self(PhantomData)
    }
}

impl<'a, M: ?Sized> DrawMode for ScissorMode<'a, M> {
    type Enum = DrawModeEnum<'a, M>;

    #[inline]
    fn into_enum(self) -> DrawModeEnum<'a, M> {
        DrawModeEnum::Scissor(self)
    }
}
