#![warn(missing_docs)]

#[cfg(target_endian = "little")]
use std::mem::MaybeUninit;
use std::{marker::PhantomData, os::raw::{c_int, c_void}, ptr::NonNull, time::Duration};
use crate::low;
use into_cstr::IntoCStr;

mod sys {
    #[allow(unused_imports, reason = "to be used later")]
    pub use crate::low::{
        sys::Vector2,
        sys::Vector3,
        sys::Vector4,
        sys::Quaternion,
        sys::Matrix,
        sys::Color,
        sys::Rectangle,
        sys::Image,
        sys::Texture,
        sys::Texture2D,
        sys::TextureCubemap,
        sys::RenderTexture,
        sys::RenderTexture2D,
        sys::NPatchInfo,
        sys::GlyphInfo,
        sys::Font,
        sys::Camera3D,
        sys::Camera,
        sys::Camera2D,
        sys::Mesh,
        sys::Shader,
        sys::MaterialMap,
        sys::Material,
        sys::Transform,
        sys::BoneInfo,
        sys::Model,
        sys::ModelAnimation,
        sys::Ray,
        sys::RayCollision,
        sys::BoundingBox,
        sys::Wave,
        sys::rAudioBuffer,
        sys::rAudioProcessor,
        sys::AudioStream,
        sys::Sound,
        sys::Music,
        sys::VrDeviceInfo,
        sys::VrStereoConfig,
        sys::FilePathList,
        sys::AutomationEvent,
        sys::AutomationEventList,
        sys::ConfigFlags,
        sys::TraceLogLevel,
        sys::KeyboardKey,
        sys::MouseButton,
        sys::MouseCursor,
        sys::GamepadButton,
        sys::GamepadAxis,
        sys::ShaderUniformDataType,
        sys::ShaderAttributeDataType,
        sys::PixelFormat,
        sys::TextureFilter,
        sys::TextureWrap,
        sys::CubemapLayout,
        sys::FontType,
        sys::BlendMode,
        sys::Gesture,
        sys::CameraMode,
        sys::CameraProjection,
        sys::NPatchLayout,
        sys::TraceLogCallback,
        sys::LoadFileDataCallback,
        sys::SaveFileDataCallback,
        sys::LoadFileTextCallback,
        sys::SaveFileTextCallback,
        sys::AudioCallback,
        sys::float3,
        sys::float16,
        sys::TouchAction,
        sys::GestureEvent,
        sys::rlVertexBuffer,
        sys::rlDrawCall,
        sys::rlRenderBatch,
        sys::rlGlVersion,
        sys::rlTraceLogLevel,
        sys::rlPixelFormat,
        sys::rlTextureFilter,
        sys::rlBlendMode,
        sys::rlShaderUniformDataType,
        sys::rlShaderAttributeDataType,
        sys::rlFramebufferAttachType,
        sys::rlFramebufferAttachTextureType,
        sys::rlCullMode,
    };
}

pub mod into_cstr;

/// Vector2, 2 components
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
    /// Vector x component
    pub x: f32,
    /// Vector y component
    pub y: f32,
}

impl Vector2 {
    #[inline]
    const fn into_sys(self) -> sys::Vector2 {
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    const fn from_sys(value: sys::Vector2) -> Self {
        unsafe { std::mem::transmute(value) }
    }

    #[inline]
    const fn as_sys(&self) -> &sys::Vector2 {
        unsafe { std::mem::transmute(self) }
    }
}

impl From<sys::Vector2> for Vector2 {
    #[inline]
    fn from(value: sys::Vector2) -> Self {
        Self::from_sys(value)
    }
}

impl From<Vector2> for sys::Vector2 {
    #[inline]
    fn from(value: Vector2) -> Self {
        value.into_sys()
    }
}

impl<'a> From<&'a Vector2> for &'a sys::Vector2 {
    #[inline]
    fn from(value: &'a Vector2) -> Self {
        value.as_sys()
    }
}

/// Vector3, 3 components
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    /// Vector x component
    pub x: f32,
    /// Vector y component
    pub y: f32,
    /// Vector z component
    pub z: f32,
}

impl Vector3 {
    #[inline]
    const fn into_sys(self) -> sys::Vector3 {
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    const fn from_sys(value: sys::Vector3) -> Self {
        unsafe { std::mem::transmute(value) }
    }

    #[inline]
    const fn as_sys(&self) -> &sys::Vector3 {
        unsafe { std::mem::transmute(self) }
    }
}

impl From<sys::Vector3> for Vector3 {
    #[inline]
    fn from(value: sys::Vector3) -> Self {
        Self::from_sys(value)
    }
}

impl From<Vector3> for sys::Vector3 {
    #[inline]
    fn from(value: Vector3) -> Self {
        value.into_sys()
    }
}

impl<'a> From<&'a Vector3> for &'a sys::Vector3 {
    #[inline]
    fn from(value: &'a Vector3) -> Self {
        value.as_sys()
    }
}

/// Vector4, 4 components
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector4 {
    /// Vector x component
    pub x: f32,
    /// Vector y component
    pub y: f32,
    /// Vector z component
    pub z: f32,
    /// Vector w component
    pub w: f32,
}

impl Vector4 {
    #[inline]
    const fn into_sys(self) -> sys::Vector4 {
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    const fn from_sys(value: sys::Vector4) -> Self {
        unsafe { std::mem::transmute(value) }
    }

    #[inline]
    const fn as_sys(&self) -> &sys::Vector4 {
        unsafe { std::mem::transmute(self) }
    }
}

impl From<sys::Vector4> for Vector4 {
    #[inline]
    fn from(value: sys::Vector4) -> Self {
        Self::from_sys(value)
    }
}

impl From<Vector4> for sys::Vector4 {
    #[inline]
    fn from(value: Vector4) -> Self {
        value.into_sys()
    }
}

impl<'a> From<&'a Vector4> for &'a sys::Vector4 {
    #[inline]
    fn from(value: &'a Vector4) -> Self {
        value.as_sys()
    }
}

/// Quaternion, 4 components (Vector4 alias)
pub type Quaternion = Vector4;

