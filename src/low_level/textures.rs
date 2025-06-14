use std::num::NonZeroI32;

use super::*;

// Image loading functions
// NOTE: These functions do not require GPU access

/// Load image from file into CPU memory (RAM)
#[inline]
pub fn load_image(
    file_name: &CStr,
) -> sys::Image {
    unsafe {
        sys::LoadImage(
            file_name.as_ptr(),
        )
    }
}

/// Load image from RAW file data
#[inline]
pub fn load_image_raw(
    file_name: &CStr,
    width: u32,
    height: u32,
    format: u32,
    header_size: u32,
) -> sys::Image {
    unsafe {
        sys::LoadImageRaw(
            file_name.as_ptr(),
            width.try_into().unwrap(),
            height.try_into().unwrap(),
            format.try_into().unwrap(),
            header_size.try_into().unwrap(),
        )
    }
}

/// Load image sequence from file (frames appended to image.data)
///  - Image.data buffer includes all frames: `[image#0][image#1][image#2][...]`
///  - Number of frames is returned through 'frames' parameter
///  - All frames are returned in RGBA format
///  - Frames delay data is discarded
#[inline]
pub fn load_image_anim(
    file_name: &CStr,
) -> (sys::Image, usize) {
    let mut frames = MaybeUninit::uninit(); // LoadImageAnim will assign with 0
    unsafe {
        let image = sys::LoadImageAnim(
            file_name.as_ptr(),
            frames.as_mut_ptr(),
        );
        (image, frames.assume_init().try_into().unwrap())
    }
}

/// Load image sequence from memory buffer
//  - Image.data buffer includes all frames: `[image#0][image#1][image#2][...]`
//  - Number of frames is returned through 'frames' parameter
//  - All frames are returned in RGBA format
//  - Frames delay data is discarded
#[inline]
pub fn load_image_anim_from_memory(
    file_type: &CStr,
    file_data: &[u8],
) -> (sys::Image, usize) {
    let mut frames = MaybeUninit::uninit(); // LoadImageAnimFromMemory will assign with 0
    unsafe {
        let image = sys::LoadImageAnimFromMemory(
            file_type.as_ptr(),
            file_data.as_ptr(),
            file_data.len().try_into().unwrap(),
            frames.as_mut_ptr(),
        );
        (image, frames.assume_init().try_into().unwrap())
    }
}

/// Load image from memory buffer, fileType refers to extension: i.e. '.png'
/// WARNING: File extension must be provided in lower-case
#[inline]
pub fn load_image_from_memory(
    file_type: &CStr,
    file_data: &[u8],
) -> sys::Image {
    unsafe {
        sys::LoadImageFromMemory(
            file_type.as_ptr(),
            file_data.as_ptr(),
            file_data.len().try_into().unwrap(),
        )
    }
}

/// Load image from GPU texture data
/// NOTE: Compressed texture formats not supported
#[inline]
pub fn load_image_from_texture(
    texture: sys::Texture2D,
) -> sys::Image {
    unsafe {
        sys::LoadImageFromTexture(
            texture,
        )
    }
}

/// Load image from screen buffer and (screenshot)
#[inline]
pub fn load_image_from_screen() -> sys::Image {
    unsafe {
        sys::LoadImageFromScreen()
    }
}

/// Check if an image is valid (data and parameters)
#[inline]
pub fn is_image_valid(
    image: sys::Image,
) -> bool {
    unsafe {
        sys::IsImageValid(
            image,
        )
    }
}

/// Unload image from CPU memory (RAM)
#[inline]
pub fn unload_image(
    image: sys::Image,
) {
    unsafe {
        sys::UnloadImage(
            image,
        );
    }
}

/// Export image data to file, returns true on success
/// NOTE: File format depends on file_name extension
#[inline]
pub fn export_image(
    image: sys::Image,
    file_name: &CStr,
) -> bool {
    unsafe {
        sys::ExportImage(
            image,
            file_name.as_ptr(),
        )
    }
}

/// Export image to memory buffer
#[inline]
pub fn export_image_to_memory(
    image: sys::Image,
    file_type: &CStr,
) -> Option<RlBuffer<[u8], MemAllocator>> {
    let mut file_size = MaybeUninit::uninit();
    unsafe {
        let ptr = sys::ExportImageToMemory(
            image,
            file_type.as_ptr(),
            file_size.as_mut_ptr(),
        );
        RlBuffer::<[u8]>::new(
            ptr,
            move || file_size.assume_init().try_into().unwrap(),
            MemAllocator,
        )
    }
}

