#![warn(missing_docs)]

#[cfg(target_endian = "little")]
use std::mem::MaybeUninit;
use std::{marker::PhantomData, ptr::NonNull, num::NonZeroU32, time::Duration};
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

trait SizedPrimitive<const N: usize> { type Type; }
impl SizedPrimitive<0> for () { type Type = (); }
impl SizedPrimitive<1> for () { type Type = i8; }
impl SizedPrimitive<2> for () { type Type = i16; }
impl SizedPrimitive<4> for () { type Type = i32; }
impl SizedPrimitive<8> for () { type Type = i64; }

macro_rules! sys_enum_wrapper {
    (
        $(#[$m:meta])*
        $vis:vis enum $Enum:ident {
            _ := $SysNone:ident,
            $(
                $(#[$vm:meta])*
                $Variant:ident := $SysVariant:ident
            ),* $(,)?
        }
    ) => {
        $(#[$m])*
        $vis enum $Enum {
            $(
                $(#[$vm])*
                $Variant = sys::$Enum::$SysVariant as isize,
            )*
        }

        impl $Enum {
            #[inline]
            const fn into_sys(self) -> sys::$Enum {
                // SAFETY: every discriminant on $Enum maps to one on sys::$Enum
                unsafe { std::mem::transmute::<i32, sys::$Enum>(self as i32) }
            }

            #[allow(dead_code, reason = "for future use")]
            #[inline]
            const fn opt_into_sys(value: Option<Self>) -> sys::$Enum {
                match value {
                    Some(v) => v.into_sys(),
                    None => sys::$Enum::$SysNone,
                }
            }

            #[inline]
            const fn from_sys(value: sys::$Enum) -> Option<Self> {
                type Primitive = <() as SizedPrimitive<{std::mem::size_of::<$Enum>()}>>::Type;
                // SAFETY:
                // - every discriminant on sys::$Enum except for $SysNone maps to one on $Enum
                // - while sys::$Enum is repr(i32), all of its discriminants fit in Primitive
                // - $SysNone fulfills the None niche
                unsafe { std::mem::transmute::<Primitive, Option<Self>>(value as Primitive) }
            }
        }

        impl From<$Enum> for sys::$Enum {
            #[inline]
            fn from(value: $Enum) -> Self {
                value.into_sys()
            }
        }

        impl TryFrom<sys::$Enum> for $Enum {
            type Error = ();

            #[inline]
            fn try_from(value: sys::$Enum) -> Result<Self, Self::Error> {
                Self::from_sys(value).ok_or(())
            }
        }
    };

    (
        $(#[$m:meta])*
        $vis:vis enum $Enum:ident {
            $(
                $(#[$vm:meta])*
                $Variant:ident := $SysVariant:ident
            ),* $(,)?
        }
    ) => {
        $(#[$m])*
        $vis enum $Enum {
            $(
                $(#[$vm])*
                $Variant = sys::$Enum::$SysVariant as isize,
            )*
        }

        impl $Enum {
            #[inline]
            const fn into_sys(self) -> sys::$Enum {
                // SAFETY: every discriminant on $Enum maps to one on sys::$Enum
                unsafe { std::mem::transmute::<i32, sys::$Enum>(self as i32) }
            }

            #[inline]
            const fn from_sys(value: sys::$Enum) -> Self {
                type Primitive = <() as SizedPrimitive<{std::mem::size_of::<$Enum>()}>>::Type;
                // SAFETY:
                // - every discriminant on sys::$Enum maps to one on $Enum
                // - while sys::$Enum is repr(i32), all of its discriminants fit in Primitive
                unsafe { std::mem::transmute::<Primitive, Self>(value as Primitive) }
            }
        }

        impl From<$Enum> for sys::$Enum {
            #[inline]
            fn from(value: $Enum) -> Self {
                value.into_sys()
            }
        }

        impl From<sys::$Enum> for $Enum {
            #[inline]
            fn from(value: sys::$Enum) -> Self {
                Self::from_sys(value)
            }
        }
    };
}

macro_rules! sys_pod_wrapper {

}

/// Vector2, 2 components
#[repr(C)]
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
        unsafe { std::mem::transmute::<Self, sys::Vector2>(self) }
    }

    #[inline]
    const fn from_sys(value: sys::Vector2) -> Self {
        unsafe { std::mem::transmute::<sys::Vector2, Self>(value) }
    }

    #[inline]
    const fn as_sys(&self) -> &sys::Vector2 {
        unsafe { std::mem::transmute::<&Self, &sys::Vector2>(self) }
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
#[repr(C)]
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
#[repr(C)]
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

sys_enum_wrapper!{
/// Keyboard keys (US keyboard layout)
///
/// NOTE: Use [`WindowInner::get_key_pressed()`] to allow redefining
/// required keys for alternative layouts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyboardKey {
    _ := KEY_NULL,
    // Alphanumeric keys
    /// Key: `'` | `"`
    Apostrophe := KEY_APOSTROPHE,
    /// Key: `,` | `<`
    Comma := KEY_COMMA,
    /// Key: `-` | `_`
    Minus := KEY_MINUS,
    /// Key: `.` | `>`
    Period := KEY_PERIOD,
    /// Key: `/` | `?`
    Slash := KEY_SLASH,
    /// Key: `0` | `)`
    Zero := KEY_ZERO,
    /// Key: `1` | `!`
    One := KEY_ONE,
    /// Key: `2` | `@`
    Two := KEY_TWO,
    /// Key: `3` | `#`
    Three := KEY_THREE,
    /// Key: `4` | `$`
    Four := KEY_FOUR,
    /// Key: `5` | `%`
    Five := KEY_FIVE,
    /// Key: `6` | `^`
    Six := KEY_SIX,
    /// Key: `7` | `&`
    Seven := KEY_SEVEN,
    /// Key: `8` | `*`
    Eight := KEY_EIGHT,
    /// Key: `9` | `(`
    Nine := KEY_NINE,
    /// Key: `;` | `:`
    Semicolon := KEY_SEMICOLON,
    /// Key: `=` | `+`
    Equal := KEY_EQUAL,
    /// Key: `a` | `A`
    A := KEY_A,
    /// Key: `b` | `B`
    B := KEY_B,
    /// Key: `c` | `C`
    C := KEY_C,
    /// Key: `d` | `D`
    D := KEY_D,
    /// Key: `e` | `E`
    E := KEY_E,
    /// Key: `f` | `F`
    F := KEY_F,
    /// Key: `g` | `G`
    G := KEY_G,
    /// Key: `h` | `H`
    H := KEY_H,
    /// Key: `i` | `I`
    I := KEY_I,
    /// Key: `j` | `J`
    J := KEY_J,
    /// Key: `k` | `K`
    K := KEY_K,
    /// Key: `l` | `L`
    L := KEY_L,
    /// Key: `m` | `M`
    M := KEY_M,
    /// Key: `n` | `N`
    N := KEY_N,
    /// Key: `o` | `O`
    O := KEY_O,
    /// Key: `p` | `P`
    P := KEY_P,
    /// Key: `q` | `Q`
    Q := KEY_Q,
    /// Key: `r` | `R`
    R := KEY_R,
    /// Key: `s` | `S`
    S := KEY_S,
    /// Key: `t` | `T`
    T := KEY_T,
    /// Key: `u` | `U`
    U := KEY_U,
    /// Key: `v` | `V`
    V := KEY_V,
    /// Key: `w` | `W`
    W := KEY_W,
    /// Key: `x` | `X`
    X := KEY_X,
    /// Key: `y` | `Y`
    Y := KEY_Y,
    /// Key: `z` | `Z`
    Z := KEY_Z,
    /// Key: `[` | `{`
    LeftBracket := KEY_LEFT_BRACKET,
    /// Key: `\` | `|`
    Backslash := KEY_BACKSLASH,
    /// Key: `]` | `}`
    RightBracket := KEY_RIGHT_BRACKET,
    /// Key: `` ` `` | `~`
    Grave := KEY_GRAVE,
    // Function keys
    /// Key: Space
    Space := KEY_SPACE,
    /// Key: Esc
    Escape := KEY_ESCAPE,
    /// Key: Enter `⏎
    Enter := KEY_ENTER,
    /// Key: Tab
    Tab := KEY_TAB,
    /// Key: Backspace
    Backspace := KEY_BACKSPACE,
    /// Key: Ins
    Insert := KEY_INSERT,
    /// Key: Del
    Delete := KEY_DELETE,
    /// Key: Cursor right `˃`
    Right := KEY_RIGHT,
    /// Key: Cursor left `˂`
    Left := KEY_LEFT,
    /// Key: Cursor down `˅`
    Down := KEY_DOWN,
    /// Key: Cursor up `˄`
    Up := KEY_UP,
    /// Key: Page up
    PageUp := KEY_PAGE_UP,
    /// Key: Page down
    PageDown := KEY_PAGE_DOWN,
    /// Key: Home
    Home := KEY_HOME,
    /// Key: End
    End := KEY_END,
    /// Key: Caps lock
    CapsLock := KEY_CAPS_LOCK,
    /// Key: Scroll lock
    ScrollLock := KEY_SCROLL_LOCK,
    /// Key: Num lock
    NumLock := KEY_NUM_LOCK,
    /// Key: Print screen
    PrintScreen := KEY_PRINT_SCREEN,
    /// Key: Pause
    Pause := KEY_PAUSE,
    /// Key: `F1`
    F1 := KEY_F1,
    /// Key: `F2`
    F2 := KEY_F2,
    /// Key: `F3`
    F3 := KEY_F3,
    /// Key: `F4`
    F4 := KEY_F4,
    /// Key: `F5`
    F5 := KEY_F5,
    /// Key: `F6`
    F6 := KEY_F6,
    /// Key: `F7`
    F7 := KEY_F7,
    /// Key: `F8`
    F8 := KEY_F8,
    /// Key: `F9`
    F9 := KEY_F9,
    /// Key: `F10`
    F10 := KEY_F10,
    /// Key: `F11`
    F11 := KEY_F11,
    /// Key: `F12`
    F12 := KEY_F12,
    /// Key: Shift left
    LeftShift := KEY_LEFT_SHIFT,
    /// Key: Ctrl left
    LeftControl := KEY_LEFT_CONTROL,
    /// Key: Alt left
    LeftAlt := KEY_LEFT_ALT,
    /// Key: Super left
    LeftSuper := KEY_LEFT_SUPER,
    /// Key: Shift right
    RightShift := KEY_RIGHT_SHIFT,
    /// Key: Ctrl right
    RightControl := KEY_RIGHT_CONTROL,
    /// Key: Alt right
    RightAlt := KEY_RIGHT_ALT,
    /// Key: Super right
    RightSuper := KEY_RIGHT_SUPER,
    /// Key: KB menu
    KbMenu := KEY_KB_MENU,
    // Keypad keys
    /// Key: Keypad `0`
    Kp0 := KEY_KP_0,
    /// Key: Keypad `1`
    Kp1 := KEY_KP_1,
    /// Key: Keypad `2`
    Kp2 := KEY_KP_2,
    /// Key: Keypad `3`
    Kp3 := KEY_KP_3,
    /// Key: Keypad `4`
    Kp4 := KEY_KP_4,
    /// Key: Keypad `5`
    Kp5 := KEY_KP_5,
    /// Key: Keypad `6`
    Kp6 := KEY_KP_6,
    /// Key: Keypad `7`
    Kp7 := KEY_KP_7,
    /// Key: Keypad `8`
    Kp8 := KEY_KP_8,
    /// Key: Keypad `9`
    Kp9 := KEY_KP_9,
    /// Key: Keypad `.`
    KpDecimal := KEY_KP_DECIMAL,
    /// Key: Keypad `/`
    KpDivide := KEY_KP_DIVIDE,
    /// Key: Keypad `*`
    KpMultiply := KEY_KP_MULTIPLY,
    /// Key: Keypad `-`
    KpSubtract := KEY_KP_SUBTRACT,
    /// Key: Keypad `+`
    KpAdd := KEY_KP_ADD,
    /// Key: Keypad Enter `⏎`
    KpEnter := KEY_KP_ENTER,
    /// Key: Keypad `=`
    KpEqual := KEY_KP_EQUAL,
    // Android key buttons
    /// Key: Android back button
    Back := KEY_BACK,
    /// Key: Android menu button
    Menu := KEY_MENU,
    /// Key: Android volume up button
    VolumeUp := KEY_VOLUME_UP,
    /// Key: Android volume down button
    VolumeDown := KEY_VOLUME_DOWN,
}
}

sys_enum_wrapper! {
/// Mouse buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    /// Mouse button left
    Left := MOUSE_BUTTON_LEFT,
    /// Mouse button right
    Right := MOUSE_BUTTON_RIGHT,
    /// Mouse button middle (pressed wheel)
    Middle := MOUSE_BUTTON_MIDDLE,
    /// Mouse button side (advanced mouse device)
    Side := MOUSE_BUTTON_SIDE,
    /// Mouse button extra (advanced mouse device)
    Extra := MOUSE_BUTTON_EXTRA,
    /// Mouse button forward (advanced mouse device)
    Forward := MOUSE_BUTTON_FORWARD,
    /// Mouse button back (advanced mouse device)
    Back := MOUSE_BUTTON_BACK,
}
}

sys_enum_wrapper! {
/// Mouse cursor
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseCursor {
    /// Default pointer shape
    Default := MOUSE_CURSOR_DEFAULT,
    /// Arrow shape
    Arrow := MOUSE_CURSOR_ARROW,
    /// Text writing cursor shape
    IBeam := MOUSE_CURSOR_IBEAM,
    /// Cross shape
    Crosshair := MOUSE_CURSOR_CROSSHAIR,
    /// Pointing hand cursor
    PointingHand := MOUSE_CURSOR_POINTING_HAND,
    /// Horizontal resize/move arrow shape
    ResizeEW := MOUSE_CURSOR_RESIZE_EW,
    /// Vertical resize/move arrow shape
    ResizeNS := MOUSE_CURSOR_RESIZE_NS,
    /// Top-left to bottom-right diagonal resize/move arrow shape
    ResizeNWSE := MOUSE_CURSOR_RESIZE_NWSE,
    /// The top-right to bottom-left diagonal resize/move arrow shape
    ResizeNESW := MOUSE_CURSOR_RESIZE_NESW,
    /// The omnidirectional resize/move cursor shape
    ResizeAll := MOUSE_CURSOR_RESIZE_ALL,
    /// The operation-not-allowed shape
    NotAllowed := MOUSE_CURSOR_NOT_ALLOWED,
}
}

sys_enum_wrapper! {
/// Gamepad buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadButton {
    _ := GAMEPAD_BUTTON_UNKNOWN,
    /// Gamepad left DPAD up button
    LeftFaceUp := GAMEPAD_BUTTON_LEFT_FACE_UP,
    /// Gamepad left DPAD right button
    LeftFaceRight := GAMEPAD_BUTTON_LEFT_FACE_RIGHT,
    /// Gamepad left DPAD down button
    LeftFaceDown := GAMEPAD_BUTTON_LEFT_FACE_DOWN,
    /// Gamepad left DPAD left button
    LeftFaceLeft := GAMEPAD_BUTTON_LEFT_FACE_LEFT,
    /// Gamepad right button up (i.e. PS3: Triangle, Xbox: Y)
    RightFaceUp := GAMEPAD_BUTTON_RIGHT_FACE_UP,
    /// Gamepad right button right (i.e. PS3: Circle, Xbox: B)
    RightFaceRight := GAMEPAD_BUTTON_RIGHT_FACE_RIGHT,
    /// Gamepad right button down (i.e. PS3: Cross, Xbox: A)
    RightFaceDown := GAMEPAD_BUTTON_RIGHT_FACE_DOWN,
    /// Gamepad right button left (i.e. PS3: Square, Xbox: X)
    RightFaceLeft := GAMEPAD_BUTTON_RIGHT_FACE_LEFT,
    /// Gamepad top/back trigger left (first), it could be a trailing button
    LeftTrigger1 := GAMEPAD_BUTTON_LEFT_TRIGGER_1,
    /// Gamepad top/back trigger left (second), it could be a trailing button
    LeftTrigger2 := GAMEPAD_BUTTON_LEFT_TRIGGER_2,
    /// Gamepad top/back trigger right (first), it could be a trailing button
    RightTrigger1 := GAMEPAD_BUTTON_RIGHT_TRIGGER_1,
    /// Gamepad top/back trigger right (second), it could be a trailing button
    RightTrigger2 := GAMEPAD_BUTTON_RIGHT_TRIGGER_2,
    /// Gamepad center buttons, left one (i.e. PS3: Select)
    MiddleLeft := GAMEPAD_BUTTON_MIDDLE_LEFT,
    /// Gamepad center buttons, middle one (i.e. PS3: PS, Xbox: XBOX)
    Middle := GAMEPAD_BUTTON_MIDDLE,
    /// Gamepad center buttons, right one (i.e. PS3: Start)
    MiddleRight := GAMEPAD_BUTTON_MIDDLE_RIGHT,
    /// Gamepad joystick pressed button left
    LeftThumb := GAMEPAD_BUTTON_LEFT_THUMB,
    /// Gamepad joystick pressed button right
    RightThumb := GAMEPAD_BUTTON_RIGHT_THUMB,
}
}

sys_enum_wrapper! {
/// Gamepad axes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadAxis {
    /// Gamepad left stick X axis
    LeftX := GAMEPAD_AXIS_LEFT_X,
    /// Gamepad left stick Y axis
    LeftY := GAMEPAD_AXIS_LEFT_Y,
    /// Gamepad right stick X axis
    RightX := GAMEPAD_AXIS_RIGHT_X,
    /// Gamepad right stick Y axis
    RightY := GAMEPAD_AXIS_RIGHT_Y,
    /// Gamepad back trigger left, pressure level: [1..-1]
    LeftTrigger := GAMEPAD_AXIS_LEFT_TRIGGER,
    /// Gamepad back trigger right, pressure level: [1..-1]
    RightTrigger := GAMEPAD_AXIS_RIGHT_TRIGGER,
}
}

sys_enum_wrapper! {
/// Pixel formats
///
/// NOTE: Support depends on OpenGL version and platform
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PixelFormat {
    /// Uncompressed opaque grayscale format -- 8 bit per pixel (no alpha)
    UncompressedGrayscale := PIXELFORMAT_UNCOMPRESSED_GRAYSCALE,
    /// Uncompressed transparent grayscale format -- 8*2 bpp (2 channels)
    UncompressedGrayAlpha := PIXELFORMAT_UNCOMPRESSED_GRAY_ALPHA,
    /// Uncompressed 16-bit RGB format: 5-bit red channel, 6-bit green channel, 5-bit blue channel -- 16 bpp
    UncompressedR5G6B5 := PIXELFORMAT_UNCOMPRESSED_R5G6B5,
    /// Uncompressed 24-bit RGB format: 8-bit red channel, 8-bit green channel, 8-bit blue channel -- 24 bpp
    UncompressedR8G8B8 := PIXELFORMAT_UNCOMPRESSED_R8G8B8,
    /// Uncompressed 16-bit RGBA format: 5-bit red channel, 5-bit green channel, 5-bit blue channel, 1-bit alpha channel -- 16 bpp (1 bit alpha)
    UncompressedR5G5B5A1 := PIXELFORMAT_UNCOMPRESSED_R5G5B5A1,
    /// Uncompressed 16-bit RGBA format: 4-bit red channel, 4-bit green channel, 4-bit blue channel, 4-bit alpha channel -- 16 bpp (4 bit alpha)
    UncompressedR4G4B4A4 := PIXELFORMAT_UNCOMPRESSED_R4G4B4A4,
    /// Uncompressed 32-bit RGBA format: 8-bit red channel, 8-bit green channel, 8-bit blue channel, 8-bit alpha channel -- 32 bpp
    UncompressedR8G8B8A8 := PIXELFORMAT_UNCOMPRESSED_R8G8B8A8,
    /// Uncompressed 32-bit color format: 32-bit red channel -- 32 bpp (1 channel - float)
    UncompressedR32 := PIXELFORMAT_UNCOMPRESSED_R32,
    /// Uncompressed 96-bit RGB format: 32-bit red channel, 32-bit green channel, 32-bit blue channel -- 32*3 bpp (3 channels - float)
    UncompressedR32G32B32 := PIXELFORMAT_UNCOMPRESSED_R32G32B32,
    /// Uncompressed 128-bit RGB format: 32-bit red channel, 32-bit green channel, 32-bit blue channel, 32-bit alpha channel -- 32*4 bpp (4 channels - float)
    UncompressedR32G32B32A32 := PIXELFORMAT_UNCOMPRESSED_R32G32B32A32,
    /// Uncompressed 16-bit color format: 16-bit red channel -- 16 bpp (1 channel - half float)
    UncompressedR16 := PIXELFORMAT_UNCOMPRESSED_R16,
    /// Uncompressed 48-bit RGB format: 16-bit red channel, 16-bit green channel, 16-bit blue channel -- 16*3 bpp (3 channels - half float)
    UncompressedR16G16B16 := PIXELFORMAT_UNCOMPRESSED_R16G16B16,
    /// Uncompressed 48-bit RGB format: 16-bit red channel, 16-bit green channel, 16-bit blue channel, 16-bit alpha channel -- 16*4 bpp (4 channels - half float)
    UncompressedR16G16B16A16 := PIXELFORMAT_UNCOMPRESSED_R16G16B16A16,
    /// Compressed DXT1 RGB format -- 4 bpp (no alpha)
    CompressedDxt1RGB := PIXELFORMAT_COMPRESSED_DXT1_RGB,
    /// Compressed DXT1 RGBA format -- 4 bpp (1 bit alpha)
    CompressedDxt1RGBA := PIXELFORMAT_COMPRESSED_DXT1_RGBA,
    /// Compressed DXT3 RGBA format -- 8 bpp
    CompressedDxt3RGBA := PIXELFORMAT_COMPRESSED_DXT3_RGBA,
    /// Compressed DXT5 RGBA format -- 8 bpp
    CompressedDxt5RGBA := PIXELFORMAT_COMPRESSED_DXT5_RGBA,
    /// Compressed ETC1 (Ericsson Texture Compression) RGB format -- 4 bpp
    CompressedEtc1RGB := PIXELFORMAT_COMPRESSED_ETC1_RGB,
    /// Compressed ETC2 (Ericsson Texture Compression) RGB format -- 4 bpp
    CompressedEtc2RGB := PIXELFORMAT_COMPRESSED_ETC2_RGB,
    /// Compressed ETC2 (Ericsson Texture Compression) EAC RGBA format -- 8 bpp
    CompressedEtc2EacRGBA := PIXELFORMAT_COMPRESSED_ETC2_EAC_RGBA,
    /// PowerVR Texture Compressed (PVRTC) RGB format -- 4 bpp
    CompressedPvrtRGB := PIXELFORMAT_COMPRESSED_PVRT_RGB,
    /// PowerVR Texture Compressed (PVRTC) RGBA format -- 4 bpp
    CompressedPvrtRGBA := PIXELFORMAT_COMPRESSED_PVRT_RGBA,
    /// 4x4-block Adaptive Scalable Texture Compression (ASTC) RGBA format -- 8 bpp
    CompressedAstc4x4RGBA := PIXELFORMAT_COMPRESSED_ASTC_4x4_RGBA,
    /// 8x8-block Adaptive Scalable Texture Compression (ASTC) RGBA format -- 2 bpp
    CompressedAstc8x8RGBA := PIXELFORMAT_COMPRESSED_ASTC_8x8_RGBA,
}
}

sys_enum_wrapper! {
/// Texture parameters: filter mode
///
/// NOTE 1: Filtering considers mipmaps if available in the texture
///
/// NOTE 2: Filter is accordingly set for minification and magnification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextureFilter {
    /// No filter, just pixel approximation
    Point := TEXTURE_FILTER_POINT,
    /// Linear filtering
    Bilinear := TEXTURE_FILTER_BILINEAR,
    /// Trilinear filtering (linear with mipmaps)
    Trilinear := TEXTURE_FILTER_TRILINEAR,
    /// Anisotropic filtering 4x
    Anisotropic4x := TEXTURE_FILTER_ANISOTROPIC_4X,
    /// Anisotropic filtering 8x
    Anisotropic8x := TEXTURE_FILTER_ANISOTROPIC_8X,
    /// Anisotropic filtering 16x
    Anisotropic16x := TEXTURE_FILTER_ANISOTROPIC_16X,
}
}

sys_enum_wrapper! {
/// Texture parameters: wrap mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextureWrap {
    /// Repeats texture in tiled mode
    Repeat := TEXTURE_WRAP_REPEAT,
    /// Clamps texture to edge pixel in tiled mode
    Clamp := TEXTURE_WRAP_CLAMP,
    /// Mirrors and repeats the texture in tiled mode
    MirrorRepeat := TEXTURE_WRAP_MIRROR_REPEAT,
    /// Mirrors and clamps to border the texture in tiled mode
    MirrorClamp := TEXTURE_WRAP_MIRROR_CLAMP,
}
}

sys_enum_wrapper! {
/// Cubemap layouts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CubemapLayout {
    /// Automatically detect layout type
    AutoDetect := CUBEMAP_LAYOUT_AUTO_DETECT,
    /// Layout is defined by a vertical line with faces
    LineVertical := CUBEMAP_LAYOUT_LINE_VERTICAL,
    /// Layout is defined by a horizontal line with faces
    LineHorizontal := CUBEMAP_LAYOUT_LINE_HORIZONTAL,
    /// Layout is defined by a 3x4 cross with cubemap faces
    CrossThreeByFour := CUBEMAP_LAYOUT_CROSS_THREE_BY_FOUR,
    /// Layout is defined by a 4x3 cross with cubemap faces
    CrossFourByThree := CUBEMAP_LAYOUT_CROSS_FOUR_BY_THREE,
}
}

/// Image, pixel data stored in CPU memory (RAM)
#[repr(C)]
#[derive(Debug)]
pub struct Image {
    // Fields are private so that users cannot modify the values and set them to an invalid state.
    // Image fields should only be modified through image methods.
    data: NonNull<u8>,
    width: NonZeroU32,
    height: NonZeroU32,
    mipmaps: NonZeroU32,
    #[cfg(target_endian = "little")]
    _format_padding: MaybeUninit<[u8; 3]>,
    format: PixelFormat,
    #[cfg(target_endian = "big")]
    _format_padding: MaybeUninit<[u8; 3]>,
}
const _: () = {
    assert!(std::mem::size_of::<Image>() == std::mem::size_of::<sys::Image>());
    assert!(std::mem::offset_of!(Image, data) == std::mem::offset_of!(sys::Image, data));
    assert!(std::mem::offset_of!(Image, width) == std::mem::offset_of!(sys::Image, width));
    assert!(std::mem::offset_of!(Image, height) == std::mem::offset_of!(sys::Image, height));
    assert!(std::mem::offset_of!(Image, mipmaps) == std::mem::offset_of!(sys::Image, mipmaps));
    assert!(std::mem::offset_of!(Image, format) == std::mem::offset_of!(sys::Image, format) + if cfg!(target_endian = "little") { 3 } else { 0 });
};

impl Drop for Image {
    /// Unload image from CPU memory (RAM)
    #[inline]
    fn drop(&mut self) {
        // SAFETY: Image must have been loaded properly
        unsafe {
            low::unload_image(*self.as_sys());
        }
    }
}

const _: () = {
    // SAFETY: The test is to check whether this is safe
    let img = unsafe {
        std::mem::transmute::<sys::Image, Option<Image>>(sys::Image {
            data: std::ptr::null_mut(),
            width: 1,
            height: 1,
            mipmaps: 1,
            format: 1,
        })
    };
    let null_transmutes_to_none = img.is_none();
    std::mem::forget(img);
    assert!(null_transmutes_to_none, "null data should transmute to None");
};

impl Image {
    /// # Panics
    /// - `width` is less than 1
    /// - `height` is less than 1
    /// - `mipmaps` is less than 1
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
    ///   (**users should only ever have access to *valid* `Images`**)
    #[inline]
    #[allow(dead_code, reason = "to be used in `load_image`")]
    unsafe fn from_sys(value: sys::Image) -> Option<Self> {
        assert!(1 <= value.width);
        assert!(1 <= value.height);
        assert!(1 <= value.mipmaps);
        assert!(1 <= value.format && value.format <= 24);

        // SAFETY:
        // - each field has been asserted as valid (except for data)
        // - transmutes to None if (and only if, due to asserts) `data` is NULL
        // - `Image` is repr(C) and contains padding for compatibility with `sys::Image`
        // - each field of `Image` is the same size and shape as the corresponding fields in `sys::Image`
        unsafe { std::mem::transmute(value) }
    }

    #[inline]
    fn as_sys(&self) -> &sys::Image {
        // SAFETY:
        // - `Image` is repr(C) and contains padding for compatibility with `sys::Image`
        // - `Image` fields cannot be set by the user and all mutators are checked/restricted
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    fn slice_as_sys(slice: &[Image]) -> &[sys::Image] {
        // SAFETY:
        // - `Image` is repr(C) and contains padding for compatibility with `sys::Image`
        // - `Image` fields cannot be set by the user and all mutators are checked/restricted
        unsafe { std::mem::transmute(slice) }
    }

    #[inline]
    const fn data_len(&self) -> usize {
        // SAFETY: `PixelFormat` discriminants are identical to `sys::PixelFormat`'s
        let format = unsafe { std::mem::transmute::<i32, sys::PixelFormat>(self.format as i32) };
        low::get_pixel_data_size(self.width.get(), self.height.get(), format)
    }

    /// Image raw data as bytes
    #[inline]
    pub const fn data(&self) -> &[u8] {
        let len = self.data_len();
        // SAFETY: `data` is guaranteed to have length `len` for valid `Image`s
        unsafe { std::slice::from_raw_parts(self.data.as_ptr(), len) }
    }

    /// Image raw data as mutable bytes
    #[inline]
    pub const fn data_mut(&mut self) -> &mut [u8] {
        let len = self.data_len();
        // SAFETY: `data` is guaranteed to have length `len` for valid `Image`s
        unsafe { std::slice::from_raw_parts_mut(self.data.as_ptr(), len) }
    }

    /// Image base width
    #[inline]
    pub const fn width(&self) -> NonZeroU32 {
        self.width
    }

    /// Image base height
    #[inline]
    pub const fn height(&self) -> NonZeroU32 {
        self.height
    }

    /// Mipmap levels, 1 by default
    #[inline]
    pub const fn mipmaps(&self) -> NonZeroU32 {
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
        // SAFETY: Existence of `WindowInner` proves window is initialized
        unsafe { low::window_should_close() }
    }

    /// Check if window is currently fullscreen
    #[inline]
    pub fn is_window_fullscreen(&self) -> bool {
        // SAFETY: Existence of `WindowInner` proves window is initialized
        unsafe { low::is_window_fullscreen() }
    }

    /// Check if window is currently hidden
    #[inline]
    pub fn is_window_hidden(&self) -> bool {
        // SAFETY: Existence of `WindowInner` proves window is initialized
        unsafe { low::is_window_hidden() }
    }

    /// Check if window is currently minimized
    #[inline]
    pub fn is_window_minimized(&self) -> bool {
        // SAFETY: Existence of `WindowInner` proves window is initialized
        unsafe { low::is_window_minimized() }
    }

    /// Check if window is currently maximized
    #[inline]
    pub fn is_window_maximized(&self) -> bool {
        // SAFETY: Existence of `WindowInner` proves window is initialized
        unsafe { low::is_window_maximized() }
    }

    /// Check if window is currently focused
    #[inline]
    pub fn is_window_focused(&self) -> bool {
        // SAFETY: Existence of `WindowInner` proves window is initialized
        unsafe { low::is_window_focused() }
    }

    /// Check if window has been resized last frame
    #[inline]
    pub fn is_window_resized(&self) -> bool {
        // SAFETY: Existence of `WindowInner` proves window is initialized
        unsafe { low::is_window_resized() }
    }

    /// Set window configuration state using flags
    #[inline]
    pub fn set_window_state(&mut self, flags: sys::ConfigFlags) {
        // SAFETY: Existence of `WindowInner` proves window is initialized
        unsafe {
            low::set_window_state(flags);
        }
    }

    /// Clear window configuration state flags
    #[inline]
    pub fn clear_window_state(&mut self, flags: sys::ConfigFlags) {
        // SAFETY: Existence of `WindowInner` proves window is initialized
        unsafe {
            low::clear_window_state(flags);
        }
    }

    /// Toggle window state: fullscreen/windowed, resizes monitor to match window resolution
    #[inline]
    pub fn toggle_fullscreen(&mut self) {
        // SAFETY: Existence of `WindowInner` proves window is initialized
        unsafe {
            low::toggle_fullscreen();
        }
    }

    /// Toggle window state: borderless windowed, resizes window to match monitor resolution
    #[inline]
    pub fn toggle_borderless_windowed(&mut self) {
        // SAFETY: Existence of `WindowInner` proves window is initialized
        unsafe {
            low::toggle_borderless_windowed();
        }
    }

    /// Set window state: maximized, if resizable
    #[inline]
    pub fn maximize_window(&mut self) {
        // SAFETY: Existence of `WindowInner` proves window is initialized
        unsafe {
            low::maximize_window();
        }
    }

    /// Set window state: minimized, if resizable
    #[inline]
    pub fn minimize_window(&mut self) {
        // SAFETY: Existence of `WindowInner` proves window is initialized
        unsafe {
            low::minimize_window();
        }
    }

    /// Restore window from being minimized/maximized
    #[inline]
    pub fn restore_window(&mut self) {
        // SAFETY: Existence of `WindowInner` proves window is initialized
        unsafe {
            low::restore_window();
        }
    }

    /// Set icon for window (single image, RGBA 32bit)
    #[inline]
    pub fn set_window_icon(&mut self, image: &Image) {
        // SAFETY:
        // - Existence of `WindowInner` proves window is initialized
        // - `Image` validity is asserted by `as_sys()`
        unsafe {
            low::set_window_icon(*image.as_sys());
        }
    }

    /// Set icon for window (multiple images, RGBA 32bit)
    #[inline]
    pub fn set_window_icons(&mut self, images: &[Image]) {
        // SAFETY:
        // - Existence of `WindowInner` proves window is initialized
        // - `Image` validity is asserted by `slice_as_sys()`
        unsafe {
            low::set_window_icons(Image::slice_as_sys(images));
        }
    }

    /// Set title for window
    ///
    /// # Panics
    /// - [`IntoCStr::into_cstr()`] fails for `title`
    #[inline]
    pub fn set_window_title(&mut self, title: impl IntoCStr) {
        let title = title.into_cstr().unwrap();
        // SAFETY: Existence of `WindowInner` proves window is initialized
        unsafe {
            low::set_window_title(title.as_ref());
        }
    }

    /// Set window position on screen
    #[inline]
    pub fn set_window_position(&mut self, x: i32, y: i32) {
        // SAFETY: Existence of `WindowInner` proves window is initialized
        unsafe {
            low::set_window_position(x, y);
        }
    }

    /// Measure string width for default font
    ///
    /// # Panics
    /// - [`IntoCStr::into_cstr()`] fails for `text`
    #[inline]
    pub fn measure_text(&self, text: impl IntoCStr, font_size: u32) -> i32 {
        let text = text.into_cstr().unwrap();
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { low::measure_text(text.as_ref(), font_size) }
    }

    /// Measure string size for Font
    ///
    /// # Panics
    /// - [`IntoCStr::into_cstr()`] fails for `text`
    #[inline]
    pub fn measure_text_ex(&self, font: sys::Font, text: impl IntoCStr, font_size: f32, spacing: f32) -> Vector2 {
        let text = text.into_cstr().unwrap();
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { Vector2::from_sys(low::measure_text_ex(font, text.as_ref(), font_size, spacing)) }
    }

    // Input-related functions: keyboard

    /// Check if a key has been pressed once
    #[inline]
    pub fn is_key_pressed(&self, key: KeyboardKey) -> bool {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { low::is_key_pressed(key.into_sys()) }
    }

    /// Check if a key has been pressed again
    #[inline]
    pub fn is_key_pressed_repeat(&self, key: KeyboardKey) -> bool {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { low::is_key_pressed_repeat(key.into_sys()) }
    }

    /// Check if a key is being pressed
    #[inline]
    pub fn is_key_down(&self, key: KeyboardKey) -> bool {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { low::is_key_down(key.into_sys()) }
    }

    /// Check if a key has been released once
    #[inline]
    pub fn is_key_released(&self, key: KeyboardKey) -> bool {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { low::is_key_released(key.into_sys()) }
    }

    /// Check if a key is NOT being pressed
    #[inline]
    pub fn is_key_up(&self, key: KeyboardKey) -> bool {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { low::is_key_up(key.into_sys()) }
    }

    /// Get key pressed (keycode), call it multiple times for keys queued, returns [`None`] when the queue is empty
    #[inline]
    pub fn get_key_pressed(&self) -> Option<KeyboardKey> {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe {
            low::get_key_pressed()
                .and_then(KeyboardKey::from_sys)
        }
    }

    /// Get char pressed (unicode), call it multiple times for chars queued, returns [`None`] when the queue is empty
    #[inline]
    pub fn get_char_pressed(&self) -> Option<char> {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { low::get_char_pressed() }
    }

    /// Get name of a QWERTY key on the current keyboard layout (eg returns string 'q' for KEY_A on an AZERTY keyboard)
    ///
    /// # Panics
    /// - Raylib returns a string that isn't UTF8
    #[inline]
    pub fn get_key_name(&self, key: KeyboardKey) -> Option<&'static str> {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe {
            low::get_key_name(key.into_sys())
                .map(|s| str::from_utf8(s.to_bytes()).unwrap())
        }
    }

    /// Set a custom key to exit program (default is ESC)
    #[inline]
    pub fn set_exit_key(&mut self, key: Option<KeyboardKey>) {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe {
            low::set_exit_key(KeyboardKey::opt_into_sys(key));
        }
    }

    // Input-related functions: gamepads

    /// Check if a gamepad is available
    #[inline]
    pub fn is_gamepad_available(&self, gamepad: usize) -> bool {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { low::is_gamepad_available(gamepad) }
    }

    /// Get gamepad internal name id
    ///
    /// # Panics
    /// - Raylib returns a string that is isn't valid utf8
    #[inline]
    pub fn get_gamepad_name(&self, gamepad: usize) -> Option<&'static str> {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe {
            low::get_gamepad_name(gamepad)
                .map(|s| str::from_utf8(s.to_bytes()).unwrap())
        }
    }

    /// Check if a gamepad button has been pressed once
    #[inline]
    pub fn is_gamepad_button_pressed(&self, gamepad: usize, button: GamepadButton) -> bool {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { low::is_gamepad_button_pressed(gamepad, button.into()) }
    }

    /// Check if a gamepad button is being pressed
    #[inline]
    pub fn is_gamepad_button_down(&self, gamepad: usize, button: GamepadButton) -> bool {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { low::is_gamepad_button_down(gamepad, button.into()) }
    }

    /// Check if a gamepad button has been released once
    #[inline]
    pub fn is_gamepad_button_released(&self, gamepad: usize, button: GamepadButton) -> bool {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { low::is_gamepad_button_released(gamepad, button.into()) }
    }

    /// Check if a gamepad button is NOT being pressed
    #[inline]
    pub fn is_gamepad_button_up(&self, gamepad: usize, button: GamepadButton) -> bool {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { low::is_gamepad_button_up(gamepad, button.into()) }
    }

    /// Get the last gamepad button pressed
    #[inline]
    pub fn get_gamepad_button_pressed(&self) -> Option<GamepadButton> {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { low::get_gamepad_button_pressed().try_into().ok() }
    }

    /// Get axis count for a gamepad
    #[inline]
    pub fn get_gamepad_axis_count(&self, gamepad: usize) -> usize {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { low::get_gamepad_axis_count(gamepad) }
    }

    /// Get movement value for a gamepad axis
    #[inline]
    pub fn get_gamepad_axis_movement(&self, gamepad: usize, axis: GamepadAxis) -> f32 {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { low::get_gamepad_axis_movement(gamepad, axis.into()) }
    }

    /// Set internal gamepad mappings (SDL_GameControllerDB)
    ///
    /// # Panics
    /// - [`IntoCStr::into_cstr()`] fails for any string in `mappings`
    #[inline]
    pub fn set_gamepad_mappings(&mut self, mappings: Option<impl IntoCStr>) -> i32 {
        let mappings = mappings.map(|s| s.into_cstr().unwrap());
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { low::set_gamepad_mappings(mappings.as_ref().map(|s| s.as_ref())) }
    }

    /// Set gamepad vibration for both motors (duration in seconds)
    #[inline]
    pub fn set_gamepad_vibration(&mut self, gamepad: usize, left_motor: f32, right_motor: f32, duration: f32) {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
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
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { low::is_mouse_button_pressed(button.into()) }
    }

    /// Check if a mouse button is being pressed
    #[inline]
    pub fn is_mouse_button_down(&self, button: MouseButton) -> bool {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { low::is_mouse_button_down(button.into()) }
    }

    /// Check if a mouse button has been released once
    #[inline]
    pub fn is_mouse_button_released(&self, button: MouseButton) -> bool {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { low::is_mouse_button_released(button.into()) }
    }

    /// Check if a mouse button is NOT being pressed
    #[inline]
    pub fn is_mouse_button_up(&self, button: MouseButton) -> bool {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { low::is_mouse_button_up(button.into()) }
    }

    /// Get mouse position X
    #[inline]
    pub fn get_mouse_x(&self) -> i32 {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { low::get_mouse_x() }
    }

    /// Get mouse position Y
    #[inline]
    pub fn get_mouse_y(&self) -> i32 {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { low::get_mouse_y() }
    }

    /// Get mouse position XY
    #[inline]
    pub fn get_mouse_position(&self) -> Vector2 {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { low::get_mouse_position().into() }
    }

    /// Get mouse delta between frames
    #[inline]
    pub fn get_mouse_delta(&self) -> Vector2 {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { low::get_mouse_delta().into() }
    }

    /// Set mouse position XY
    #[inline]
    pub fn set_mouse_position(&mut self, x: i32, y: i32) {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe {
            low::set_mouse_position(x, y);
        }
    }

    /// Set mouse offset
    #[inline]
    pub fn set_mouse_offset(&mut self, offset_x: i32, offset_y: i32) {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe {
            low::set_mouse_offset(offset_x, offset_y);
        }
    }

    /// Set mouse scaling
    #[inline]
    pub fn set_mouse_scale(&mut self, scale_x: f32, scale_y: f32) {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe {
            low::set_mouse_scale(scale_x, scale_y);
        }
    }

    /// Get mouse wheel movement for X or Y, whichever is larger
    #[inline]
    pub fn get_mouse_wheel_move(&self) -> f32 {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { low::get_mouse_wheel_move() }
    }

    /// Get mouse wheel movement for both X and Y
    #[inline]
    pub fn get_mouse_wheel_move_v(&self) -> Vector2 {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { low::get_mouse_wheel_move_v().into() }
    }

    /// Set mouse cursor
    #[inline]
    pub fn set_mouse_cursor(&mut self, cursor: MouseCursor) {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe {
            low::set_mouse_cursor(cursor.into());
        }
    }

    // Input-related functions: touch

    /// Get touch position X for touch point 0 (relative to screen size)
    #[inline]
    pub fn get_touch_x(&self) -> i32 {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { low::get_touch_x() }
    }

    /// Get touch position Y for touch point 0 (relative to screen size)
    #[inline]
    pub fn get_touch_y(&self) -> i32 {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { low::get_touch_y() }
    }

    /// Get touch position XY for a touch point index (relative to screen size)
    #[inline]
    pub fn get_touch_position(&self, index: usize) -> Vector2 {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { low::get_touch_position(index).into() }
    }

    /// Get touch point identifier for given index
    #[inline]
    pub fn get_touch_point_id(&self, index: usize) -> Option<u32> {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { low::get_touch_point_id(index) }
    }

    /// Get number of touch points
    #[inline]
    pub fn get_touch_point_count(&self) -> usize {
        // SAFETY: Existence of `WindowInner` proves Raylib is initialized
        unsafe { low::get_touch_point_count() }
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
        // SAFETY: Existence of `Window` proves window is initialized and not drawing
        unsafe {
            low::close_window();
        }
    }
}

