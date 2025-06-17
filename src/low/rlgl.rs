//! rlgl v5.0 - A multi-OpenGL abstraction layer with an immediate-mode style API
//!
//! ## DESCRIPTION:
//! An abstraction layer for multiple OpenGL versions (1.1, 2.1, 3.3 Core, 4.3 Core, ES 2.0, ES 3.0)
//! that provides a pseudo-OpenGL 1.1 immediate-mode style API (rlVertex, rlTranslate, rlRotate...)
//!
//! ## ADDITIONAL NOTES:
//! When choosing an OpenGL backend different than OpenGL 1.1, some internal buffer are
//! initialized on rlglInit() to accumulate vertex data
//!
//! When an internal state change is required all the stored vertex data is rendered in batch,
//! additionally, rlDrawRenderBatchActive() could be called to force flushing of the batch
//!
//! Some resources are also loaded for convenience, here the complete list:
//! - Default batch (RLGL.defaultBatch): RenderBatch system to accumulate vertex data
//! - Default texture (RLGL.defaultTextureId): 1x1 white pixel R8G8B8A8
//! - Default shader (RLGL.State.defaultShaderId, RLGL.State.defaultShaderLocs)
//!
//! Internal buffer (and resources) must be manually unloaded calling rlglClose()
//!
//! ## DEPENDENCIES:
//! - OpenGL libraries (depending on platform and OpenGL version selected)
//! - GLAD OpenGL extensions loading library (only for OpenGL 3.3 Core, 4.3 Core)
//!
//!
//! LICENSE: zlib/libpng
//!
//! Copyright (c) 2014-2025 Ramon Santamaria (@raysan5)
//!
//! This software is provided "as-is", without any express or implied warranty. In no event
//! will the authors be held liable for any damages arising from the use of this software.
//!
//! Permission is granted to anyone to use this software for any purpose, including commercial
//! applications, and to alter it and redistribute it freely, subject to the following restrictions:
//!
//! 1. The origin of this software must not be misrepresented; you must not claim that you
//!    wrote the original software. If you use this software in a product, an acknowledgment
//!    in the product documentation would be appreciated but is not required.
//!
//! 2. Altered source versions must be plainly marked as such, and must not be misrepresented
//!    as being the original software.
//!
//! 3. This notice may not be removed or altered from any source distribution.

use super::*;

//------------------------------------------------------------------------------------
// Functions Declaration - sys::Matrix operations
//------------------------------------------------------------------------------------

// /// Choose the current matrix to be transformed
// #[inline]
// pub unsafe fn rlMatrixMode(
//     mode: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

/// Push the current matrix to stack
#[inline]
pub unsafe fn rl_push_matrix() {
    unsafe {
        sys::rlPushMatrix();
    }
}

/// Pop latest inserted matrix from stack
#[inline]
pub unsafe fn rl_pop_matrix() {
    unsafe {
        sys::rlPopMatrix();
    }
}

/// Reset current matrix to identity matrix
#[inline]
pub unsafe fn rl_load_identity() {
    unsafe {
        sys::rlLoadIdentity();
    }
}

// /// Multiply the current matrix by a translation matrix
// #[inline]
// pub unsafe fn rlTranslatef(
//     x: f32,
//     y: f32,
//     z: f32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Multiply the current matrix by a rotation matrix
// #[inline]
// pub unsafe fn rlRotatef(
//     angle: f32,
//     x: f32,
//     y: f32,
//     z: f32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Multiply the current matrix by a scaling matrix
// #[inline]
// pub unsafe fn rlScalef(
//     x: f32,
//     y: f32,
//     z: f32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Multiply the current matrix by another matrix
// #[inline]
// pub unsafe fn rlMultMatrixf(
//     matf: *const f32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// #[inline]
// pub unsafe fn rlFrustum(
//     left: f64,
//     right: f64,
//     bottom: f64,
//     top: f64,
//     znear: f64,
//     zfar: f64,
// ) {
//     unsafe {
//         sys::
//     }
// }