/// Export image as code file (.h) defining an array of bytes
#[inline]
pub fn export_image_as_code(
    image: sys::Image,
    file_name: &CStr,
) -> Result<(), ()> {
    match unsafe {
        sys::ExportImageAsCode(
            image,
            file_name.as_ptr(),
        )
    } {
        true => Ok(()),
        false => Err(()),
    }
}

// Image generation functions

/// Generate image: plain color
#[inline]
pub fn gen_image_color(
    width: u32,
    height: u32,
    color: sys::Color,
) -> sys::Image {
    unsafe {
        sys::GenImageColor(
            width.try_into().unwrap(),
            height.try_into().unwrap(),
            color,
        )
    }
}

/// Generate image: linear gradient, direction in degrees [0..360], 0=Vertical gradient
#[inline]
pub fn gen_image_gradient_linear(
    width: u32,
    height: u32,
    direction: i32,
    start: sys::Color,
    end: sys::Color,
) -> sys::Image {
    unsafe {
        sys::GenImageGradientLinear(
            width.try_into().unwrap(),
            height.try_into().unwrap(),
            direction,
            start,
            end,
        )
    }
}

/// Generate image: radial gradient
#[inline]
pub fn gen_image_gradient_radial(
    width: u32,
    height: u32,
    density: f32,
    inner: sys::Color,
    outer: sys::Color,
) -> sys::Image {
    unsafe {
        sys::GenImageGradientRadial(
            width.try_into().unwrap(),
            height.try_into().unwrap(),
            density,
            inner,
            outer,
        )
    }
}

/// Generate image: square gradient
#[inline]
pub fn gen_image_gradient_square(
    width: u32,
    height: u32,
    density: f32,
    inner: sys::Color,
    outer: sys::Color,
) -> sys::Image {
    unsafe {
        sys::GenImageGradientSquare(
            width.try_into().unwrap(),
            height.try_into().unwrap(),
            density,
            inner,
            outer,
        )
    }
}

/// Generate image: checked
#[inline]
pub fn gen_image_checked(
    width: u32,
    height: u32,
    checks_x: u32,
    checks_y: u32,
    col1: sys::Color,
    col2: sys::Color,
) -> sys::Image {
    unsafe {
        sys::GenImageChecked(
            width.try_into().unwrap(),
            height.try_into().unwrap(),
            checks_x.try_into().unwrap(),
            checks_y.try_into().unwrap(),
            col1,
            col2,
        )
    }
}

/// Generate image: white noise
#[inline]
pub fn gen_image_white_noise(
    width: u32,
    height: u32,
    factor: f32,
) -> sys::Image {
    unsafe {
        sys::GenImageWhiteNoise(
            width.try_into().unwrap(),
            height.try_into().unwrap(),
            factor,
        )
    }
}

/// Generate image: perlin noise
#[inline]
pub fn gen_image_perlin_noise(
    width: u32,
    height: u32,
    offset_x: i32,
    offset_y: i32,
    scale: f32,
) -> sys::Image {
    unsafe {
        sys::GenImagePerlinNoise(
            width.try_into().unwrap(),
            height.try_into().unwrap(),
            offset_x,
            offset_y,
            scale,
        )
    }
}

/// Generate image: cellular algorithm, bigger tile_size means bigger cells
#[inline]
pub fn gen_image_cellular(
    width: u32,
    height: u32,
    tile_size: NonZeroI32, // this value is used to divide other numbers
) -> sys::Image {
    unsafe {
        sys::GenImageCellular(
            width.try_into().unwrap(),
            height.try_into().unwrap(),
            tile_size.get(),
        )
    }
}

/// Generate image: grayscale image from text data
#[inline]
pub fn gen_image_text(
    width: u32,
    height: u32,
    text: &CStr,
) -> sys::Image {
    unsafe {
        sys::GenImageText(
            width.try_into().unwrap(),
            height.try_into().unwrap(),
            text.as_ptr(),
        )
    }
}

// Image manipulation functions

/// Create an image duplicate (useful for transformations)
#[inline]
pub fn image_copy(
    image: sys::Image,
) -> sys::Image {
    unsafe {
        sys::ImageCopy(
            image,
        )
    }
}