impl Window {
    /// Initialize window and OpenGL context
    ///
    /// # Panics
    /// - [`IntoCStr::into_cstr()`] fails for `title`
    #[inline]
    pub fn init(width: u32, height: u32, title: impl IntoCStr) -> Option<Self> {
        // SAFETY:
        // - `is_window_ready` returns a static bool that is initialized with false
        // - TODO: Is this thread-safe??
        if !unsafe { low::is_window_ready() } {
            let title = title.into_cstr().unwrap();
            // SAFETY:
            // - Window must be uninitialized to reach this branch
            unsafe {
                low::init_window(width, height, title.as_ref());
            }
            // SAFETY:
            // - `is_window_ready` returns a static bool that is initialized with false
            // - TODO: Is this thread-safe??
            if unsafe { low::is_window_ready() } {
                return Some(Self(WindowInner(())));
            }
        }
        None
    }

    /// Setup canvas (framebuffer) to start drawing
    #[inline]
    pub fn draw<'w>(&'w mut self, f: impl for<'d> FnOnce(&'w mut WindowInner, &'d mut Drawing, &'d mut BaseDrawMode)) {
        // SAFETY:
        // - Existence of `Window` proves window is initialized
        // - `Window` is borrowed exclusively, and `WindowInner` (which cannot create draw handles) is passed to `f` instead
        unsafe {
            low::begin_drawing();
        }
        f(&mut self.0, &mut Drawing(()), &mut BaseDrawMode(()));
    }

    /// Begin drawing to render texture
    #[inline]
    pub fn texture_mode<'w>(&'w mut self, target: &mut sys::RenderTexture2D, f: impl for<'d> FnOnce(&'w mut WindowInner, &'d mut TextureMode, &'d mut BaseDrawMode)) {
        // SAFETY:
        // - Existence of `Window` proves window is initialized
        // - `Window` is borrowed exclusively, and `WindowInner` (which cannot create draw handles) is passed to `f` instead
        // - `target` is borrowed exclusively and not passed to `f`
        unsafe {
            low::begin_texture_mode(*target);
        }
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
        // SAFETY:
        // - `begin_drawing` must have been called for `Drawing` to be constructed
        // - `Drawing` does not implement `Clone`
        unsafe {
            low::end_drawing();
        }
    }
}

