#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TryFromEnumError(());

impl std::fmt::Display for TryFromEnumError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("invalid enum discriminant")
    }
}

impl std::error::Error for TryFromEnumError {}

macro_rules! define_rl_enum {
    (
        $(#[$m:meta])*
        $vis:vis struct $Struct:ident: $ReprTy:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                const $FLAG:ident as $ALIAS:ident = $value:expr;
            )*
        }
    ) => {
        ::bitflags::bitflags!{
            $(#[$m])*
            $vis struct $Struct: $ReprTy {
                $(
                    $(#[$inner $($args)*])*
                    const $FLAG = $value;
                )*
            }
        }

        $($vis const $ALIAS: $Struct = $Struct::$FLAG;)*
    };

    (
        $(#[$m:meta])*
        $vis:vis mod $Mod:ident: $ReprTy:ty {
            $(
                $(#[$vm:meta])*
                const $Constant:ident as $ALIAS:ident = $value:expr;
            )*
        }
    ) => {
        $(#[$m])*
        #[allow(non_snake_case)]
        $vis mod $Mod {
            $(pub const $Constant: $ReprTy = $value;)*
        }
        $vis use $Mod::{
            $($Constant as $ALIAS,)*
        };
    };

    (
        $(#[$m:meta])*
        $vis:vis enum $Enum:ident: $ReprTy:ty {
            $(
                $(#[$vm:meta])*
                $Variant:ident as $ALIAS:ident = $value:expr
            ),* $(,)?
        }
    ) => {
        #[repr($ReprTy)]
        $(#[$m])*
        $vis enum $Enum {
            $(
                $(#[$vm])*
                $Variant = $value,
            )*
        }

        $vis use $Enum::{
            $($Variant as $ALIAS,)*
        };

        impl $Enum {
            /// Cast enum from int
            /// # Panics
            /// Will panic if `value` is not a valid variant
            #[inline]
            pub const fn from_int(value: $ReprTy) -> Self {
                Self::try_from_int(value)
                    .expect(concat!("value should be a valid ", stringify!($Enum), " variant"))
            }

            /// Cast enum from int, returning [`None`] if `value` is not a valid variant
            #[inline]
            pub const fn try_from_int(value: $ReprTy) -> Option<Self> {
                #![allow(non_upper_case_globals)]
                $(
                    const $ALIAS: $ReprTy = $Enum::$Variant as $ReprTy;
                )*
                match value {
                    $($ALIAS)|* => Some(
                        // SAFETY: Just confirmed `value` is in range
                        unsafe { Self::from_int_unchecked(value) }
                    ),
                    _ => None,
                }
            }

            /// Cast enum from int without checking if the value is in range
            /// # Safety
            /// `value` must be a valid variant
            #[inline]
            pub const unsafe fn from_int_unchecked(value: $ReprTy) -> Self {
                // SAFETY: Caller must uphold safety contract
                unsafe { std::mem::transmute::<$ReprTy, $Enum>(value) }
            }
        }

        impl TryFrom<$ReprTy> for $Enum {
            type Error = TryFromEnumError;

            #[inline]
            fn try_from(value: $ReprTy) -> Result<Self, TryFromEnumError> {
                Self::try_from_int(value)
                    .ok_or(TryFromEnumError(()))
            }
        }

        impl From<$Enum> for $ReprTy {
            #[inline]
            fn from(value: $Enum) -> $ReprTy {
                value as $ReprTy
            }
        }
    };
}

define_rl_enum! {
/// System/Window config flags
///
/// NOTE: Every bit registers one state (use it with bit masks)
/// By default all flags are set to 0
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConfigFlags: i32 {
    /// Set to try enabling V-Sync on GPU
    const VSYNC_HINT as FLAG_VSYNC_HINT = 64;
    /// Set to run program in fullscreen
    const FULLSCREEN_MODE as FLAG_FULLSCREEN_MODE = 2;
    /// Set to allow resizable window
    const WINDOW_RESIZABLE as FLAG_WINDOW_RESIZABLE = 4;
    /// Set to disable window decoration (frame and buttons)
    const WINDOW_UNDECORATED as FLAG_WINDOW_UNDECORATED = 8;
    /// Set to hide window
    const WINDOW_HIDDEN as FLAG_WINDOW_HIDDEN = 128;
    /// Set to minimize window (iconify)
    const WINDOW_MINIMIZED as FLAG_WINDOW_MINIMIZED = 512;
    /// Set to maximize window (expanded to monitor)
    const WINDOW_MAXIMIZED as FLAG_WINDOW_MAXIMIZED = 1024;
    /// Set to window non focused
    const WINDOW_UNFOCUSED as FLAG_WINDOW_UNFOCUSED = 2048;
    /// Set to window always on top
    const WINDOW_TOPMOST as FLAG_WINDOW_TOPMOST = 4096;
    /// Set to allow windows running while minimized
    const WINDOW_ALWAYS_RUN as FLAG_WINDOW_ALWAYS_RUN = 256;
    /// Set to allow transparent framebuffer
    const WINDOW_TRANSPARENT as FLAG_WINDOW_TRANSPARENT = 16;
    /// Set to support HighDPI
    const WINDOW_HIGHDPI as FLAG_WINDOW_HIGHDPI = 8192;
    /// Set to support mouse passthrough, only supported when FLAG_WINDOW_UNDECORATED
    const WINDOW_MOUSE_PASSTHROUGH as FLAG_WINDOW_MOUSE_PASSTHROUGH = 16384;
    /// Set to run program in borderless windowed mode
    const BORDERLESS_WINDOWED_MODE as FLAG_BORDERLESS_WINDOWED_MODE = 32768;
    /// Set to try enabling MSAA 4X
    const MSAA_4X_HINT as FLAG_MSAA_4X_HINT = 32;
    /// Set to try enabling interlaced video format (for V3D)
    const INTERLACED_HINT as FLAG_INTERLACED_HINT = 65536;
}
}

define_rl_enum!{
/// Trace log level
/// NOTE: Organized by priority level
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TraceLogLevel: i32 {
    /// Display all logs
    All as LOG_ALL = 0,
    /// Trace logging, intended for internal use only
    Trace as LOG_TRACE = 1,
    /// Debug logging, used for internal debugging, it should be disabled on release builds
    Debug as LOG_DEBUG = 2,
    /// Info logging, used for program execution info
    Info as LOG_INFO = 3,
    /// Warning logging, used on recoverable failures
    Warning as LOG_WARNING = 4,
    /// Error logging, used on unrecoverable failures
    Error as LOG_ERROR = 5,
    /// Fatal logging, used to abort program: exit(EXIT_FAILURE)
    Fatal as LOG_FATAL = 6,
    /// Disable logging
    None as LOG_NONE = 7,
}
}

define_rl_enum!{
/// Keyboard keys (US keyboard layout)
/// NOTE: Use GetKeyPressed() to allow redefining
/// required keys for alternative layouts
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum KeyboardKey: i32 {
    #[default]
    Null as KEY_NULL = 0,
    // Alphanumeric keys
    /// Key: '
    Apostrophe as KEY_APOSTROPHE = 39,
    /// Key: ,
    Comma as KEY_COMMA = 44,
    /// Key: -
    Minus as KEY_MINUS = 45,
    /// Key: .
    Period as KEY_PERIOD = 46,
    /// Key: /
    Slash as KEY_SLASH = 47,
    /// Key: 0
    Zero as KEY_ZERO = 48,
    /// Key: 1
    One as KEY_ONE = 49,
    /// Key: 2
    Two as KEY_TWO = 50,
    /// Key: 3
    Three as KEY_THREE = 51,
    /// Key: 4
    Four as KEY_FOUR = 52,
    /// Key: 5
    Five as KEY_FIVE = 53,
    /// Key: 6
    Six as KEY_SIX = 54,
    /// Key: 7
    Seven as KEY_SEVEN = 55,
    /// Key: 8
    Eight as KEY_EIGHT = 56,
    /// Key: 9
    Nine as KEY_NINE = 57,
    /// Key: ;
    Semicolon as KEY_SEMICOLON = 59,
    /// Key: =
    Equal as KEY_EQUAL = 61,
    /// Key: A | a
    A as KEY_A = 65,
    /// Key: B | b
    B as KEY_B = 66,
    /// Key: C | c
    C as KEY_C = 67,
    /// Key: D | d
    D as KEY_D = 68,
    /// Key: E | e
    E as KEY_E = 69,
    /// Key: F | f
    F as KEY_F = 70,
    /// Key: G | g
    G as KEY_G = 71,
    /// Key: H | h
    H as KEY_H = 72,
    /// Key: I | i
    I as KEY_I = 73,
    /// Key: J | j
    J as KEY_J = 74,
    /// Key: K | k
    K as KEY_K = 75,
    /// Key: L | l
    L as KEY_L = 76,
    /// Key: M | m
    M as KEY_M = 77,
    /// Key: N | n
    N as KEY_N = 78,
    /// Key: O | o
    O as KEY_O = 79,
    /// Key: P | p
    P as KEY_P = 80,
    /// Key: Q | q
    Q as KEY_Q = 81,
    /// Key: R | r
    R as KEY_R = 82,
    /// Key: S | s
    S as KEY_S = 83,
    /// Key: T | t
    T as KEY_T = 84,
    /// Key: U | u
    U as KEY_U = 85,
    /// Key: V | v
    V as KEY_V = 86,
    /// Key: W | w
    W as KEY_W = 87,
    /// Key: X | x
    X as KEY_X = 88,
    /// Key: Y | y
    Y as KEY_Y = 89,
    /// Key: Z | z
    Z as KEY_Z = 90,
    /// Key: [
    LeftBracket as KEY_LEFT_BRACKET = 91,
    /// Key: '\'
    Backslash as KEY_BACKSLASH = 92,
    /// Key: ]
    RightBracket as KEY_RIGHT_BRACKET = 93,
    /// Key: `
    Grave as KEY_GRAVE = 96,
    // Function keys
    /// Key: Space
    Space as KEY_SPACE = 32,
    /// Key: Esc
    Escape as KEY_ESCAPE = 256,
    /// Key: Enter
    Enter as KEY_ENTER = 257,
    /// Key: Tab
    Tab as KEY_TAB = 258,
    /// Key: Backspace
    Backspace as KEY_BACKSPACE = 259,
    /// Key: Ins
    Insert as KEY_INSERT = 260,
    /// Key: Del
    Delete as KEY_DELETE = 261,
    /// Key: Cursor right
    Right as KEY_RIGHT = 262,
    /// Key: Cursor left
    Left as KEY_LEFT = 263,
    /// Key: Cursor down
    Down as KEY_DOWN = 264,
    /// Key: Cursor up
    Up as KEY_UP = 265,
    /// Key: Page up
    PageUp as KEY_PAGE_UP = 266,
    /// Key: Page down
    PageDown as KEY_PAGE_DOWN = 267,
    /// Key: Home
    Home as KEY_HOME = 268,
    /// Key: End
    End as KEY_END = 269,
    /// Key: Caps lock
    CapsLock as KEY_CAPS_LOCK = 280,
    /// Key: Scroll down
    ScrollLock as KEY_SCROLL_LOCK = 281,
    /// Key: Num lock
    NumLock as KEY_NUM_LOCK = 282,
    /// Key: Print screen
    PrintScreen as KEY_PRINT_SCREEN = 283,
    /// Key: Pause
    Pause as KEY_PAUSE = 284,
    /// Key: F1
    F1 as KEY_F1 = 290,
    /// Key: F2
    F2 as KEY_F2 = 291,
    /// Key: F3
    F3 as KEY_F3 = 292,
    /// Key: F4
    F4 as KEY_F4 = 293,
    /// Key: F5
    F5 as KEY_F5 = 294,
    /// Key: F6
    F6 as KEY_F6 = 295,
    /// Key: F7
    F7 as KEY_F7 = 296,
    /// Key: F8
    F8 as KEY_F8 = 297,
    /// Key: F9
    F9 as KEY_F9 = 298,
    /// Key: F10
    F10 as KEY_F10 = 299,
    /// Key: F11
    F11 as KEY_F11 = 300,
    /// Key: F12
    F12 as KEY_F12 = 301,
    /// Key: Shift left
    LeftShift as KEY_LEFT_SHIFT = 340,
    /// Key: Control left
    LeftControl as KEY_LEFT_CONTROL = 341,
    /// Key: Alt left
    LeftAlt as KEY_LEFT_ALT = 342,
    /// Key: Super left
    LeftSuper as KEY_LEFT_SUPER = 343,
    /// Key: Shift right
    RightShift as KEY_RIGHT_SHIFT = 344,
    /// Key: Control right
    RightControl as KEY_RIGHT_CONTROL = 345,
    /// Key: Alt right
    RightAlt as KEY_RIGHT_ALT = 346,
    /// Key: Super right
    RightSuper as KEY_RIGHT_SUPER = 347,
    /// Key: KB menu
    KbMenu as KEY_KB_MENU = 348,
    // Keypad keys
    /// Key: Keypad 0
    Kp0 as KEY_KP_0 = 320,
    /// Key: Keypad 1
    Kp1 as KEY_KP_1 = 321,
    /// Key: Keypad 2
    Kp2 as KEY_KP_2 = 322,
    /// Key: Keypad 3
    Kp3 as KEY_KP_3 = 323,
    /// Key: Keypad 4
    Kp4 as KEY_KP_4 = 324,
    /// Key: Keypad 5
    Kp5 as KEY_KP_5 = 325,
    /// Key: Keypad 6
    Kp6 as KEY_KP_6 = 326,
    /// Key: Keypad 7
    Kp7 as KEY_KP_7 = 327,
    /// Key: Keypad 8
    Kp8 as KEY_KP_8 = 328,
    /// Key: Keypad 9
    Kp9 as KEY_KP_9 = 329,
    /// Key: Keypad .
    KpDecimal as KEY_KP_DECIMAL = 330,
    /// Key: Keypad /
    KpDivide as KEY_KP_DIVIDE = 331,
    /// Key: Keypad *
    KpMultiply as KEY_KP_MULTIPLY = 332,
    /// Key: Keypad -
    KpSubtract as KEY_KP_SUBTRACT = 333,
    /// Key: Keypad +
    KpAdd as KEY_KP_ADD = 334,
    /// Key: Keypad Enter
    KpEnter as KEY_KP_ENTER = 335,
    /// Key: Keypad =
    KpEqual as KEY_KP_EQUAL = 336,
    // Android key buttons
    /// Key: Android back button// Key: Android back button
    Back as KEY_BACK = 4,
    /// Key: Android menu button// Key: Android menu button
    Menu as KEY_MENU = 5,
    /// Key: Android volume up button// Key: Android volume up button
    VolumeUp as KEY_VOLUME_UP = 24,
    /// Key: Android volume down button// Key: Android volume down button
    VolumeDown as KEY_VOLUME_DOWN = 25,
}
}

define_rl_enum!{
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MouseButton: i32 {
    Left as MOUSE_BUTTON_LEFT = 0,
    Right as MOUSE_BUTTON_RIGHT = 1,
    Middle as MOUSE_BUTTON_MIDDLE = 2,
    Side as MOUSE_BUTTON_SIDE = 3,
    Extra as MOUSE_BUTTON_EXTRA = 4,
    Forward as MOUSE_BUTTON_FORWARD = 5,
    Back as MOUSE_BUTTON_BACK = 6,
}
}

define_rl_enum!{
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum MouseCursor: i32 {
    #[default]
    Default as MOUSE_CURSOR_DEFAULT = 0,
    Arrow as MOUSE_CURSOR_ARROW = 1,
    IBeam as MOUSE_CURSOR_IBEAM = 2,
    Crosshair as MOUSE_CURSOR_CROSSHAIR = 3,
    PointingHand as MOUSE_CURSOR_POINTING_HAND = 4,
    ResizeEW as MOUSE_CURSOR_RESIZE_EW = 5,
    ResizeNS as MOUSE_CURSOR_RESIZE_NS = 6,
    ResizeNWSE as MOUSE_CURSOR_RESIZE_NWSE = 7,
    ResizeNESW as MOUSE_CURSOR_RESIZE_NESW = 8,
    ResizeAll as MOUSE_CURSOR_RESIZE_ALL = 9,
    NotAllowed as MOUSE_CURSOR_NOT_ALLOWED = 10,
}
}

define_rl_enum!{
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum GamepadButton: i32 {
    #[default]
    Unknown as GAMEPAD_BUTTON_UNKNOWN = 0,
    LeftFaceUp as GAMEPAD_BUTTON_LEFT_FACE_UP = 1,
    LeftFaceRight as GAMEPAD_BUTTON_LEFT_FACE_RIGHT = 2,
    LeftFaceDown as GAMEPAD_BUTTON_LEFT_FACE_DOWN = 3,
    LeftFaceLeft as GAMEPAD_BUTTON_LEFT_FACE_LEFT = 4,
    RightFaceUp as GAMEPAD_BUTTON_RIGHT_FACE_UP = 5,
    RightFaceRight as GAMEPAD_BUTTON_RIGHT_FACE_RIGHT = 6,
    RightFaceDown as GAMEPAD_BUTTON_RIGHT_FACE_DOWN = 7,
    RightFaceLeft as GAMEPAD_BUTTON_RIGHT_FACE_LEFT = 8,
    LeftTrigger1 as GAMEPAD_BUTTON_LEFT_TRIGGER_1 = 9,
    LeftTrigger2 as GAMEPAD_BUTTON_LEFT_TRIGGER_2 = 10,
    RightTrigger1 as GAMEPAD_BUTTON_RIGHT_TRIGGER_1 = 11,
    RightTrigger2 as GAMEPAD_BUTTON_RIGHT_TRIGGER_2 = 12,
    MiddleLeft as GAMEPAD_BUTTON_MIDDLE_LEFT = 13,
    Middle as GAMEPAD_BUTTON_MIDDLE = 14,
    MiddleRight as GAMEPAD_BUTTON_MIDDLE_RIGHT = 15,
    LeftThumb as GAMEPAD_BUTTON_LEFT_THUMB = 16,
    RightThumb as GAMEPAD_BUTTON_RIGHT_THUMB = 17,
}
}

define_rl_enum!{
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GamepadAxis: i32 {
    LeftX as GAMEPAD_AXIS_LEFT_X = 0,
    LeftY as GAMEPAD_AXIS_LEFT_Y = 1,
    RightX as GAMEPAD_AXIS_RIGHT_X = 2,
    RightY as GAMEPAD_AXIS_RIGHT_Y = 3,
    LeftTrigger as GAMEPAD_AXIS_LEFT_TRIGGER = 4,
    RightTrigger as GAMEPAD_AXIS_RIGHT_TRIGGER = 5,
}
}

define_rl_enum!{
pub mod MaterialMapIndex: u32 {
    const ALBEDO as MATERIAL_MAP_ALBEDO = 0;
    const METALNESS as MATERIAL_MAP_METALNESS = 1;
    const NORMAL as MATERIAL_MAP_NORMAL = 2;
    const ROUGHNESS as MATERIAL_MAP_ROUGHNESS = 3;
    const OCCLUSION as MATERIAL_MAP_OCCLUSION = 4;
    const EMISSION as MATERIAL_MAP_EMISSION = 5;
    const HEIGHT as MATERIAL_MAP_HEIGHT = 6;
    const CUBEMAP as MATERIAL_MAP_CUBEMAP = 7;
    const IRRADIANCE as MATERIAL_MAP_IRRADIANCE = 8;
    const PREFILTER as MATERIAL_MAP_PREFILTER = 9;
    const BRDF as MATERIAL_MAP_BRDF = 10;
}
}

define_rl_enum!{
pub mod ShaderLocationIndex: u32 {
    const VERTEX_POSITION as SHADER_LOC_VERTEX_POSITION = 0;
    const VERTEX_TEXCOORD01 as SHADER_LOC_VERTEX_TEXCOORD01 = 1;
    const VERTEX_TEXCOORD02 as SHADER_LOC_VERTEX_TEXCOORD02 = 2;
    const VERTEX_NORMAL as SHADER_LOC_VERTEX_NORMAL = 3;
    const VERTEX_TANGENT as SHADER_LOC_VERTEX_TANGENT = 4;
    const VERTEX_COLOR as SHADER_LOC_VERTEX_COLOR = 5;
    const MATRIX_MVP as SHADER_LOC_MATRIX_MVP = 6;
    const MATRIX_VIEW as SHADER_LOC_MATRIX_VIEW = 7;
    const MATRIX_PROJECTION as SHADER_LOC_MATRIX_PROJECTION = 8;
    const MATRIX_MODEL as SHADER_LOC_MATRIX_MODEL = 9;
    const MATRIX_NORMAL as SHADER_LOC_MATRIX_NORMAL = 10;
    const VECTOR_VIEW as SHADER_LOC_VECTOR_VIEW = 11;
    const COLOR_DIFFUSE as SHADER_LOC_COLOR_DIFFUSE = 12;
    const COLOR_SPECULAR as SHADER_LOC_COLOR_SPECULAR = 13;
    const COLOR_AMBIENT as SHADER_LOC_COLOR_AMBIENT = 14;
    const MAP_ALBEDO as SHADER_LOC_MAP_ALBEDO = 15;
    const MAP_METALNESS as SHADER_LOC_MAP_METALNESS = 16;
    const MAP_NORMAL as SHADER_LOC_MAP_NORMAL = 17;
    const MAP_ROUGHNESS as SHADER_LOC_MAP_ROUGHNESS = 18;
    const MAP_OCCLUSION as SHADER_LOC_MAP_OCCLUSION = 19;
    const MAP_EMISSION as SHADER_LOC_MAP_EMISSION = 20;
    const MAP_HEIGHT as SHADER_LOC_MAP_HEIGHT = 21;
    const MAP_CUBEMAP as SHADER_LOC_MAP_CUBEMAP = 22;
    const MAP_IRRADIANCE as SHADER_LOC_MAP_IRRADIANCE = 23;
    const MAP_PREFILTER as SHADER_LOC_MAP_PREFILTER = 24;
    const MAP_BRDF as SHADER_LOC_MAP_BRDF = 25;
    const VERTEX_BONEIDS as SHADER_LOC_VERTEX_BONEIDS = 26;
    const VERTEX_BONEWEIGHTS as SHADER_LOC_VERTEX_BONEWEIGHTS = 27;
    const BONE_MATRICES as SHADER_LOC_BONE_MATRICES = 28;
    const VERTEX_INSTANCE_TX as SHADER_LOC_VERTEX_INSTANCE_TX = 29;
}
}

define_rl_enum!{
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShaderUniformDataType: i32 {
    Float as SHADER_UNIFORM_FLOAT = 0,
    Vec2 as SHADER_UNIFORM_VEC2 = 1,
    Vec3 as SHADER_UNIFORM_VEC3 = 2,
    Vec4 as SHADER_UNIFORM_VEC4 = 3,
    Int as SHADER_UNIFORM_INT = 4,
    IVec2 as SHADER_UNIFORM_IVEC2 = 5,
    IVec3 as SHADER_UNIFORM_IVEC3 = 6,
    IVec4 as SHADER_UNIFORM_IVEC4 = 7,
    UInt as SHADER_UNIFORM_UINT = 8,
    UIVec2 as SHADER_UNIFORM_UIVEC2 = 9,
    UIVec3 as SHADER_UNIFORM_UIVEC3 = 10,
    UIVec4 as SHADER_UNIFORM_UIVEC4 = 11,
    Sampler2D as SHADER_UNIFORM_SAMPLER2D = 12,
}
}

define_rl_enum!{
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShaderAttributeDataType: i32 {
    Float as SHADER_ATTRIB_FLOAT = 0,
    Vec2 as SHADER_ATTRIB_VEC2 = 1,
    Vec3 as SHADER_ATTRIB_VEC3 = 2,
    Vec4 as SHADER_ATTRIB_VEC4 = 3,
}
}

define_rl_enum!{
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PixelFormat: i32 {
    UncompressedGrayscale as PIXELFORMAT_UNCOMPRESSED_GRAYSCALE = 1,
    UncompressedGrayAlpha as PIXELFORMAT_UNCOMPRESSED_GRAY_ALPHA = 2,
    UncompressedR5G6B5 as PIXELFORMAT_UNCOMPRESSED_R5G6B5 = 3,
    UncompressedR8G8B8 as PIXELFORMAT_UNCOMPRESSED_R8G8B8 = 4,
    UncompressedR5G5B5A1 as PIXELFORMAT_UNCOMPRESSED_R5G5B5A1 = 5,
    UncompressedR4G4B4A4 as PIXELFORMAT_UNCOMPRESSED_R4G4B4A4 = 6,
    UncompressedR8G8B8A8 as PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 = 7,
    UncompressedR32 as PIXELFORMAT_UNCOMPRESSED_R32 = 8,
    UncompressedR32G32B32 as PIXELFORMAT_UNCOMPRESSED_R32G32B32 = 9,
    UncompressedR32G32B32A32 as PIXELFORMAT_UNCOMPRESSED_R32G32B32A32 = 10,
    UncompressedR16 as PIXELFORMAT_UNCOMPRESSED_R16 = 11,
    UncompressedR16G16B16 as PIXELFORMAT_UNCOMPRESSED_R16G16B16 = 12,
    UncompressedR16G16B16A16 as PIXELFORMAT_UNCOMPRESSED_R16G16B16A16 = 13,
    CompressedDxt1Rgb as PIXELFORMAT_COMPRESSED_DXT1_RGB = 14,
    CompressedDxt1Rgba as PIXELFORMAT_COMPRESSED_DXT1_RGBA = 15,
    CompressedDxt3Rgba as PIXELFORMAT_COMPRESSED_DXT3_RGBA = 16,
    CompressedDxt5Rgba as PIXELFORMAT_COMPRESSED_DXT5_RGBA = 17,
    CompressedEtc1Rgb as PIXELFORMAT_COMPRESSED_ETC1_RGB = 18,
    CompressedEtc2Rgb as PIXELFORMAT_COMPRESSED_ETC2_RGB = 19,
    CompressedEtc2EacRgba as PIXELFORMAT_COMPRESSED_ETC2_EAC_RGBA = 20,
    CompressedPvrtRgb as PIXELFORMAT_COMPRESSED_PVRT_RGB = 21,
    CompressedPvrtRgba as PIXELFORMAT_COMPRESSED_PVRT_RGBA = 22,
    CompressedAstc4x4Rgba as PIXELFORMAT_COMPRESSED_ASTC_4x4_RGBA = 23,
    CompressedAstc8x8Rgba as PIXELFORMAT_COMPRESSED_ASTC_8x8_RGBA = 24,
}
}

define_rl_enum!{
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TextureFilter: i32 {
    Point as TEXTURE_FILTER_POINT = 0,
    Bilinear as TEXTURE_FILTER_BILINEAR = 1,
    Trilinear as TEXTURE_FILTER_TRILINEAR = 2,
    Anisotropic4x as TEXTURE_FILTER_ANISOTROPIC_4X = 3,
    Anisotropic8x as TEXTURE_FILTER_ANISOTROPIC_8X = 4,
    Anisotropic16x as TEXTURE_FILTER_ANISOTROPIC_16X = 5,
}
}

define_rl_enum!{
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TextureWrap: i32 {
    Repeat as TEXTURE_WRAP_REPEAT = 0,
    Clamp as TEXTURE_WRAP_CLAMP = 1,
    MirrorRepeat as TEXTURE_WRAP_MIRROR_REPEAT = 2,
    MirrorClamp as TEXTURE_WRAP_MIRROR_CLAMP = 3,
}
}

define_rl_enum!{
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CubemapLayout: i32 {
    AutoDetect as CUBEMAP_LAYOUT_AUTO_DETECT = 0,
    LineVertical as CUBEMAP_LAYOUT_LINE_VERTICAL = 1,
    LineHorizontal as CUBEMAP_LAYOUT_LINE_HORIZONTAL = 2,
    CrossThreeByFour as CUBEMAP_LAYOUT_CROSS_THREE_BY_FOUR = 3,
    CrossFourByThree as CUBEMAP_LAYOUT_CROSS_FOUR_BY_THREE = 4,
}
}

define_rl_enum!{
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum FontType: i32 {
    #[default]
    Default as FONT_DEFAULT = 0,
    Bitmap as FONT_BITMAP = 1,
    Sdf as FONT_SDF = 2,
}
}

define_rl_enum!{
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlendMode: i32 {
    Alpha as BLEND_ALPHA = 0,
    Additive as BLEND_ADDITIVE = 1,
    Multiplied as BLEND_MULTIPLIED = 2,
    AddColors as BLEND_ADD_COLORS = 3,
    SubtractColors as BLEND_SUBTRACT_COLORS = 4,
    AlphaPremultiply as BLEND_ALPHA_PREMULTIPLY = 5,
    Custom as BLEND_CUSTOM = 6,
    CustomSeparate as BLEND_CUSTOM_SEPARATE = 7,
}
}

define_rl_enum!{
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Gesture: i32 {
    const None as GESTURE_NONE = 0;
    const Tap as GESTURE_TAP = 1;
    const Doubletap as GESTURE_DOUBLETAP = 2;
    const Hold as GESTURE_HOLD = 4;
    const Drag as GESTURE_DRAG = 8;
    const SwipeRight as GESTURE_SWIPE_RIGHT = 16;
    const SwipeLeft as GESTURE_SWIPE_LEFT = 32;
    const SwipeUp as GESTURE_SWIPE_UP = 64;
    const SwipeDown as GESTURE_SWIPE_DOWN = 128;
    const PinchIn as GESTURE_PINCH_IN = 256;
    const PinchOut as GESTURE_PINCH_OUT = 512;
}
}

define_rl_enum!{
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CameraMode: i32 {
    Custom as CAMERA_CUSTOM = 0,
    Free as CAMERA_FREE = 1,
    Orbital as CAMERA_ORBITAL = 2,
    FirstPerson as CAMERA_FIRST_PERSON = 3,
    ThirdPerson as CAMERA_THIRD_PERSON = 4,
}
}

define_rl_enum!{
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CameraProjection: i32 {
    Perspective as CAMERA_PERSPECTIVE = 0,
    Orthographic as CAMERA_ORTHOGRAPHIC = 1,
}
}

define_rl_enum!{
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NPatchLayout: i32 {
    NinePatch as NPATCH_NINE_PATCH = 0,
    ThreePatchVertical as NPATCH_THREE_PATCH_VERTICAL = 1,
    ThreePatchHorizontal as NPATCH_THREE_PATCH_HORIZONTAL = 2,
}
}

define_rl_enum!{
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TouchAction: i32 {
    Up as TOUCH_ACTION_UP = 0,
    Down as TOUCH_ACTION_DOWN = 1,
    Move as TOUCH_ACTION_MOVE = 2,
    Cancel as TOUCH_ACTION_CANCEL = 3,
}
}

define_rl_enum!{
#[allow(non_camel_case_types, reason = "consistency with RLGL naming convention")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum rlGlVersion: i32 {
    Opengl11 as RL_OPENGL_11 = 1,
    Opengl21 as RL_OPENGL_21 = 2,
    Opengl33 as RL_OPENGL_33 = 3,
    Opengl43 as RL_OPENGL_43 = 4,
    OpenglEs20 as RL_OPENGL_ES_20 = 5,
    OpenglEs30 as RL_OPENGL_ES_30 = 6,
}
}

define_rl_enum!{
#[allow(non_camel_case_types, reason = "consistency with RLGL naming convention")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum rlTraceLogLevel: i32 {
    All as RL_LOG_ALL = 0,
    Trace as RL_LOG_TRACE = 1,
    Debug as RL_LOG_DEBUG = 2,
    Info as RL_LOG_INFO = 3,
    Warning as RL_LOG_WARNING = 4,
    Error as RL_LOG_ERROR = 5,
    Fatal as RL_LOG_FATAL = 6,
    None as RL_LOG_NONE = 7,
}
}

define_rl_enum!{
#[allow(non_camel_case_types, reason = "consistency with RLGL naming convention")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum rlPixelFormat: i32 {
    UncompressedGrayscale as RL_PIXELFORMAT_UNCOMPRESSED_GRAYSCALE = 1,
    UncompressedGrayAlpha as RL_PIXELFORMAT_UNCOMPRESSED_GRAY_ALPHA = 2,
    UncompressedR5G6B5 as RL_PIXELFORMAT_UNCOMPRESSED_R5G6B5 = 3,
    UncompressedR8G8B8 as RL_PIXELFORMAT_UNCOMPRESSED_R8G8B8 = 4,
    UncompressedR5G5B5A1 as RL_PIXELFORMAT_UNCOMPRESSED_R5G5B5A1 = 5,
    UncompressedR4G4B4A4 as RL_PIXELFORMAT_UNCOMPRESSED_R4G4B4A4 = 6,
    UncompressedR8G8B8A8 as RL_PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 = 7,
    UncompressedR32 as RL_PIXELFORMAT_UNCOMPRESSED_R32 = 8,
    UncompressedR32G32B32 as RL_PIXELFORMAT_UNCOMPRESSED_R32G32B32 = 9,
    UncompressedR32G32B32A32 as RL_PIXELFORMAT_UNCOMPRESSED_R32G32B32A32 = 10,
    UncompressedR16 as RL_PIXELFORMAT_UNCOMPRESSED_R16 = 11,
    UncompressedR16G16B16 as RL_PIXELFORMAT_UNCOMPRESSED_R16G16B16 = 12,
    UncompressedR16G16B16A16 as RL_PIXELFORMAT_UNCOMPRESSED_R16G16B16A16 = 13,
    CompressedDxt1Rgb as RL_PIXELFORMAT_COMPRESSED_DXT1_RGB = 14,
    CompressedDxt1Rgba as RL_PIXELFORMAT_COMPRESSED_DXT1_RGBA = 15,
    CompressedDxt3Rgba as RL_PIXELFORMAT_COMPRESSED_DXT3_RGBA = 16,
    CompressedDxt5Rgba as RL_PIXELFORMAT_COMPRESSED_DXT5_RGBA = 17,
    CompressedEtc1Rgb as RL_PIXELFORMAT_COMPRESSED_ETC1_RGB = 18,
    CompressedEtc2Rgb as RL_PIXELFORMAT_COMPRESSED_ETC2_RGB = 19,
    CompressedEtc2EacRgba as RL_PIXELFORMAT_COMPRESSED_ETC2_EAC_RGBA = 20,
    CompressedPvrtRgb as RL_PIXELFORMAT_COMPRESSED_PVRT_RGB = 21,
    CompressedPvrtRgba as RL_PIXELFORMAT_COMPRESSED_PVRT_RGBA = 22,
    CompressedAstc4x4Rgba as RL_PIXELFORMAT_COMPRESSED_ASTC_4x4_RGBA = 23,
    CompressedAstc8x8Rgba as RL_PIXELFORMAT_COMPRESSED_ASTC_8x8_RGBA = 24,
}
}

define_rl_enum!{
#[allow(non_camel_case_types, reason = "consistency with RLGL naming convention")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum rlTextureFilter: i32 {
    Point as RL_TEXTURE_FILTER_POINT = 0,
    Bilinear as RL_TEXTURE_FILTER_BILINEAR = 1,
    Trilinear as RL_TEXTURE_FILTER_TRILINEAR = 2,
    Anisotropic4x as RL_TEXTURE_FILTER_ANISOTROPIC_4X = 3,
    Anisotropic8x as RL_TEXTURE_FILTER_ANISOTROPIC_8X = 4,
    Anisotropic16x as RL_TEXTURE_FILTER_ANISOTROPIC_16X = 5,
}
}

define_rl_enum!{
#[allow(non_camel_case_types, reason = "consistency with RLGL naming convention")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum rlBlendMode: i32 {
    Alpha as RL_BLEND_ALPHA = 0,
    Additive as RL_BLEND_ADDITIVE = 1,
    Multiplied as RL_BLEND_MULTIPLIED = 2,
    AddColors as RL_BLEND_ADD_COLORS = 3,
    SubtractColors as RL_BLEND_SUBTRACT_COLORS = 4,
    AlphaPremultiply as RL_BLEND_ALPHA_PREMULTIPLY = 5,
    Custom as RL_BLEND_CUSTOM = 6,
    CustomSeparate as RL_BLEND_CUSTOM_SEPARATE = 7,
}
}

define_rl_enum!{
pub mod rlShaderLocationIndex: u32 {
    const VERTEX_POSITION as RL_SHADER_LOC_VERTEX_POSITION = 0;
    const VERTEX_TEXCOORD01 as RL_SHADER_LOC_VERTEX_TEXCOORD01 = 1;
    const VERTEX_TEXCOORD02 as RL_SHADER_LOC_VERTEX_TEXCOORD02 = 2;
    const VERTEX_NORMAL as RL_SHADER_LOC_VERTEX_NORMAL = 3;
    const VERTEX_TANGENT as RL_SHADER_LOC_VERTEX_TANGENT = 4;
    const VERTEX_COLOR as RL_SHADER_LOC_VERTEX_COLOR = 5;
    const MATRIX_MVP as RL_SHADER_LOC_MATRIX_MVP = 6;
    const MATRIX_VIEW as RL_SHADER_LOC_MATRIX_VIEW = 7;
    const MATRIX_PROJECTION as RL_SHADER_LOC_MATRIX_PROJECTION = 8;
    const MATRIX_MODEL as RL_SHADER_LOC_MATRIX_MODEL = 9;
    const MATRIX_NORMAL as RL_SHADER_LOC_MATRIX_NORMAL = 10;
    const VECTOR_VIEW as RL_SHADER_LOC_VECTOR_VIEW = 11;
    const COLOR_DIFFUSE as RL_SHADER_LOC_COLOR_DIFFUSE = 12;
    const COLOR_SPECULAR as RL_SHADER_LOC_COLOR_SPECULAR = 13;
    const COLOR_AMBIENT as RL_SHADER_LOC_COLOR_AMBIENT = 14;
    const MAP_ALBEDO as RL_SHADER_LOC_MAP_ALBEDO = 15;
    const MAP_METALNESS as RL_SHADER_LOC_MAP_METALNESS = 16;
    const MAP_NORMAL as RL_SHADER_LOC_MAP_NORMAL = 17;
    const MAP_ROUGHNESS as RL_SHADER_LOC_MAP_ROUGHNESS = 18;
    const MAP_OCCLUSION as RL_SHADER_LOC_MAP_OCCLUSION = 19;
    const MAP_EMISSION as RL_SHADER_LOC_MAP_EMISSION = 20;
    const MAP_HEIGHT as RL_SHADER_LOC_MAP_HEIGHT = 21;
    const MAP_CUBEMAP as RL_SHADER_LOC_MAP_CUBEMAP = 22;
    const MAP_IRRADIANCE as RL_SHADER_LOC_MAP_IRRADIANCE = 23;
    const MAP_PREFILTER as RL_SHADER_LOC_MAP_PREFILTER = 24;
    const MAP_BRDF as RL_SHADER_LOC_MAP_BRDF = 25;
}
}

define_rl_enum!{
#[allow(non_camel_case_types, reason = "consistency with RLGL naming convention")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum rlShaderUniformDataType: i32 {
    Float as RL_SHADER_UNIFORM_FLOAT = 0,
    Vec2 as RL_SHADER_UNIFORM_VEC2 = 1,
    Vec3 as RL_SHADER_UNIFORM_VEC3 = 2,
    Vec4 as RL_SHADER_UNIFORM_VEC4 = 3,
    Int as RL_SHADER_UNIFORM_INT = 4,
    IVec2 as RL_SHADER_UNIFORM_IVEC2 = 5,
    IVec3 as RL_SHADER_UNIFORM_IVEC3 = 6,
    IVec4 as RL_SHADER_UNIFORM_IVEC4 = 7,
    UInt as RL_SHADER_UNIFORM_UINT = 8,
    UIVec2 as RL_SHADER_UNIFORM_UIVEC2 = 9,
    UIVec3 as RL_SHADER_UNIFORM_UIVEC3 = 10,
    UIVec4 as RL_SHADER_UNIFORM_UIVEC4 = 11,
    Sampler2D as RL_SHADER_UNIFORM_SAMPLER2D = 12,
}
}

define_rl_enum!{
#[allow(non_camel_case_types, reason = "consistency with RLGL naming convention")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum rlShaderAttributeDataType: i32 {
    Float as RL_SHADER_ATTRIB_FLOAT = 0,
    Vec2 as RL_SHADER_ATTRIB_VEC2 = 1,
    Vec3 as RL_SHADER_ATTRIB_VEC3 = 2,
    Vec4 as RL_SHADER_ATTRIB_VEC4 = 3,
}
}

define_rl_enum!{
#[allow(non_camel_case_types, reason = "consistency with RLGL naming convention")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum rlFramebufferAttachType: i32 {
    ColorChannel0 as RL_ATTACHMENT_COLOR_CHANNEL0 = 0,
    ColorChannel1 as RL_ATTACHMENT_COLOR_CHANNEL1 = 1,
    ColorChannel2 as RL_ATTACHMENT_COLOR_CHANNEL2 = 2,
    ColorChannel3 as RL_ATTACHMENT_COLOR_CHANNEL3 = 3,
    ColorChannel4 as RL_ATTACHMENT_COLOR_CHANNEL4 = 4,
    ColorChannel5 as RL_ATTACHMENT_COLOR_CHANNEL5 = 5,
    ColorChannel6 as RL_ATTACHMENT_COLOR_CHANNEL6 = 6,
    ColorChannel7 as RL_ATTACHMENT_COLOR_CHANNEL7 = 7,
    Depth as RL_ATTACHMENT_DEPTH = 100,
    Stencil as RL_ATTACHMENT_STENCIL = 200,
}
}

