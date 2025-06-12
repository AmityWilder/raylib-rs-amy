use super::*;

// Font loading/unloading functions

/// Get the default Font
#[inline]
pub fn GetFontDefault() -> Font;

/// Load font from file into GPU memory (VRAM)
#[inline]
pub fn LoadFont(fileName: *const ::std::os::raw::c_char) -> Font;

/// Load font from file with extended parameters, use NULL for codepoints and 0 for codepointCount to load the default character set, font size is provided in pixels height
#[inline]
pub fn LoadFontEx(
    fileName: *const ::std::os::raw::c_char,
    fontSize: ::std::os::raw::c_int,
    codepoints: *mut ::std::os::raw::c_int,
    codepointCount: ::std::os::raw::c_int,
) -> Font;

/// Load font from Image (XNA style)
#[inline]
pub fn LoadFontFromImage(image: Image, key: Color, firstChar: ::std::os::raw::c_int) -> Font;

/// Load font from memory buffer, fileType refers to extension: i.e. '.ttf'
#[inline]
pub fn LoadFontFromMemory(
    fileType: *const ::std::os::raw::c_char,
    fileData: *const ::std::os::raw::c_uchar,
    dataSize: ::std::os::raw::c_int,
    fontSize: ::std::os::raw::c_int,
    codepoints: *mut ::std::os::raw::c_int,
    codepointCount: ::std::os::raw::c_int,
) -> Font;

/// Check if a font is valid (font data loaded, WARNING: GPU texture not checked)
#[inline]
pub fn IsFontValid(font: Font) -> bool;

/// Load font data for further use
#[inline]
pub fn LoadFontData(
    fileData: *const ::std::os::raw::c_uchar,
    dataSize: ::std::os::raw::c_int,
    fontSize: ::std::os::raw::c_int,
    codepoints: *mut ::std::os::raw::c_int,
    codepointCount: ::std::os::raw::c_int,
    type_: ::std::os::raw::c_int,
) -> *mut GlyphInfo;

/// Generate image font atlas using chars info
#[inline]
pub fn GenImageFontAtlas(
    glyphs: *const GlyphInfo,
    glyphRecs: *mut *mut Rectangle,
    glyphCount: ::std::os::raw::c_int,
    fontSize: ::std::os::raw::c_int,
    padding: ::std::os::raw::c_int,
    packMethod: ::std::os::raw::c_int,
) -> Image;

/// Unload font chars info data (RAM)
#[inline]
pub fn UnloadFontData(glyphs: *mut GlyphInfo, glyphCount: ::std::os::raw::c_int);

/// Unload font from GPU memory (VRAM)
#[inline]
pub fn UnloadFont(font: Font);

/// Export font as code file, returns true on success
#[inline]
pub fn ExportFontAsCode(font: Font, fileName: *const ::std::os::raw::c_char) -> bool;

// Text drawing functions

/// Draw current FPS
#[inline]
pub fn DrawFPS(posX: ::std::os::raw::c_int, posY: ::std::os::raw::c_int);

/// Draw text (using default font)
#[inline]
pub fn DrawText(
    text: *const ::std::os::raw::c_char,
    posX: ::std::os::raw::c_int,
    posY: ::std::os::raw::c_int,
    fontSize: ::std::os::raw::c_int,
    color: Color,
);

/// Draw text using font and additional parameters
#[inline]
pub fn DrawTextEx(
    font: Font,
    text: *const ::std::os::raw::c_char,
    position: Vector2,
    fontSize: ::std::os::raw::c_float,
    spacing: ::std::os::raw::c_float,
    tint: Color,
);

/// Draw text using Font and pro parameters (rotation)
#[inline]
pub fn DrawTextPro(
    font: Font,
    text: *const ::std::os::raw::c_char,
    position: Vector2,
    origin: Vector2,
    rotation: ::std::os::raw::c_float,
    fontSize: ::std::os::raw::c_float,
    spacing: ::std::os::raw::c_float,
    tint: Color,
);

/// Draw one character (codepoint)
#[inline]
pub fn DrawTextCodepoint(
    font: Font,
    codepoint: ::std::os::raw::c_int,
    position: Vector2,
    fontSize: ::std::os::raw::c_float,
    tint: Color,
);

/// Draw multiple character (codepoint)
#[inline]
pub fn DrawTextCodepoints(
    font: Font,
    codepoints: *const ::std::os::raw::c_int,
    codepointCount: ::std::os::raw::c_int,
    position: Vector2,
    fontSize: ::std::os::raw::c_float,
    spacing: ::std::os::raw::c_float,
    tint: Color,
);

// Text font info functions

/// Set vertical line spacing when drawing with line-breaks
#[inline]
pub fn SetTextLineSpacing(spacing: ::std::os::raw::c_int);

