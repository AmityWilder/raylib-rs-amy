use super::*;

// Font loading/unloading functions

/// Get the default Font
#[inline]
pub fn get_font_default() -> sys::Font {
    unsafe {
        sys::GetFontDefault()
    }
}

/// Load font from file into GPU memory (VRAM)
#[inline]
pub fn load_font(
    file_name: &CStr,
) -> sys::Font {
    unsafe {
        sys::LoadFont(
            file_name.as_ptr(),
        )
    }
}

/// Load font from file with extended parameters, use NULL for codepoints and 0 for codepointCount to load the default character set, font size is provided in pixels height
#[inline]
pub fn load_font_ex(
    file_name: &CStr,
    font_size: u32,
    codepoints: &[char],
) -> sys::Font {
    unsafe {
        sys::LoadFontEx(
            file_name.as_ptr(),
            font_size.try_into().unwrap(),
            codepoints.as_ptr().cast_mut().cast::<i32>(),
            codepoints.len().try_into().unwrap(),
        )
    }
}

/// Load font from Image (XNA style)
#[inline]
pub fn load_font_from_image(
    image: sys::Image,
    key: sys::Color,
    first_char: char,
) -> sys::Font {
    unsafe {
        sys::LoadFontFromImage(
            image,
            key,
            first_char as i32,
        )
    }
}

/// Load font from memory buffer, fileType refers to extension: i.e. '.ttf'
#[inline]
pub fn load_font_from_memory(
    file_type: &CStr,
    file_data: &[u8],
    font_size: u32,
    codepoints: &[char],
) -> sys::Font {
    unsafe {
        sys::LoadFontFromMemory(
            file_type.as_ptr(),
            file_data.as_ptr(),
            file_data.len().try_into().unwrap(),
            font_size.try_into().unwrap(),
            codepoints.as_ptr().cast_mut().cast::<i32>(),
            codepoints.len().try_into().unwrap(),
        )
    }
}

/// Check if a font is valid (font data loaded, WARNING: GPU texture not checked)
#[inline]
pub fn is_font_valid(
    font: sys::Font,
) -> bool {
    unsafe {
        sys::IsFontValid(
            font,
        )
    }
}

/// Load font data for further use
#[inline]
pub fn load_font_data(
    file_data: &[u8],
    font_size: i32,
    codepoints: &[char],
    type_: i32,
) -> *mut sys::GlyphInfo {
    unsafe {
        sys::LoadFontData(
            file_data.as_ptr(),
            file_data.len().try_into().unwrap(),
            font_size,
            codepoints.as_ptr().cast_mut().cast::<i32>(),
            codepoints.len().try_into().unwrap(),
            type_,
        )
    }
}

pub struct GlyphRecs(NonNull<[sys::Rectangle]>);

impl Deref for GlyphRecs {
    type Target = [sys::Rectangle];

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe {
            self.0.as_ref()
        }
    }
}

impl DerefMut for GlyphRecs {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            self.0.as_mut()
        }
    }
}

impl GlyphRecs {
    unsafe fn new(ptr: *mut sys::Rectangle, len: usize) -> Option<Self> {
        if !ptr.is_null() {
            Some(Self(unsafe {
                NonNull::new_unchecked(std::ptr::slice_from_raw_parts_mut(ptr, len))
            }))
        } else {
            None
        }
    }
}

/// Generate image font atlas using chars info
#[inline]
pub fn gen_image_font_atlas(
    glyphs: &[sys::GlyphInfo],
    font_size: u32,
    padding: i32,
    pack_method: i32,
) -> (sys::Image, Option<GlyphRecs>) {
    let mut glyph_recs = MaybeUninit::uninit();
    unsafe {
        let image = sys::GenImageFontAtlas(
            glyphs.as_ptr(),
            glyph_recs.as_mut_ptr(),
            glyphs.len().try_into().unwrap(),
            font_size.try_into().unwrap(),
            padding,
            pack_method,
        );
        (image,  GlyphRecs::new(glyph_recs.assume_init(), glyphs.len()))
    }
}