/// Keyboard keys (US keyboard layout)
///
/// NOTE: Use [`WindowInner::get_key_pressed()`] to allow redefining
/// required keys for alternative layouts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyboardKey {
    // Alphanumeric keys
    /** Key: `'` | `"` */ Apostrophe = sys::KeyboardKey::KEY_APOSTROPHE as isize,
    /** Key: `,` | `<` */ Comma = sys::KeyboardKey::KEY_COMMA as isize,
    /** Key: `-` | `_` */ Minus = sys::KeyboardKey::KEY_MINUS as isize,
    /** Key: `.` | `>` */ Period = sys::KeyboardKey::KEY_PERIOD as isize,
    /** Key: `/` | `?` */ Slash = sys::KeyboardKey::KEY_SLASH as isize,
    /** Key: `0` | `)` */ Zero = sys::KeyboardKey::KEY_ZERO as isize,
    /** Key: `1` | `!` */ One = sys::KeyboardKey::KEY_ONE as isize,
    /** Key: `2` | `@` */ Two = sys::KeyboardKey::KEY_TWO as isize,
    /** Key: `3` | `#` */ Three = sys::KeyboardKey::KEY_THREE as isize,
    /** Key: `4` | `$` */ Four = sys::KeyboardKey::KEY_FOUR as isize,
    /** Key: `5` | `%` */ Five = sys::KeyboardKey::KEY_FIVE as isize,
    /** Key: `6` | `^` */ Six = sys::KeyboardKey::KEY_SIX as isize,
    /** Key: `7` | `&` */ Seven = sys::KeyboardKey::KEY_SEVEN as isize,
    /** Key: `8` | `*` */ Eight = sys::KeyboardKey::KEY_EIGHT as isize,
    /** Key: `9` | `(` */ Nine = sys::KeyboardKey::KEY_NINE as isize,
    /** Key: `;` | `:` */ Semicolon = sys::KeyboardKey::KEY_SEMICOLON as isize,
    /** Key: `=` | `+` */ Equal = sys::KeyboardKey::KEY_EQUAL as isize,
    /** Key: `a` | `A` */ A = sys::KeyboardKey::KEY_A as isize,
    /** Key: `b` | `B` */ B = sys::KeyboardKey::KEY_B as isize,
    /** Key: `c` | `C` */ C = sys::KeyboardKey::KEY_C as isize,
    /** Key: `d` | `D` */ D = sys::KeyboardKey::KEY_D as isize,
    /** Key: `e` | `E` */ E = sys::KeyboardKey::KEY_E as isize,
    /** Key: `f` | `F` */ F = sys::KeyboardKey::KEY_F as isize,
    /** Key: `g` | `G` */ G = sys::KeyboardKey::KEY_G as isize,
    /** Key: `h` | `H` */ H = sys::KeyboardKey::KEY_H as isize,
    /** Key: `i` | `I` */ I = sys::KeyboardKey::KEY_I as isize,
    /** Key: `j` | `J` */ J = sys::KeyboardKey::KEY_J as isize,
    /** Key: `k` | `K` */ K = sys::KeyboardKey::KEY_K as isize,
    /** Key: `l` | `L` */ L = sys::KeyboardKey::KEY_L as isize,
    /** Key: `m` | `M` */ M = sys::KeyboardKey::KEY_M as isize,
    /** Key: `n` | `N` */ N = sys::KeyboardKey::KEY_N as isize,
    /** Key: `o` | `O` */ O = sys::KeyboardKey::KEY_O as isize,
    /** Key: `p` | `P` */ P = sys::KeyboardKey::KEY_P as isize,
    /** Key: `q` | `Q` */ Q = sys::KeyboardKey::KEY_Q as isize,
    /** Key: `r` | `R` */ R = sys::KeyboardKey::KEY_R as isize,
    /** Key: `s` | `S` */ S = sys::KeyboardKey::KEY_S as isize,
    /** Key: `t` | `T` */ T = sys::KeyboardKey::KEY_T as isize,
    /** Key: `u` | `U` */ U = sys::KeyboardKey::KEY_U as isize,
    /** Key: `v` | `V` */ V = sys::KeyboardKey::KEY_V as isize,
    /** Key: `w` | `W` */ W = sys::KeyboardKey::KEY_W as isize,
    /** Key: `x` | `X` */ X = sys::KeyboardKey::KEY_X as isize,
    /** Key: `y` | `Y` */ Y = sys::KeyboardKey::KEY_Y as isize,
    /** Key: `z` | `Z` */ Z = sys::KeyboardKey::KEY_Z as isize,
    /** Key: `[` | `{` */ LeftBracket = sys::KeyboardKey::KEY_LEFT_BRACKET as isize,
    /** Key: `\\` | `|` */ Backslash = sys::KeyboardKey::KEY_BACKSLASH as isize,
    /** Key: `]` | `}` */ RightBracket = sys::KeyboardKey::KEY_RIGHT_BRACKET as isize,
    /** Key: `` ` `` | `~` */ Grave = sys::KeyboardKey::KEY_GRAVE as isize,
    // Function keys
    /** Key: Space */ Space = sys::KeyboardKey::KEY_SPACE as isize,
    /** Key: Esc */ Escape = sys::KeyboardKey::KEY_ESCAPE as isize,
    /** Key: Enter */ Enter = sys::KeyboardKey::KEY_ENTER as isize,
    /** Key: Tab */ Tab = sys::KeyboardKey::KEY_TAB as isize,
    /** Key: Backspace */ Backspace = sys::KeyboardKey::KEY_BACKSPACE as isize,
    /** Key: Ins */ Insert = sys::KeyboardKey::KEY_INSERT as isize,
    /** Key: Del */ Delete = sys::KeyboardKey::KEY_DELETE as isize,
    /** Key: Cursor right */ Right = sys::KeyboardKey::KEY_RIGHT as isize,
    /** Key: Cursor left */ Left = sys::KeyboardKey::KEY_LEFT as isize,
    /** Key: Cursor down */ Down = sys::KeyboardKey::KEY_DOWN as isize,
    /** Key: Cursor up */ Up = sys::KeyboardKey::KEY_UP as isize,
    /** Key: Page up */ PageUp = sys::KeyboardKey::KEY_PAGE_UP as isize,
    /** Key: Page down */ PageDown = sys::KeyboardKey::KEY_PAGE_DOWN as isize,
    /** Key: Home */ Home = sys::KeyboardKey::KEY_HOME as isize,
    /** Key: End */ End = sys::KeyboardKey::KEY_END as isize,
    /** Key: Caps lock */ CapsLock = sys::KeyboardKey::KEY_CAPS_LOCK as isize,
    /** Key: Scroll lock */ ScrollLock = sys::KeyboardKey::KEY_SCROLL_LOCK as isize,
    /** Key: Num lock */ NumLock = sys::KeyboardKey::KEY_NUM_LOCK as isize,
    /** Key: Print screen */ PrintScreen = sys::KeyboardKey::KEY_PRINT_SCREEN as isize,
    /** Key: Pause */ Pause = sys::KeyboardKey::KEY_PAUSE as isize,
    /** Key: `F1` */ F1 = sys::KeyboardKey::KEY_F1 as isize,
    /** Key: `F2` */ F2 = sys::KeyboardKey::KEY_F2 as isize,
    /** Key: `F3` */ F3 = sys::KeyboardKey::KEY_F3 as isize,
    /** Key: `F4` */ F4 = sys::KeyboardKey::KEY_F4 as isize,
    /** Key: `F5` */ F5 = sys::KeyboardKey::KEY_F5 as isize,
    /** Key: `F6` */ F6 = sys::KeyboardKey::KEY_F6 as isize,
    /** Key: `F7` */ F7 = sys::KeyboardKey::KEY_F7 as isize,
    /** Key: `F8` */ F8 = sys::KeyboardKey::KEY_F8 as isize,
    /** Key: `F9` */ F9 = sys::KeyboardKey::KEY_F9 as isize,
    /** Key: `F10` */ F10 = sys::KeyboardKey::KEY_F10 as isize,
    /** Key: `F11` */ F11 = sys::KeyboardKey::KEY_F11 as isize,
    /** Key: `F12` */ F12 = sys::KeyboardKey::KEY_F12 as isize,
    /** Key: Shift left */ LeftShift = sys::KeyboardKey::KEY_LEFT_SHIFT as isize,
    /** Key: Ctrl left */ LeftControl = sys::KeyboardKey::KEY_LEFT_CONTROL as isize,
    /** Key: Alt left */ LeftAlt = sys::KeyboardKey::KEY_LEFT_ALT as isize,
    /** Key: Super left */ LeftSuper = sys::KeyboardKey::KEY_LEFT_SUPER as isize,
    /** Key: Shift right */ RightShift = sys::KeyboardKey::KEY_RIGHT_SHIFT as isize,
    /** Key: Ctrl right */ RightControl = sys::KeyboardKey::KEY_RIGHT_CONTROL as isize,
    /** Key: Alt right */ RightAlt = sys::KeyboardKey::KEY_RIGHT_ALT as isize,
    /** Key: Super right */ RightSuper = sys::KeyboardKey::KEY_RIGHT_SUPER as isize,
    /** Key: KB menu */ KbMenu = sys::KeyboardKey::KEY_KB_MENU as isize,
    // Keypad keys
    /** Key: Keypad `0` */ Kp0 = sys::KeyboardKey::KEY_KP_0 as isize,
    /** Key: Keypad `1` */ Kp1 = sys::KeyboardKey::KEY_KP_1 as isize,
    /** Key: Keypad `2` */ Kp2 = sys::KeyboardKey::KEY_KP_2 as isize,
    /** Key: Keypad `3` */ Kp3 = sys::KeyboardKey::KEY_KP_3 as isize,
    /** Key: Keypad `4` */ Kp4 = sys::KeyboardKey::KEY_KP_4 as isize,
    /** Key: Keypad `5` */ Kp5 = sys::KeyboardKey::KEY_KP_5 as isize,
    /** Key: Keypad `6` */ Kp6 = sys::KeyboardKey::KEY_KP_6 as isize,
    /** Key: Keypad `7` */ Kp7 = sys::KeyboardKey::KEY_KP_7 as isize,
    /** Key: Keypad `8` */ Kp8 = sys::KeyboardKey::KEY_KP_8 as isize,
    /** Key: Keypad `9` */ Kp9 = sys::KeyboardKey::KEY_KP_9 as isize,
    /** Key: Keypad `.` */ KpDecimal = sys::KeyboardKey::KEY_KP_DECIMAL as isize,
    /** Key: Keypad `/` */ KpDivide = sys::KeyboardKey::KEY_KP_DIVIDE as isize,
    /** Key: Keypad `*` */ KpMultiply = sys::KeyboardKey::KEY_KP_MULTIPLY as isize,
    /** Key: Keypad `-` */ KpSubtract = sys::KeyboardKey::KEY_KP_SUBTRACT as isize,
    /** Key: Keypad `+` */ KpAdd = sys::KeyboardKey::KEY_KP_ADD as isize,
    /** Key: Keypad Enter */ KpEnter = sys::KeyboardKey::KEY_KP_ENTER as isize,
    /** Key: Keypad `=` */ KpEqual = sys::KeyboardKey::KEY_KP_EQUAL as isize,
    // Android key buttons
    /** Key: Android back button */ Back = sys::KeyboardKey::KEY_BACK as isize,
    /** Key: Android menu button */ Menu = sys::KeyboardKey::KEY_MENU as isize,
    /** Key: Android volume up button */ VolumeUp = sys::KeyboardKey::KEY_VOLUME_UP as isize,
    /** Key: Android volume down button */ VolumeDown = sys::KeyboardKey::KEY_VOLUME_DOWN as isize,
}

