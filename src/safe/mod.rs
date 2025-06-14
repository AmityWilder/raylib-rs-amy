use crate::{low_level::{self as ll, sys}, safe::into_cstr::IntoCStr};

pub mod into_cstr;

/// Evidence that the window has been initialized
///
/// Closes the window upon exiting scope
pub struct Window(());

impl Drop for Window {
    #[inline]
    fn drop(&mut self) {
        ll::close_window();
    }
}

impl Window {
    /// Initialize window and OpenGL context
    #[inline]
    pub fn init(width: u32, height: u32, title: impl IntoCStr) -> Option<Self> {
        if !ll::is_window_ready() {
            let title = title.into_cstr().unwrap();
            ll::init_window(width, height, &*title);
            if ll::is_window_ready() {
                return Some(Self(()));
            }
        }
        None
    }

    /// Check if application should close ([`ll::sys::KeyboardKey::KEY_ESCAPE`] pressed or windows close icon clicked)
    #[inline]
    pub fn should_close(&self) -> bool {
        ll::window_should_close()
    }

    /// Close window and unload OpenGL context
    #[inline]
    pub fn close(self) {
        drop(self);
    }

    /// Check if window is currently fullscreen
    #[inline]
    pub fn is_window_fullscreen(&self) -> bool {
        ll::is_window_fullscreen()
    }

    /// Check if window is currently hidden
    #[inline]
    pub fn is_window_hidden(&self) -> bool {
        ll::is_window_hidden()
    }

    /// Check if window is currently minimized
    #[inline]
    pub fn is_window_minimized(&self) -> bool {
        ll::is_window_minimized()
    }

    /// Check if window is currently maximized
    #[inline]
    pub fn is_window_maximized(&self) -> bool {
        ll::is_window_maximized()
    }

    /// Check if window is currently focused
    #[inline]
    pub fn is_window_focused(&self) -> bool {
        ll::is_window_focused()
    }

    /// Check if window has been resized last frame
    #[inline]
    pub fn is_window_resized(&self) -> bool {
        ll::is_window_resized()
    }

    /// Set window configuration state using flags
    #[inline]
    pub fn set_window_state(&mut self, flags: sys::ConfigFlags) {
        ll::set_window_state(flags);
    }

    /// Clear window configuration state flags
    #[inline]
    pub fn clear_window_state(&mut self, flags: sys::ConfigFlags) {
        ll::clear_window_state(flags);
    }

    /// Toggle window state: fullscreen/windowed, resizes monitor to match window resolution
    #[inline]
    pub fn toggle_fullscreen(&mut self) {
        ll::toggle_fullscreen();
    }

    /// Toggle window state: borderless windowed, resizes window to match monitor resolution
    #[inline]
    pub fn toggle_borderless_windowed(&mut self) {
        ll::toggle_borderless_windowed();
    }

    /// Set window state: maximized, if resizable
    #[inline]
    pub fn maximize_window(&mut self) {
        ll::maximize_window();
    }

    /// Set window state: minimized, if resizable
    #[inline]
    pub fn minimize_window(&mut self) {
        ll::minimize_window();
    }

    /// Restore window from being minimized/maximized
    #[inline]
    pub fn restore_window(&mut self) {
        ll::restore_window();
    }

    /// Set icon for window (single image, RGBA 32bit)
    #[inline]
    pub fn set_window_icon(&mut self, image: sys::Image) {
        ll::set_window_icon(image);
    }

    /// Set icon for window (multiple images, RGBA 32bit)
    #[inline]
    pub fn set_window_icons(&mut self, images: &[sys::Image]) {
        ll::set_window_icons(images);
    }

    /// Set title for window
    #[inline]
    pub fn set_window_title(&mut self, title: impl IntoCStr) {
        let title = title.into_cstr().unwrap();
        ll::set_window_title(&*title)
    }

    /// Set window position on screen
    #[inline]
    pub fn set_window_position(&mut self, x: i32, y: i32) {
        ll::set_window_position(x, y)
    }

    /// Setup canvas (framebuffer) to start drawing
    #[inline]
    pub fn begin_drawing(&mut self) -> Drawing {
        Drawing::begin(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
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
    #[inline]
    fn drop(&mut self) {
        ll::end_drawing();
    }
}

impl Drawing {
    /// Setup canvas (framebuffer) to start drawing
    #[inline]
    pub fn begin(_window: &mut Window) -> Self {
        ll::begin_drawing();
        Self(())
    }

    /// End canvas drawing and swap buffers (double buffering)
    #[inline]
    pub fn end(self) {
        drop(self);
    }

    /// Set background color (framebuffer clear color)
    #[inline]
    pub fn clear_background(&mut self, color: Color) {
        ll::clear_background(color.into());
    }
}
