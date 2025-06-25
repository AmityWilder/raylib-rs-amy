use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ConfigFlags: i32 {
        const VSYNC_HINT = 64;
        const FULLSCREEN_MODE = 2;
        const WINDOW_RESIZABLE = 4;
        const WINDOW_UNDECORATED = 8;
        const WINDOW_HIDDEN = 128;
        const WINDOW_MINIMIZED = 512;
        const WINDOW_MAXIMIZED = 1024;
        const WINDOW_UNFOCUSED = 2048;
        const WINDOW_TOPMOST = 4096;
        const WINDOW_ALWAYS_RUN = 256;
        const WINDOW_TRANSPARENT = 16;
        const WINDOW_HIGHDPI = 8192;
        const WINDOW_MOUSE_PASSTHROUGH = 16384;
        const BORDERLESS_WINDOWED_MODE = 32768;
        const MSAA_4X_HINT = 32;
        const INTERLACED_HINT = 65536;
    }
}
pub const FLAG_VSYNC_HINT: ConfigFlags = ConfigFlags::VSYNC_HINT;
pub const FLAG_FULLSCREEN_MODE: ConfigFlags = ConfigFlags::FULLSCREEN_MODE;
pub const FLAG_WINDOW_RESIZABLE: ConfigFlags = ConfigFlags::WINDOW_RESIZABLE;
pub const FLAG_WINDOW_UNDECORATED: ConfigFlags = ConfigFlags::WINDOW_UNDECORATED;
pub const FLAG_WINDOW_HIDDEN: ConfigFlags = ConfigFlags::WINDOW_HIDDEN;
pub const FLAG_WINDOW_MINIMIZED: ConfigFlags = ConfigFlags::WINDOW_MINIMIZED;
pub const FLAG_WINDOW_MAXIMIZED: ConfigFlags = ConfigFlags::WINDOW_MAXIMIZED;
pub const FLAG_WINDOW_UNFOCUSED: ConfigFlags = ConfigFlags::WINDOW_UNFOCUSED;
pub const FLAG_WINDOW_TOPMOST: ConfigFlags = ConfigFlags::WINDOW_TOPMOST;
pub const FLAG_WINDOW_ALWAYS_RUN: ConfigFlags = ConfigFlags::WINDOW_ALWAYS_RUN;
pub const FLAG_WINDOW_TRANSPARENT: ConfigFlags = ConfigFlags::WINDOW_TRANSPARENT;
pub const FLAG_WINDOW_HIGHDPI: ConfigFlags = ConfigFlags::WINDOW_HIGHDPI;
pub const FLAG_WINDOW_MOUSE_PASSTHROUGH: ConfigFlags = ConfigFlags::WINDOW_MOUSE_PASSTHROUGH;
pub const FLAG_BORDERLESS_WINDOWED_MODE: ConfigFlags = ConfigFlags::BORDERLESS_WINDOWED_MODE;
pub const FLAG_MSAA_4X_HINT: ConfigFlags = ConfigFlags::MSAA_4X_HINT;
pub const FLAG_INTERLACED_HINT: ConfigFlags = ConfigFlags::INTERLACED_HINT;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum TraceLogLevel {
    All = 0,
    Trace = 1,
    Debug = 2,
    Info = 3,
    Warning = 4,
    Error = 5,
    Fatal = 6,
    None = 7,
}
pub use TraceLogLevel::{
    All as LOG_ALL,
    Trace as LOG_TRACE,
    Debug as LOG_DEBUG,
    Info as LOG_INFO,
    Warning as LOG_WARNING,
    Error as LOG_ERROR,
    Fatal as LOG_FATAL,
    None as LOG_NONE,
};

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Default, Hash)]
pub enum KeyboardKey {
    #[default]
    Null = 0,
    Apostrophe = 39,
    Comma = 44,
    Minus = 45,
    Period = 46,
    Slash = 47,
    Zero = 48,
    One = 49,
    Two = 50,
    Three = 51,
    Four = 52,
    Five = 53,
    Six = 54,
    Seven = 55,
    Eight = 56,
    Nine = 57,
    Semicolon = 59,
    Equal = 61,
    A = 65,
    B = 66,
    C = 67,
    D = 68,
    E = 69,
    F = 70,
    G = 71,
    H = 72,
    I = 73,
    J = 74,
    K = 75,
    L = 76,
    M = 77,
    N = 78,
    O = 79,
    P = 80,
    Q = 81,
    R = 82,
    S = 83,
    T = 84,
    U = 85,
    V = 86,
    W = 87,
    X = 88,
    Y = 89,
    Z = 90,
    LeftBracket = 91,
    Backslash = 92,
    RightBracket = 93,
    Grave = 96,
    Space = 32,
    Escape = 256,
    Enter = 257,
    Tab = 258,
    Backspace = 259,
    Insert = 260,
    Delete = 261,
    Right = 262,
    Left = 263,
    Down = 264,
    Up = 265,
    PageUp = 266,
    PageDown = 267,
    Home = 268,
    End = 269,
    CapsLock = 280,
    ScrollLock = 281,
    NumLock = 282,
    PrintScreen = 283,
    Pause = 284,
    F1 = 290,
    F2 = 291,
    F3 = 292,
    F4 = 293,
    F5 = 294,
    F6 = 295,
    F7 = 296,
    F8 = 297,
    F9 = 298,
    F10 = 299,
    F11 = 300,
    F12 = 301,
    LeftShift = 340,
    LeftControl = 341,
    LeftAlt = 342,
    LeftSuper = 343,
    RightShift = 344,
    RightControl = 345,
    RightAlt = 346,
    RightSuper = 347,
    KbMenu = 348,
    Kp0 = 320,
    Kp1 = 321,
    Kp2 = 322,
    Kp3 = 323,
    Kp4 = 324,
    Kp5 = 325,
    Kp6 = 326,
    Kp7 = 327,
    Kp8 = 328,
    Kp9 = 329,
    KpDecimal = 330,
    KpDivide = 331,
    KpMultiply = 332,
    KpSubtract = 333,
    KpAdd = 334,
    KpEnter = 335,
    KpEqual = 336,
    Back = 4,
    Menu = 5,
    VolumeUp = 24,
    VolumeDown = 25,
}
pub use KeyboardKey::{
    Null as KEY_NULL,
    Apostrophe as KEY_APOSTROPHE,
    Comma as KEY_COMMA,
    Minus as KEY_MINUS,
    Period as KEY_PERIOD,
    Slash as KEY_SLASH,
    Zero as KEY_ZERO,
    One as KEY_ONE,
    Two as KEY_TWO,
    Three as KEY_THREE,
    Four as KEY_FOUR,
    Five as KEY_FIVE,
    Six as KEY_SIX,
    Seven as KEY_SEVEN,
    Eight as KEY_EIGHT,
    Nine as KEY_NINE,
    Semicolon as KEY_SEMICOLON,
    Equal as KEY_EQUAL,
    A as KEY_A,
    B as KEY_B,
    C as KEY_C,
    D as KEY_D,
    E as KEY_E,
    F as KEY_F,
    G as KEY_G,
    H as KEY_H,
    I as KEY_I,
    J as KEY_J,
    K as KEY_K,
    L as KEY_L,
    M as KEY_M,
    N as KEY_N,
    O as KEY_O,
    P as KEY_P,
    Q as KEY_Q,
    R as KEY_R,
    S as KEY_S,
    T as KEY_T,
    U as KEY_U,
    V as KEY_V,
    W as KEY_W,
    X as KEY_X,
    Y as KEY_Y,
    Z as KEY_Z,
    LeftBracket as KEY_LEFT_BRACKET,
    Backslash as KEY_BACKSLASH,
    RightBracket as KEY_RIGHT_BRACKET,
    Grave as KEY_GRAVE,
    Space as KEY_SPACE,
    Escape as KEY_ESCAPE,
    Enter as KEY_ENTER,
    Tab as KEY_TAB,
    Backspace as KEY_BACKSPACE,
    Insert as KEY_INSERT,
    Delete as KEY_DELETE,
    Right as KEY_RIGHT,
    Left as KEY_LEFT,
    Down as KEY_DOWN,
    Up as KEY_UP,
    PageUp as KEY_PAGE_UP,
    PageDown as KEY_PAGE_DOWN,
    Home as KEY_HOME,
    End as KEY_END,
    CapsLock as KEY_CAPS_LOCK,
    ScrollLock as KEY_SCROLL_LOCK,
    NumLock as KEY_NUM_LOCK,
    PrintScreen as KEY_PRINT_SCREEN,
    Pause as KEY_PAUSE,
    F1 as KEY_F1,
    F2 as KEY_F2,
    F3 as KEY_F3,
    F4 as KEY_F4,
    F5 as KEY_F5,
    F6 as KEY_F6,
    F7 as KEY_F7,
    F8 as KEY_F8,
    F9 as KEY_F9,
    F10 as KEY_F10,
    F11 as KEY_F11,
    F12 as KEY_F12,
    LeftShift as KEY_LEFT_SHIFT,
    LeftControl as KEY_LEFT_CONTROL,
    LeftAlt as KEY_LEFT_ALT,
    LeftSuper as KEY_LEFT_SUPER,
    RightShift as KEY_RIGHT_SHIFT,
    RightControl as KEY_RIGHT_CONTROL,
    RightAlt as KEY_RIGHT_ALT,
    RightSuper as KEY_RIGHT_SUPER,
    KbMenu as KEY_KB_MENU,
    Kp0 as KEY_KP_0,
    Kp1 as KEY_KP_1,
    Kp2 as KEY_KP_2,
    Kp3 as KEY_KP_3,
    Kp4 as KEY_KP_4,
    Kp5 as KEY_KP_5,
    Kp6 as KEY_KP_6,
    Kp7 as KEY_KP_7,
    Kp8 as KEY_KP_8,
    Kp9 as KEY_KP_9,
    KpDecimal as KEY_KP_DECIMAL,
    KpDivide as KEY_KP_DIVIDE,
    KpMultiply as KEY_KP_MULTIPLY,
    KpSubtract as KEY_KP_SUBTRACT,
    KpAdd as KEY_KP_ADD,
    KpEnter as KEY_KP_ENTER,
    KpEqual as KEY_KP_EQUAL,
    Back as KEY_BACK,
    Menu as KEY_MENU,
    VolumeUp as KEY_VOLUME_UP,
    VolumeDown as KEY_VOLUME_DOWN,
};

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left = 0,
    Right = 1,
    Middle = 2,
    Side = 3,
    Extra = 4,
    Forward = 5,
    Back = 6,
}
pub use MouseButton::{
    Left as MOUSE_BUTTON_LEFT,
    Right as MOUSE_BUTTON_RIGHT,
    Middle as MOUSE_BUTTON_MIDDLE,
    Side as MOUSE_BUTTON_SIDE,
    Extra as MOUSE_BUTTON_EXTRA,
    Forward as MOUSE_BUTTON_FORWARD,
    Back as MOUSE_BUTTON_BACK,
};

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Default, Hash)]
pub enum MouseCursor {
    #[default]
    Default = 0,
    Arrow = 1,
    IBeam = 2,
    Crosshair = 3,
    PointingHand = 4,
    ResizeEW = 5,
    ResizeNS = 6,
    ResizeNWSE = 7,
    ResizeNESW = 8,
    ResizeAll = 9,
    NotAllowed = 10,
}
pub use MouseCursor::{
    Default as MOUSE_CURSOR_DEFAULT,
    Arrow as MOUSE_CURSOR_ARROW,
    IBeam as MOUSE_CURSOR_IBEAM,
    Crosshair as MOUSE_CURSOR_CROSSHAIR,
    PointingHand as MOUSE_CURSOR_POINTING_HAND,
    ResizeEW as MOUSE_CURSOR_RESIZE_EW,
    ResizeNS as MOUSE_CURSOR_RESIZE_NS,
    ResizeNWSE as MOUSE_CURSOR_RESIZE_NWSE,
    ResizeNESW as MOUSE_CURSOR_RESIZE_NESW,
    ResizeAll as MOUSE_CURSOR_RESIZE_ALL,
    NotAllowed as MOUSE_CURSOR_NOT_ALLOWED,
};

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Default, Hash)]
pub enum GamepadButton {
    #[default]
    Unknown = 0,
    LeftFaceUp = 1,
    LeftFaceRight = 2,
    LeftFaceDown = 3,
    LeftFaceLeft = 4,
    RightFaceUp = 5,
    RightFaceRight = 6,
    RightFaceDown = 7,
    RightFaceLeft = 8,
    LeftTrigger1 = 9,
    LeftTrigger2 = 10,
    RightTrigger1 = 11,
    RightTrigger2 = 12,
    MiddleLeft = 13,
    Middle = 14,
    MiddleRight = 15,
    LeftThumb = 16,
    RightThumb = 17,
}
pub use GamepadButton::{
    Unknown as GAMEPAD_BUTTON_UNKNOWN,
    LeftFaceUp as GAMEPAD_BUTTON_LEFT_FACE_UP,
    LeftFaceRight as GAMEPAD_BUTTON_LEFT_FACE_RIGHT,
    LeftFaceDown as GAMEPAD_BUTTON_LEFT_FACE_DOWN,
    LeftFaceLeft as GAMEPAD_BUTTON_LEFT_FACE_LEFT,
    RightFaceUp as GAMEPAD_BUTTON_RIGHT_FACE_UP,
    RightFaceRight as GAMEPAD_BUTTON_RIGHT_FACE_RIGHT,
    RightFaceDown as GAMEPAD_BUTTON_RIGHT_FACE_DOWN,
    RightFaceLeft as GAMEPAD_BUTTON_RIGHT_FACE_LEFT,
    LeftTrigger1 as GAMEPAD_BUTTON_LEFT_TRIGGER_1,
    LeftTrigger2 as GAMEPAD_BUTTON_LEFT_TRIGGER_2,
    RightTrigger1 as GAMEPAD_BUTTON_RIGHT_TRIGGER_1,
    RightTrigger2 as GAMEPAD_BUTTON_RIGHT_TRIGGER_2,
    MiddleLeft as GAMEPAD_BUTTON_MIDDLE_LEFT,
    Middle as GAMEPAD_BUTTON_MIDDLE,
    MiddleRight as GAMEPAD_BUTTON_MIDDLE_RIGHT,
    LeftThumb as GAMEPAD_BUTTON_LEFT_THUMB,
    RightThumb as GAMEPAD_BUTTON_RIGHT_THUMB,
};

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum GamepadAxis {
    LeftX = 0,
    LeftY = 1,
    RightX = 2,
    RightY = 3,
    LeftTrigger = 4,
    RightTrigger = 5,
}
pub use GamepadAxis::{
    LeftX as GAMEPAD_AXIS_LEFT_X,
    LeftY as GAMEPAD_AXIS_LEFT_Y,
    RightX as GAMEPAD_AXIS_RIGHT_X,
    RightY as GAMEPAD_AXIS_RIGHT_Y,
    LeftTrigger as GAMEPAD_AXIS_LEFT_TRIGGER,
    RightTrigger as GAMEPAD_AXIS_RIGHT_TRIGGER,
};