impl KeyboardKey {
    #[inline]
    const fn into_sys(self) -> sys::KeyboardKey {
        unsafe { std::mem::transmute(self as i32) }
    }

    #[inline]
    const fn opt_into_sys(value: Option<Self>) -> sys::KeyboardKey {
        match value {
            Some(k) => unsafe { std::mem::transmute(k as i32) },
            None => sys::KeyboardKey::KEY_NULL,
        }
    }

    #[inline]
    const fn from_sys(value: sys::KeyboardKey) -> Option<Self> {
        unsafe { std::mem::transmute(value as i16) }
    }
}

/// Mouse buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    /// Mouse button left
    Left = sys::MouseButton::MOUSE_BUTTON_LEFT as isize,
    /// Mouse button right
    Right = sys::MouseButton::MOUSE_BUTTON_RIGHT as isize,
    /// Mouse button middle (pressed wheel)
    Middle = sys::MouseButton::MOUSE_BUTTON_MIDDLE as isize,
    /// Mouse button side (advanced mouse device)
    Side = sys::MouseButton::MOUSE_BUTTON_SIDE as isize,
    /// Mouse button extra (advanced mouse device)
    Extra = sys::MouseButton::MOUSE_BUTTON_EXTRA as isize,
    /// Mouse button forward (advanced mouse device)
    Forward = sys::MouseButton::MOUSE_BUTTON_FORWARD as isize,
    /// Mouse button back (advanced mouse device)
    Back = sys::MouseButton::MOUSE_BUTTON_BACK as isize,
}

impl From<MouseButton> for sys::MouseButton {
    #[inline]
    fn from(value: MouseButton) -> Self {
        unsafe { std::mem::transmute(value as i32) }
    }
}

impl From<sys::MouseButton> for MouseButton {
    #[inline]
    fn from(value: sys::MouseButton) -> Self {
        unsafe { std::mem::transmute(value as i8) }
    }
}

/// Mouse cursor
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseCursor {
    /// Default pointer shape
    Default = sys::MouseCursor::MOUSE_CURSOR_DEFAULT as isize,
    /// Arrow shape
    Arrow = sys::MouseCursor::MOUSE_CURSOR_ARROW as isize,
    /// Text writing cursor shape
    IBeam = sys::MouseCursor::MOUSE_CURSOR_IBEAM as isize,
    /// Cross shape
    Crosshair = sys::MouseCursor::MOUSE_CURSOR_CROSSHAIR as isize,
    /// Pointing hand cursor
    PointingHand = sys::MouseCursor::MOUSE_CURSOR_POINTING_HAND as isize,
    /// Horizontal resize/move arrow shape
    ResizeEW = sys::MouseCursor::MOUSE_CURSOR_RESIZE_EW as isize,
    /// Vertical resize/move arrow shape
    ResizeNS = sys::MouseCursor::MOUSE_CURSOR_RESIZE_NS as isize,
    /// Top-left to bottom-right diagonal resize/move arrow shape
    ResizeNWSE = sys::MouseCursor::MOUSE_CURSOR_RESIZE_NWSE as isize,
    /// The top-right to bottom-left diagonal resize/move arrow shape
    ResizeNESW = sys::MouseCursor::MOUSE_CURSOR_RESIZE_NESW as isize,
    /// The omnidirectional resize/move cursor shape
    ResizeAll = sys::MouseCursor::MOUSE_CURSOR_RESIZE_ALL as isize,
    /// The operation-not-allowed shape
    NotAllowed = sys::MouseCursor::MOUSE_CURSOR_NOT_ALLOWED as isize,
}