/// Create an image from another image piece
#[inline]
pub fn image_from_image(
    image: sys::Image,
    rec: sys::Rectangle,
) -> sys::Image {
    unsafe {
        sys::ImageFromImage(
            image,
            rec,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum ColorChannel {
    #[default]
    Red   = 0,
    Green = 1,
    Blue  = 2,
    Alpha = 3,
}

/// Create an image from a selected channel of another image (GRAYSCALE)
#[inline]
pub fn image_from_channel(
    image: sys::Image,
    selected_channel: ColorChannel,
) -> sys::Image {
    unsafe {
        sys::ImageFromChannel(
            image,
            selected_channel as i32,
        )
    }
}

/// Create an image from text (default font)
#[inline]
pub fn image_text(
    text: &CStr,
    font_size: u32,
    color: sys::Color,
) -> sys::Image {
    unsafe {
        sys::ImageText(
            text.as_ptr(),
            font_size.try_into().unwrap(),
            color,
        )
    }
}

/// Create an image from text (custom sprite font)
#[inline]
pub fn image_text_ex(
    font: sys::Font,
    text: &CStr,
    font_size: f32,
    spacing: f32,
    tint: sys::Color,
) -> sys::Image {
    unsafe {
        sys::ImageTextEx(
            font,
            text.as_ptr(),
            font_size,
            spacing,
            tint,
        )
    }
}

/// Convert image data to desired format
#[inline]
pub fn image_format(
    image: &mut sys::Image,
    new_format: sys::PixelFormat,
) {
    unsafe {
        sys::ImageFormat(
            image,
            new_format as i32,
        );
    }
}

/// Convert image to POT (power-of-two)
#[inline]
pub fn image_to_pot(
    image: &mut sys::Image,
    fill: sys::Color,
) {
    unsafe {
        sys::ImageToPOT(
            image,
            fill,
        );
    }
}

/// Crop an image to a defined rectangle
#[inline]
pub fn image_crop(
    image: &mut sys::Image,
    crop: sys::Rectangle,
) {
    unsafe {
        sys::ImageCrop(
            image,
            crop,
        );
    }
}

/// Crop image depending on alpha value
#[inline]
pub fn image_alpha_crop(
    image: &mut sys::Image,
    threshold: f32,
) {
    unsafe {
        sys::ImageAlphaCrop(
            image,
            threshold
        );
    }
}

/// Clear alpha channel to desired color
#[inline]
pub fn image_alpha_clear(
    image: &mut sys::Image,
    color: sys::Color,
    threshold: f32,
) {
    unsafe {
        sys::ImageAlphaClear(
            image,
            color,
            threshold,
        );
    }
}

/// Apply alpha mask to image
#[inline]
pub fn image_alpha_mask(
    image: &mut sys::Image,
    alpha_mask: sys::Image,
) {
    unsafe {
        sys::ImageAlphaMask(
            image,
            alpha_mask,
        )
    }
}

/// Premultiply alpha channel
#[inline]
pub fn image_alpha_premultiply(
    image: &mut sys::Image,
) {
    unsafe {
        sys::ImageAlphaPremultiply(
            image,
        );
    }
}

/// Apply Gaussian blur using a box blur approximation
#[inline]
pub fn image_blur_gaussian(
    image: &mut sys::Image,
    blur_size: u32,
) {
    unsafe {
        sys::ImageBlurGaussian(
            image,
            blur_size.try_into().unwrap(),
        )
    }
}

/// Apply custom square convolution kernel to image
#[inline]
pub fn image_kernel_convolution(
    image: &mut sys::Image,
    kernel: &[f32],
) {
    unsafe {
        sys::ImageKernelConvolution(
            image,
            kernel.as_ptr(),
            kernel.len().try_into().unwrap(),
        );
    }
}

/// Resize image (Bicubic scaling algorithm)
#[inline]
pub fn image_resize(
    image: &mut sys::Image,
    new_width: u32,
    new_height: u32,
) {
    unsafe {
        sys::ImageResize(
            image,
            new_width.try_into().unwrap(),
            new_height.try_into().unwrap(),
        );
    }
}

/// Resize image (Nearest-Neighbor scaling algorithm)
#[inline]
pub fn image_resize_nn(
    image: &mut sys::Image,
    new_width: u32,
    new_height: u32,
) {
    unsafe {
        sys::ImageResizeNN(
            image,
            new_width.try_into().unwrap(),
            new_height.try_into().unwrap(),
        );
    }
}

/// Resize canvas and fill with color
#[inline]
pub fn image_resize_canvas(
    image: &mut sys::Image,
    new_width: u32,
    new_height: u32,
    offset_x: i32,
    offset_y: i32,
    fill: sys::Color,
) {
    unsafe {
        sys::ImageResizeCanvas(
            image,
            new_width.try_into().unwrap(),
            new_height.try_into().unwrap(),
            offset_x,
            offset_y,
            fill,
        );
    }
}

/// Compute all mipmap levels for a provided image
#[inline]
pub fn image_mipmaps(
    image: &mut sys::Image,
) {
    unsafe {
        sys::ImageMipmaps(
            image,
        );
    }
}

/// Dither image data to 16bpp or lower (Floyd-Steinberg dithering)
/// NOTE: In case selected bpp do not represent a known 16bit format,
/// dithered data is stored in the LSB part of the unsigned short
#[inline]
pub fn image_dither(
    image: &mut sys::Image,
    r_bpp: u8,
    g_bpp: u8,
    b_bpp: u8,
    a_bpp: u8,
) {
    unsafe {
        sys::ImageDither(
            image,
            r_bpp.into(),
            g_bpp.into(),
            b_bpp.into(),
            a_bpp.into(),
        );
    }
}

/// Flip image vertically
#[inline]
pub fn image_flip_vertical(
    image: &mut sys::Image,
) {
    unsafe {
        sys::ImageFlipVertical(
            image,
        );
    }
}

/// Flip image horizontally
#[inline]
pub fn image_flip_horizontal(
    image: &mut sys::Image,
) {
    unsafe {
        sys::ImageFlipHorizontal(
            image,
        );
    }
}

/// Rotate image by input angle in degrees (-359 to 359)
#[inline]
pub fn image_rotate(
    image: &mut sys::Image,
    degrees: i32,
) {
    unsafe {
        sys::ImageRotate(
            image,
            degrees,
        );
    }
}

/// Rotate image clockwise 90deg
#[inline]
pub fn image_rotate_cw(
    image: &mut sys::Image,
) {
    unsafe {
        sys::ImageRotateCW(
            image,
        );
    }
}

/// Rotate image counter-clockwise 90deg
#[inline]
pub fn image_rotate_ccw(
    image: &mut sys::Image,
) {
    unsafe {
        sys::ImageRotateCCW(
            image,
        );
    }
}

/// Modify image color: tint
#[inline]
pub fn image_color_tint(
    image: &mut sys::Image,
    color: sys::Color,
) {
    unsafe {
        sys::ImageColorTint(
            image,
            color,
        );
    }
}

/// Modify image color: invert
#[inline]
pub fn image_color_invert(
    image: &mut sys::Image,
) {
    unsafe {
        sys::ImageColorInvert(
            image,
        );
    }
}

/// Modify image color: grayscale
#[inline]
pub fn image_color_grayscale(
    image: &mut sys::Image,
) {
    unsafe {
        sys::ImageColorGrayscale(
            image,
        );
    }
}

/// Modify image color: contrast (-100 to 100)
#[inline]
pub fn image_color_contrast(
    image: &mut sys::Image,
    contrast: f32,
) {
    unsafe {
        sys::ImageColorContrast(
            image,
            contrast,
        );
    }
}

/// Modify image color: brightness (-255 to 255)
#[inline]
pub fn image_color_brightness(
    image: &mut sys::Image,
    brightness: i32,
) {
    unsafe {
        sys::ImageColorBrightness(
            image,
            brightness,
        );
    }
}

/// Modify image color: replace color
#[inline]
pub fn image_color_replace(
    image: &mut sys::Image,
    color: sys::Color,
    replace: sys::Color,
) {
    unsafe {
        sys::ImageColorReplace(
            image,
            color,
            replace,
        );
    }
}

/// An owned slice of colors that must be freed manually with [`unload_image_colors()`]
pub struct ImageColors(NonNull<[sys::Color]>);

impl Deref for ImageColors {
    type Target = [sys::Color];

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe {
            self.0.as_ref()
        }
    }
}

impl DerefMut for ImageColors {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            self.0.as_mut()
        }
    }
}