#[allow(non_snake_case)]
pub mod MaterialMapIndex {
    pub const ALBEDO: u32 = 0;
    pub const METALNESS: u32 = 1;
    pub const NORMAL: u32 = 2;
    pub const ROUGHNESS: u32 = 3;
    pub const OCCLUSION: u32 = 4;
    pub const EMISSION: u32 = 5;
    pub const HEIGHT: u32 = 6;
    pub const CUBEMAP: u32 = 7;
    pub const IRRADIANCE: u32 = 8;
    pub const PREFILTER: u32 = 9;
    pub const BRDF: u32 = 10;
}
pub const MATERIAL_MAP_ALBEDO: u32 = MaterialMapIndex::ALBEDO;
pub const MATERIAL_MAP_METALNESS: u32 = MaterialMapIndex::METALNESS;
pub const MATERIAL_MAP_NORMAL: u32 = MaterialMapIndex::NORMAL;
pub const MATERIAL_MAP_ROUGHNESS: u32 = MaterialMapIndex::ROUGHNESS;
pub const MATERIAL_MAP_OCCLUSION: u32 = MaterialMapIndex::OCCLUSION;
pub const MATERIAL_MAP_EMISSION: u32 = MaterialMapIndex::EMISSION;
pub const MATERIAL_MAP_HEIGHT: u32 = MaterialMapIndex::HEIGHT;
pub const MATERIAL_MAP_CUBEMAP: u32 = MaterialMapIndex::CUBEMAP;
pub const MATERIAL_MAP_IRRADIANCE: u32 = MaterialMapIndex::IRRADIANCE;
pub const MATERIAL_MAP_PREFILTER: u32 = MaterialMapIndex::PREFILTER;
pub const MATERIAL_MAP_BRDF: u32 = MaterialMapIndex::BRDF;