/// A handle for draw functions available in [`Window::texture_mode`]
///
/// Ends texture mode when dropped
pub struct TextureMode(());

impl Drop for TextureMode {
    /// Ends drawing to render texture
    #[inline]
    fn drop(&mut self) {
        // SAFETY:
        // - `begin_texture_mode` must have been called for `TextureMode` to be constructed
        // - `TextureMode` does not implement `Clone`
        unsafe {
            low::end_texture_mode();
        }
    }
}

mod private {
    use super::*;

    pub trait SealedDraw {}
    impl SealedDraw for Drawing {}
    impl SealedDraw for TextureMode {}

    pub trait SealedDrawMode {}
    impl SealedDrawMode for BaseDrawMode {}
    impl<T: ?Sized + DrawMode> SealedDrawMode for &mut T {}
    impl<T: ?Sized + DrawMode> SealedDrawMode for DrawModeEnum<'_, T> {}
    impl<T: ?Sized + DrawMode> SealedDrawMode for ScissorMode<'_, T> {}
}

/// Raylib drawing functions
pub trait Draw: private::SealedDraw {
    /// Set background color (framebuffer clear color)
    #[inline]
    fn clear_background(&mut self, color: Color) {
        // SAFETY: `Draw` has sealed implementation restricted to draw handles
        unsafe {
            low::clear_background(color.into());
        }
    }