// #[inline]
// pub unsafe fn rlOrtho(
//     left: f64,
//     right: f64,
//     bottom: f64,
//     top: f64,
//     znear: f64,
//     zfar: f64,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Set the viewport area
// #[inline]
// pub unsafe fn rlViewport(
//     x: i32,
//     y: i32,
//     width: i32,
//     height: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Set clip planes distances
// #[inline]
// pub unsafe fn rlSetClipPlanes(
//     nearPlane: f64,
//     farPlane: f64,
// ) {
//     unsafe {
//         sys::
//     }
// }

/// Get cull plane distance near
#[inline]
pub unsafe fn rl_get_cull_distance_near() -> f64 {
    unsafe {
        sys::rlGetCullDistanceNear()
    }
}

/// Get cull plane distance far
#[inline]
pub unsafe fn rl_get_cull_distance_far() -> f64 {
    unsafe {
        sys::rlGetCullDistanceFar()
    }
}

// //------------------------------------------------------------------------------------
// // Functions Declaration - Vertex level operations
// //------------------------------------------------------------------------------------

// /// Initialize drawing mode (how to organize vertex)
// #[inline]
// pub unsafe fn rlBegin(
//     mode: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Finish vertex providing
// #[inline]
// pub unsafe fn rlEnd() {
//     unsafe {
//         sys::
//     }
// }

// /// Define one vertex (position) - 2 int
// #[inline]
// pub unsafe fn rlVertex2i(
//     x: i32,
//     y: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Define one vertex (position) - 2 float
// #[inline]
// pub unsafe fn rlVertex2f(
//     x: f32,
//     y: f32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Define one vertex (position) - 3 float
// #[inline]
// pub unsafe fn rlVertex3f(
//     x: f32,
//     y: f32,
//     z: f32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Define one vertex (texture coordinate) - 2 float
// #[inline]
// pub unsafe fn rlTexCoord2f(
//     x: f32,
//     y: f32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Define one vertex (normal) - 3 float
// #[inline]
// pub unsafe fn rlNormal3f(
//     x: f32,
//     y: f32,
//     z: f32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Define one vertex (color) - 4 byte
// #[inline]
// pub unsafe fn rlColor4ub(
//     r: u8,
//     g: u8,
//     b: u8,
//     a: u8,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Define one vertex (color) - 3 float
// #[inline]
// pub unsafe fn rlColor3f(
//     x: f32,
//     y: f32,
//     z: f32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Define one vertex (color) - 4 float
// #[inline]
// pub unsafe fn rlColor4f(
//     x: f32,
//     y: f32,
//     z: f32,
//     w: f32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// //------------------------------------------------------------------------------------
// // Functions Declaration - OpenGL style functions (common to 1.1, 3.3+, ES2)
// // NOTE: This functions are used to completely abstract raylib code from OpenGL layer,
// // some of them are direct wrappers over OpenGL calls, some others are custom
// //------------------------------------------------------------------------------------

// // Vertex buffers state

// /// Enable vertex array (VAO, if supported)
// #[inline]
// pub unsafe fn rlEnableVertexArray(
//     vaoId: u32,
// ) -> bool {
//     unsafe {
//         sys::
//     }
// }

/// Disable vertex array (VAO, if supported)
#[inline]
pub unsafe fn rl_disable_vertex_array() {
    unsafe {
        sys::rlDisableVertexArray();
    }
}

// /// Enable vertex buffer (VBO)
// #[inline]
// pub unsafe fn rlEnableVertexBuffer(
//     id: u32,
// ) {
//     unsafe {
//         sys::
//     }
// }

/// Disable vertex buffer (VBO)
#[inline]
pub unsafe fn rl_disable_vertex_buffer() {
    unsafe {
        sys::rlDisableVertexBuffer();
    }
}

// /// Enable vertex buffer element (VBO element)
// #[inline]
// pub unsafe fn rlEnableVertexBufferElement(
//     id: u32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Disable vertex buffer element (VBO element)
// #[inline]
// pub unsafe fn rlDisableVertexBufferElement() {
//     unsafe {
//         sys::
//     }
// }