#[allow(non_snake_case)]
pub mod ShaderLocationIndex {
    pub const VERTEX_POSITION: u32 = 0;
    pub const VERTEX_TEXCOORD01: u32 = 1;
    pub const VERTEX_TEXCOORD02: u32 = 2;
    pub const VERTEX_NORMAL: u32 = 3;
    pub const VERTEX_TANGENT: u32 = 4;
    pub const VERTEX_COLOR: u32 = 5;
    pub const MATRIX_MVP: u32 = 6;
    pub const MATRIX_VIEW: u32 = 7;
    pub const MATRIX_PROJECTION: u32 = 8;
    pub const MATRIX_MODEL: u32 = 9;
    pub const MATRIX_NORMAL: u32 = 10;
    pub const VECTOR_VIEW: u32 = 11;
    pub const COLOR_DIFFUSE: u32 = 12;
    pub const COLOR_SPECULAR: u32 = 13;
    pub const COLOR_AMBIENT: u32 = 14;
    pub const MAP_ALBEDO: u32 = 15;
    pub const MAP_METALNESS: u32 = 16;
    pub const MAP_NORMAL: u32 = 17;
    pub const MAP_ROUGHNESS: u32 = 18;
    pub const MAP_OCCLUSION: u32 = 19;
    pub const MAP_EMISSION: u32 = 20;
    pub const MAP_HEIGHT: u32 = 21;
    pub const MAP_CUBEMAP: u32 = 22;
    pub const MAP_IRRADIANCE: u32 = 23;
    pub const MAP_PREFILTER: u32 = 24;
    pub const MAP_BRDF: u32 = 25;
    pub const VERTEX_BONEIDS: u32 = 26;
    pub const VERTEX_BONEWEIGHTS: u32 = 27;
    pub const BONE_MATRICES: u32 = 28;
    pub const VERTEX_INSTANCE_TX: u32 = 29;
}
pub const SHADER_LOC_VERTEX_POSITION: u32 = ShaderLocationIndex::VERTEX_POSITION;
pub const SHADER_LOC_VERTEX_TEXCOORD01: u32 = ShaderLocationIndex::VERTEX_TEXCOORD01;
pub const SHADER_LOC_VERTEX_TEXCOORD02: u32 = ShaderLocationIndex::VERTEX_TEXCOORD02;
pub const SHADER_LOC_VERTEX_NORMAL: u32 = ShaderLocationIndex::VERTEX_NORMAL;
pub const SHADER_LOC_VERTEX_TANGENT: u32 = ShaderLocationIndex::VERTEX_TANGENT;
pub const SHADER_LOC_VERTEX_COLOR: u32 = ShaderLocationIndex::VERTEX_COLOR;
pub const SHADER_LOC_MATRIX_MVP: u32 = ShaderLocationIndex::MATRIX_MVP;
pub const SHADER_LOC_MATRIX_VIEW: u32 = ShaderLocationIndex::MATRIX_VIEW;
pub const SHADER_LOC_MATRIX_PROJECTION: u32 = ShaderLocationIndex::MATRIX_PROJECTION;
pub const SHADER_LOC_MATRIX_MODEL: u32 = ShaderLocationIndex::MATRIX_MODEL;
pub const SHADER_LOC_MATRIX_NORMAL: u32 = ShaderLocationIndex::MATRIX_NORMAL;
pub const SHADER_LOC_VECTOR_VIEW: u32 = ShaderLocationIndex::VECTOR_VIEW;
pub const SHADER_LOC_COLOR_DIFFUSE: u32 = ShaderLocationIndex::COLOR_DIFFUSE;
pub const SHADER_LOC_COLOR_SPECULAR: u32 = ShaderLocationIndex::COLOR_SPECULAR;
pub const SHADER_LOC_COLOR_AMBIENT: u32 = ShaderLocationIndex::COLOR_AMBIENT;
pub const SHADER_LOC_MAP_ALBEDO: u32 = ShaderLocationIndex::MAP_ALBEDO;
pub const SHADER_LOC_MAP_METALNESS: u32 = ShaderLocationIndex::MAP_METALNESS;
pub const SHADER_LOC_MAP_NORMAL: u32 = ShaderLocationIndex::MAP_NORMAL;
pub const SHADER_LOC_MAP_ROUGHNESS: u32 = ShaderLocationIndex::MAP_ROUGHNESS;
pub const SHADER_LOC_MAP_OCCLUSION: u32 = ShaderLocationIndex::MAP_OCCLUSION;
pub const SHADER_LOC_MAP_EMISSION: u32 = ShaderLocationIndex::MAP_EMISSION;
pub const SHADER_LOC_MAP_HEIGHT: u32 = ShaderLocationIndex::MAP_HEIGHT;
pub const SHADER_LOC_MAP_CUBEMAP: u32 = ShaderLocationIndex::MAP_CUBEMAP;
pub const SHADER_LOC_MAP_IRRADIANCE: u32 = ShaderLocationIndex::MAP_IRRADIANCE;
pub const SHADER_LOC_MAP_PREFILTER: u32 = ShaderLocationIndex::MAP_PREFILTER;
pub const SHADER_LOC_MAP_BRDF: u32 = ShaderLocationIndex::MAP_BRDF;
pub const SHADER_LOC_VERTEX_BONEIDS: u32 = ShaderLocationIndex::VERTEX_BONEIDS;
pub const SHADER_LOC_VERTEX_BONEWEIGHTS: u32 = ShaderLocationIndex::VERTEX_BONEWEIGHTS;
pub const SHADER_LOC_BONE_MATRICES: u32 = ShaderLocationIndex::BONE_MATRICES;
pub const SHADER_LOC_VERTEX_INSTANCE_TX: u32 = ShaderLocationIndex::VERTEX_INSTANCE_TX;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ShaderUniformDataType {
    Float = 0,
    Vec2 = 1,
    Vec3 = 2,
    Vec4 = 3,
    Int = 4,
    IVec2 = 5,
    IVec3 = 6,
    IVec4 = 7,
    UInt = 8,
    UIVec2 = 9,
    UIVec3 = 10,
    UIVec4 = 11,
    Sampler2D = 12,
}
pub use ShaderUniformDataType::{
    Float as SHADER_UNIFORM_FLOAT,
    Vec2 as SHADER_UNIFORM_VEC2,
    Vec3 as SHADER_UNIFORM_VEC3,
    Vec4 as SHADER_UNIFORM_VEC4,
    Int as SHADER_UNIFORM_INT,
    IVec2 as SHADER_UNIFORM_IVEC2,
    IVec3 as SHADER_UNIFORM_IVEC3,
    IVec4 as SHADER_UNIFORM_IVEC4,
    UInt as SHADER_UNIFORM_UINT,
    UIVec2 as SHADER_UNIFORM_UIVEC2,
    UIVec3 as SHADER_UNIFORM_UIVEC3,
    UIVec4 as SHADER_UNIFORM_UIVEC4,
    Sampler2D as SHADER_UNIFORM_SAMPLER2D,
};

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ShaderAttributeDataType {
    Float = 0,
    Vec2 = 1,
    Vec3 = 2,
    Vec4 = 3,
}
pub use ShaderAttributeDataType::{
    Float as SHADER_ATTRIB_FLOAT,
    Vec2 as SHADER_ATTRIB_VEC2,
    Vec3 as SHADER_ATTRIB_VEC3,
    Vec4 as SHADER_ATTRIB_VEC4,
};

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PixelFormat {
    UncompressedGrayscale = 1,
    UncompressedGrayAlpha = 2,
    UncompressedR5G6B5 = 3,
    UncompressedR8G8B8 = 4,
    UncompressedR5G5B5A1 = 5,
    UncompressedR4G4B4A4 = 6,
    UncompressedR8G8B8A8 = 7,
    UncompressedR32 = 8,
    UncompressedR32G32B32 = 9,
    UncompressedR32G32B32A32 = 10,
    UncompressedR16 = 11,
    UncompressedR16G16B16 = 12,
    UncompressedR16G16B16A16 = 13,
    CompressedDxt1Rgb = 14,
    CompressedDxt1Rgba = 15,
    CompressedDxt3Rgba = 16,
    CompressedDxt5Rgba = 17,
    CompressedEtc1Rgb = 18,
    CompressedEtc2Rgb = 19,
    CompressedEtc2EacRgba = 20,
    CompressedPvrtRgb = 21,
    CompressedPvrtRgba = 22,
    CompressedAstc4x4Rgba = 23,
    CompressedAstc8x8Rgba = 24,
}
pub use PixelFormat::{
    UncompressedGrayscale as PIXELFORMAT_UNCOMPRESSED_GRAYSCALE,
    UncompressedGrayAlpha as PIXELFORMAT_UNCOMPRESSED_GRAY_ALPHA,
    UncompressedR5G6B5 as PIXELFORMAT_UNCOMPRESSED_R5G6B5,
    UncompressedR8G8B8 as PIXELFORMAT_UNCOMPRESSED_R8G8B8,
    UncompressedR5G5B5A1 as PIXELFORMAT_UNCOMPRESSED_R5G5B5A1,
    UncompressedR4G4B4A4 as PIXELFORMAT_UNCOMPRESSED_R4G4B4A4,
    UncompressedR8G8B8A8 as PIXELFORMAT_UNCOMPRESSED_R8G8B8A8,
    UncompressedR32 as PIXELFORMAT_UNCOMPRESSED_R32,
    UncompressedR32G32B32 as PIXELFORMAT_UNCOMPRESSED_R32G32B32,
    UncompressedR32G32B32A32 as PIXELFORMAT_UNCOMPRESSED_R32G32B32A32,
    UncompressedR16 as PIXELFORMAT_UNCOMPRESSED_R16,
    UncompressedR16G16B16 as PIXELFORMAT_UNCOMPRESSED_R16G16B16,
    UncompressedR16G16B16A16 as PIXELFORMAT_UNCOMPRESSED_R16G16B16A16,
    CompressedDxt1Rgb as PIXELFORMAT_COMPRESSED_DXT1_RGB,
    CompressedDxt1Rgba as PIXELFORMAT_COMPRESSED_DXT1_RGBA,
    CompressedDxt3Rgba as PIXELFORMAT_COMPRESSED_DXT3_RGBA,
    CompressedDxt5Rgba as PIXELFORMAT_COMPRESSED_DXT5_RGBA,
    CompressedEtc1Rgb as PIXELFORMAT_COMPRESSED_ETC1_RGB,
    CompressedEtc2Rgb as PIXELFORMAT_COMPRESSED_ETC2_RGB,
    CompressedEtc2EacRgba as PIXELFORMAT_COMPRESSED_ETC2_EAC_RGBA,
    CompressedPvrtRgb as PIXELFORMAT_COMPRESSED_PVRT_RGB,
    CompressedPvrtRgba as PIXELFORMAT_COMPRESSED_PVRT_RGBA,
    CompressedAstc4x4Rgba as PIXELFORMAT_COMPRESSED_ASTC_4x4_RGBA,
    CompressedAstc8x8Rgba as PIXELFORMAT_COMPRESSED_ASTC_8x8_RGBA,
};

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum TextureFilter {
    Point = 0,
    Bilinear = 1,
    Trilinear = 2,
    Anisotropic4x = 3,
    Anisotropic8x = 4,
    Anisotropic16x = 5,
}
pub use TextureFilter::{
    Point as TEXTURE_FILTER_POINT,
    Bilinear as TEXTURE_FILTER_BILINEAR,
    Trilinear as TEXTURE_FILTER_TRILINEAR,
    Anisotropic4x as TEXTURE_FILTER_ANISOTROPIC_4X,
    Anisotropic8x as TEXTURE_FILTER_ANISOTROPIC_8X,
    Anisotropic16x as TEXTURE_FILTER_ANISOTROPIC_16X,
};

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum TextureWrap {
    Repeat = 0,
    Clamp = 1,
    MirrorRepeat = 2,
    MirrorClamp = 3,
}
pub use TextureWrap::{
    Repeat as TEXTURE_WRAP_REPEAT,
    Clamp as TEXTURE_WRAP_CLAMP,
    MirrorRepeat as TEXTURE_WRAP_MIRROR_REPEAT,
    MirrorClamp as TEXTURE_WRAP_MIRROR_CLAMP,
};

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CubemapLayout {
    AutoDetect = 0,
    LineVertical = 1,
    LineHorizontal = 2,
    CrossThreeByFour = 3,
    CrossFourByThree = 4,
}
pub use CubemapLayout::{
    AutoDetect as CUBEMAP_LAYOUT_AUTO_DETECT,
    LineVertical as CUBEMAP_LAYOUT_LINE_VERTICAL,
    LineHorizontal as CUBEMAP_LAYOUT_LINE_HORIZONTAL,
    CrossThreeByFour as CUBEMAP_LAYOUT_CROSS_THREE_BY_FOUR,
    CrossFourByThree as CUBEMAP_LAYOUT_CROSS_FOUR_BY_THREE,
};

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
pub enum FontType {
    #[default]
    Default = 0,
    Bitmap = 1,
    Sdf = 2,
}
pub use FontType::{
    Default as FONT_DEFAULT,
    Bitmap as FONT_BITMAP,
    Sdf as FONT_SDF,
};

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum BlendMode {
    Alpha = 0,
    Additive = 1,
    Multiplied = 2,
    AddColors = 3,
    SubtractColors = 4,
    AlphaPremultiply = 5,
    Custom = 6,
    CustomSeparate = 7,
}
pub use BlendMode::{
    Alpha as BLEND_ALPHA,
    Additive as BLEND_ADDITIVE,
    Multiplied as BLEND_MULTIPLIED,
    AddColors as BLEND_ADD_COLORS,
    SubtractColors as BLEND_SUBTRACT_COLORS,
    AlphaPremultiply as BLEND_ALPHA_PREMULTIPLY,
    Custom as BLEND_CUSTOM,
    CustomSeparate as BLEND_CUSTOM_SEPARATE,
};