/// Unload font chars info data (RAM)
#[inline]
pub fn unload_font_data(
    glyphs: &mut [sys::GlyphInfo],
) {
    unsafe {
        sys::UnloadFontData(
            glyphs.as_mut_ptr(),
            glyphs.len().try_into().unwrap(),
        );
    }
}

/// Unload font from GPU memory (VRAM)
#[inline]
pub fn unload_font(
    font: sys::Font,
) {
    unsafe {
        sys::UnloadFont(
            font,
        );
    }
}

/// Export font as code file, returns true on success
#[inline]
pub fn export_font_as_code(
    font: sys::Font,
    file_name: &CStr,
) -> bool {
    unsafe {
        sys::ExportFontAsCode(
            font,
            file_name.as_ptr(),
        )
    }
}

// Text drawing functions

/// Draw current FPS
#[inline]
pub fn draw_fps(
    pos_x: i32,
    pos_y: i32,
) {
    unsafe {
        sys::DrawFPS(
            pos_x,
            pos_y,
        );
    }
}

/// Draw text (using default font)
#[inline]
pub fn draw_text(
    text: &CStr,
    pos_x: i32,
    pos_y: i32,
    font_size: u32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawText(
            text.as_ptr(),
            pos_x,
            pos_y,
            font_size.try_into().unwrap(),
            color,
        );
    }
}

/// Draw text using font and additional parameters
#[inline]
pub fn draw_text_ex(
    font: sys::Font,
    text: &CStr,
    position: sys::Vector2,
    font_size: f32,
    spacing: f32,
    tint: sys::Color,
) {
    unsafe {
        sys::DrawTextEx(
            font,
            text.as_ptr(),
            position,
            font_size,
            spacing,
            tint,
        );
    }
}

/// Draw text using Font and pro parameters (rotation)
#[inline]
pub fn draw_text_pro(
    font: sys::Font,
    text: &CStr,
    position: sys::Vector2,
    origin: sys::Vector2,
    rotation: f32,
    font_size: f32,
    spacing: f32,
    tint: sys::Color,
) {
    unsafe {
        sys::DrawTextPro(
            font,
            text.as_ptr(),
            position,
            origin,
            rotation,
            font_size,
            spacing,
            tint,
        );
    }
}

/// Draw one character (codepoint)
#[inline]
pub fn draw_text_codepoint(
    font: sys::Font,
    codepoint: i32,
    position: sys::Vector2,
    font_size: f32,
    tint: sys::Color,
) {
    unsafe {
        sys::DrawTextCodepoint(
            font,
            codepoint,
            position,
            font_size,
            tint,
        )
    }
}

/// Draw multiple character (codepoint)
#[inline]
pub fn draw_text_codepoints(
    font: sys::Font,
    codepoints: &[char],
    position: sys::Vector2,
    font_size: f32,
    spacing: f32,
    tint: sys::Color,
) {
    unsafe {
        sys::DrawTextCodepoints(
            font,
            codepoints.as_ptr().cast::<i32>(),
            codepoints.len().try_into().unwrap(),
            position,
            font_size,
            spacing,
            tint,
        )
    }
}

// Text font info functions

/// Set vertical line spacing when drawing with line-breaks
#[inline]
pub fn set_text_line_spacing(
    spacing: i32,
) {
    unsafe {
        sys::SetTextLineSpacing(
            spacing,
        )
    }
}

/// Measure string width for default font
#[inline]
pub fn measure_text(
    text: &CStr,
    font_size: u32,
) -> i32 {
    unsafe {
        sys::MeasureText(
            text.as_ptr(),
            font_size.try_into().unwrap(),
        )
    }
}