    /// Draw text (using default font)
    #[inline]
    fn draw_text(&mut self, text: impl IntoCStr, pos_x: i32, pos_y: i32, font_size: u32, color: Color) {
        let text = text.into_cstr().unwrap();
        // SAFETY: `Draw` has sealed implementation restricted to draw handles
        unsafe {
            low::draw_text(text.as_ref(), pos_x, pos_y, font_size, color.into());
        }
    }

    /// Draw a color-filled rectangle
    #[inline]
    fn draw_rectangle(&mut self, pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
        // SAFETY: `Draw` has sealed implementation restricted to draw handles
        unsafe {
            low::draw_rectangle(pos_x, pos_y, width, height, color.into());
        }
    }
}

impl Draw for Drawing {}
impl Draw for TextureMode {}

/// Union of all Raylib draw modes
///
/// Implements [`DrawMode`] and ends the stored draw mode when dropped, except in the case of [`Self::Ignore`]
pub enum DrawModeEnum<'a, M: ?Sized + DrawMode + 'a> {
    /// Passthrough to outer draw mode
    ///
    /// Does not end outer draw mode when dropped
    Ignore(&'a mut M),

    /// See [`ScissorMode`]
    Scissor(ScissorMode<'a, M>),
}

/// Conversion to [`DrawModeEnum`], shared by both [`DrawMode`] implementors and their references
pub trait IntoDrawModeEnum<'a, M: ?Sized + DrawMode + 'a>: private::SealedDrawMode {
    /// Convert `self` into a [`DrawModeEnum`]
    ///
    /// Does not change the mode if `self` is a reference instead of a value
    fn into_enum(self) -> DrawModeEnum<'a, M>;
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
impl<'a, M: ?Sized + DrawMode + 'a> IntoDrawModeEnum<'a, M> for &'a mut M {
    #[inline]
    fn into_enum(self) -> DrawModeEnum<'a, M> {
        DrawModeEnum::Ignore(self)
    }
}

/// Change the draw mode
///
/// This trait is intentionally separate from the [`Drawing`] handle.
/// It is, on its own, zero-cost if the draw mode stack is statically known.
///
/// Conditional draw modes are possible by using [`DrawModeEnum`].
pub trait DrawMode: private::SealedDrawMode {}

impl<'a, M: ?Sized + DrawMode + 'a> DrawMode for DrawModeEnum<'a, M> {}

/// The default draw mode -- created alongside [`Drawing`] or [`TextureMode`]
pub struct BaseDrawMode(());

impl DrawMode for BaseDrawMode {}

/// Raylib scisor mode
pub struct ScissorMode<'a, M: ?Sized + DrawMode>(PhantomData<&'a mut M>);

impl<M: ?Sized + DrawMode> Drop for ScissorMode<'_, M> {
    /// End scissor mode
    #[inline]
    fn drop(&mut self) {
        // SAFETY:
        // - `begin_scissor_mode` must have been called to construct `ScissorMode`
        // - `ScissorMode` contains an inaccessible, exclusive reference to the
        //   outer draw mode, requiring it to outlive this one
        unsafe {
            low::end_scissor_mode();
        }
    }
}

impl<'a, M: ?Sized + DrawMode> ScissorMode<'a, M> {
    /// Begin scissor mode (define screen area for following drawing)
    #[inline]
    pub fn begin(_outer_mode: &'a mut M, x: i32, y: i32, width: i32, height: i32) -> ScissorMode<'a, M> {
        // SAFETY:
        // - `_outer_mode` must implement `DrawMode`
        //   - `DrawMode` has sealed implementation restricted to types originating from a draw handle
        unsafe {
            low::begin_scissor_mode(x, y, width, height);
        }
        Self(PhantomData)
    }
}

impl<'a, M: ?Sized + DrawMode> IntoDrawModeEnum<'a, M> for ScissorMode<'a, M> {
    #[inline]
    fn into_enum(self) -> DrawModeEnum<'a, M> {
        DrawModeEnum::Scissor(self)
    }
}

impl<'a, M: ?Sized + DrawMode> DrawMode for ScissorMode<'a, M> {}