impl ImageColors {
    unsafe fn new(data: *mut sys::Color, len: usize) -> Option<Self> {
        if !data.is_null() {
            Some(Self(unsafe {
                NonNull::new_unchecked(std::ptr::slice_from_raw_parts_mut(data, len))
            }))
        } else {
            None
        }
    }
}

/// Load color data from image as a Color array (RGBA - 32bit)
/// NOTE: Memory allocated should be freed using [`unload_image_colors()`];
#[inline]
pub fn load_image_colors(
    image: sys::Image,
) -> Option<ImageColors> {
    unsafe {
        ImageColors::new(
            sys::LoadImageColors(
                image,
            ),
            (image.width*image.height).try_into().unwrap(),
        )
    }
}

/// An owned slice of colors that must be freed manually with [`unload_image_palette()`]
pub struct ImagePalette(NonNull<[sys::Color]>);

impl Deref for ImagePalette {
    type Target = [sys::Color];

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe {
            self.0.as_ref()
        }
    }
}

impl DerefMut for ImagePalette {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            self.0.as_mut()
        }
    }
}

impl ImagePalette {
    unsafe fn new(data: *mut sys::Color, len: MaybeUninit<c_int>) -> Option<Self> {
        if !data.is_null() {
            Some(Self(unsafe {
                NonNull::new_unchecked(std::ptr::slice_from_raw_parts_mut(data, len.assume_init().try_into().unwrap()))
            }))
        } else {
            None
        }
    }
}