/// Measure string size for Font
#[inline]
pub fn measure_text_ex(
    font: sys::Font,
    text: &CStr,
    font_size: f32,
    spacing: f32,
) -> sys::Vector2 {
    unsafe {
        sys::MeasureTextEx(
            font,
            text.as_ptr(),
            font_size,
            spacing,
        )
    }
}

/// Get glyph index position in font for a codepoint (unicode character), fallback to '?' if not found
#[inline]
pub fn get_glyph_index(
    font: sys::Font,
    codepoint: char,
) -> usize {
    unsafe {
        sys::GetGlyphIndex(
            font,
            codepoint as i32,
        ).try_into().unwrap()
    }
}

/// Get glyph font info data for a codepoint (unicode character), fallback to '?' if not found
#[inline]
pub fn get_glyph_info(
    font: sys::Font,
    codepoint: char,
) -> sys::GlyphInfo {
    unsafe {
        sys::GetGlyphInfo(
            font,
            codepoint as i32,
        )
    }
}

/// Get glyph rectangle in font atlas for a codepoint (unicode character), fallback to '?' if not found
#[inline]
pub fn get_glyph_atlas_rec(
    font: sys::Font,
    codepoint: char,
) -> sys::Rectangle {
    unsafe {
        sys::GetGlyphAtlasRec(
            font,
            codepoint as i32,
        )
    }
}

pub struct LoadUTF8Allocator;

impl RlAllocator<str> for LoadUTF8Allocator {
    unsafe fn unload(&mut self, mut data: NonNull<str>) {
        unsafe {
            sys::UnloadUTF8(
                data.as_mut().as_bytes_mut().as_mut_ptr().cast(),
            )
        }
    }
}

// Text codepoints management functions (unicode characters)

/// Load UTF-8 text encoded from codepoints array
///
/// WARNING: Allocated memory must be manually freed
#[inline]
pub fn load_utf8(
    codepoints: &[char],
) -> Option<RlString<LoadUTF8Allocator>> {
    unsafe {
        let ptr = sys::LoadUTF8(
            codepoints.as_ptr().cast::<i32>(),
            codepoints.len().try_into().unwrap(),
        );
        let len = CStr::from_ptr(ptr).count_bytes();
        RlString::new(
            ptr,
            move || len,
            LoadUTF8Allocator,
        )
    }
}

/// Unload UTF-8 text encoded from codepoints array
#[inline]
pub fn unload_utf8(
    text: RlString<LoadUTF8Allocator>,
) {
    drop(text);
}

pub struct LoadCodepointsAllocator;

impl RlAllocator<[char]> for LoadCodepointsAllocator {
    unsafe fn unload(&mut self, mut data: NonNull<[char]>) {
        unsafe {
            sys::UnloadCodepoints(
                data.as_mut().as_mut_ptr().cast::<i32>(),
            );
        }
    }
}

/// Load all codepoints from a UTF-8 text string, codepoints count returned by parameter
///
/// NOTE: Despite being a Rust `&str`, the `text` parameter must be nul-terminated
#[inline]
pub fn load_codepoints(
    text: &str,
) -> Option<RlCodepoints<LoadCodepointsAllocator>> {
    assert!(text.ends_with('\0'), "must be nul-terminated");
    let mut count = MaybeUninit::uninit();
    unsafe {
        let ptr = sys::LoadCodepoints(
            text.as_ptr().cast::<c_char>(),
            count.as_mut_ptr(),
        );
        RlCodepoints::new(
            ptr.cast::<char>(),
            move || count.assume_init().try_into().unwrap(),
            LoadCodepointsAllocator,
        )
    }
}

/// Unload codepoints data from memory
#[inline]
pub fn unload_codepoints(
    codepoints: RlCodepoints<LoadCodepointsAllocator>,
) {
    drop(codepoints);
}