// /// Enable vertex attribute index
// #[inline]
// pub unsafe fn rlEnableVertexAttribute(
//     index: u32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Disable vertex attribute index
// #[inline]
// pub unsafe fn rlDisableVertexAttribute(
//     index: u32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// // Textures state

// /// Select and active a texture slot
// #[inline]
// pub unsafe fn rlActiveTextureSlot(
//     slot: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Enable texture
// #[inline]
// pub unsafe fn rlEnableTexture(
//     id: u32,
// ) {
//     unsafe {
//         sys::
//     }
// }

/// Disable texture
#[inline]
pub unsafe fn rl_disable_texture() {
    unsafe {
        sys::rlDisableTexture();
    }
}

// /// Enable texture cubemap
// #[inline]
// pub unsafe fn rlEnableTextureCubemap(
//     id: u32,
// ) {
//     unsafe {
//         sys::
//     }
// }

/// Disable texture cubemap
#[inline]
pub unsafe fn rl_disable_texture_cubemap() {
    unsafe {
        sys::rlDisableTextureCubemap();
    }
}

// /// Set texture parameters (filter, wrap)
// #[inline]
// pub unsafe fn rlTextureParameters(
//     id: u32,
//     param: i32,
//     value: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Set cubemap parameters (filter, wrap)
// #[inline]
// pub unsafe fn rlCubemapParameters(
//     id: u32,
//     param: i32,
//     value: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// Shader state

// /// Enable shader program
// #[inline]
// pub unsafe fn rlEnableShader(
//     id: u32,
// ) {
//     unsafe {
//         sys::
//     }
// }

/// Disable shader program
#[inline]
pub unsafe fn rl_disable_shader() {
    unsafe {
        sys::rlDisableShader();
    }
}

// Framebuffer state

// /// Enable render texture (fbo)
// #[inline]
// pub unsafe fn rlEnableFramebuffer(
//     id: u32,
// ) {
//     unsafe {
//         sys::
//     }
// }

/// Disable render texture (fbo), return to default framebuffer
#[inline]
pub unsafe fn rl_disable_framebuffer() {
    unsafe {
        sys::rlDisableFramebuffer();
    }
}

// /// Get the currently active render texture (fbo), 0 for default framebuffer
// #[inline]
// pub unsafe fn rlGetActiveFramebuffer() -> u32 {
//     unsafe {
//         sys::
//     }
// }

// /// Activate multiple draw color buffers
// #[inline]
// pub unsafe fn rlActiveDrawBuffers(
//     count: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Blit active framebuffer to main framebuffer
// #[inline]
// pub unsafe fn rlBlitFramebuffer(
//     srcX: i32,
//     srcY: i32,
//     srcWidth: i32,
//     srcHeight: i32,
//     dstX: i32,
//     dstY: i32,
//     dstWidth: i32,
//     dstHeight: i32,
//     bufferMask: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Bind framebuffer (FBO)
// #[inline]
// pub unsafe fn rlBindFramebuffer(
//     target: u32,
//     framebuffer: u32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// // General render state

// /// Enable color blending
// #[inline]
// pub unsafe fn rlEnableColorBlend() {
//     unsafe {
//         sys::
//     }
// }

// /// Disable color blending
// #[inline]
// pub unsafe fn rlDisableColorBlend() {
//     unsafe {
//         sys::
//     }
// }

// /// Enable depth test
// #[inline]
// pub unsafe fn rlEnableDepthTest() {
//     unsafe {
//         sys::
//     }
// }

// /// Disable depth test
// #[inline]
// pub unsafe fn rlDisableDepthTest() {
//     unsafe {
//         sys::
//     }
// }

// /// Enable depth write
// #[inline]
// pub unsafe fn rlEnableDepthMask() {
//     unsafe {
//         sys::
//     }
// }

// /// Disable depth write
// #[inline]
// pub unsafe fn rlDisableDepthMask() {
//     unsafe {
//         sys::
//     }
// }

// /// Enable backface culling
// #[inline]
// pub unsafe fn rlEnableBackfaceCulling() {
//     unsafe {
//         sys::
//     }
// }