define_rl_enum!{
#[allow(non_camel_case_types, reason = "consistency with RLGL naming convention")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum rlFramebufferAttachTextureType: i32 {
    CubemapPositiveX as RL_ATTACHMENT_CUBEMAP_POSITIVE_X = 0,
    CubemapNegativeX as RL_ATTACHMENT_CUBEMAP_NEGATIVE_X = 1,
    CubemapPositiveY as RL_ATTACHMENT_CUBEMAP_POSITIVE_Y = 2,
    CubemapNegativeY as RL_ATTACHMENT_CUBEMAP_NEGATIVE_Y = 3,
    CubemapPositiveZ as RL_ATTACHMENT_CUBEMAP_POSITIVE_Z = 4,
    CubemapNegativeZ as RL_ATTACHMENT_CUBEMAP_NEGATIVE_Z = 5,
    Texture2D as RL_ATTACHMENT_TEXTURE2D = 100,
    RenderBuffer as RL_ATTACHMENT_RENDERBUFFER = 200,
}
}

define_rl_enum!{
#[allow(non_camel_case_types, reason = "consistency with RLGL naming convention")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum rlCullMode: i32 {
    Front as RL_CULL_FACE_FRONT = 0,
    Back as RL_CULL_FACE_BACK = 1,
}
}