bitflags! {
    #[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
    pub struct Gesture: i32 {
        const None = 0;
        const Tap = 1;
        const Doubletap = 2;
        const Hold = 4;
        const Drag = 8;
        const SwipeRight = 16;
        const SwipeLeft = 32;
        const SwipeUp = 64;
        const SwipeDown = 128;
        const PinchIn = 256;
        const PinchOut = 512;
    }
}
pub const GESTURE_NONE: Gesture = Gesture::None;
pub const GESTURE_TAP: Gesture = Gesture::Tap;
pub const GESTURE_DOUBLETAP: Gesture = Gesture::Doubletap;
pub const GESTURE_HOLD: Gesture = Gesture::Hold;
pub const GESTURE_DRAG: Gesture = Gesture::Drag;
pub const GESTURE_SWIPE_RIGHT: Gesture = Gesture::SwipeRight;
pub const GESTURE_SWIPE_LEFT: Gesture = Gesture::SwipeLeft;
pub const GESTURE_SWIPE_UP: Gesture = Gesture::SwipeUp;
pub const GESTURE_SWIPE_DOWN: Gesture = Gesture::SwipeDown;
pub const GESTURE_PINCH_IN: Gesture = Gesture::PinchIn;
pub const GESTURE_PINCH_OUT: Gesture = Gesture::PinchOut;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CameraMode {
    Custom = 0,
    Free = 1,
    Orbital = 2,
    FirstPerson = 3,
    ThirdPerson = 4,
}
pub use CameraMode::{
    Custom as CAMERA_CUSTOM,
    Free as CAMERA_FREE,
    Orbital as CAMERA_ORBITAL,
    FirstPerson as CAMERA_FIRST_PERSON,
    ThirdPerson as CAMERA_THIRD_PERSON,
};

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CameraProjection {
    Perspective = 0,
    Orthographic = 1,
}
pub use CameraProjection::{
    Perspective as CAMERA_PERSPECTIVE,
    Orthographic as CAMERA_ORTHOGRAPHIC,
};

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum NPatchLayout {
    NPATCH_NINE_PATCH = 0,
    NPATCH_THREE_PATCH_VERTICAL = 1,
    NPATCH_THREE_PATCH_HORIZONTAL = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum TouchAction {
    TOUCH_ACTION_UP = 0,
    TOUCH_ACTION_DOWN = 1,
    TOUCH_ACTION_MOVE = 2,
    TOUCH_ACTION_CANCEL = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum rlGlVersion {
    RL_OPENGL_11 = 1,
    RL_OPENGL_21 = 2,
    RL_OPENGL_33 = 3,
    RL_OPENGL_43 = 4,
    RL_OPENGL_ES_20 = 5,
    RL_OPENGL_ES_30 = 6,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum rlTraceLogLevel {
    RL_LOG_ALL = 0,
    RL_LOG_TRACE = 1,
    RL_LOG_DEBUG = 2,
    RL_LOG_INFO = 3,
    RL_LOG_WARNING = 4,
    RL_LOG_ERROR = 5,
    RL_LOG_FATAL = 6,
    RL_LOG_NONE = 7,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum rlPixelFormat {
    RL_PIXELFORMAT_UNCOMPRESSED_GRAYSCALE = 1,
    RL_PIXELFORMAT_UNCOMPRESSED_GRAY_ALPHA = 2,
    RL_PIXELFORMAT_UNCOMPRESSED_R5G6B5 = 3,
    RL_PIXELFORMAT_UNCOMPRESSED_R8G8B8 = 4,
    RL_PIXELFORMAT_UNCOMPRESSED_R5G5B5A1 = 5,
    RL_PIXELFORMAT_UNCOMPRESSED_R4G4B4A4 = 6,
    RL_PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 = 7,
    RL_PIXELFORMAT_UNCOMPRESSED_R32 = 8,
    RL_PIXELFORMAT_UNCOMPRESSED_R32G32B32 = 9,
    RL_PIXELFORMAT_UNCOMPRESSED_R32G32B32A32 = 10,
    RL_PIXELFORMAT_UNCOMPRESSED_R16 = 11,
    RL_PIXELFORMAT_UNCOMPRESSED_R16G16B16 = 12,
    RL_PIXELFORMAT_UNCOMPRESSED_R16G16B16A16 = 13,
    RL_PIXELFORMAT_COMPRESSED_DXT1_RGB = 14,
    RL_PIXELFORMAT_COMPRESSED_DXT1_RGBA = 15,
    RL_PIXELFORMAT_COMPRESSED_DXT3_RGBA = 16,
    RL_PIXELFORMAT_COMPRESSED_DXT5_RGBA = 17,
    RL_PIXELFORMAT_COMPRESSED_ETC1_RGB = 18,
    RL_PIXELFORMAT_COMPRESSED_ETC2_RGB = 19,
    RL_PIXELFORMAT_COMPRESSED_ETC2_EAC_RGBA = 20,
    RL_PIXELFORMAT_COMPRESSED_PVRT_RGB = 21,
    RL_PIXELFORMAT_COMPRESSED_PVRT_RGBA = 22,
    RL_PIXELFORMAT_COMPRESSED_ASTC_4x4_RGBA = 23,
    RL_PIXELFORMAT_COMPRESSED_ASTC_8x8_RGBA = 24,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum rlTextureFilter {
    RL_TEXTURE_FILTER_POINT = 0,
    RL_TEXTURE_FILTER_BILINEAR = 1,
    RL_TEXTURE_FILTER_TRILINEAR = 2,
    RL_TEXTURE_FILTER_ANISOTROPIC_4X = 3,
    RL_TEXTURE_FILTER_ANISOTROPIC_8X = 4,
    RL_TEXTURE_FILTER_ANISOTROPIC_16X = 5,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum rlBlendMode {
    RL_BLEND_ALPHA = 0,
    RL_BLEND_ADDITIVE = 1,
    RL_BLEND_MULTIPLIED = 2,
    RL_BLEND_ADD_COLORS = 3,
    RL_BLEND_SUBTRACT_COLORS = 4,
    RL_BLEND_ALPHA_PREMULTIPLY = 5,
    RL_BLEND_CUSTOM = 6,
    RL_BLEND_CUSTOM_SEPARATE = 7,
}
pub mod rlShaderLocationIndex {
    pub type Type = u32;
    pub const RL_SHADER_LOC_VERTEX_POSITION: Type = 0;
    pub const RL_SHADER_LOC_VERTEX_TEXCOORD01: Type = 1;
    pub const RL_SHADER_LOC_VERTEX_TEXCOORD02: Type = 2;
    pub const RL_SHADER_LOC_VERTEX_NORMAL: Type = 3;
    pub const RL_SHADER_LOC_VERTEX_TANGENT: Type = 4;
    pub const RL_SHADER_LOC_VERTEX_COLOR: Type = 5;
    pub const RL_SHADER_LOC_MATRIX_MVP: Type = 6;
    pub const RL_SHADER_LOC_MATRIX_VIEW: Type = 7;
    pub const RL_SHADER_LOC_MATRIX_PROJECTION: Type = 8;
    pub const RL_SHADER_LOC_MATRIX_MODEL: Type = 9;
    pub const RL_SHADER_LOC_MATRIX_NORMAL: Type = 10;
    pub const RL_SHADER_LOC_VECTOR_VIEW: Type = 11;
    pub const RL_SHADER_LOC_COLOR_DIFFUSE: Type = 12;
    pub const RL_SHADER_LOC_COLOR_SPECULAR: Type = 13;
    pub const RL_SHADER_LOC_COLOR_AMBIENT: Type = 14;
    pub const RL_SHADER_LOC_MAP_ALBEDO: Type = 15;
    pub const RL_SHADER_LOC_MAP_METALNESS: Type = 16;
    pub const RL_SHADER_LOC_MAP_NORMAL: Type = 17;
    pub const RL_SHADER_LOC_MAP_ROUGHNESS: Type = 18;
    pub const RL_SHADER_LOC_MAP_OCCLUSION: Type = 19;
    pub const RL_SHADER_LOC_MAP_EMISSION: Type = 20;
    pub const RL_SHADER_LOC_MAP_HEIGHT: Type = 21;
    pub const RL_SHADER_LOC_MAP_CUBEMAP: Type = 22;
    pub const RL_SHADER_LOC_MAP_IRRADIANCE: Type = 23;
    pub const RL_SHADER_LOC_MAP_PREFILTER: Type = 24;
    pub const RL_SHADER_LOC_MAP_BRDF: Type = 25;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum rlShaderUniformDataType {
    RL_SHADER_UNIFORM_FLOAT = 0,
    RL_SHADER_UNIFORM_VEC2 = 1,
    RL_SHADER_UNIFORM_VEC3 = 2,
    RL_SHADER_UNIFORM_VEC4 = 3,
    RL_SHADER_UNIFORM_INT = 4,
    RL_SHADER_UNIFORM_IVEC2 = 5,
    RL_SHADER_UNIFORM_IVEC3 = 6,
    RL_SHADER_UNIFORM_IVEC4 = 7,
    RL_SHADER_UNIFORM_UINT = 8,
    RL_SHADER_UNIFORM_UIVEC2 = 9,
    RL_SHADER_UNIFORM_UIVEC3 = 10,
    RL_SHADER_UNIFORM_UIVEC4 = 11,
    RL_SHADER_UNIFORM_SAMPLER2D = 12,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum rlShaderAttributeDataType {
    RL_SHADER_ATTRIB_FLOAT = 0,
    RL_SHADER_ATTRIB_VEC2 = 1,
    RL_SHADER_ATTRIB_VEC3 = 2,
    RL_SHADER_ATTRIB_VEC4 = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum rlFramebufferAttachType {
    RL_ATTACHMENT_COLOR_CHANNEL0 = 0,
    RL_ATTACHMENT_COLOR_CHANNEL1 = 1,
    RL_ATTACHMENT_COLOR_CHANNEL2 = 2,
    RL_ATTACHMENT_COLOR_CHANNEL3 = 3,
    RL_ATTACHMENT_COLOR_CHANNEL4 = 4,
    RL_ATTACHMENT_COLOR_CHANNEL5 = 5,
    RL_ATTACHMENT_COLOR_CHANNEL6 = 6,
    RL_ATTACHMENT_COLOR_CHANNEL7 = 7,
    RL_ATTACHMENT_DEPTH = 100,
    RL_ATTACHMENT_STENCIL = 200,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum rlFramebufferAttachTextureType {
    RL_ATTACHMENT_CUBEMAP_POSITIVE_X = 0,
    RL_ATTACHMENT_CUBEMAP_NEGATIVE_X = 1,
    RL_ATTACHMENT_CUBEMAP_POSITIVE_Y = 2,
    RL_ATTACHMENT_CUBEMAP_NEGATIVE_Y = 3,
    RL_ATTACHMENT_CUBEMAP_POSITIVE_Z = 4,
    RL_ATTACHMENT_CUBEMAP_NEGATIVE_Z = 5,
    RL_ATTACHMENT_TEXTURE2D = 100,
    RL_ATTACHMENT_RENDERBUFFER = 200,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum rlCullMode {
    RL_CULL_FACE_FRONT = 0,
    RL_CULL_FACE_BACK = 1,
}