// /// Disable backface culling
// #[inline]
// pub unsafe fn rlDisableBackfaceCulling() {
//     unsafe {
//         sys::
//     }
// }

// /// Color mask control
// #[inline]
// pub unsafe fn rlColorMask(
//     r: bool,
//     g: bool,
//     b: bool,
//     a: bool,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Set face culling mode
// #[inline]
// pub unsafe fn rlSetCullFace(
//     mode: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Enable scissor test
// #[inline]
// pub unsafe fn rlEnableScissorTest() {
//     unsafe {
//         sys::
//     }
// }

// /// Disable scissor test
// #[inline]
// pub unsafe fn rlDisableScissorTest() {
//     unsafe {
//         sys::
//     }
// }

// /// Scissor test
// #[inline]
// pub unsafe fn rlScissor(
//     x: i32,
//     y: i32,
//     width: i32,
//     height: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Enable point mode
// #[inline]
// pub unsafe fn rlEnablePointMode() {
//     unsafe {
//         sys::
//     }
// }

// /// Disable point mode
// #[inline]
// pub unsafe fn rlDisablePointMode() {
//     unsafe {
//         sys::
//     }
// }

// /// Enable wire mode
// #[inline]
// pub unsafe fn rlEnableWireMode() {
//     unsafe {
//         sys::
//     }
// }

// /// Disable wire mode
// #[inline]
// pub unsafe fn rlDisableWireMode() {
//     unsafe {
//         sys::
//     }
// }

// /// Set the line drawing width
// #[inline]
// pub unsafe fn rlSetLineWidth(
//     width: f32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Get the line drawing width
// #[inline]
// pub unsafe fn rlGetLineWidth() -> f32 {
//     unsafe {
//         sys::
//     }
// }

// /// Enable line aliasing
// #[inline]
// pub unsafe fn rlEnableSmoothLines() {
//     unsafe {
//         sys::
//     }
// }

// /// Disable line aliasing
// #[inline]
// pub unsafe fn rlDisableSmoothLines() {
//     unsafe {
//         sys::
//     }
// }

// /// Enable stereo rendering
// #[inline]
// pub unsafe fn rlEnableStereoRender() {
//     unsafe {
//         sys::
//     }
// }

// /// Disable stereo rendering
// #[inline]
// pub unsafe fn rlDisableStereoRender() {
//     unsafe {
//         sys::
//     }
// }

// /// Check if stereo render is enabled
// #[inline]
// pub unsafe fn rlIsStereoRenderEnabled() -> bool {
//     unsafe {
//         sys::
//     }
// }

// /// Clear color buffer with color
// #[inline]
// pub unsafe fn rlClearColor(
//     r: u8,
//     g: u8,
//     b: u8,
//     a: u8,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Clear used screen buffers (color and depth)
// #[inline]
// pub unsafe fn rlClearScreenBuffers() {
//     unsafe {
//         sys::
//     }
// }

// /// Check and log OpenGL error codes
// #[inline]
// pub unsafe fn rlCheckErrors() {
//     unsafe {
//         sys::
//     }
// }

// /// Set blending mode
// #[inline]
// pub unsafe fn rlSetBlendMode(
//     mode: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Set blending mode factor and equation (using OpenGL factors)
// #[inline]
// pub unsafe fn rlSetBlendFactors(
//     glSrcFactor: i32,
//     glDstFactor: i32,
//     glEquation: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Set blending mode factors and equations separately (using OpenGL factors)
// #[inline]
// pub unsafe fn rlSetBlendFactorsSeparate(
//     glSrcRGB: i32,
//     glDstRGB: i32,
//     glSrcAlpha: i32,
//     glDstAlpha: i32,
//     glEqRGB: i32,
//     glEqAlpha: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// //------------------------------------------------------------------------------------
// // Functions Declaration - rlgl functionality
// //------------------------------------------------------------------------------------
// // rlgl initialization functions

// /// Initialize rlgl (buffers, shaders, textures, states)
// #[inline]
// pub unsafe fn rlglInit(
//     width: i32,
//     height: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// De-initialize rlgl (buffers, shaders, textures)
// #[inline]
// pub unsafe fn rlglClose() {
//     unsafe {
//         sys::
//     }
// }