/// Get total number of codepoints in a UTF-8 encoded string
#[inline]
pub fn get_codepoint_count(
    text: &CStr,
) -> usize {
    unsafe {
        sys::GetCodepointCount(
            text.as_ptr(),
        ).try_into().unwrap()
    }
}

/// Get next codepoint in a UTF-8 encoded text, scanning until '\0' is found
/// When an invalid UTF-8 byte is encountered we exit as soon as possible and a '?'(0x3f) codepoint is returned
/// Total number of bytes processed are returned as a parameter
///
/// NOTE: The standard says U+FFFD should be returned in case of errors
/// but that character is not supported by the default font in raylib
#[inline]
pub fn get_codepoint(
    text: &CStr,
) -> (char, usize) {
    let mut codepoint_size = MaybeUninit::uninit();
    unsafe {
        let ch = sys::GetCodepoint(
            text.as_ptr(),
            codepoint_size.as_mut_ptr(),
        ).cast_unsigned();
        (char::from_u32(ch).unwrap(), codepoint_size.assume_init().try_into().unwrap())
    }
}

/// Get next codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
#[inline]
pub fn get_codepoint_next(
    text: &CStr,
) -> (char, usize) {
    let mut codepoint_size = MaybeUninit::uninit();
    unsafe {
        let ch = sys::GetCodepointNext(
            text.as_ptr(),
            codepoint_size.as_mut_ptr(),
        ).cast_unsigned();
        (char::from_u32(ch).unwrap(), codepoint_size.assume_init().try_into().unwrap())
    }
}

/// Get previous codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
#[inline]
pub fn get_codepoint_previous(
    text: &CStr,
) -> (char, usize) {
    let mut codepoint_size = MaybeUninit::uninit();
    unsafe {
        let ch = sys::GetCodepointPrevious(
            text.as_ptr(),
            codepoint_size.as_mut_ptr(),
        ).cast_unsigned();
        (char::from_u32(ch).unwrap(), codepoint_size.assume_init().try_into().unwrap())
    }
}

/// Encode one codepoint into UTF-8 byte array (array length returned as parameter)
///
/// NOTE: It uses a static array to store UTF-8 bytes
#[inline]
pub fn codepoint_to_utf8(
    codepoint: char,
) -> ([u8; 4], usize) {
    let mut utf8_size = MaybeUninit::uninit();
    unsafe {
        let ptr = sys::CodepointToUTF8(
            codepoint as i32,
            utf8_size.as_mut_ptr(),
        );
        let arr = *ptr.cast::<[c_char; 6]>().cast::<[u8; 4]>();
        (arr, utf8_size.assume_init().try_into().unwrap())
    }
}

// Text strings management functions (no UTF-8 strings, only byte chars)
// WARNING 1: Most of these functions use internal static buffers, it's recommended to store returned data on user-side for re-use
// WARNING 2: Some strings allocate memory internally for the returned strings, those strings must be free by user using MemFree()

/// Copy one string to another, returns bytes copied
#[inline]
pub fn text_copy(
    dst: &mut CStr,
    src: &CStr,
) -> usize {
    assert!(dst.count_bytes() >= src.count_bytes(), "cannot copy text, buffer would overrun");
    unsafe {
        sys::TextCopy(
            dst.as_ptr().cast_mut(),
            src.as_ptr(),
        ).try_into().unwrap()
    }
}

/// Check if two text string are equal
#[inline]
pub fn text_is_equal(
    text1: &CStr,
    text2: &CStr,
) -> bool {
    unsafe {
        sys::TextIsEqual(
            text1.as_ptr(),
            text2.as_ptr(),
        )
    }
}

/// Get text length, checks for '\0' ending
#[inline]
pub fn text_length(
    text: &CStr,
) -> u32 {
    unsafe {
        sys::TextLength(
            text.as_ptr(),
        )
    }
}

// variadic does not convert to rust
// /// Text formatting with variables (sprintf() style)
// #[inline]
// pub fn TextFormat(
//     text: *const ::std::os::raw::c_char,
//     ...
// ) -> *const ::std::os::raw::c_char;