impl From<MouseCursor> for sys::MouseCursor {
    #[inline]
    fn from(value: MouseCursor) -> Self {
        unsafe { std::mem::transmute(value as i32) }
    }
}

impl From<sys::MouseCursor> for MouseCursor {
    #[inline]
    fn from(value: sys::MouseCursor) -> Self {
        unsafe { std::mem::transmute(value as i8) }
    }
}

/// Gamepad buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadButton {
    /// Gamepad left DPAD up button
    LeftFaceUp = sys::GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_UP as isize,
    /// Gamepad left DPAD right button
    LeftFaceRight = sys::GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_RIGHT as isize,
    /// Gamepad left DPAD down button
    LeftFaceDown = sys::GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_DOWN as isize,
    /// Gamepad left DPAD left button
    LeftFaceLeft = sys::GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_LEFT as isize,
    /// Gamepad right button up (i.e. PS3: Triangle, Xbox: Y)
    RightFaceUp = sys::GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_UP as isize,
    /// Gamepad right button right (i.e. PS3: Circle, Xbox: B)
    RightFaceRight = sys::GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_RIGHT as isize,
    /// Gamepad right button down (i.e. PS3: Cross, Xbox: A)
    RightFaceDown = sys::GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN as isize,
    /// Gamepad right button left (i.e. PS3: Square, Xbox: X)
    RightFaceLeft = sys::GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_LEFT as isize,
    /// Gamepad top/back trigger left (first), it could be a trailing button
    LeftTrigger1 = sys::GamepadButton::GAMEPAD_BUTTON_LEFT_TRIGGER_1 as isize,
    /// Gamepad top/back trigger left (second), it could be a trailing button
    LeftTrigger2 = sys::GamepadButton::GAMEPAD_BUTTON_LEFT_TRIGGER_2 as isize,
    /// Gamepad top/back trigger right (first), it could be a trailing button
    RightTrigger1 = sys::GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_1 as isize,
    /// Gamepad top/back trigger right (second), it could be a trailing button
    RightTrigger2 = sys::GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_2 as isize,
    /// Gamepad center buttons, left one (i.e. PS3: Select)
    MiddleLeft = sys::GamepadButton::GAMEPAD_BUTTON_MIDDLE_LEFT as isize,
    /// Gamepad center buttons, middle one (i.e. PS3: PS, Xbox: XBOX)
    Middle = sys::GamepadButton::GAMEPAD_BUTTON_MIDDLE as isize,
    /// Gamepad center buttons, right one (i.e. PS3: Start)
    MiddleRight = sys::GamepadButton::GAMEPAD_BUTTON_MIDDLE_RIGHT as isize,
    /// Gamepad joystick pressed button left
    LeftThumb = sys::GamepadButton::GAMEPAD_BUTTON_LEFT_THUMB as isize,
    /// Gamepad joystick pressed button right
    RightThumb = sys::GamepadButton::GAMEPAD_BUTTON_RIGHT_THUMB as isize,
}

impl From<GamepadButton> for sys::GamepadButton {
    #[inline]
    fn from(value: GamepadButton) -> Self {
        unsafe { std::mem::transmute(value as i32) }
    }
}

impl TryFrom<sys::GamepadButton> for GamepadButton {
    type Error = ();

    #[inline]
    fn try_from(value: sys::GamepadButton) -> Result<Self, Self::Error> {
        match value {
            sys::GamepadButton::GAMEPAD_BUTTON_UNKNOWN => Err(()),
            _ => Ok(unsafe { std::mem::transmute(value as i8) }),
        }
    }
}

/// Gamepad axes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadAxis {
    /// Gamepad left stick X axis
    LeftX = sys::GamepadAxis::GAMEPAD_AXIS_LEFT_X as isize,
    /// Gamepad left stick Y axis
    LeftY = sys::GamepadAxis::GAMEPAD_AXIS_LEFT_Y as isize,
    /// Gamepad right stick X axis
    RightX = sys::GamepadAxis::GAMEPAD_AXIS_RIGHT_X as isize,
    /// Gamepad right stick Y axis
    RightY = sys::GamepadAxis::GAMEPAD_AXIS_RIGHT_Y as isize,
    /// Gamepad back trigger left, pressure level: [1..-1]
    LeftTrigger = sys::GamepadAxis::GAMEPAD_AXIS_LEFT_TRIGGER as isize,
    /// Gamepad back trigger right, pressure level: [1..-1]
    RightTrigger = sys::GamepadAxis::GAMEPAD_AXIS_RIGHT_TRIGGER as isize,
}

impl From<GamepadAxis> for sys::GamepadAxis {
    #[inline]
    fn from(value: GamepadAxis) -> Self {
        unsafe { std::mem::transmute(value as i32) }
    }
}

impl From<sys::GamepadAxis> for GamepadAxis {
    #[inline]
    fn from(value: sys::GamepadAxis) -> Self {
        unsafe { std::mem::transmute(value as i8) }
    }
}