// /// Load OpenGL extensions (loader function required)
// #[inline]
// pub unsafe fn rlLoadExtensions(
//     loader: *mut ::std::os::raw::c_void,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Get current OpenGL version
// #[inline]
// pub unsafe fn rlGetVersion() -> i32 {
//     unsafe {
//         sys::
//     }
// }

// /// Set current framebuffer width
// #[inline]
// pub unsafe fn rlSetFramebufferWidth(
//     width: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Get default framebuffer width
// #[inline]
// pub unsafe fn rlGetFramebufferWidth() -> i32 {
//     unsafe {
//         sys::
//     }
// }

// /// Set current framebuffer height
// #[inline]
// pub unsafe fn rlSetFramebufferHeight(
//     height: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Get default framebuffer height
// #[inline]
// pub unsafe fn rlGetFramebufferHeight() -> i32 {
//     unsafe {
//         sys::
//     }
// }

// /// Get default texture id
// #[inline]
// pub unsafe fn rlGetTextureIdDefault() -> u32 {
//     unsafe {
//         sys::
//     }
// }

// /// Get default shader id
// #[inline]
// pub unsafe fn rlGetShaderIdDefault() -> u32 {
//     unsafe {
//         sys::
//     }
// }

// /// Get default shader locations
// #[inline]
// pub unsafe fn rlGetShaderLocsDefault() -> *mut i32 {
//     unsafe {
//         sys::
//     }
// }

// // Render batch management
// // NOTE: rlgl provides a default render batch to behave like OpenGL 1.1 immediate mode
// // but this render batch API is exposed in case of custom batches are required

// /// Load a render batch system
// #[inline]
// pub unsafe fn rlLoadRenderBatch(
//     numBuffers: i32,
//     bufferElements: i32,
// ) -> sys::rlRenderBatch {
//     unsafe {
//         sys::
//     }
// }

// /// Unload render batch system
// #[inline]
// pub unsafe fn rlUnloadRenderBatch(
//     batch: sys::rlRenderBatch,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Draw render batch data (Update->Draw->Reset)
// #[inline]
// pub unsafe fn rlDrawRenderBatch(
//     batch: *mut sys::rlRenderBatch,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Set the active render batch for rlgl (NULL for default internal)
// #[inline]
// pub unsafe fn rlSetRenderBatchActive(
//     batch: *mut sys::rlRenderBatch,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Update and draw internal render batch
// #[inline]
// pub unsafe fn rlDrawRenderBatchActive() {
//     unsafe {
//         sys::
//     }
// }

// /// Check internal buffer overflow for a given number of vertex
// #[inline]
// pub unsafe fn rlCheckRenderBatchLimit(
//     vCount: i32,
// ) -> bool {
//     unsafe {
//         sys::
//     }
// }

// /// Set current texture for render batch and check buffers limits
// #[inline]
// pub unsafe fn rlSetTexture(
//     id: u32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// //------------------------------------------------------------------------------------------------------------------------

// // Vertex buffers management

// /// Load vertex array (vao) if supported
// #[inline]
// pub unsafe fn rlLoadVertexArray() -> u32 {
//     unsafe {
//         sys::
//     }
// }

// /// Load a vertex buffer object
// #[inline]
// pub unsafe fn rlLoadVertexBuffer(
//     buffer: *const ::std::os::raw::c_void,
//     size: i32,
//     dynamic: bool,
// ) -> u32 {
//     unsafe {
//         sys::
//     }
// }

// /// Load vertex buffer elements object
// #[inline]
// pub unsafe fn rlLoadVertexBufferElement(
//     buffer: *const ::std::os::raw::c_void,
//     size: i32,
//     dynamic: bool,
// ) -> u32 {
//     unsafe {
//         sys::
//     }
// }

