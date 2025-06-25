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
                const $FLAG:ident as $($ALIASES:ident),+ = $value:expr;
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

        $($($vis const $ALIASES: $Struct = $Struct::$FLAG;)+)*
    };

    (
        $(#[$m:meta])*
        $vis:vis mod $Mod:ident: $ReprTy:ty {
            $(
                $(#[$vm:meta])*
                const $Constant:ident as $($ALIASES:ident),+ = $value:expr;
            )*
        }
    ) => {
        $(#[$m])*
        #[allow(non_snake_case)]
        $vis mod $Mod {
            $(pub const $Constant: $ReprTy = $value;)*
        }
        $vis use $Mod::{
            $($($Constant as $ALIASES,)+)*
        };
    };

    (
        $(#[$m:meta])*
        $vis:vis enum $Enum:ident: $ReprTy:ty {
            $(
                $(#[$vm:meta])*
                $Variant:ident as $($ALIASES:ident),+ = $value:expr
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
            $($($Variant as $ALIASES,)+)*
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
                $($(
                    const $ALIASES: $ReprTy = $Enum::$Variant as $ReprTy;
                )+)*
                match value {
                    $($($ALIASES)+)|* => Some(
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
    /// Key: \\
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
/// Mouse buttons
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MouseButton: i32 {
    /// Mouse button left
    Left as MOUSE_BUTTON_LEFT = 0,
    /// Mouse button right
    Right as MOUSE_BUTTON_RIGHT = 1,
    /// Mouse button middle (pressed wheel)
    Middle as MOUSE_BUTTON_MIDDLE = 2,
    /// Mouse button side (advanced mouse device)
    Side as MOUSE_BUTTON_SIDE = 3,
    /// Mouse button extra (advanced mouse device)
    Extra as MOUSE_BUTTON_EXTRA = 4,
    /// Mouse button forward (advanced mouse device)
    Forward as MOUSE_BUTTON_FORWARD = 5,
    /// Mouse button back (advanced mouse device)
    Back as MOUSE_BUTTON_BACK = 6,
}
}

define_rl_enum!{
/// Mouse cursor
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum MouseCursor: i32 {
    #[default]
    /// Default pointer shape
    Default as MOUSE_CURSOR_DEFAULT = 0,
    /// Arrow shape
    Arrow as MOUSE_CURSOR_ARROW = 1,
    /// Text writing cursor shape
    IBeam as MOUSE_CURSOR_IBEAM = 2,
    /// Cross shape
    Crosshair as MOUSE_CURSOR_CROSSHAIR = 3,
    /// Pointing hand cursor
    PointingHand as MOUSE_CURSOR_POINTING_HAND = 4,
    /// Horizontal resize/move arrow shape
    ResizeEW as MOUSE_CURSOR_RESIZE_EW = 5,
    /// Vertical resize/move arrow shape
    ResizeNS as MOUSE_CURSOR_RESIZE_NS = 6,
    /// Top-left to bottom-right diagonal resize/move arrow shape
    ResizeNWSE as MOUSE_CURSOR_RESIZE_NWSE = 7,
    /// The top-right to bottom-left diagonal resize/move arrow shape
    ResizeNESW as MOUSE_CURSOR_RESIZE_NESW = 8,
    /// The omnidirectional resize/move cursor shape
    ResizeAll as MOUSE_CURSOR_RESIZE_ALL = 9,
    /// The operation-not-allowed shape
    NotAllowed as MOUSE_CURSOR_NOT_ALLOWED = 10,
}
}

define_rl_enum!{
/// Gamepad buttons
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum GamepadButton: i32 {
    #[default]
    /// Unknown button, just for error checking
    Unknown as GAMEPAD_BUTTON_UNKNOWN = 0,
    /// Gamepad left DPAD up button
    LeftFaceUp as GAMEPAD_BUTTON_LEFT_FACE_UP = 1,
    /// Gamepad left DPAD right button
    LeftFaceRight as GAMEPAD_BUTTON_LEFT_FACE_RIGHT = 2,
    /// Gamepad left DPAD down button
    LeftFaceDown as GAMEPAD_BUTTON_LEFT_FACE_DOWN = 3,
    /// Gamepad left DPAD left button
    LeftFaceLeft as GAMEPAD_BUTTON_LEFT_FACE_LEFT = 4,
    /// Gamepad right button up (i.e. PS3: Triangle, Xbox: Y)
    RightFaceUp as GAMEPAD_BUTTON_RIGHT_FACE_UP = 5,
    /// Gamepad right button right (i.e. PS3: Circle, Xbox: B)
    RightFaceRight as GAMEPAD_BUTTON_RIGHT_FACE_RIGHT = 6,
    /// Gamepad right button down (i.e. PS3: Cross, Xbox: A)
    RightFaceDown as GAMEPAD_BUTTON_RIGHT_FACE_DOWN = 7,
    /// Gamepad right button left (i.e. PS3: Square, Xbox: X)
    RightFaceLeft as GAMEPAD_BUTTON_RIGHT_FACE_LEFT = 8,
    /// Gamepad top/back trigger left (first), it could be a trailing button
    LeftTrigger1 as GAMEPAD_BUTTON_LEFT_TRIGGER_1 = 9,
    /// Gamepad top/back trigger left (second), it could be a trailing button
    LeftTrigger2 as GAMEPAD_BUTTON_LEFT_TRIGGER_2 = 10,
    /// Gamepad top/back trigger right (first), it could be a trailing button
    RightTrigger1 as GAMEPAD_BUTTON_RIGHT_TRIGGER_1 = 11,
    /// Gamepad top/back trigger right (second), it could be a trailing button
    RightTrigger2 as GAMEPAD_BUTTON_RIGHT_TRIGGER_2 = 12,
    /// Gamepad center buttons, left one (i.e. PS3: Select)
    MiddleLeft as GAMEPAD_BUTTON_MIDDLE_LEFT = 13,
    /// Gamepad center buttons, middle one (i.e. PS3: PS, Xbox: XBOX)
    Middle as GAMEPAD_BUTTON_MIDDLE = 14,
    /// Gamepad center buttons, right one (i.e. PS3: Start)
    MiddleRight as GAMEPAD_BUTTON_MIDDLE_RIGHT = 15,
    /// Gamepad joystick pressed button left
    LeftThumb as GAMEPAD_BUTTON_LEFT_THUMB = 16,
    /// Gamepad joystick pressed button right
    RightThumb as GAMEPAD_BUTTON_RIGHT_THUMB = 17,
}
}

define_rl_enum!{
/// Gamepad axes
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GamepadAxis: i32 {
    /// Gamepad left stick X axis
    LeftX as GAMEPAD_AXIS_LEFT_X = 0,
    /// Gamepad left stick Y axis
    LeftY as GAMEPAD_AXIS_LEFT_Y = 1,
    /// Gamepad right stick X axis
    RightX as GAMEPAD_AXIS_RIGHT_X = 2,
    /// Gamepad right stick Y axis
    RightY as GAMEPAD_AXIS_RIGHT_Y = 3,
    /// Gamepad back trigger left, pressure level: [1..-1]
    LeftTrigger as GAMEPAD_AXIS_LEFT_TRIGGER = 4,
    /// Gamepad back trigger right, pressure level: [1..-1]
    RightTrigger as GAMEPAD_AXIS_RIGHT_TRIGGER = 5,
}
}

define_rl_enum!{
/// Material map index
pub mod MaterialMapIndex: u32 {
    /// Albedo material (same as: MATERIAL_MAP_DIFFUSE)
    const ALBEDO as MATERIAL_MAP_ALBEDO, DIFFUSE, MATERIAL_MAP_DIFFUSE = 0;
    /// Metalness material (same as: MATERIAL_MAP_SPECULAR)
    const METALNESS as MATERIAL_MAP_METALNESS, SPECULAR, MATERIAL_MAP_SPECULAR = 1;
    /// Normal material
    const NORMAL as MATERIAL_MAP_NORMAL = 2;
    /// Roughness material
    const ROUGHNESS as MATERIAL_MAP_ROUGHNESS = 3;
    /// Ambient occlusion material
    const OCCLUSION as MATERIAL_MAP_OCCLUSION = 4;
    /// Emission material
    const EMISSION as MATERIAL_MAP_EMISSION = 5;
    /// Heightmap material
    const HEIGHT as MATERIAL_MAP_HEIGHT = 6;
    /// Cubemap material (NOTE: Uses GL_TEXTURE_CUBE_MAP)
    const CUBEMAP as MATERIAL_MAP_CUBEMAP = 7;
    /// Irradiance material (NOTE: Uses GL_TEXTURE_CUBE_MAP)
    const IRRADIANCE as MATERIAL_MAP_IRRADIANCE = 8;
    /// Prefilter material (NOTE: Uses GL_TEXTURE_CUBE_MAP)
    const PREFILTER as MATERIAL_MAP_PREFILTER = 9;
    /// Brdf material
    const BRDF as MATERIAL_MAP_BRDF = 10;
}
}

define_rl_enum!{
/// Shader location index
pub mod ShaderLocationIndex: u32 {
    /// Shader location: vertex attribute: position
    const VERTEX_POSITION as SHADER_LOC_VERTEX_POSITION = 0;
    /// Shader location: vertex attribute: texcoord01
    const VERTEX_TEXCOORD01 as SHADER_LOC_VERTEX_TEXCOORD01 = 1;
    /// Shader location: vertex attribute: texcoord02
    const VERTEX_TEXCOORD02 as SHADER_LOC_VERTEX_TEXCOORD02 = 2;
    /// Shader location: vertex attribute: normal
    const VERTEX_NORMAL as SHADER_LOC_VERTEX_NORMAL = 3;
    /// Shader location: vertex attribute: tangent
    const VERTEX_TANGENT as SHADER_LOC_VERTEX_TANGENT = 4;
    /// Shader location: vertex attribute: color
    const VERTEX_COLOR as SHADER_LOC_VERTEX_COLOR = 5;
    /// Shader location: matrix uniform: model-view-projection
    const MATRIX_MVP as SHADER_LOC_MATRIX_MVP = 6;
    /// Shader location: matrix uniform: view (camera transform)
    const MATRIX_VIEW as SHADER_LOC_MATRIX_VIEW = 7;
    /// Shader location: matrix uniform: projection
    const MATRIX_PROJECTION as SHADER_LOC_MATRIX_PROJECTION = 8;
    /// Shader location: matrix uniform: model (transform)
    const MATRIX_MODEL as SHADER_LOC_MATRIX_MODEL = 9;
    /// Shader location: matrix uniform: normal
    const MATRIX_NORMAL as SHADER_LOC_MATRIX_NORMAL = 10;
    /// Shader location: vector uniform: view
    const VECTOR_VIEW as SHADER_LOC_VECTOR_VIEW = 11;
    /// Shader location: vector uniform: diffuse color
    const COLOR_DIFFUSE as SHADER_LOC_COLOR_DIFFUSE = 12;
    /// Shader location: vector uniform: specular color
    const COLOR_SPECULAR as SHADER_LOC_COLOR_SPECULAR = 13;
    /// Shader location: vector uniform: ambient color
    const COLOR_AMBIENT as SHADER_LOC_COLOR_AMBIENT = 14;
    /// Shader location: sampler2d texture: albedo (same as: SHADER_LOC_MAP_DIFFUSE)
    const MAP_ALBEDO as SHADER_LOC_MAP_ALBEDO, MAP_DIFFUSE, SHADER_LOC_MAP_DIFFUSE = 15;
    /// Shader location: sampler2d texture: metalness (same as: SHADER_LOC_MAP_SPECULAR)
    const MAP_METALNESS as SHADER_LOC_MAP_METALNESS, MAP_SPECULAR, SHADER_LOC_MAP_SPECULAR = 16;
    /// Shader location: sampler2d texture: normal
    const MAP_NORMAL as SHADER_LOC_MAP_NORMAL = 17;
    /// Shader location: sampler2d texture: roughness
    const MAP_ROUGHNESS as SHADER_LOC_MAP_ROUGHNESS = 18;
    /// Shader location: sampler2d texture: occlusion
    const MAP_OCCLUSION as SHADER_LOC_MAP_OCCLUSION = 19;
    /// Shader location: sampler2d texture: emission
    const MAP_EMISSION as SHADER_LOC_MAP_EMISSION = 20;
    /// Shader location: sampler2d texture: height
    const MAP_HEIGHT as SHADER_LOC_MAP_HEIGHT = 21;
    /// Shader location: samplerCube texture: cubemap
    const MAP_CUBEMAP as SHADER_LOC_MAP_CUBEMAP = 22;
    /// Shader location: samplerCube texture: irradiance
    const MAP_IRRADIANCE as SHADER_LOC_MAP_IRRADIANCE = 23;
    /// Shader location: samplerCube texture: prefilter
    const MAP_PREFILTER as SHADER_LOC_MAP_PREFILTER = 24;
    /// Shader location: sampler2d texture: brdf
    const MAP_BRDF as SHADER_LOC_MAP_BRDF = 25;
    /// Shader location: vertex attribute: boneIds
    const VERTEX_BONEIDS as SHADER_LOC_VERTEX_BONEIDS = 26;
    /// Shader location: vertex attribute: boneWeights
    const VERTEX_BONEWEIGHTS as SHADER_LOC_VERTEX_BONEWEIGHTS = 27;
    /// Shader location: array of matrices uniform: boneMatrices
    const BONE_MATRICES as SHADER_LOC_BONE_MATRICES = 28;
    /// Shader location: vertex attribute: instanceTransform
    const VERTEX_INSTANCE_TX as SHADER_LOC_VERTEX_INSTANCE_TX = 29;
}
}

define_rl_enum!{
/// Shader uniform data type
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShaderUniformDataType: i32 {
    /// Shader uniform type: float
    Float as SHADER_UNIFORM_FLOAT = 0,
    /// Shader uniform type: vec2 (2 float)
    Vec2 as SHADER_UNIFORM_VEC2 = 1,
    /// Shader uniform type: vec3 (3 float)
    Vec3 as SHADER_UNIFORM_VEC3 = 2,
    /// Shader uniform type: vec4 (4 float)
    Vec4 as SHADER_UNIFORM_VEC4 = 3,
    /// Shader uniform type: int
    Int as SHADER_UNIFORM_INT = 4,
    /// Shader uniform type: ivec2 (2 int)
    IVec2 as SHADER_UNIFORM_IVEC2 = 5,
    /// Shader uniform type: ivec3 (3 int)
    IVec3 as SHADER_UNIFORM_IVEC3 = 6,
    /// Shader uniform type: ivec4 (4 int)
    IVec4 as SHADER_UNIFORM_IVEC4 = 7,
    /// Shader uniform type: unsigned int
    UInt as SHADER_UNIFORM_UINT = 8,
    /// Shader uniform type: uivec2 (2 unsigned int)
    UIVec2 as SHADER_UNIFORM_UIVEC2 = 9,
    /// Shader uniform type: uivec3 (3 unsigned int)
    UIVec3 as SHADER_UNIFORM_UIVEC3 = 10,
    /// Shader uniform type: uivec4 (4 unsigned int)
    UIVec4 as SHADER_UNIFORM_UIVEC4 = 11,
    /// Shader uniform type: sampler2d
    Sampler2D as SHADER_UNIFORM_SAMPLER2D = 12,
}
}

define_rl_enum!{
/// Shader attribute data types
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShaderAttributeDataType: i32 {
    /// Shader attribute type: float
    Float as SHADER_ATTRIB_FLOAT = 0,
    /// Shader attribute type: vec2 (2 float)
    Vec2 as SHADER_ATTRIB_VEC2 = 1,
    /// Shader attribute type: vec3 (3 float)
    Vec3 as SHADER_ATTRIB_VEC3 = 2,
    /// Shader attribute type: vec4 (4 float)
    Vec4 as SHADER_ATTRIB_VEC4 = 3,
}
}

define_rl_enum!{
/// Pixel formats
///
/// NOTE: Support depends on OpenGL version and platform
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PixelFormat: i32 {
    /// 8 bit per pixel (no alpha)
    UncompressedGrayscale as PIXELFORMAT_UNCOMPRESSED_GRAYSCALE = 1,
    /// 8*2 bpp (2 channels)
    UncompressedGrayAlpha as PIXELFORMAT_UNCOMPRESSED_GRAY_ALPHA = 2,
    /// 16 bpp
    UncompressedR5G6B5 as PIXELFORMAT_UNCOMPRESSED_R5G6B5 = 3,
    /// 24 bpp
    UncompressedR8G8B8 as PIXELFORMAT_UNCOMPRESSED_R8G8B8 = 4,
    /// 16 bpp (1 bit alpha)
    UncompressedR5G5B5A1 as PIXELFORMAT_UNCOMPRESSED_R5G5B5A1 = 5,
    /// 16 bpp (4 bit alpha)
    UncompressedR4G4B4A4 as PIXELFORMAT_UNCOMPRESSED_R4G4B4A4 = 6,
    /// 32 bpp
    UncompressedR8G8B8A8 as PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 = 7,
    /// 32 bpp (1 channel - float)
    UncompressedR32 as PIXELFORMAT_UNCOMPRESSED_R32 = 8,
    /// 32*3 bpp (3 channels - float)
    UncompressedR32G32B32 as PIXELFORMAT_UNCOMPRESSED_R32G32B32 = 9,
    /// 32*4 bpp (4 channels - float)
    UncompressedR32G32B32A32 as PIXELFORMAT_UNCOMPRESSED_R32G32B32A32 = 10,
    /// 16 bpp (1 channel - half float)
    UncompressedR16 as PIXELFORMAT_UNCOMPRESSED_R16 = 11,
    /// 16*3 bpp (3 channels - half float)
    UncompressedR16G16B16 as PIXELFORMAT_UNCOMPRESSED_R16G16B16 = 12,
    /// 16*4 bpp (4 channels - half float)
    UncompressedR16G16B16A16 as PIXELFORMAT_UNCOMPRESSED_R16G16B16A16 = 13,
    /// 4 bpp (no alpha)
    CompressedDxt1Rgb as PIXELFORMAT_COMPRESSED_DXT1_RGB = 14,
    /// 4 bpp (1 bit alpha)
    CompressedDxt1Rgba as PIXELFORMAT_COMPRESSED_DXT1_RGBA = 15,
    /// 8 bpp
    CompressedDxt3Rgba as PIXELFORMAT_COMPRESSED_DXT3_RGBA = 16,
    /// 8 bpp
    CompressedDxt5Rgba as PIXELFORMAT_COMPRESSED_DXT5_RGBA = 17,
    /// 4 bpp
    CompressedEtc1Rgb as PIXELFORMAT_COMPRESSED_ETC1_RGB = 18,
    /// 4 bpp
    CompressedEtc2Rgb as PIXELFORMAT_COMPRESSED_ETC2_RGB = 19,
    /// 8 bpp
    CompressedEtc2EacRgba as PIXELFORMAT_COMPRESSED_ETC2_EAC_RGBA = 20,
    /// 4 bpp
    CompressedPvrtRgb as PIXELFORMAT_COMPRESSED_PVRT_RGB = 21,
    /// 4 bpp
    CompressedPvrtRgba as PIXELFORMAT_COMPRESSED_PVRT_RGBA = 22,
    /// 8 bpp
    CompressedAstc4x4Rgba as PIXELFORMAT_COMPRESSED_ASTC_4x4_RGBA = 23,
    /// 2 bpp
    CompressedAstc8x8Rgba as PIXELFORMAT_COMPRESSED_ASTC_8x8_RGBA = 24,
}
}

define_rl_enum!{
/// Texture parameters: filter mode
///
/// NOTE 1: Filtering considers mipmaps if available in the texture
///
/// NOTE 2: Filter is accordingly set for minification and magnification
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TextureFilter: i32 {
    /// No filter, just pixel approximation
    Point as TEXTURE_FILTER_POINT = 0,
    /// Linear filtering
    Bilinear as TEXTURE_FILTER_BILINEAR = 1,
    /// Trilinear filtering (linear with mipmaps)
    Trilinear as TEXTURE_FILTER_TRILINEAR = 2,
    /// Anisotropic filtering 4x
    Anisotropic4x as TEXTURE_FILTER_ANISOTROPIC_4X = 3,
    /// Anisotropic filtering 8x
    Anisotropic8x as TEXTURE_FILTER_ANISOTROPIC_8X = 4,
    /// Anisotropic filtering 16x
    Anisotropic16x as TEXTURE_FILTER_ANISOTROPIC_16X = 5,
}
}

define_rl_enum!{
/// Texture parameters: wrap mode
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TextureWrap: i32 {
    /// Repeats texture in tiled mode
    Repeat as TEXTURE_WRAP_REPEAT = 0,
    /// Clamps texture to edge pixel in tiled mode
    Clamp as TEXTURE_WRAP_CLAMP = 1,
    /// Mirrors and repeats the texture in tiled mode
    MirrorRepeat as TEXTURE_WRAP_MIRROR_REPEAT = 2,
    /// Mirrors and clamps to border the texture in tiled mode
    MirrorClamp as TEXTURE_WRAP_MIRROR_CLAMP = 3,
}
}

define_rl_enum!{
/// Cubemap layouts
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CubemapLayout: i32 {
    /// Automatically detect layout type
    AutoDetect as CUBEMAP_LAYOUT_AUTO_DETECT = 0,
    /// Layout is defined by a vertical line with faces
    LineVertical as CUBEMAP_LAYOUT_LINE_VERTICAL = 1,
    /// Layout is defined by a horizontal line with faces
    LineHorizontal as CUBEMAP_LAYOUT_LINE_HORIZONTAL = 2,
    /// Layout is defined by a 3x4 cross with cubemap faces
    CrossThreeByFour as CUBEMAP_LAYOUT_CROSS_THREE_BY_FOUR = 3,
    /// Layout is defined by a 4x3 cross with cubemap faces
    CrossFourByThree as CUBEMAP_LAYOUT_CROSS_FOUR_BY_THREE = 4,
}
}

define_rl_enum!{
/// Font type, defines generation method
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum FontType: i32 {
    #[default]
    /// Default font generation, anti-aliased
    Default as FONT_DEFAULT = 0,
    /// Bitmap font generation, no anti-aliasing
    Bitmap as FONT_BITMAP = 1,
    /// SDF font generation, requires external shader
    Sdf as FONT_SDF = 2,
}
}

define_rl_enum!{
/// Color blending modes (pre-defined)
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlendMode: i32 {
    /// Blend textures considering alpha (default)
    Alpha as BLEND_ALPHA = 0,
    /// Blend textures adding colors
    Additive as BLEND_ADDITIVE = 1,
    /// Blend textures multiplying colors
    Multiplied as BLEND_MULTIPLIED = 2,
    /// Blend textures adding colors (alternative)
    AddColors as BLEND_ADD_COLORS = 3,
    /// Blend textures subtracting colors (alternative)
    SubtractColors as BLEND_SUBTRACT_COLORS = 4,
    /// Blend premultiplied textures considering alpha
    AlphaPremultiply as BLEND_ALPHA_PREMULTIPLY = 5,
    /// Blend textures using custom src/dst factors (use rlSetBlendFactors())
    Custom as BLEND_CUSTOM = 6,
    /// Blend textures using custom rgb/alpha separate src/dst factors (use rlSetBlendFactorsSeparate())
    CustomSeparate as BLEND_CUSTOM_SEPARATE = 7,
}
}

define_rl_enum!{
/// Gesture
///
/// NOTE: Provided as bit-wise flags to enable only desired gestures
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Gesture: i32 {
    /// No gesture
    const None as GESTURE_NONE = 0;
    /// Tap gesture
    const Tap as GESTURE_TAP = 1;
    /// Double tap gesture
    const Doubletap as GESTURE_DOUBLETAP = 2;
    /// Hold gesture
    const Hold as GESTURE_HOLD = 4;
    /// Drag gesture
    const Drag as GESTURE_DRAG = 8;
    /// Swipe right gesture
    const SwipeRight as GESTURE_SWIPE_RIGHT = 16;
    /// Swipe left gesture
    const SwipeLeft as GESTURE_SWIPE_LEFT = 32;
    /// Swipe up gesture
    const SwipeUp as GESTURE_SWIPE_UP = 64;
    /// Swipe down gesture
    const SwipeDown as GESTURE_SWIPE_DOWN = 128;
    /// Pinch in gesture
    const PinchIn as GESTURE_PINCH_IN = 256;
    /// Pinch out gesture
    const PinchOut as GESTURE_PINCH_OUT = 512;
}
}

define_rl_enum!{
/// Camera system modes
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CameraMode: i32 {
    /// Camera custom, controlled by user (UpdateCamera() does nothing)
    Custom as CAMERA_CUSTOM = 0,
    /// Camera free mode
    Free as CAMERA_FREE = 1,
    /// Camera orbital, around target, zoom supported
    Orbital as CAMERA_ORBITAL = 2,
    /// Camera first person
    FirstPerson as CAMERA_FIRST_PERSON = 3,
    /// Camera third person
    ThirdPerson as CAMERA_THIRD_PERSON = 4,
}
}

define_rl_enum!{
/// Camera projection
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CameraProjection: i32 {
    /// Perspective projection
    Perspective as CAMERA_PERSPECTIVE = 0,
    /// Orthographic projection
    Orthographic as CAMERA_ORTHOGRAPHIC = 1,
}
}

define_rl_enum!{
/// N-patch layout
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NPatchLayout: i32 {
    /// Npatch layout: 3x3 tiles
    NinePatch as NPATCH_NINE_PATCH = 0,
    /// Npatch layout: 1x3 tiles
    ThreePatchVertical as NPATCH_THREE_PATCH_VERTICAL = 1,
    /// Npatch layout: 3x1 tiles
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
/// OpenGL version
#[allow(non_camel_case_types, reason = "consistency with RLGL naming convention")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum rlGlVersion: i32 {
    /// OpenGL 1.1
    Opengl11 as RL_OPENGL_11 = 1,
    /// OpenGL 2.1 (GLSL 120)
    Opengl21 as RL_OPENGL_21 = 2,
    /// OpenGL 3.3 (GLSL 330)
    Opengl33 as RL_OPENGL_33 = 3,
    /// OpenGL 4.3 (using GLSL 330)
    Opengl43 as RL_OPENGL_43 = 4,
    /// OpenGL ES 2.0 (GLSL 100)
    OpenglEs20 as RL_OPENGL_ES_20 = 5,
    /// OpenGL ES 3.0 (GLSL 300 es)
    OpenglEs30 as RL_OPENGL_ES_30 = 6,
}
}

define_rl_enum!{
/// Trace log level
///
/// NOTE: Organized by priority level
#[allow(non_camel_case_types, reason = "consistency with RLGL naming convention")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum rlTraceLogLevel: i32 {
    /// Display all logs
    All as RL_LOG_ALL = 0,
    /// Trace logging, intended for internal use only
    Trace as RL_LOG_TRACE = 1,
    /// Debug logging, used for internal debugging, it should be disabled on release builds
    Debug as RL_LOG_DEBUG = 2,
    /// Info logging, used for program execution info
    Info as RL_LOG_INFO = 3,
    /// Warning logging, used on recoverable failures
    Warning as RL_LOG_WARNING = 4,
    /// Error logging, used on unrecoverable failures
    Error as RL_LOG_ERROR = 5,
    /// Fatal logging, used to abort program: exit(EXIT_FAILURE)
    Fatal as RL_LOG_FATAL = 6,
    /// Disable logging
    None as RL_LOG_NONE = 7,
}
}

define_rl_enum!{
/// Texture pixel formats
///
/// NOTE: Support depends on OpenGL version
#[allow(non_camel_case_types, reason = "consistency with RLGL naming convention")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum rlPixelFormat: i32 {
    /// 8 bit per pixel (no alpha)
    UncompressedGrayscale as RL_PIXELFORMAT_UNCOMPRESSED_GRAYSCALE = 1,
    /// 8*2 bpp (2 channels)
    UncompressedGrayAlpha as RL_PIXELFORMAT_UNCOMPRESSED_GRAY_ALPHA = 2,
    /// 16 bpp
    UncompressedR5G6B5 as RL_PIXELFORMAT_UNCOMPRESSED_R5G6B5 = 3,
    /// 24 bpp
    UncompressedR8G8B8 as RL_PIXELFORMAT_UNCOMPRESSED_R8G8B8 = 4,
    /// 16 bpp (1 bit alpha)
    UncompressedR5G5B5A1 as RL_PIXELFORMAT_UNCOMPRESSED_R5G5B5A1 = 5,
    /// 16 bpp (4 bit alpha)
    UncompressedR4G4B4A4 as RL_PIXELFORMAT_UNCOMPRESSED_R4G4B4A4 = 6,
    /// 32 bpp
    UncompressedR8G8B8A8 as RL_PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 = 7,
    /// 32 bpp (1 channel - float)
    UncompressedR32 as RL_PIXELFORMAT_UNCOMPRESSED_R32 = 8,
    /// 32*3 bpp (3 channels - float)
    UncompressedR32G32B32 as RL_PIXELFORMAT_UNCOMPRESSED_R32G32B32 = 9,
    /// 32*4 bpp (4 channels - float)
    UncompressedR32G32B32A32 as RL_PIXELFORMAT_UNCOMPRESSED_R32G32B32A32 = 10,
    /// 16 bpp (1 channel - half float)
    UncompressedR16 as RL_PIXELFORMAT_UNCOMPRESSED_R16 = 11,
    /// 16*3 bpp (3 channels - half float)
    UncompressedR16G16B16 as RL_PIXELFORMAT_UNCOMPRESSED_R16G16B16 = 12,
    /// 16*4 bpp (4 channels - half float)
    UncompressedR16G16B16A16 as RL_PIXELFORMAT_UNCOMPRESSED_R16G16B16A16 = 13,
    /// 4 bpp (no alpha)
    CompressedDxt1Rgb as RL_PIXELFORMAT_COMPRESSED_DXT1_RGB = 14,
    /// 4 bpp (1 bit alpha)
    CompressedDxt1Rgba as RL_PIXELFORMAT_COMPRESSED_DXT1_RGBA = 15,
    /// 8 bpp
    CompressedDxt3Rgba as RL_PIXELFORMAT_COMPRESSED_DXT3_RGBA = 16,
    /// 8 bpp
    CompressedDxt5Rgba as RL_PIXELFORMAT_COMPRESSED_DXT5_RGBA = 17,
    /// 4 bpp
    CompressedEtc1Rgb as RL_PIXELFORMAT_COMPRESSED_ETC1_RGB = 18,
    /// 4 bpp
    CompressedEtc2Rgb as RL_PIXELFORMAT_COMPRESSED_ETC2_RGB = 19,
    /// 8 bpp
    CompressedEtc2EacRgba as RL_PIXELFORMAT_COMPRESSED_ETC2_EAC_RGBA = 20,
    /// 4 bpp
    CompressedPvrtRgb as RL_PIXELFORMAT_COMPRESSED_PVRT_RGB = 21,
    /// 4 bpp
    CompressedPvrtRgba as RL_PIXELFORMAT_COMPRESSED_PVRT_RGBA = 22,
    /// 8 bpp
    CompressedAstc4x4Rgba as RL_PIXELFORMAT_COMPRESSED_ASTC_4x4_RGBA = 23,
    /// 2 bpp
    CompressedAstc8x8Rgba as RL_PIXELFORMAT_COMPRESSED_ASTC_8x8_RGBA = 24,
}
}

define_rl_enum!{
/// Texture parameters: filter mode
///
/// NOTE 1: Filtering considers mipmaps if available in the texture
///
/// NOTE 2: Filter is accordingly set for minification and magnification
#[allow(non_camel_case_types, reason = "consistency with RLGL naming convention")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum rlTextureFilter: i32 {
    /// No filter, just pixel approximation
    Point as RL_TEXTURE_FILTER_POINT = 0,
    /// Linear filtering
    Bilinear as RL_TEXTURE_FILTER_BILINEAR = 1,
    /// Trilinear filtering (linear with mipmaps)
    Trilinear as RL_TEXTURE_FILTER_TRILINEAR = 2,
    /// Anisotropic filtering 4x
    Anisotropic4x as RL_TEXTURE_FILTER_ANISOTROPIC_4X = 3,
    /// Anisotropic filtering 8x
    Anisotropic8x as RL_TEXTURE_FILTER_ANISOTROPIC_8X = 4,
    /// Anisotropic filtering 16x
    Anisotropic16x as RL_TEXTURE_FILTER_ANISOTROPIC_16X = 5,
}
}

define_rl_enum!{
/// Color blending modes (pre-defined)
#[allow(non_camel_case_types, reason = "consistency with RLGL naming convention")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum rlBlendMode: i32 {
    #[default]
    /// Blend textures considering alpha (default)
    Alpha as RL_BLEND_ALPHA = 0,
    /// Blend textures adding colors
    Additive as RL_BLEND_ADDITIVE = 1,
    /// Blend textures multiplying colors
    Multiplied as RL_BLEND_MULTIPLIED = 2,
    /// Blend textures adding colors (alternative)
    AddColors as RL_BLEND_ADD_COLORS = 3,
    /// Blend textures subtracting colors (alternative)
    SubtractColors as RL_BLEND_SUBTRACT_COLORS = 4,
    /// Blend premultiplied textures considering alpha
    AlphaPremultiply as RL_BLEND_ALPHA_PREMULTIPLY = 5,
    /// Blend textures using custom src/dst factors (use rlSetBlendFactors())
    Custom as RL_BLEND_CUSTOM = 6,
    /// Blend textures using custom src/dst factors (use rlSetBlendFactorsSeparate())
    CustomSeparate as RL_BLEND_CUSTOM_SEPARATE = 7,
}
}

define_rl_enum!{
/// Shader location point type
pub mod rlShaderLocationIndex: u32 {
    /// Shader location: vertex attribute: position
    const VERTEX_POSITION as RL_SHADER_LOC_VERTEX_POSITION = 0;
    /// Shader location: vertex attribute: texcoord01
    const VERTEX_TEXCOORD01 as RL_SHADER_LOC_VERTEX_TEXCOORD01 = 1;
    /// Shader location: vertex attribute: texcoord02
    const VERTEX_TEXCOORD02 as RL_SHADER_LOC_VERTEX_TEXCOORD02 = 2;
    /// Shader location: vertex attribute: normal
    const VERTEX_NORMAL as RL_SHADER_LOC_VERTEX_NORMAL = 3;
    /// Shader location: vertex attribute: tangent
    const VERTEX_TANGENT as RL_SHADER_LOC_VERTEX_TANGENT = 4;
    /// Shader location: vertex attribute: color
    const VERTEX_COLOR as RL_SHADER_LOC_VERTEX_COLOR = 5;
    /// Shader location: matrix uniform: model-view-projection
    const MATRIX_MVP as RL_SHADER_LOC_MATRIX_MVP = 6;
    /// Shader location: matrix uniform: view (camera transform)
    const MATRIX_VIEW as RL_SHADER_LOC_MATRIX_VIEW = 7;
    /// Shader location: matrix uniform: projection
    const MATRIX_PROJECTION as RL_SHADER_LOC_MATRIX_PROJECTION = 8;
    /// Shader location: matrix uniform: model (transform)
    const MATRIX_MODEL as RL_SHADER_LOC_MATRIX_MODEL = 9;
    /// Shader location: matrix uniform: normal
    const MATRIX_NORMAL as RL_SHADER_LOC_MATRIX_NORMAL = 10;
    /// Shader location: vector uniform: view
    const VECTOR_VIEW as RL_SHADER_LOC_VECTOR_VIEW = 11;
    /// Shader location: vector uniform: diffuse color
    const COLOR_DIFFUSE as RL_SHADER_LOC_COLOR_DIFFUSE = 12;
    /// Shader location: vector uniform: specular color
    const COLOR_SPECULAR as RL_SHADER_LOC_COLOR_SPECULAR = 13;
    /// Shader location: vector uniform: ambient color
    const COLOR_AMBIENT as RL_SHADER_LOC_COLOR_AMBIENT = 14;
    /// Shader location: sampler2d texture: albedo (same as: RL_SHADER_LOC_MAP_DIFFUSE)
    const MAP_ALBEDO as RL_SHADER_LOC_MAP_ALBEDO = 15;
    const MAP_DIFFUSE as RL_SHADER_LOC_MAP_DIFFUSE = MAP_ALBEDO;
    /// Shader location: sampler2d texture: metalness (same as: RL_SHADER_LOC_MAP_SPECULAR)
    const MAP_METALNESS as RL_SHADER_LOC_MAP_METALNESS = 16;
    const MAP_SPECULAR as RL_SHADER_LOC_MAP_SPECULAR = MAP_METALNESS;
    /// Shader location: sampler2d texture: normal
    const MAP_NORMAL as RL_SHADER_LOC_MAP_NORMAL = 17;
    /// Shader location: sampler2d texture: roughness
    const MAP_ROUGHNESS as RL_SHADER_LOC_MAP_ROUGHNESS = 18;
    /// Shader location: sampler2d texture: occlusion
    const MAP_OCCLUSION as RL_SHADER_LOC_MAP_OCCLUSION = 19;
    /// Shader location: sampler2d texture: emission
    const MAP_EMISSION as RL_SHADER_LOC_MAP_EMISSION = 20;
    /// Shader location: sampler2d texture: height
    const MAP_HEIGHT as RL_SHADER_LOC_MAP_HEIGHT = 21;
    /// Shader location: samplerCube texture: cubemap
    const MAP_CUBEMAP as RL_SHADER_LOC_MAP_CUBEMAP = 22;
    /// Shader location: samplerCube texture: irradiance
    const MAP_IRRADIANCE as RL_SHADER_LOC_MAP_IRRADIANCE = 23;
    /// Shader location: samplerCube texture: prefilter
    const MAP_PREFILTER as RL_SHADER_LOC_MAP_PREFILTER = 24;
    /// Shader location: sampler2d texture: brdf
    const MAP_BRDF as RL_SHADER_LOC_MAP_BRDF = 25;
}
}

define_rl_enum!{
/// Shader uniform data type
#[allow(non_camel_case_types, reason = "consistency with RLGL naming convention")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum rlShaderUniformDataType: i32 {
    /// Shader uniform type: float
    Float as RL_SHADER_UNIFORM_FLOAT = 0,
    /// Shader uniform type: vec2 (2 float)
    Vec2 as RL_SHADER_UNIFORM_VEC2 = 1,
    /// Shader uniform type: vec3 (3 float)
    Vec3 as RL_SHADER_UNIFORM_VEC3 = 2,
    /// Shader uniform type: vec4 (4 float)
    Vec4 as RL_SHADER_UNIFORM_VEC4 = 3,
    /// Shader uniform type: int
    Int as RL_SHADER_UNIFORM_INT = 4,
    /// Shader uniform type: ivec2 (2 int)
    IVec2 as RL_SHADER_UNIFORM_IVEC2 = 5,
    /// Shader uniform type: ivec3 (3 int)
    IVec3 as RL_SHADER_UNIFORM_IVEC3 = 6,
    /// Shader uniform type: ivec4 (4 int)
    IVec4 as RL_SHADER_UNIFORM_IVEC4 = 7,
    /// Shader uniform type: unsigned int
    UInt as RL_SHADER_UNIFORM_UINT = 8,
    /// Shader uniform type: uivec2 (2 unsigned int)
    UIVec2 as RL_SHADER_UNIFORM_UIVEC2 = 9,
    /// Shader uniform type: uivec3 (3 unsigned int)
    UIVec3 as RL_SHADER_UNIFORM_UIVEC3 = 10,
    /// Shader uniform type: uivec4 (4 unsigned int)
    UIVec4 as RL_SHADER_UNIFORM_UIVEC4 = 11,
    /// Shader uniform type: sampler2d
    Sampler2D as RL_SHADER_UNIFORM_SAMPLER2D = 12,
}
}

define_rl_enum!{
/// Shader attribute data types
#[allow(non_camel_case_types, reason = "consistency with RLGL naming convention")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum rlShaderAttributeDataType: i32 {
    /// Shader attribute type: float
    Float as RL_SHADER_ATTRIB_FLOAT = 0,
    /// Shader attribute type: vec2 (2 float)
    Vec2 as RL_SHADER_ATTRIB_VEC2 = 1,
    /// Shader attribute type: vec3 (3 float)
    Vec3 as RL_SHADER_ATTRIB_VEC3 = 2,
    /// Shader attribute type: vec4 (4 float)
    Vec4 as RL_SHADER_ATTRIB_VEC4 = 3,
}
}

define_rl_enum!{
/// Framebuffer attachment type
///
/// NOTE: By default up to 8 color channels defined, but it can be more
#[allow(non_camel_case_types, reason = "consistency with RLGL naming convention")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum rlFramebufferAttachType: i32 {
    /// Framebuffer attachment type: color 0
    ColorChannel0 as RL_ATTACHMENT_COLOR_CHANNEL0 = 0,
    /// Framebuffer attachment type: color 1
    ColorChannel1 as RL_ATTACHMENT_COLOR_CHANNEL1 = 1,
    /// Framebuffer attachment type: color 2
    ColorChannel2 as RL_ATTACHMENT_COLOR_CHANNEL2 = 2,
    /// Framebuffer attachment type: color 3
    ColorChannel3 as RL_ATTACHMENT_COLOR_CHANNEL3 = 3,
    /// Framebuffer attachment type: color 4
    ColorChannel4 as RL_ATTACHMENT_COLOR_CHANNEL4 = 4,
    /// Framebuffer attachment type: color 5
    ColorChannel5 as RL_ATTACHMENT_COLOR_CHANNEL5 = 5,
    /// Framebuffer attachment type: color 6
    ColorChannel6 as RL_ATTACHMENT_COLOR_CHANNEL6 = 6,
    /// Framebuffer attachment type: color 7
    ColorChannel7 as RL_ATTACHMENT_COLOR_CHANNEL7 = 7,
    /// Framebuffer attachment type: depth
    Depth as RL_ATTACHMENT_DEPTH = 100,
    /// Framebuffer attachment type: stencil
    Stencil as RL_ATTACHMENT_STENCIL = 200,
}
}

define_rl_enum!{
/// Framebuffer texture attachment type
#[allow(non_camel_case_types, reason = "consistency with RLGL naming convention")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum rlFramebufferAttachTextureType: i32 {
    /// Framebuffer texture attachment type: cubemap, +X side
    CubemapPositiveX as RL_ATTACHMENT_CUBEMAP_POSITIVE_X = 0,
    /// Framebuffer texture attachment type: cubemap, -X side
    CubemapNegativeX as RL_ATTACHMENT_CUBEMAP_NEGATIVE_X = 1,
    /// Framebuffer texture attachment type: cubemap, +Y side
    CubemapPositiveY as RL_ATTACHMENT_CUBEMAP_POSITIVE_Y = 2,
    /// Framebuffer texture attachment type: cubemap, -Y side
    CubemapNegativeY as RL_ATTACHMENT_CUBEMAP_NEGATIVE_Y = 3,
    /// Framebuffer texture attachment type: cubemap, +Z side
    CubemapPositiveZ as RL_ATTACHMENT_CUBEMAP_POSITIVE_Z = 4,
    /// Framebuffer texture attachment type: cubemap, -Z side
    CubemapNegativeZ as RL_ATTACHMENT_CUBEMAP_NEGATIVE_Z = 5,
    /// Framebuffer texture attachment type: texture2d
    Texture2D as RL_ATTACHMENT_TEXTURE2D = 100,
    /// Framebuffer texture attachment type: renderbuffer
    RenderBuffer as RL_ATTACHMENT_RENDERBUFFER = 200,
}
}

define_rl_enum!{
/// Face culling mode
#[allow(non_camel_case_types, reason = "consistency with RLGL naming convention")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum rlCullMode: i32 {
    Front as RL_CULL_FACE_FRONT = 0,
    Back as RL_CULL_FACE_BACK = 1,
}
}