/// Measure string width for default font
#[inline]
pub fn MeasureText(
    text: *const ::std::os::raw::c_char,
    fontSize: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int;

/// Measure string size for Font
#[inline]
pub fn MeasureTextEx(
    font: Font,
    text: *const ::std::os::raw::c_char,
    fontSize: ::std::os::raw::c_float,
    spacing: ::std::os::raw::c_float,
) -> Vector2;

/// Get glyph index position in font for a codepoint (unicode character), fallback to '?' if not found
#[inline]
pub fn GetGlyphIndex(font: Font, codepoint: ::std::os::raw::c_int) -> ::std::os::raw::c_int;

/// Get glyph font info data for a codepoint (unicode character), fallback to '?' if not found
#[inline]
pub fn GetGlyphInfo(font: Font, codepoint: ::std::os::raw::c_int) -> GlyphInfo;

/// Get glyph rectangle in font atlas for a codepoint (unicode character), fallback to '?' if not found
#[inline]
pub fn GetGlyphAtlasRec(font: Font, codepoint: ::std::os::raw::c_int) -> Rectangle;

// Text codepoints management functions (unicode characters)

/// Load UTF-8 text encoded from codepoints array
#[inline]
pub fn LoadUTF8(
    codepoints: *const ::std::os::raw::c_int,
    length: ::std::os::raw::c_int,
) -> *mut ::std::os::raw::c_char;

/// Unload UTF-8 text encoded from codepoints array
#[inline]
pub fn UnloadUTF8(text: *mut ::std::os::raw::c_char);

/// Load all codepoints from a UTF-8 text string, codepoints count returned by parameter
#[inline]
pub fn LoadCodepoints(
    text: *const ::std::os::raw::c_char,
    count: *mut ::std::os::raw::c_int,
) -> *mut ::std::os::raw::c_int;

/// Unload codepoints data from memory
#[inline]
pub fn UnloadCodepoints(codepoints: *mut ::std::os::raw::c_int);

/// Get total number of codepoints in a UTF-8 encoded string
#[inline]
pub fn GetCodepointCount(text: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;

/// Get next codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
#[inline]
pub fn GetCodepoint(
    text: *const ::std::os::raw::c_char,
    codepointSize: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int;

/// Get next codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
#[inline]
pub fn GetCodepointNext(
    text: *const ::std::os::raw::c_char,
    codepointSize: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int;

/// Get previous codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
#[inline]
pub fn GetCodepointPrevious(
    text: *const ::std::os::raw::c_char,
    codepointSize: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int;

/// Encode one codepoint into UTF-8 byte array (array length returned as parameter)
#[inline]
pub fn CodepointToUTF8(
    codepoint: ::std::os::raw::c_int,
    utf8Size: *mut ::std::os::raw::c_int,
) -> *const ::std::os::raw::c_char;

// Text strings management functions (no UTF-8 strings, only byte chars)
// WARNING 1: Most of these functions use internal static buffers, it's recommended to store returned data on user-side for re-use
// WARNING 2: Some strings allocate memory internally for the returned strings, those strings must be free by user using MemFree()

/// Copy one string to another, returns bytes copied
#[inline]
pub fn TextCopy(
    dst: *mut ::std::os::raw::c_char,
    src: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int;

/// Check if two text string are equal
#[inline]
pub fn TextIsEqual(
    text1: *const ::std::os::raw::c_char,
    text2: *const ::std::os::raw::c_char,
) -> bool;

/// Get text length, checks for '\0' ending
#[inline]
pub fn TextLength(text: *const ::std::os::raw::c_char) -> ::std::os::raw::c_uint;

/// Text formatting with variables (sprintf() style)
#[inline]
pub fn TextFormat(text: *const ::std::os::raw::c_char, ...) -> *const ::std::os::raw::c_char;

/// Get a piece of a text string
#[inline]
pub fn TextSubtext(
    text: *const ::std::os::raw::c_char,
    position: ::std::os::raw::c_int,
    length: ::std::os::raw::c_int,
) -> *const ::std::os::raw::c_char;

/// Replace text string (WARNING: memory must be freed!)
#[inline]
pub fn TextReplace(
    text: *const ::std::os::raw::c_char,
    replace: *const ::std::os::raw::c_char,
    by: *const ::std::os::raw::c_char,
) -> *mut ::std::os::raw::c_char;

/// Insert text in a position (WARNING: memory must be freed!)
#[inline]
pub fn TextInsert(
    text: *const ::std::os::raw::c_char,
    insert: *const ::std::os::raw::c_char,
    position: ::std::os::raw::c_int,
) -> *mut ::std::os::raw::c_char;

/// Join text strings with delimiter
#[inline]
pub fn TextJoin(
    textList: *mut *mut ::std::os::raw::c_char,
    count: ::std::os::raw::c_int,
    delimiter: *const ::std::os::raw::c_char,
) -> *mut ::std::os::raw::c_char;

/// Split text into multiple strings
#[inline]
pub fn TextSplit(
    text: *const ::std::os::raw::c_char,
    delimiter: ::std::os::raw::c_char,
    count: *mut ::std::os::raw::c_int,
) -> *mut *mut ::std::os::raw::c_char;

/// Append text at specific position and move cursor!
#[inline]
pub fn TextAppend(
    text: *mut ::std::os::raw::c_char,
    append: *const ::std::os::raw::c_char,
    position: *mut ::std::os::raw::c_int,
);

/// Find first text occurrence within a string
#[inline]
pub fn TextFindIndex(
    text: *const ::std::os::raw::c_char,
    find: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int;

/// Get upper case version of provided string
#[inline]
pub fn TextToUpper(text: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;

/// Get lower case version of provided string
#[inline]
pub fn TextToLower(text: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;

/// Get Pascal case notation version of provided string
#[inline]
pub fn TextToPascal(text: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;

/// Get Snake case notation version of provided string
#[inline]
pub fn TextToSnake(text: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;

/// Get Camel case notation version of provided string
#[inline]
pub fn TextToCamel(text: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;


/// Get integer value from text
#[inline]
pub fn TextToInteger(text: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;

/// Get float value from text
#[inline]
pub fn TextToFloat(text: *const ::std::os::raw::c_char) -> ::std::os::raw::c_float;