// /// Update vertex buffer object data on GPU buffer
// #[inline]
// pub unsafe fn rlUpdateVertexBuffer(
//     bufferId: u32,
//     data: *const ::std::os::raw::c_void,
//     dataSize: i32,
//     offset: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Update vertex buffer elements data on GPU buffer
// #[inline]
// pub unsafe fn rlUpdateVertexBufferElements(
//     id: u32,
//     data: *const ::std::os::raw::c_void,
//     dataSize: i32,
//     offset: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Unload vertex array (vao)
// #[inline]
// pub unsafe fn rlUnloadVertexArray(
//     vaoId: u32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Unload vertex buffer object
// #[inline]
// pub unsafe fn rlUnloadVertexBuffer(
//     vboId: u32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Set vertex attribute data configuration
// #[inline]
// pub unsafe fn rlSetVertexAttribute(
//     index: u32,
//     compSize: i32,
//     type_: i32,
//     normalized: bool,
//     stride: i32,
//     offset: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Set vertex attribute data divisor
// #[inline]
// pub unsafe fn rlSetVertexAttributeDivisor(
//     index: u32,
//     divisor: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Set vertex attribute default value, when attribute to provided
// #[inline]
// pub unsafe fn rlSetVertexAttributeDefault(
//     locIndex: i32,
//     value: *const ::std::os::raw::c_void,
//     attribType: i32,
//     count: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Draw vertex array (currently active vao)
// #[inline]
// pub unsafe fn rlDrawVertexArray(
//     offset: i32,
//     count: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Draw vertex array elements
// #[inline]
// pub unsafe fn rlDrawVertexArrayElements(
//     offset: i32,
//     count: i32,
//     buffer: *const ::std::os::raw::c_void,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Draw vertex array (currently active vao) with instancing
// #[inline]
// pub unsafe fn rlDrawVertexArrayInstanced(
//     offset: i32,
//     count: i32,
//     instances: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Draw vertex array elements with instancing
// #[inline]
// pub unsafe fn rlDrawVertexArrayElementsInstanced(
//     offset: i32,
//     count: i32,
//     buffer: *const ::std::os::raw::c_void,
//     instances: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// // Textures management

// /// Load texture data
// #[inline]
// pub unsafe fn rlLoadTexture(
//     data: *const ::std::os::raw::c_void,
//     width: i32,
//     height: i32,
//     format: i32,
//     mipmapCount: i32,
// ) -> u32 {
//     unsafe {
//         sys::
//     }
// }

// /// Load depth texture/renderbuffer (to be attached to fbo)
// #[inline]
// pub unsafe fn rlLoadTextureDepth(
//     width: i32,
//     height: i32,
//     useRenderBuffer: bool,
// ) -> u32 {
//     unsafe {
//         sys::
//     }
// }

// /// Load texture cubemap data
// #[inline]
// pub unsafe fn rlLoadTextureCubemap(
//     data: *const ::std::os::raw::c_void,
//     size: i32,
//     format: i32,
//     mipmapCount: i32,
// ) -> u32 {
//     unsafe {
//         sys::
//     }
// }

// /// Update texture with new data on GPU
// #[inline]
// pub unsafe fn rlUpdateTexture(
//     id: u32,
//     offsetX: i32,
//     offsetY: i32,
//     width: i32,
//     height: i32,
//     format: i32,
//     data: *const ::std::os::raw::c_void,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Get OpenGL internal formats
// #[inline]
// pub unsafe fn rlGetGlTextureFormats(
//     format: i32,
//     glInternalFormat: *mut u32,
//     glFormat: *mut u32,
//     glType: *mut u32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Get name string for pixel format
// #[inline]
// pub unsafe fn rlGetPixelFormatName(
//     format: u32,
// ) -> *const ::std::os::raw::c_char {
//     unsafe {
//         sys::
//     }
// }

// /// Unload texture from GPU memory
// #[inline]
// pub unsafe fn rlUnloadTexture(
//     id: u32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Generate mipmap data for selected texture
// #[inline]
// pub unsafe fn rlGenTextureMipmaps(
//     id: u32,
//     width: i32,
//     height: i32,
//     format: i32,
//     mipmaps: *mut i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Read texture pixel data
// #[inline]
// pub unsafe fn rlReadTexturePixels(
//     id: u32,
//     width: i32,
//     height: i32,
//     format: i32,
// ) -> *mut ::std::os::raw::c_void {
//     unsafe {
//         sys::
//     }
// }