/// Pixel formats
///
/// NOTE: Support depends on OpenGL version and platform
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PixelFormat {
    /// Uncompressed opaque grayscale format -- 8 bit per pixel (no alpha)
    UncompressedGrayscale = sys::PixelFormat::PIXELFORMAT_UNCOMPRESSED_GRAYSCALE as isize,
    /// Uncompressed transparent grayscale format -- 8*2 bpp (2 channels)
    UncompressedGrayAlpha = sys::PixelFormat::PIXELFORMAT_UNCOMPRESSED_GRAY_ALPHA as isize,
    /// Uncompressed 16-bit RGB format: 5-bit red channel, 6-bit green channel, 5-bit blue channel -- 16 bpp
    UncompressedR5G6B5 = sys::PixelFormat::PIXELFORMAT_UNCOMPRESSED_R5G6B5 as isize,
    /// Uncompressed 24-bit RGB format: 8-bit red channel, 8-bit green channel, 8-bit blue channel -- 24 bpp
    UncompressedR8G8B8 = sys::PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8 as isize,
    /// Uncompressed 16-bit RGBA format: 5-bit red channel, 5-bit green channel, 5-bit blue channel, 1-bit alpha channel -- 16 bpp (1 bit alpha)
    UncompressedR5G5B5A1 = sys::PixelFormat::PIXELFORMAT_UNCOMPRESSED_R5G5B5A1 as isize,
    /// Uncompressed 16-bit RGBA format: 4-bit red channel, 4-bit green channel, 4-bit blue channel, 4-bit alpha channel -- 16 bpp (4 bit alpha)
    UncompressedR4G4B4A4 = sys::PixelFormat::PIXELFORMAT_UNCOMPRESSED_R4G4B4A4 as isize,
    /// Uncompressed 32-bit RGBA format: 8-bit red channel, 8-bit green channel, 8-bit blue channel, 8-bit alpha channel -- 32 bpp
    UncompressedR8G8B8A8 = sys::PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 as isize,
    /// Uncompressed 32-bit color format: 32-bit red channel -- 32 bpp (1 channel - float)
    UncompressedR32 = sys::PixelFormat::PIXELFORMAT_UNCOMPRESSED_R32 as isize,
    /// Uncompressed 96-bit RGB format: 32-bit red channel, 32-bit green channel, 32-bit blue channel -- 32*3 bpp (3 channels - float)
    UncompressedR32G32B32 = sys::PixelFormat::PIXELFORMAT_UNCOMPRESSED_R32G32B32 as isize,
    /// Uncompressed 128-bit RGB format: 32-bit red channel, 32-bit green channel, 32-bit blue channel, 32-bit alpha channel -- 32*4 bpp (4 channels - float)
    UncompressedR32G32B32A32 = sys::PixelFormat::PIXELFORMAT_UNCOMPRESSED_R32G32B32A32 as isize,
    /// Uncompressed 16-bit color format: 16-bit red channel -- 16 bpp (1 channel - half float)
    UncompressedR16 = sys::PixelFormat::PIXELFORMAT_UNCOMPRESSED_R16 as isize,
    /// Uncompressed 48-bit RGB format: 16-bit red channel, 16-bit green channel, 16-bit blue channel -- 16*3 bpp (3 channels - half float)
    UncompressedR16G16B16 = sys::PixelFormat::PIXELFORMAT_UNCOMPRESSED_R16G16B16 as isize,
    /// Uncompressed 48-bit RGB format: 16-bit red channel, 16-bit green channel, 16-bit blue channel, 16-bit alpha channel -- 16*4 bpp (4 channels - half float)
    UncompressedR16G16B16A16 = sys::PixelFormat::PIXELFORMAT_UNCOMPRESSED_R16G16B16A16 as isize,
    /// Compressed DXT1 RGB format -- 4 bpp (no alpha)
    CompressedDxt1RGB = sys::PixelFormat::PIXELFORMAT_COMPRESSED_DXT1_RGB as isize,
    /// Compressed DXT1 RGBA format -- 4 bpp (1 bit alpha)
    CompressedDxt1RGBA = sys::PixelFormat::PIXELFORMAT_COMPRESSED_DXT1_RGBA as isize,
    /// Compressed DXT3 RGBA format -- 8 bpp
    CompressedDxt3RGBA = sys::PixelFormat::PIXELFORMAT_COMPRESSED_DXT3_RGBA as isize,
    /// Compressed DXT5 RGBA format -- 8 bpp
    CompressedDxt5RGBA = sys::PixelFormat::PIXELFORMAT_COMPRESSED_DXT5_RGBA as isize,
    /// Compressed ETC1 (Ericsson Texture Compression) RGB format -- 4 bpp
    CompressedEtc1RGB = sys::PixelFormat::PIXELFORMAT_COMPRESSED_ETC1_RGB as isize,
    /// Compressed ETC2 (Ericsson Texture Compression) RGB format -- 4 bpp
    CompressedEtc2RGB = sys::PixelFormat::PIXELFORMAT_COMPRESSED_ETC2_RGB as isize,
    /// Compressed ETC2 (Ericsson Texture Compression) EAC RGBA format -- 8 bpp
    CompressedEtc2EacRGBA = sys::PixelFormat::PIXELFORMAT_COMPRESSED_ETC2_EAC_RGBA as isize,
    /// PowerVR Texture Compressed (PVRTC) RGB format -- 4 bpp
    CompressedPvrtRGB = sys::PixelFormat::PIXELFORMAT_COMPRESSED_PVRT_RGB as isize,
    /// PowerVR Texture Compressed (PVRTC) RGBA format -- 4 bpp
    CompressedPvrtRGBA = sys::PixelFormat::PIXELFORMAT_COMPRESSED_PVRT_RGBA as isize,
    /// 4x4-block Adaptive Scalable Texture Compression (ASTC) RGBA format -- 8 bpp
    CompressedAstc4x4RGBA = sys::PixelFormat::PIXELFORMAT_COMPRESSED_ASTC_4x4_RGBA as isize,
    /// 8x8-block Adaptive Scalable Texture Compression (ASTC) RGBA format -- 2 bpp
    CompressedAstc8x8RGBA = sys::PixelFormat::PIXELFORMAT_COMPRESSED_ASTC_8x8_RGBA as isize,
}

impl From<PixelFormat> for sys::PixelFormat {
    #[inline]
    fn from(value: PixelFormat) -> Self {
        unsafe { std::mem::transmute(value as i32) }
    }
}

impl From<sys::PixelFormat> for PixelFormat {
    #[inline]
    fn from(value: sys::PixelFormat) -> Self {
        unsafe { std::mem::transmute(value as i8) }
    }
}

/// Image, pixel data stored in CPU memory (RAM)
#[repr(C)]
#[derive(Debug)]
pub struct Image {
    // Fields are private so that users cannot modify the values and set them to an invalid state.
    // Image fields should only be modified through image methods.
    data: NonNull<u8>,
    width: u32,
    height: u32,
    mipmaps: u32,
    #[cfg(target_endian = "little")]
    _format_padding: MaybeUninit<[u8; 3]>,
    format: PixelFormat,
    #[cfg(target_endian = "big")]
    _format_padding: MaybeUninit<[u8; 3]>,
}
const _: () = {
    assert!(std::mem::size_of::<NonNull<u8>>() == std::mem::size_of::<*mut c_void>());
    assert!(std::mem::size_of::<u32>() == std::mem::size_of::<c_int>());
    assert!(std::mem::size_of::<Image>() == std::mem::size_of::<sys::Image>());
};

impl Drop for Image {
    /// Unload image from CPU memory (RAM)
    #[inline]
    fn drop(&mut self) {
        low::unload_image(*self.as_sys());
    }
}

impl Image {
    /// # Panics
    /// - `width` is negative
    /// - `height` is negative
    /// - `format` does not correspond to a variant of [`PixelFormat`]
    ///
    /// # Safety
    ///
    /// - [`Self::drop`] must run for *at most* one unique call of [`low::load_image`].
    /// - [`Self::drop`] must **not** run if `value` did not come from [`low::load_image`].
    ///
    /// It is recommended to avoid having multiple [`Image`]s that correspond to the same [`low::load_image`] call.
    ///
    /// - A **valid** Image's `data` must have a length of [`low::get_pixel_data_size`] with `value`'s fields.
    #[inline]
    #[allow(dead_code, reason = "to be used in `load_image`")]
    unsafe fn from_sys(value: sys::Image) -> Option<Self> {
        const _: () = {
            let img = unsafe {
                std::mem::transmute::<_, Option<Image>>(sys::Image {
                    data: std::ptr::null_mut(),
                    width: 0,
                    height: 0,
                    mipmaps: 0,
                    format: 1,
                })
            };
            let null_transmutes_to_none = img.is_none();
            std::mem::forget(img);
            assert!(null_transmutes_to_none, "null data should transmute to None");
        };

        assert!(0 <= value.width);
        assert!(0 <= value.height);
        assert!(1 <= value.format && value.format <= 24);

        // SAFETY:
        // - each field has been asserted as valid (except for data)
        // - transmutes to None if (and only if, due to asserts) `data` is NULL
        // - `Image` is repr(C) and contains padding for compatibility with `sys::Image`
        // - each field of `Image` is the same size and shape as the corresponding fields in `sys::Image`
        unsafe { std::mem::transmute(value) }
    }