/// Load colors palette from image as a Color array (RGBA - 32bit)
/// NOTE: Memory allocated should be freed using [`unload_image_palette()`]
#[inline]
pub fn load_image_palette(
    image: sys::Image,
    max_palette_size: usize,
) -> Option<ImagePalette> {
    let mut color_count = MaybeUninit::uninit();
    unsafe {
        let ptr = sys::LoadImagePalette(
            image,
            max_palette_size.try_into().unwrap(),
            color_count.as_mut_ptr(),
        );
        ImagePalette::new(
            ptr,
            color_count,
        )
    }
}

/// Unload color data loaded with [`load_image_colors()`]
#[inline]
pub fn unload_image_colors(
    colors: ImageColors,
) {
    unsafe {
        sys::UnloadImageColors(
            colors.0.as_ptr().cast(),
        );
    }
}

/// Unload colors palette loaded with [`load_image_palette()`]
#[inline]
pub fn unload_image_palette(
    colors: ImagePalette,
) {
    unsafe {
        sys::UnloadImagePalette(
            colors.0.as_ptr().cast(),
        );
    }
}

/// Get image alpha border rectangle
#[inline]
pub fn get_image_alpha_border(
    image: sys::Image,
    threshold: f32,
) -> sys::Rectangle {
    unsafe {
        sys::GetImageAlphaBorder(
            image,
            threshold,
        )
    }
}

/// Get image pixel color at (x, y) position
#[inline]
pub fn get_image_color(
    image: sys::Image,
    x: i32,
    y: i32,
) -> sys::Color {
    unsafe {
        sys::GetImageColor(
            image,
            x,
            y,
        )
    }
}

// Image drawing functions
// NOTE: Image software-rendering functions (CPU)

/// Clear image background with given color
#[inline]
pub fn image_clear_background(
    dst: &mut sys::Image,
    color: sys::Color,
) {
    unsafe {
        sys::ImageClearBackground(
            dst,
            color,
        );
    }
}

/// Draw pixel within an image
#[inline]
pub fn image_draw_pixel(
    dst: &mut sys::Image,
    pos_x: i32,
    pos_y: i32,
    color: sys::Color,
) {
    unsafe {
        sys::ImageDrawPixel(
            dst,
            pos_x,
            pos_y,
            color,
        );
    }
}

/// Draw pixel within an image (Vector version)
#[inline]
pub fn image_draw_pixel_v(
    dst: &mut sys::Image,
    position: sys::Vector2,
    color: sys::Color,
) {
    unsafe {
        sys::ImageDrawPixelV(
            dst,
            position,
            color,
        );
    }
}

/// Draw line within an image
#[inline]
pub fn image_draw_line(
    dst: &mut sys::Image,
    start_pos_x: i32,
    start_pos_y: i32,
    end_pos_x: i32,
    end_pos_y: i32,
    color: sys::Color,
) {
    unsafe {
        sys::ImageDrawLine(
            dst,
            start_pos_x,
            start_pos_y,
            end_pos_x,
            end_pos_y,
            color,
        );
    }
}

/// Draw line within an image (Vector version)
#[inline]
pub fn image_draw_line_v(
    dst: &mut sys::Image,
    start: sys::Vector2,
    end: sys::Vector2,
    color: sys::Color,
) {
    unsafe {
        sys::ImageDrawLineV(
            dst,
            start,
            end,
            color,
        );
    }
}

/// Draw a line defining thickness within an image
#[inline]
pub fn image_draw_line_ex(
    dst: &mut sys::Image,
    start: sys::Vector2,
    end: sys::Vector2,
    thick: i32,
    color: sys::Color,
) {
    unsafe {
        sys::ImageDrawLineEx(
            dst,
            start,
            end,
            thick,
            color,
        );
    }
}

/// Draw a filled circle within an image
#[inline]
pub fn image_draw_circle(
    dst: &mut sys::Image,
    center_x: i32,
    center_y: i32,
    radius: i32,
    color: sys::Color,
) {
    unsafe {
        sys::ImageDrawCircle(
            dst,
            center_x,
            center_y,
            radius,
            color,
        );
    }
}

/// Draw a filled circle within an image (Vector version)
#[inline]
pub fn image_draw_circle_v(
    dst: &mut sys::Image,
    center: sys::Vector2,
    radius: i32,
    color: sys::Color,
) {
    unsafe {
        sys::ImageDrawCircleV(
            dst,
            center,
            radius,
            color,
        );
    }
}