define_buffer_handle!(TextSubtextHandle);

impl TextSubtextHandle {
    /// Get a piece of a text string
    #[inline]
    pub fn text_subtext<'a>(
        &'a mut self,
        text: &CStr,
        position: usize,
        length: usize,
    ) -> &'a CStr {
        unsafe {
            CStr::from_ptr(sys::TextSubtext(
                text.as_ptr(),
                position.try_into().unwrap(),
                length.try_into().unwrap(),
            ))
        }
    }
}

/// Replace text string (WARNING: memory must be freed!)
#[inline]
pub fn text_replace(
    text: &CStr,
    replace: &CStr,
    by: &CStr,
) -> Option<RlCString> {
    unsafe {
        RlCString::new(
            sys::TextReplace(
                text.as_ptr(),
                replace.as_ptr(),
                by.as_ptr(),
            ),
            MemAllocator,
        )
    }
}

/// Insert text in a position (WARNING: memory must be freed!)
#[inline]
pub fn text_insert(
    text: &CStr,
    insert: &CStr,
    position: usize,
) -> Option<RlCString> {
    unsafe {
        RlCString::new(
            sys::TextInsert(
                text.as_ptr(),
                insert.as_ptr(),
                position.try_into().unwrap(),
            ),
            MemAllocator,
        )
    }
}

define_buffer_handle!(TextJoinHandle);

impl TextJoinHandle {
    /// Join text strings with delimiter
    #[inline]
    pub fn text_join<'a>(
        &'a mut self,
        text_list: &[&c_char],
        count: i32,
        delimiter: &CStr,
    ) -> &'a CStr {
        unsafe {
            CStr::from_ptr(sys::TextJoin(
                text_list.as_ptr().cast_mut().cast(),
                count,
                delimiter.as_ptr(),
            ))
        }
    }
}

define_buffer_handle!(TextSplitHandle);

impl TextSplitHandle {
    /// Split text into multiple strings
    ///
    /// NOTE: Current implementation returns a copy of the provided string with '\0' (string end delimiter)
    /// inserted between strings defined by "delimiter" parameter. No memory is dynamically allocated,
    /// all used memory is static... it has some limitations:
    ///      1. Maximum number of possible split strings is set by MAX_TEXTSPLIT_COUNT
    ///      2. Maximum size of text to split is MAX_TEXT_BUFFER_LENGTH
    #[inline]
    pub fn text_split<'a>(
        &'a mut self,
        text: &CStr,
        delimiter: c_char,
    ) -> &'a [*mut c_char] {
        let mut count = MaybeUninit::uninit();
        unsafe {
            let ptr = sys::TextSplit(
                text.as_ptr(),
                delimiter,
                count.as_mut_ptr(),
            );
            std::slice::from_raw_parts(ptr, count.assume_init().try_into().unwrap())
        }
    }
}

/// Append text at specific position and move cursor
///
/// WARNING: It's up to the user to make sure appended text does not overflow the buffer!
#[inline]
pub fn text_append<'a>(
    text: &'a mut [c_char],
    append: &CStr,
    position: &mut usize,
) -> Result<&'a CStr, std::ffi::FromBytesUntilNulError> {
    assert!(text.len() >= append.count_bytes() + *position);
    let mut temp = (*position).try_into().unwrap();
    unsafe {
        sys::TextAppend(
            text.as_ptr().cast_mut(),
            append.as_ptr(),
            &mut temp,
        );
    }
    *position = temp.try_into().unwrap();
    CStr::from_bytes_until_nul(unsafe {
        transmute::<&'a mut [c_char], &'a [u8]>(text)
    })
}