    /// # Panics
    /// - `width` overflows when cast to signed
    /// - `height` overflows when cast to signed
    /// - `mipmaps` overflows when cast to signed
    #[inline]
    fn as_sys(&self) -> &sys::Image {
        assert!(self.width <= i32::MAX as u32);
        assert!(self.height <= i32::MAX as u32);
        assert!(self.mipmaps <= i32::MAX as u32);
        unsafe { std::mem::transmute(self) }
    }

    /// # Panics
    /// - any `width` overflows when cast to signed
    /// - any `height` overflows when cast to signed
    /// - any `mipmaps` overflows when cast to signed
    #[inline]
    fn slice_as_sys(slice: &[Image]) -> &[sys::Image] {
        for img in slice {
            assert!(img.width <= i32::MAX as u32);
            assert!(img.height <= i32::MAX as u32);
            assert!(img.mipmaps <= i32::MAX as u32);
        }
        unsafe { std::mem::transmute(slice) }
    }

    #[inline]
    const fn data_len(&self) -> usize {
        // SAFETY: `PixelFormat` discriminants are identical to `sys::PixelFormat`'s
        let format = unsafe { std::mem::transmute(self.format as i32) };
        low::get_pixel_data_size(self.width, self.height, format)
    }

    /// Image raw data as bytes
    #[inline]
    pub const fn data(&self) -> &[u8] {
        let len = self.data_len();
        // SAFETY: `data` is guaranteed to have length `len` if `from_raw` safety is upheld
        unsafe { std::slice::from_raw_parts(self.data.as_ptr(), len) }
    }

    /// Image raw data as mutable bytes
    #[inline]
    pub const fn data_mut(&mut self) -> &mut [u8] {
        let len = self.data_len();
        // SAFETY: `data` is guaranteed to have length `len` if `from_raw` safety is upheld
        unsafe { std::slice::from_raw_parts_mut(self.data.as_ptr(), len) }
    }

    /// Image base width
    #[inline]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// Image base height
    #[inline]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// Mipmap levels, 1 by default
    #[inline]
    pub const fn mipmaps(&self) -> u32 {
        self.mipmaps
    }

    /// Data format
    #[inline]
    pub const fn format(&self) -> PixelFormat {
        self.format
    }
}

/// Handle to the open window
///
/// Cannot call [`Window::draw`] or [`Window::texture_mode`] and does not close the window.
/// Most window-related methods are implemented on this structure instead of [`Window`].
pub struct WindowInner(());

impl WindowInner {
    /// Check if application should close ([`sys::KeyboardKey::KEY_ESCAPE`] pressed or windows close icon clicked)
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
    pub fn set_window_icon(&mut self, image: &Image) {
        low::set_window_icon(*image.as_sys());
    }

    /// Set icon for window (multiple images, RGBA 32bit)
    #[inline]
    pub fn set_window_icons(&mut self, images: &[Image]) {
        low::set_window_icons(Image::slice_as_sys(images));
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

    /// Measure string width for default font
    #[inline]
    pub fn measure_text(&self, text: impl IntoCStr, font_size: u32) -> i32 {
        let text = text.into_cstr().unwrap();
        low::measure_text(text.as_ref(), font_size)
    }

    /// Measure string size for Font
    #[inline]
    pub fn measure_text_ex(&self, font: sys::Font, text: impl IntoCStr, font_size: f32, spacing: f32) -> Vector2 {
        let text = text.into_cstr().unwrap();
        Vector2::from_sys(low::measure_text_ex(font, text.as_ref(), font_size, spacing))
    }

    // Input-related functions: keyboard

    /// Check if a key has been pressed once
    #[inline]
    pub fn is_key_pressed(&self, key: KeyboardKey) -> bool {
        unsafe {
            low::is_key_pressed(key.into_sys())
        }
    }

    /// Check if a key has been pressed again
    #[inline]
    pub fn is_key_pressed_repeat(&self, key: KeyboardKey) -> bool {
        unsafe {
            low::is_key_pressed_repeat(key.into_sys())
        }
    }

    /// Check if a key is being pressed
    #[inline]
    pub fn is_key_down(&self, key: KeyboardKey) -> bool {
        unsafe {
            low::is_key_down(key.into_sys())
        }
    }

    /// Check if a key has been released once
    #[inline]
    pub fn is_key_released(&self, key: KeyboardKey) -> bool {
        unsafe {
            low::is_key_released(key.into_sys())
        }
    }

    /// Check if a key is NOT being pressed
    #[inline]
    pub fn is_key_up(&self, key: KeyboardKey) -> bool {
        unsafe {
            low::is_key_up(key.into_sys())
        }
    }

    /// Get key pressed (keycode), call it multiple times for keys queued, returns [`None`] when the queue is empty
    #[inline]
    pub fn get_key_pressed(&self) -> Option<KeyboardKey> {
        unsafe {
            low::get_key_pressed()
                .and_then(KeyboardKey::from_sys)
        }
    }

    /// Get char pressed (unicode), call it multiple times for chars queued, returns [`None`] when the queue is empty
    #[inline]
    pub fn get_char_pressed(&self) -> Option<char> {
        unsafe {
            low::get_char_pressed()
        }
    }

    /// Get name of a QWERTY key on the current keyboard layout (eg returns string 'q' for KEY_A on an AZERTY keyboard)
    #[inline]
    pub fn get_key_name(&self, key: KeyboardKey) -> Option<&'static str> {
        unsafe {
            low::get_key_name(key.into_sys())
                .map(|s| str::from_utf8(s.to_bytes()).unwrap())
        }
    }

    /// Set a custom key to exit program (default is ESC)
    #[inline]
    pub fn set_exit_key(&mut self, key: Option<KeyboardKey>) {
        unsafe {
            low::set_exit_key(KeyboardKey::opt_into_sys(key));
        }
    }

    // Input-related functions: gamepads

    /// Check if a gamepad is available
    #[inline]
    pub fn is_gamepad_available(&self, gamepad: usize) -> bool {
        unsafe {
            low::is_gamepad_available(gamepad)
        }
    }