/// Draw circle outline within an image
#[inline]
pub fn image_draw_circle_lines(
    dst: &mut sys::Image,
    center_x: i32,
    center_y: i32,
    radius: i32,
    color: sys::Color,
) {
    unsafe {
        sys::ImageDrawCircleLines(
            dst,
            center_x,
            center_y,
            radius,
            color,
        );
    }
}

/// Draw circle outline within an image (Vector version)
#[inline]
pub fn image_draw_circle_lines_v(
    dst: *mut sys::Image,
    center: sys::Vector2,
    radius: i32,
    color: sys::Color,
) {
    unsafe {
        sys::ImageDrawCircleLinesV(
            dst,
            center,
            radius,
            color,
        );
    }
}

/// Draw rectangle within an image
#[inline]
pub fn image_draw_rectangle(
    dst: *mut sys::Image,
    pos_x: i32,
    pos_y: i32,
    width: i32,
    height: i32,
    color: sys::Color,
) {
    unsafe {
        sys::ImageDrawRectangle(
            dst,
            pos_x,
            pos_y,
            width,
            height,
            color,
        );
    }
}

/// Draw rectangle within an image (Vector version)
#[inline]
pub fn image_draw_rectangle_v(
    dst: &mut sys::Image,
    position: sys::Vector2,
    size: sys::Vector2,
    color: sys::Color,
) {
    unsafe {
        sys::ImageDrawRectangleV(
            dst,
            position,
            size,
            color,
        );
    }
}

/// Draw rectangle within an image
#[inline]
pub fn image_draw_rectangle_rec(
    dst: &mut sys::Image,
    rec: sys::Rectangle,
    color: sys::Color,
) {
    unsafe {
        sys::ImageDrawRectangleRec(
            dst,
            rec,
            color,
        );
    }
}

/// Draw rectangle lines within an image
#[inline]
pub fn image_draw_rectangle_lines(
    dst: &mut sys::Image,
    rec: sys::Rectangle,
    thick: i32,
    color: sys::Color,
) {
    unsafe {
        sys::ImageDrawRectangleLines(
            dst,
            rec,
            thick,
            color,
        );
    }
}

/// Draw triangle within an image
#[inline]
pub fn image_draw_triangle(
    dst: &mut sys::Image,
    v1: sys::Vector2,
    v2: sys::Vector2,
    v3: sys::Vector2,
    color: sys::Color,
) {
    unsafe {
        sys::ImageDrawTriangle(
            dst,
            v1,
            v2,
            v3,
            color,
        );
    }
}

/// Draw triangle with interpolated colors within an image
#[inline]
pub fn image_draw_triangle_ex(
    dst: &mut sys::Image,
    v1: sys::Vector2,
    v2: sys::Vector2,
    v3: sys::Vector2,
    c1: sys::Color,
    c2: sys::Color,
    c3: sys::Color,
) {
    unsafe {
        sys::ImageDrawTriangleEx(
            dst,
            v1,
            v2,
            v3,
            c1,
            c2,
            c3,
        )
    }
}

/// Draw triangle outline within an image
#[inline]
pub fn image_draw_triangle_lines(
    dst: &mut sys::Image,
    v1: sys::Vector2,
    v2: sys::Vector2,
    v3: sys::Vector2,
    color: sys::Color,
) {
    unsafe {
        sys::ImageDrawTriangleLines(
            dst,
            v1,
            v2,
            v3,
            color,
        );
    }
}

/// Draw a triangle fan defined by points within an image (first vertex is the center)
#[inline]
pub fn image_draw_triangle_fan(
    dst: &mut sys::Image,
    points: &[sys::Vector2],
    color: sys::Color,
) {
    unsafe {
        sys::ImageDrawTriangleFan(
            dst,
            points.as_ptr(),
            points.len().try_into().unwrap(),
            color,
        );
    }
}

/// Draw a triangle strip defined by points within an image
#[inline]
pub fn image_draw_triangle_strip(
    dst: &mut sys::Image,
    points: &[sys::Vector2],
    color: sys::Color,
) {
    unsafe {
        sys::ImageDrawTriangleStrip(
            dst,
            points.as_ptr(),
            points.len().try_into().unwrap(),
            color,
        );
    }
}

/// Draw a source image within a destination image (tint applied to source)
#[inline]
pub fn image_draw(
    dst: &mut sys::Image,
    src: sys::Image,
    src_rec: sys::Rectangle,
    dst_rec: sys::Rectangle,
    tint: sys::Color,
) {
    unsafe {
        sys::ImageDraw(
            dst,
            src,
            src_rec,
            dst_rec,
            tint,
        );
    }
}