// /// Read screen pixel data (color buffer)
// #[inline]
// pub unsafe fn rlReadScreenPixels(
//     width: i32,
//     height: i32,
// ) -> *mut u8 {
//     unsafe {
//         sys::
//     }
// }

// // Framebuffer management (fbo)

// /// Load an empty framebuffer
// #[inline]
// pub unsafe fn rlLoadFramebuffer() -> u32 {
//     unsafe {
//         sys::
//     }
// }

// /// Attach texture/renderbuffer to a framebuffer
// #[inline]
// pub unsafe fn rlFramebufferAttach(
//     fboId: u32,
//     texId: u32,
//     attachType: i32,
//     texType: i32,
//     mipLevel: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Verify framebuffer is complete
// #[inline]
// pub unsafe fn rlFramebufferComplete(
//     id: u32,
// ) -> bool {
//     unsafe {
//         sys::
//     }
// }

// /// Delete framebuffer from GPU
// #[inline]
// pub unsafe fn rlUnloadFramebuffer(
//     id: u32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// // Shaders management

// /// Load shader from code strings
// #[inline]
// pub unsafe fn rlLoadShaderCode(
//     vsCode: *const ::std::os::raw::c_char,
//     fsCode: *const ::std::os::raw::c_char,
// ) -> u32 {
//     unsafe {
//         sys::
//     }
// }

// /// Compile custom shader and return shader id (type: RL_VERTEX_SHADER, RL_FRAGMENT_SHADER, RL_COMPUTE_SHADER)
// #[inline]
// pub unsafe fn rlCompileShader(
//     shaderCode: *const ::std::os::raw::c_char,
//     type_: i32,
// ) -> u32 {
//     unsafe {
//         sys::
//     }
// }

// /// Load custom shader program
// #[inline]
// pub unsafe fn rlLoadShaderProgram(
//     vShaderId: u32,
//     fShaderId: u32,
// ) -> u32 {
//     unsafe {
//         sys::
//     }
// }

// /// Unload shader program
// #[inline]
// pub unsafe fn rlUnloadShaderProgram(
//     id: u32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Get shader location uniform
// #[inline]
// pub unsafe fn rlGetLocationUniform(
//     shaderId: u32,
//     uniformName: *const ::std::os::raw::c_char,
// ) -> i32 {
//     unsafe {
//         sys::
//     }
// }

// /// Get shader location attribute
// #[inline]
// pub unsafe fn rlGetLocationAttrib(
//     shaderId: u32,
//     attribName: *const ::std::os::raw::c_char,
// ) -> i32 {
//     unsafe {
//         sys::
//     }
// }

// /// Set shader value uniform
// #[inline]
// pub unsafe fn rlSetUniform(
//     locIndex: i32,
//     value: *const ::std::os::raw::c_void,
//     uniformType: i32,
//     count: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Set shader value matrix
// #[inline]
// pub unsafe fn rlSetUniformMatrix(
//     locIndex: i32,
//     mat: sys::Matrix,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Set shader value matrices
// #[inline]
// pub unsafe fn rlSetUniformMatrices(
//     locIndex: i32,
//     mat: *const sys::Matrix,
//     count: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Set shader value sampler
// #[inline]
// pub unsafe fn rlSetUniformSampler(
//     locIndex: i32,
//     textureId: u32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Set shader currently active (id and locations)
// #[inline]
// pub unsafe fn rlSetShader(
//     id: u32,
//     locs: *mut i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// // Compute shader management

// /// Load compute shader program
// #[inline]
// pub unsafe fn rlLoadComputeShaderProgram(
//     shaderId: u32,
// ) -> u32 {
//     unsafe {
//         sys::
//     }
// }

// /// Dispatch compute shader (equivalent to *draw* for graphics pipeline)
// #[inline]
// pub unsafe fn rlComputeShaderDispatch(
//     groupX: u32,
//     groupY: u32,
//     groupZ: u32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// // Shader buffer storage object management (ssbo)