/// Find first text occurrence within a string
#[inline]
pub fn text_find_index(
    text: &CStr,
    find: &CStr,
) -> Result<usize, ()> {
    const _: () = assert!(std::mem::size_of::<*const c_char>() <= std::mem::size_of::<usize>(),
        "cannot reliably confirm CStr will not overflow by testing its length",
    );

    if std::mem::size_of::<usize>() > std::mem::size_of::<c_int>() {
        const INT_MAX: usize = c_int::MAX as usize;

        assert!(text.count_bytes() <= INT_MAX,
            "`text` exceeds {INT_MAX} bytes. \
            it is possible for the first instance of `find` to be at a position that can be stored in a pointer difference, but would overflow when cast to int. \
            it is also possible that the value might be large enough to wrap all the way back into the positives, making the output of TextFindIndex unreliable.");
    }

    let pos = unsafe {
        sys::TextFindIndex(
            text.as_ptr(),
            find.as_ptr(),
        )
    };
    match pos {
        0.. => Ok((pos as u32).try_into().unwrap()), // safe to assume positive i32 is compatible with u32
        -1 => Err(()),
        _ => unreachable!(),
    }
}

define_buffer_handle!(TextToUpperHandle);

impl TextToUpperHandle {
    /// Get upper case version of provided string
    ///
    /// WARNING: Limited functionality, only basic characters set
    /// TODO: Support UTF-8 diacritics to upper-case, check codepoints
    #[inline]
    pub fn text_to_upper<'a>(
        &'a mut self,
        text: &CStr,
    ) -> &'a CStr {
        unsafe {
            CStr::from_ptr(sys::TextToUpper(
                text.as_ptr(),
            ))
        }
    }
}

define_buffer_handle!(TextToLowerHandle);

impl TextToLowerHandle {
    /// Get lower case version of provided string
    ///
    /// WARNING: Limited functionality, only basic characters set
    #[inline]
    pub fn text_to_lower<'a>(
        &'a mut self,
        text: &CStr,
    ) -> &'a CStr {
        unsafe {
            CStr::from_ptr(sys::TextToLower(
                text.as_ptr(),
            ))
        }
    }
}

define_buffer_handle!(TextToPascalHandle);

impl TextToPascalHandle {
    /// Get Pascal case notation version of provided string
    ///
    /// WARNING: Limited functionality, only basic characters set
    #[inline]
    pub fn text_to_pascal<'a>(
        &'a mut self,
        text: &CStr,
    ) -> &'a CStr {
        unsafe {
            CStr::from_ptr(sys::TextToPascal(
                text.as_ptr(),
            ))
        }
    }
}

define_buffer_handle!(TextToSnakeHandle);

impl TextToSnakeHandle {
    /// Get Snake case notation version of provided string
    ///
    /// WARNING: Limited functionality, only basic characters set
    #[inline]
    pub fn text_to_snake<'a>(
        &'a mut self,
        text: &CStr,
    ) -> &'a CStr {
        unsafe {
            CStr::from_ptr(sys::TextToSnake(
                text.as_ptr(),
            ))
        }
    }
}

define_buffer_handle!(TextToCamelHandle);

impl TextToCamelHandle {
    /// Get Camel case notation version of provided string
    ///
    /// WARNING: Limited functionality, only basic characters set
    #[inline]
    pub fn text_to_camel<'a>(
        &'a mut self,
        text: &CStr,
    ) -> &'a CStr {
        unsafe {
            CStr::from_ptr(sys::TextToCamel(
                text.as_ptr(),
            ))
        }
    }
}

/// Get integer value from text
#[inline]
pub fn text_to_integer(
    text: &CStr,
) -> i32 {
    unsafe {
        sys::TextToInteger(
            text.as_ptr(),
        )
    }
}

/// Get float value from text
///
/// WARNING: Only '.' character is understood as decimal point
#[inline]
pub fn text_to_float(
    text: &CStr,
) -> f32 {
    unsafe {
        sys::TextToFloat(
            text.as_ptr(),
        )
    }
}