/// Draw text (using default font) within an image (destination)
#[inline]
pub fn image_draw_text(
    dst: &mut sys::Image,
    text: &CStr,
    pos_x: i32,
    pos_y: i32,
    font_size: u32,
    color: sys::Color,
) {
    unsafe {
        sys::ImageDrawText(
            dst,
            text.as_ptr(),
            pos_x,
            pos_y,
            font_size.try_into().unwrap(),
            color,
        );
    }
}

/// Draw text (custom sprite font) within an image (destination)
#[inline]
pub fn image_draw_text_ex(
    dst: &mut sys::Image,
    font: sys::Font,
    text: &CStr,
    position: sys::Vector2,
    font_size: f32,
    spacing: f32,
    tint: sys::Color,
) {
    unsafe {
        sys::ImageDrawTextEx(
            dst,
            font,
            text.as_ptr(),
            position,
            font_size,
            spacing,
            tint,
        );
    }
}

// Texture loading functions
// NOTE: These functions require GPU access

// /// Load texture from file into GPU memory (VRAM)
// #[inline]
// pub fn LoadTexture(
//     fileName: *const ::std::os::raw::c_char,
// ) -> sys::Texture2D {
//     unsafe {
//         sys::
//     }
// }

// /// Load texture from image data
// #[inline]
// pub fn LoadTextureFromImage(
//     image: sys::Image,
// ) -> sys::Texture2D {
//     unsafe {
//         sys::
//     }
// }

// /// Load cubemap from image, multiple image cubemap layouts supported
// #[inline]
// pub fn LoadTextureCubemap(
//     image: sys::Image,
//     layout: i32,
// ) -> sys::TextureCubemap {
//     unsafe {
//         sys::
//     }
// }

// /// Load texture for rendering (framebuffer)
// #[inline]
// pub fn LoadRenderTexture(
//     width: i32,
//     height: i32,
// ) -> sys::RenderTexture2D {
//     unsafe {
//         sys::
//     }
// }

// /// Check if a texture is valid (loaded in GPU)
// #[inline]
// pub fn IsTextureValid(
//     texture: sys::Texture2D,
// ) -> bool {
//     unsafe {
//         sys::
//     }
// }

// /// Unload texture from GPU memory (VRAM)
// #[inline]
// pub fn UnloadTexture(
//     texture: sys::Texture2D,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Check if a render texture is valid (loaded in GPU)
// #[inline]
// pub fn IsRenderTextureValid(
//     target: sys::RenderTexture2D,
// ) -> bool {
//     unsafe {
//         sys::
//     }
// }

// /// Unload render texture from GPU memory (VRAM)
// #[inline]
// pub fn UnloadRenderTexture(
//     target: sys::RenderTexture2D,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Update GPU texture with new data
// #[inline]
// pub fn UpdateTexture(
//     texture: sys::Texture2D,
//     pixels: *const ::std::os::raw::c_void,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Update GPU texture rectangle with new data
// #[inline]
// pub fn UpdateTextureRec(
//     texture: sys::Texture2D,
//     rec: sys::Rectangle,
//     pixels: *const ::std::os::raw::c_void,
// ) {
//     unsafe {
//         sys::
//     }
// }

// // Texture configuration functions

// /// Generate GPU mipmaps for a texture
// #[inline]
// pub fn GenTextureMipmaps(
//     texture: *mut sys::Texture2D,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Set texture scaling filter mode
// #[inline]
// pub fn SetTextureFilter(
//     texture: sys::Texture2D,
//     filter: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Set texture wrapping mode
// #[inline]
// pub fn SetTextureWrap(
//     texture: sys::Texture2D,
//     wrap: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// // Texture drawing functions

// /// Draw a Texture2D
// #[inline]
// pub fn DrawTexture(
//     texture: sys::Texture2D,
//     posX: i32,
//     posY: i32,
//     tint: sys::Color,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Draw a Texture2D with position defined as Vector2
// #[inline]
// pub fn DrawTextureV(
//     texture: sys::Texture2D,
//     position: sys::Vector2,
//     tint: sys::Color,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Draw a Texture2D with extended parameters
// #[inline]
// pub fn DrawTextureEx(
//     texture: sys::Texture2D,
//     position: sys::Vector2,
//     rotation: f32,
//     scale: f32,
//     tint: sys::Color,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Draw a part of a texture defined by a rectangle
// #[inline]
// pub fn DrawTextureRec(
//     texture: sys::Texture2D,
//     source: sys::Rectangle,
//     position: sys::Vector2,
//     tint: sys::Color,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Draw a part of a texture defined by a rectangle with 'pro' parameters
// #[inline]
// pub fn DrawTexturePro(
//     texture: sys::Texture2D,
//     source: sys::Rectangle,
//     dest: sys::Rectangle,
//     origin: sys::Vector2,
//     rotation: f32,
//     tint: sys::Color,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Draws a texture (or part of it) that stretches or shrinks nicely
// #[inline]
// pub fn DrawTextureNPatch(
//     texture: sys::Texture2D,
//     nPatchInfo: sys::NPatchInfo,
//     dest: sys::Rectangle,
//     origin: sys::Vector2,
//     rotation: f32,
//     tint: sys::Color,
// ) {
//     unsafe {
//         sys::
//     }
// }