// /// Load shader storage buffer object (SSBO)
// #[inline]
// pub unsafe fn rlLoadShaderBuffer(
//     size: u32,
//     data: *const ::std::os::raw::c_void,
//     usageHint: i32,
// ) -> u32 {
//     unsafe {
//         sys::
//     }
// }

// /// Unload shader storage buffer object (SSBO)
// #[inline]
// pub unsafe fn rlUnloadShaderBuffer(
//     ssboId: u32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Update SSBO buffer data
// #[inline]
// pub unsafe fn rlUpdateShaderBuffer(
//     id: u32,
//     data: *const ::std::os::raw::c_void,
//     dataSize: u32,
//     offset: u32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Bind SSBO buffer
// #[inline]
// pub unsafe fn rlBindShaderBuffer(
//     id: u32,
//     index: u32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Read SSBO buffer data (GPU->CPU)
// #[inline]
// pub unsafe fn rlReadShaderBuffer(
//     id: u32,
//     dest: *mut ::std::os::raw::c_void,
//     count: u32,
//     offset: u32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Copy SSBO data between buffers
// #[inline]
// pub unsafe fn rlCopyShaderBuffer(
//     destId: u32,
//     srcId: u32,
//     destOffset: u32,
//     srcOffset: u32,
//     count: u32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Get SSBO buffer size
// #[inline]
// pub unsafe fn rlGetShaderBufferSize(
//     id: u32,
// ) -> u32 {
//     unsafe {
//         sys::
//     }
// }

// // Buffer management

// /// Bind image texture
// #[inline]
// pub unsafe fn rlBindImageTexture(
//     id: u32,
//     index: u32,
//     format: i32,
//     readonly: bool,
// ) {
//     unsafe {
//         sys::
//     }
// }

// // Matrix state management

// /// Get internal modelview matrix
// #[inline]
// pub unsafe fn rlGetMatrixModelview() -> sys::Matrix {
//     unsafe {
//         sys::
//     }
// }

// /// Get internal projection matrix
// #[inline]
// pub unsafe fn rlGetMatrixProjection() -> sys::Matrix {
//     unsafe {
//         sys::
//     }
// }

// /// Get internal accumulated transform matrix
// #[inline]
// pub unsafe fn rlGetMatrixTransform() -> sys::Matrix {
//     unsafe {
//         sys::
//     }
// }

// /// Get internal projection matrix for stereo render (selected eye)
// #[inline]
// pub unsafe fn rlGetMatrixProjectionStereo(
//     eye: i32,
// ) -> sys::Matrix {
//     unsafe {
//         sys::
//     }
// }

// /// Get internal view offset matrix for stereo render (selected eye)
// #[inline]
// pub unsafe fn rlGetMatrixViewOffsetStereo(
//     eye: i32,
// ) -> sys::Matrix {
//     unsafe {
//         sys::
//     }
// }

// /// Set a custom projection matrix (replaces internal projection matrix)
// #[inline]
// pub unsafe fn rlSetMatrixProjection(
//     proj: sys::Matrix,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Set a custom modelview matrix (replaces internal modelview matrix)
// #[inline]
// pub unsafe fn rlSetMatrixModelview(
//     view: sys::Matrix,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Set eyes projection matrices for stereo rendering
// #[inline]
// pub unsafe fn rlSetMatrixProjectionStereo(
//     right: sys::Matrix,
//     left: sys::Matrix,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Set eyes view offsets matrices for stereo rendering
// #[inline]
// pub unsafe fn rlSetMatrixViewOffsetStereo(
//     right: sys::Matrix,
//     left: sys::Matrix,
// ) {
//     unsafe {
//         sys::
//     }
// }

// // Quick and dirty cube/quad buffers load->draw->unload

// /// Load and draw a cube
// #[inline]
// pub unsafe fn rlLoadDrawCube() {
//     unsafe {
//         sys::
//     }
// }

// /// Load and draw a quad
// #[inline]
// pub unsafe fn rlLoadDrawQuad() {
//     unsafe {
//         sys::
//     }
// }