    /// Get gamepad internal name id
    #[inline]
    pub fn get_gamepad_name(&self, gamepad: usize) -> Option<&'static str> {
        unsafe {
            low::get_gamepad_name(gamepad)
                .map(|s| str::from_utf8(s.to_bytes()).unwrap())
        }
    }

    /// Check if a gamepad button has been pressed once
    #[inline]
    pub fn is_gamepad_button_pressed(&self, gamepad: usize, button: GamepadButton) -> bool {
        unsafe {
            low::is_gamepad_button_pressed(gamepad, button.into())
        }
    }

    /// Check if a gamepad button is being pressed
    #[inline]
    pub fn is_gamepad_button_down(&self, gamepad: usize, button: GamepadButton) -> bool {
        unsafe {
            low::is_gamepad_button_down(gamepad, button.into())
        }
    }

    /// Check if a gamepad button has been released once
    #[inline]
    pub fn is_gamepad_button_released(&self, gamepad: usize, button: GamepadButton) -> bool {
        unsafe {
            low::is_gamepad_button_released(gamepad, button.into())
        }
    }

    /// Check if a gamepad button is NOT being pressed
    #[inline]
    pub fn is_gamepad_button_up(&self, gamepad: usize, button: GamepadButton) -> bool {
        unsafe {
            low::is_gamepad_button_up(gamepad, button.into())
        }
    }

    /// Get the last gamepad button pressed
    #[inline]
    pub fn get_gamepad_button_pressed(&self) -> Option<GamepadButton> {
        unsafe {
            low::get_gamepad_button_pressed().try_into().ok()
        }
    }

    /// Get axis count for a gamepad
    #[inline]
    pub fn get_gamepad_axis_count(&self, gamepad: usize) -> usize {
        unsafe {
            low::get_gamepad_axis_count(gamepad)
        }
    }

    /// Get movement value for a gamepad axis
    #[inline]
    pub fn get_gamepad_axis_movement(&self, gamepad: usize, axis: GamepadAxis) -> f32 {
        unsafe {
            low::get_gamepad_axis_movement(gamepad, axis.into())
        }
    }

    /// Set internal gamepad mappings (SDL_GameControllerDB)
    #[inline]
    pub fn set_gamepad_mappings(&mut self, mappings: Option<impl IntoCStr>) -> i32 {
        let mappings = mappings.map(|s| s.into_cstr().unwrap());
        unsafe {
            low::set_gamepad_mappings(mappings.as_ref().map(|s| s.as_ref()))
        }
    }

    /// Set gamepad vibration for both motors (duration in seconds)
    #[inline]
    pub fn set_gamepad_vibration(&mut self, gamepad: usize, left_motor: f32, right_motor: f32, duration: f32) {
        unsafe {
            low::set_gamepad_vibration(gamepad, left_motor, right_motor, duration);
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
            low::is_mouse_button_pressed(button.into())
        }
    }

    /// Check if a mouse button is being pressed
    #[inline]
    pub fn is_mouse_button_down(&self, button: MouseButton) -> bool {
        unsafe {
            low::is_mouse_button_down(button.into())
        }
    }

    /// Check if a mouse button has been released once
    #[inline]
    pub fn is_mouse_button_released(&self, button: MouseButton) -> bool {
        unsafe {
            low::is_mouse_button_released(button.into())
        }
    }

    /// Check if a mouse button is NOT being pressed
    #[inline]
    pub fn is_mouse_button_up(&self, button: MouseButton) -> bool {
        unsafe {
            low::is_mouse_button_up(button.into())
        }
    }

    /// Get mouse position X
    #[inline]
    pub fn get_mouse_x(&self) -> i32 {
        unsafe {
            low::get_mouse_x()
        }
    }

    /// Get mouse position Y
    #[inline]
    pub fn get_mouse_y(&self) -> i32 {
        unsafe {
            low::get_mouse_y()
        }
    }

    /// Get mouse position XY
    #[inline]
    pub fn get_mouse_position(&self) -> Vector2 {
        unsafe {
            low::get_mouse_position().into()
        }
    }

    /// Get mouse delta between frames
    #[inline]
    pub fn get_mouse_delta(&self) -> Vector2 {
        unsafe {
            low::get_mouse_delta().into()
        }
    }

    /// Set mouse position XY
    #[inline]
    pub fn set_mouse_position(&mut self, x: i32, y: i32) {
        unsafe {
            low::set_mouse_position(x, y);
        }
    }

    /// Set mouse offset
    #[inline]
    pub fn set_mouse_offset(&mut self, offset_x: i32, offset_y: i32) {
        unsafe {
            low::set_mouse_offset(offset_x, offset_y);
        }
    }

    /// Set mouse scaling
    #[inline]
    pub fn set_mouse_scale(&mut self, scale_x: f32, scale_y: f32) {
        unsafe {
            low::set_mouse_scale(scale_x, scale_y);
        }
    }

    /// Get mouse wheel movement for X or Y, whichever is larger
    #[inline]
    pub fn get_mouse_wheel_move(&self) -> f32 {
        unsafe {
            low::get_mouse_wheel_move()
        }
    }

    /// Get mouse wheel movement for both X and Y
    #[inline]
    pub fn get_mouse_wheel_move_v(&self) -> Vector2 {
        unsafe {
            low::get_mouse_wheel_move_v().into()
        }
    }

    /// Set mouse cursor
    #[inline]
    pub fn set_mouse_cursor(&mut self, cursor: MouseCursor) {
        unsafe {
            low::set_mouse_cursor(cursor.into());
        }
    }

    // Input-related functions: touch

    /// Get touch position X for touch point 0 (relative to screen size)
    #[inline]
    pub fn get_touch_x(&self) -> i32 {
        unsafe {
            low::get_touch_x()
        }
    }

    /// Get touch position Y for touch point 0 (relative to screen size)
    #[inline]
    pub fn get_touch_y(&self) -> i32 {
        unsafe {
            low::get_touch_y()
        }
    }

    /// Get touch position XY for a touch point index (relative to screen size)
    #[inline]
    pub fn get_touch_position(&self, index: usize) -> Vector2 {
        unsafe {
            low::get_touch_position(index).into()
        }
    }

    /// Get touch point identifier for given index
    #[inline]
    pub fn get_touch_point_id(&self, index: usize) -> Option<u32> {
        unsafe {
            low::get_touch_point_id(index)
        }
    }

    /// Get number of touch points
    #[inline]
    pub fn get_touch_point_count(&self) -> usize {
        unsafe {
            low::get_touch_point_count()
        }
    }
}

/// Handle for Raylib window functions
///
/// Closes the window upon exiting scope
pub struct Window(WindowInner);

impl std::ops::Deref for Window {
    type Target = WindowInner;

    #[inline]
    fn deref(&self) -> &WindowInner {
        &self.0
    }
}

impl std::ops::DerefMut for Window {
    #[inline]
    fn deref_mut(&mut self) -> &mut WindowInner {
        &mut self.0
    }
}

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
                return Some(Self(WindowInner(())));
            }
        }
        None
    }

    /// Setup canvas (framebuffer) to start drawing
    #[inline]
    pub fn draw<'w>(&'w mut self, f: impl for<'d> FnOnce(&'w mut WindowInner, &'d mut Drawing, &'d mut BaseDrawMode)) {
        low::begin_drawing();
        f(&mut self.0, &mut Drawing(()), &mut BaseDrawMode(()));
    }

    /// Begin drawing to render texture
    #[inline]
    pub fn texture_mode<'w>(&'w mut self, target: &mut sys::RenderTexture2D, f: impl for<'d> FnOnce(&'w mut WindowInner, &'d mut TextureMode, &'d mut BaseDrawMode)) {
        low::begin_texture_mode(*target);
        f(&mut self.0, &mut TextureMode(()), &mut BaseDrawMode(()))
    }
}