// // Color/pixel related functions

// /// Check if two colors are equal
// #[inline]
// pub fn ColorIsEqual(
//     col1: sys::Color,
//     col2: sys::Color,
// ) -> bool {
//     unsafe {
//         sys::
//     }
// }

// /// Get color with alpha applied, alpha goes from 0.0f to 1.0f
// #[inline]
// pub fn Fade(
//     color: sys::Color,
//     alpha: f32,
// ) -> sys::Color {
//     unsafe {
//         sys::
//     }
// }

// /// Get hexadecimal value for a Color (0xRRGGBBAA)
// #[inline]
// pub fn ColorToInt(
//     color: sys::Color,
// ) -> i32 {
//     unsafe {
//         sys::
//     }
// }

// /// Get Color normalized as float [0..1]
// #[inline]
// pub fn ColorNormalize(
//     color: sys::Color,
// ) -> sys::Vector4 {
//     unsafe {
//         sys::
//     }
// }

// /// Get Color from normalized values [0..1]
// #[inline]
// pub fn ColorFromNormalized(
//     normalized: sys::Vector4,
// ) -> sys::Color {
//     unsafe {
//         sys::
//     }
// }

// /// Get HSV values for a Color, hue [0..360], saturation/value [0..1]
// #[inline]
// pub fn ColorToHSV(
//     color: sys::Color,
// ) -> sys::Vector3 {
//     unsafe {
//         sys::
//     }
// }

// /// Get a Color from HSV values, hue [0..360], saturation/value [0..1]
// #[inline]
// pub fn ColorFromHSV(
//     hue: f32,
//     saturation: f32,
//     value: f32,
// ) -> sys::Color {
//     unsafe {
//         sys::
//     }
// }

// /// Get color multiplied with another color
// #[inline]
// pub fn ColorTint(
//     color: sys::Color,
//     tint: sys::Color,
// ) -> sys::Color {
//     unsafe {
//         sys::
//     }
// }

// /// Get color with brightness correction, brightness factor goes from -1.0f to 1.0f
// #[inline]
// pub fn ColorBrightness(
//     color: sys::Color,
//     factor: f32,
// ) -> sys::Color {
//     unsafe {
//         sys::
//     }
// }

// /// Get color with contrast correction, contrast values between -1.0f and 1.0f
// #[inline]
// pub fn ColorContrast(
//     color: sys::Color,
//     contrast: f32,
// ) -> sys::Color {
//     unsafe {
//         sys::
//     }
// }

// /// Get color with alpha applied, alpha goes from 0.0f to 1.0f
// #[inline]
// pub fn ColorAlpha(
//     color: sys::Color,
//     alpha: f32,
// ) -> sys::Color {
//     unsafe {
//         sys::
//     }
// }

// /// Get src alpha-blended into dst color with tint
// #[inline]
// pub fn ColorAlphaBlend(
//     dst: sys::Color,
//     src: sys::Color,
//     tint: sys::Color,
// ) -> sys::Color {
//     unsafe {
//         sys::
//     }
// }

// /// Get color lerp interpolation between two colors, factor [0.0f..1.0f]
// #[inline]
// pub fn ColorLerp(
//     color1: sys::Color,
//     color2: sys::Color,
//     factor: f32,
// ) -> sys::Color {
//     unsafe {
//         sys::
//     }
// }

// /// Get Color structure from hexadecimal value
// #[inline]
// pub fn GetColor(
//     hexValue: u32,
// ) -> sys::Color {
//     unsafe {
//         sys::
//     }
// }

// /// Get Color from a source pixel pointer of certain format
// #[inline]
// pub fn GetPixelColor(
//     srcPtr: *mut ::std::os::raw::c_void,
//     format: i32,
// ) -> sys::Color {
//     unsafe {
//         sys::
//     }
// }

// /// Set color formatted into destination pixel pointer
// #[inline]
// pub fn SetPixelColor(
//     dstPtr: *mut ::std::os::raw::c_void,
//     color: sys::Color,
//     format: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Get pixel data size in bytes for certain format
// #[inline]
// pub fn GetPixelDataSize(
//     width: i32,
//     height: i32,
//     format: i32,
// ) -> i32 {
//     unsafe {
//         sys::
//     }
// }