/// Color, 4 components, R8G8B8A8 (32bit)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Color {
    /// Color red value
    pub r: u8,
    /// Color green value
    pub g: u8,
    /// Color blue value
    pub b: u8,
    /// Color alpha value
    pub a: u8,
}

impl From<Color> for sys::Color {
    #[inline]
    fn from(Color { r, g, b, a }: Color) -> Self {
        Self { r, g, b, a }
    }
}

impl From<sys::Color> for Color {
    #[inline]
    fn from(sys::Color { r, g, b, a }: sys::Color) -> Self {
        Self { r, g, b, a }
    }
}

/// Construct an opaque grayscale color
#[inline]
pub const fn gray(v: u8) -> Color {
    rgb(v, v, v)
}

/// Construct a grayscale color with transparency
#[inline]
pub const fn gray_a(v: u8, a: u8) -> Color {
    rgba(v, v, v, a)
}

/// Construct an opaque RGB color
#[inline]
pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color { r, g, b, a: 255 }
}

/// Construct an RGBA color with transparency
#[inline]
pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color { r, g, b, a }
}

impl Color {
    /// Construct a new color from RGBA components
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

/// CSS named colors for [`Color`]
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
    /** #00000000 */ const TRANSPARENT:          Color = Color::BLANK;
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

/// A handle for draw functions available in [`Window::draw`]
///
/// Ends drawing when dropped
pub struct Drawing(());

impl Drop for Drawing {
    /// End canvas drawing and swap buffers (double buffering)
    #[inline]
    fn drop(&mut self) {
        low::end_drawing();
    }
}

/// Evidence [`Window::texture_mode`] has been called and hasn't returned yet
///
/// Ends texture mode when dropped
pub struct TextureMode(());

impl Drop for TextureMode {
    /// Ends drawing to render texture
    #[inline]
    fn drop(&mut self) {
        low::end_texture_mode();
    }
}

mod private {
    pub trait SealedDraw {}
    impl SealedDraw for super::Drawing {}
    impl SealedDraw for super::TextureMode {}

    pub trait SealedDrawMode {}
    impl<T: ?Sized> SealedDrawMode for super::DrawModeEnum<'_, T> {}
    impl SealedDrawMode for super::BaseDrawMode {}
    impl<T: ?Sized> SealedDrawMode for super::ScissorMode<'_, T> {}
    impl<T: ?Sized + SealedDrawMode> SealedDrawMode for &mut T {}
}

/// Raylib drawing functions
pub trait Draw: private::SealedDraw {
    /// Set background color (framebuffer clear color)
    #[inline]
    fn clear_background(&mut self, color: Color) {
        low::clear_background(color.into());
    }

    /// Draw text (using default font)
    #[inline]
    fn draw_text(&mut self, text: impl IntoCStr, pos_x: i32, pos_y: i32, font_size: u32, color: Color) {
        let text = text.into_cstr().unwrap();
        low::draw_text(text.as_ref(), pos_x, pos_y, font_size, color.into());
    }

    /// Draw a color-filled rectangle
    #[inline]
    fn draw_rectangle(&mut self, pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
        low::draw_rectangle(pos_x, pos_y, width, height, color.into());
    }
}

impl Draw for Drawing {}
impl Draw for TextureMode {}

/// Union of all Raylib draw modes
///
/// Implements [`DrawMode`] and ends the stored draw mode when dropped, except in the case of [`Self::Ignore`]
pub enum DrawModeEnum<'a, M: ?Sized + 'a> {
    /// Passthrough to outer draw mode
    ///
    /// Does not end outer draw mode when dropped
    Ignore(&'a mut M),

    /// See [`BaseDrawMode`]
    Base(BaseDrawMode),

    /// See [`ScissorMode`]
    Scissor(ScissorMode<'a, M>),
}

/// Conversion to [`DrawModeEnum`], shared by both [`DrawMode`] implementors and their references
pub trait IntoDrawModeEnum: private::SealedDrawMode {
    /// The [`DrawModeEnum`] whose generics can store `self`
    type Enum: DrawMode;

    /// Convert `self` into a [`DrawModeEnum`]
    ///
    /// Does not change the mode if `self` is a reference instead of a value
    fn into_enum(self) -> Self::Enum;
}

/// The "`None`" variant of draw modes
///
/// # Example
/// ```ignore
/// let m = if condition {
///     m.begin_scissor(0, 0, 50, 50).into_enum()
/// } else {
///     m.into_enum()
/// };
/// ```
///
/// Necessary to prevent the inner draw mode from popping when you would expect
/// one of the other branches to drop, by adding a buffer layer
///
/// --------------------------------------------------------------------------------
///
/// ### Aside
///
/// No, storing the draw mode in enums is not "zero-cost", it stores a discriminant.
/// But genuinely *intended*, well-defined behavior of Raylib shouldn't be outright
/// removed for the sake of having EXCLUSIVELY zero-cost abstractions. Sometimes
/// runtime conditions need to be done *at runtime* to ensure safety.
///
/// Plus, if you want ZERO cost, you always have the option of calling into the
/// [`low`] module itself, if you trust yourself to call the corresponding `end_*`
/// methods in the right order.
impl<'a, M: ?Sized + DrawMode> IntoDrawModeEnum for &'a mut M {
    type Enum = DrawModeEnum<'a, M>;

    #[inline]
    fn into_enum(self) -> DrawModeEnum<'a, M> {
        DrawModeEnum::Ignore(self)
    }
}

impl<M: ?Sized> IntoDrawModeEnum for DrawModeEnum<'_, M> {
    type Enum = Self;

    #[inline]
    fn into_enum(self) -> Self {
        self
    }
}

/// Change the draw mode
///
/// This trait is intentionally separate from the [`Drawing`] handle.
/// It is, on its own, zero-cost if the draw mode stack is statically known.
///
/// Conditional draw modes are possible by using [`DrawModeEnum`].
pub trait DrawMode: IntoDrawModeEnum {}

impl<M: ?Sized> DrawMode for DrawModeEnum<'_, M> {}

/// The default draw mode -- created alongside [`Drawing`] or [`TextureMode`]
pub struct BaseDrawMode(());

impl IntoDrawModeEnum for BaseDrawMode {
    type Enum = DrawModeEnum<'static, ()>;

    #[inline]
    fn into_enum(self) -> DrawModeEnum<'static, ()> {
        DrawModeEnum::Base(self)
    }
}

impl DrawMode for BaseDrawMode {}

/// Raylib scisor mode
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

impl<'a, M: ?Sized> IntoDrawModeEnum for ScissorMode<'a, M> {
    type Enum = DrawModeEnum<'a, M>;

    #[inline]
    fn into_enum(self) -> DrawModeEnum<'a, M> {
        DrawModeEnum::Scissor(self)
    }
}

impl<'a, M: ?Sized> DrawMode for ScissorMode<'a, M> {}
