//!   rlgl v5.0 - A multi-OpenGL abstraction layer with an immediate-mode style API
//!
//!   DESCRIPTION:
//!       An abstraction layer for multiple OpenGL versions (1.1, 2.1, 3.3 Core, 4.3 Core, ES 2.0, ES 3.0)
//!       that provides a pseudo-OpenGL 1.1 immediate-mode style API (rlVertex, rlTranslate, rlRotate...)
//!
//!   ADDITIONAL NOTES:
//!       When choosing an OpenGL backend different than OpenGL 1.1, some internal buffer are
//!       initialized on rlglInit() to accumulate vertex data
//!
//!       When an internal state change is required all the stored vertex data is rendered in batch,
//!       additionally, rlDrawRenderBatchActive() could be called to force flushing of the batch
//!
//!       Some resources are also loaded for convenience, here the complete list:
//!          - Default batch (RLGL.defaultBatch): RenderBatch system to accumulate vertex data
//!          - Default texture (RLGL.defaultTextureId): 1x1 white pixel R8G8B8A8
//!          - Default shader (RLGL.State.defaultShaderId, RLGL.State.defaultShaderLocs)
//!
//!       Internal buffer (and resources) must be manually unloaded calling rlglClose()
//!
//!   CONFIGURATION:
//!       #define GRAPHICS_API_OPENGL_11
//!       #define GRAPHICS_API_OPENGL_21
//!       #define GRAPHICS_API_OPENGL_33
//!       #define GRAPHICS_API_OPENGL_43
//!       #define GRAPHICS_API_OPENGL_ES2
//!       #define GRAPHICS_API_OPENGL_ES3
//!           Use selected OpenGL graphics backend, should be supported by platform
//!           Those preprocessor defines are only used on rlgl module, if OpenGL version is
//!           required by any other module, use rlGetVersion() to check it
//!
//!       #define RLGL_IMPLEMENTATION
//!           Generates the implementation of the library into the included file
//!           If not defined, the library is in header only mode and can be included in other headers
//!           or source files without problems. But only ONE file should hold the implementation
//!
//!       #define RLGL_RENDER_TEXTURES_HINT
//!           Enable framebuffer objects (fbo) support (enabled by default)
//!           Some GPUs could not support them despite the OpenGL version
//!
//!       #define RLGL_SHOW_GL_DETAILS_INFO
//!           Show OpenGL extensions and capabilities detailed logs on init
//!
//!       #define RLGL_ENABLE_OPENGL_DEBUG_CONTEXT
//!           Enable debug context (only available on OpenGL 4.3)
//!
//!       rlgl capabilities could be customized just defining some internal
//!       values before library inclusion (default values listed):
//!
//!       #define RL_DEFAULT_BATCH_BUFFER_ELEMENTS   8192    // Default internal render batch elements limits
//!       #define RL_DEFAULT_BATCH_BUFFERS              1    // Default number of batch buffers (multi-buffering)
//!       #define RL_DEFAULT_BATCH_DRAWCALLS          256    // Default number of batch draw calls (by state changes: mode, texture)
//!       #define RL_DEFAULT_BATCH_MAX_TEXTURE_UNITS    4    // Maximum number of textures units that can be activated on batch drawing (SetShaderValueTexture())
//!
//!       #define RL_MAX_MATRIX_STACK_SIZE             32    // Maximum size of internal Matrix stack
//!       #define RL_MAX_SHADER_LOCATIONS              32    // Maximum number of shader locations supported
//!       #define RL_CULL_DISTANCE_NEAR              0.05    // Default projection matrix near cull distance
//!       #define RL_CULL_DISTANCE_FAR             4000.0    // Default projection matrix far cull distance
//!
//!       When loading a shader, the following vertex attributes and uniform
//!       location names are tried to be set automatically:
//!
//!       #define RL_DEFAULT_SHADER_ATTRIB_NAME_POSITION     "vertexPosition"    // Bound by default to shader location: RL_DEFAULT_SHADER_ATTRIB_LOCATION_POSITION
//!       #define RL_DEFAULT_SHADER_ATTRIB_NAME_TEXCOORD     "vertexTexCoord"    // Bound by default to shader location: RL_DEFAULT_SHADER_ATTRIB_LOCATION_TEXCOORD
//!       #define RL_DEFAULT_SHADER_ATTRIB_NAME_NORMAL       "vertexNormal"      // Bound by default to shader location: RL_DEFAULT_SHADER_ATTRIB_LOCATION_NORMAL
//!       #define RL_DEFAULT_SHADER_ATTRIB_NAME_COLOR        "vertexColor"       // Bound by default to shader location: RL_DEFAULT_SHADER_ATTRIB_LOCATION_COLOR
//!       #define RL_DEFAULT_SHADER_ATTRIB_NAME_TANGENT      "vertexTangent"     // Bound by default to shader location: RL_DEFAULT_SHADER_ATTRIB_LOCATION_TANGENT
//!       #define RL_DEFAULT_SHADER_ATTRIB_NAME_TEXCOORD2    "vertexTexCoord2"   // Bound by default to shader location: RL_DEFAULT_SHADER_ATTRIB_LOCATION_TEXCOORD2
//!       #define RL_DEFAULT_SHADER_ATTRIB_NAME_BONEIDS      "vertexBoneIds"     // Bound by default to shader location: RL_DEFAULT_SHADER_ATTRIB_LOCATION_BONEIDS
//!       #define RL_DEFAULT_SHADER_ATTRIB_NAME_BONEWEIGHTS  "vertexBoneWeights" // Bound by default to shader location: RL_DEFAULT_SHADER_ATTRIB_LOCATION_BONEWEIGHTS
//!       #define RL_DEFAULT_SHADER_UNIFORM_NAME_MVP         "mvp"               // model-view-projection matrix
//!       #define RL_DEFAULT_SHADER_UNIFORM_NAME_VIEW        "matView"           // view matrix
//!       #define RL_DEFAULT_SHADER_UNIFORM_NAME_PROJECTION  "matProjection"     // projection matrix
//!       #define RL_DEFAULT_SHADER_UNIFORM_NAME_MODEL       "matModel"          // model matrix
//!       #define RL_DEFAULT_SHADER_UNIFORM_NAME_NORMAL      "matNormal"         // normal matrix (transpose(inverse(matModelView)))
//!       #define RL_DEFAULT_SHADER_UNIFORM_NAME_COLOR       "colDiffuse"        // color diffuse (base tint color, multiplied by texture color)
//!       #define RL_DEFAULT_SHADER_UNIFORM_NAME_BONE_MATRICES  "boneMatrices"   // bone matrices
//!       #define RL_DEFAULT_SHADER_SAMPLER2D_NAME_TEXTURE0  "texture0"          // texture0 (texture slot active 0)
//!       #define RL_DEFAULT_SHADER_SAMPLER2D_NAME_TEXTURE1  "texture1"          // texture1 (texture slot active 1)
//!       #define RL_DEFAULT_SHADER_SAMPLER2D_NAME_TEXTURE2  "texture2"          // texture2 (texture slot active 2)
//!
//!   DEPENDENCIES:
//!      - OpenGL libraries (depending on platform and OpenGL version selected)
//!      - GLAD OpenGL extensions loading library (only for OpenGL 3.3 Core, 4.3 Core)
//!
//!
//!   LICENSE: zlib/libpng
//!
//!   Copyright (c) 2014-2025 Ramon Santamaria (@raysan5)
//!
//!   This software is provided "as-is", without any express or implied warranty. In no event
//!   will the authors be held liable for any damages arising from the use of this software.
//!
//!   Permission is granted to anyone to use this software for any purpose, including commercial
//!   applications, and to alter it and redistribute it freely, subject to the following restrictions:
//!
//!     1. The origin of this software must not be misrepresented; you must not claim that you
//!     wrote the original software. If you use this software in a product, an acknowledgment
//!     in the product documentation would be appreciated but is not required.
//!
//!     2. Altered source versions must be plainly marked as such, and must not be misrepresented
//!     as being the original software.
//!
//!     3. This notice may not be removed or altered from any source distribution.

use super::sys;

//------------------------------------------------------------------------------------
// Functions Declaration - Matrix operations
//------------------------------------------------------------------------------------

/// Choose the current matrix to be transformed
#[inline]
pub fn rlMatrixMode(
    mode: i32,
);

/// Push the current matrix to stack
#[inline]
pub fn rlPushMatrix();

/// Pop latest inserted matrix from stack
#[inline]
pub fn rlPopMatrix();

/// Reset current matrix to identity matrix
#[inline]
pub fn rlLoadIdentity();

/// Multiply the current matrix by a translation matrix
#[inline]
pub fn rlTranslatef(
    x: f32,
    y: f32,
    z: f32,
);

/// Multiply the current matrix by a rotation matrix
#[inline]
pub fn rlRotatef(
    angle: f32,
    x: f32,
    y: f32,
    z: f32,
);

/// Multiply the current matrix by a scaling matrix
#[inline]
pub fn rlScalef(
    x: f32,
    y: f32,
    z: f32,
);

/// Multiply the current matrix by another matrix
#[inline]
pub fn rlMultMatrixf(
    matf: *const f32,
);

#[inline]
pub fn rlFrustum(
    left: f64,
    right: f64,
    bottom: f64,
    top: f64,
    znear: f64,
    zfar: f64,
);

#[inline]
pub fn rlOrtho(
    left: f64,
    right: f64,
    bottom: f64,
    top: f64,
    znear: f64,
    zfar: f64,
);

/// Set the viewport area
#[inline]
pub fn rlViewport(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
);

/// Set clip planes distances
#[inline]
pub fn rlSetClipPlanes(
    nearPlane: f64,
    farPlane: f64,
);

/// Get cull plane distance near
#[inline]
pub fn rlGetCullDistanceNear() -> f64;

/// Get cull plane distance far
#[inline]
pub fn rlGetCullDistanceFar() -> f64;

//------------------------------------------------------------------------------------
// Functions Declaration - Vertex level operations
//------------------------------------------------------------------------------------

/// Initialize drawing mode (how to organize vertex)
#[inline]
pub fn rlBegin(
    mode: i32,
);

/// Finish vertex providing
#[inline]
pub fn rlEnd();

/// Define one vertex (position) - 2 int
#[inline]
pub fn rlVertex2i(
    x: i32,
    y: i32,
);

/// Define one vertex (position) - 2 float
#[inline]
pub fn rlVertex2f(
    x: f32,
    y: f32,
);

/// Define one vertex (position) - 3 float
#[inline]
pub fn rlVertex3f(
    x: f32,
    y: f32,
    z: f32,
);

/// Define one vertex (texture coordinate) - 2 float
#[inline]
pub fn rlTexCoord2f(
    x: f32,
    y: f32,
);

/// Define one vertex (normal) - 3 float
#[inline]
pub fn rlNormal3f(
    x: f32,
    y: f32,
    z: f32,
);

/// Define one vertex (color) - 4 byte
#[inline]
pub fn rlColor4ub(
    r: u8,
    g: u8,
    b: u8,
    a: u8,
);

/// Define one vertex (color) - 3 float
#[inline]
pub fn rlColor3f(
    x: f32,
    y: f32,
    z: f32,
);

/// Define one vertex (color) - 4 float
#[inline]
pub fn rlColor4f(
    x: f32,
    y: f32,
    z: f32,
    w: f32,
);

//------------------------------------------------------------------------------------
// Functions Declaration - OpenGL style functions (common to 1.1, 3.3+, ES2)
// NOTE: This functions are used to completely abstract raylib code from OpenGL layer,
// some of them are direct wrappers over OpenGL calls, some others are custom
//------------------------------------------------------------------------------------

// Vertex buffers state

/// Enable vertex array (VAO, if supported)
#[inline]
pub fn rlEnableVertexArray(
    vaoId: u32,
) -> bool;

/// Disable vertex array (VAO, if supported)
#[inline]
pub fn rlDisableVertexArray();

/// Enable vertex buffer (VBO)
#[inline]
pub fn rlEnableVertexBuffer(
    id: u32,
);

/// Disable vertex buffer (VBO)
#[inline]
pub fn rlDisableVertexBuffer();

/// Enable vertex buffer element (VBO element)
#[inline]
pub fn rlEnableVertexBufferElement(
    id: u32,
);

/// Disable vertex buffer element (VBO element)
#[inline]
pub fn rlDisableVertexBufferElement();

/// Enable vertex attribute index
#[inline]
pub fn rlEnableVertexAttribute(
    index: u32,
);

/// Disable vertex attribute index
#[inline]
pub fn rlDisableVertexAttribute(
    index: u32,
);

// Textures state

/// Select and active a texture slot
#[inline]
pub fn rlActiveTextureSlot(
    slot: i32,
);

/// Enable texture
#[inline]
pub fn rlEnableTexture(
    id: u32,
);

/// Disable texture
#[inline]
pub fn rlDisableTexture();

/// Enable texture cubemap
#[inline]
pub fn rlEnableTextureCubemap(
    id: u32,
);

/// Disable texture cubemap
#[inline]
pub fn rlDisableTextureCubemap();

/// Set texture parameters (filter, wrap)
#[inline]
pub fn rlTextureParameters(
    id: u32,
    param: i32,
    value: i32,
);

/// Set cubemap parameters (filter, wrap)
#[inline]
pub fn rlCubemapParameters(
    id: u32,
    param: i32,
    value: i32,
);

// Shader state

/// Enable shader program
#[inline]
pub fn rlEnableShader(
    id: u32,
);

/// Disable shader program
#[inline]
pub fn rlDisableShader();

// Framebuffer state

/// Enable render texture (fbo)
#[inline]
pub fn rlEnableFramebuffer(
    id: u32,
);

/// Disable render texture (fbo), return to default framebuffer
#[inline]
pub fn rlDisableFramebuffer();

/// Get the currently active render texture (fbo), 0 for default framebuffer
#[inline]
pub fn rlGetActiveFramebuffer() -> u32;

/// Activate multiple draw color buffers
#[inline]
pub fn rlActiveDrawBuffers(
    count: i32,
);

/// Blit active framebuffer to main framebuffer
#[inline]
pub fn rlBlitFramebuffer(
    srcX: i32,
    srcY: i32,
    srcWidth: i32,
    srcHeight: i32,
    dstX: i32,
    dstY: i32,
    dstWidth: i32,
    dstHeight: i32,
    bufferMask: i32,
);

/// Bind framebuffer (FBO)
#[inline]
pub fn rlBindFramebuffer(
    target: u32,
    framebuffer: u32,
);

// General render state

/// Enable color blending
#[inline]
pub fn rlEnableColorBlend();

/// Disable color blending
#[inline]
pub fn rlDisableColorBlend();

/// Enable depth test
#[inline]
pub fn rlEnableDepthTest();

/// Disable depth test
#[inline]
pub fn rlDisableDepthTest();

/// Enable depth write
#[inline]
pub fn rlEnableDepthMask();

/// Disable depth write
#[inline]
pub fn rlDisableDepthMask();

/// Enable backface culling
#[inline]
pub fn rlEnableBackfaceCulling();

/// Disable backface culling
#[inline]
pub fn rlDisableBackfaceCulling();

/// Color mask control
#[inline]
pub fn rlColorMask(
    r: bool,
    g: bool,
    b: bool,
    a: bool,
);

/// Set face culling mode
#[inline]
pub fn rlSetCullFace(
    mode: i32,
);

/// Enable scissor test
#[inline]
pub fn rlEnableScissorTest();

/// Disable scissor test
#[inline]
pub fn rlDisableScissorTest();

/// Scissor test
#[inline]
pub fn rlScissor(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
);

/// Enable point mode
#[inline]
pub fn rlEnablePointMode();

/// Disable point mode
#[inline]
pub fn rlDisablePointMode();

/// Enable wire mode
#[inline]
pub fn rlEnableWireMode();

/// Disable wire mode
#[inline]
pub fn rlDisableWireMode();

/// Set the line drawing width
#[inline]
pub fn rlSetLineWidth(
    width: f32,
);

/// Get the line drawing width
#[inline]
pub fn rlGetLineWidth() -> f32;

/// Enable line aliasing
#[inline]
pub fn rlEnableSmoothLines();

/// Disable line aliasing
#[inline]
pub fn rlDisableSmoothLines();

/// Enable stereo rendering
#[inline]
pub fn rlEnableStereoRender();

/// Disable stereo rendering
#[inline]
pub fn rlDisableStereoRender();

/// Check if stereo render is enabled
#[inline]
pub fn rlIsStereoRenderEnabled() -> bool;

/// Clear color buffer with color
#[inline]
pub fn rlClearColor(
    r: u8,
    g: u8,
    b: u8,
    a: u8,
);

/// Clear used screen buffers (color and depth)
#[inline]
pub fn rlClearScreenBuffers();

/// Check and log OpenGL error codes
#[inline]
pub fn rlCheckErrors();

/// Set blending mode
#[inline]
pub fn rlSetBlendMode(
    mode: i32,
);

/// Set blending mode factor and equation (using OpenGL factors)
#[inline]
pub fn rlSetBlendFactors(
    glSrcFactor: i32,
    glDstFactor: i32,
    glEquation: i32,
);

/// Set blending mode factors and equations separately (using OpenGL factors)
#[inline]
pub fn rlSetBlendFactorsSeparate(
    glSrcRGB: i32,
    glDstRGB: i32,
    glSrcAlpha: i32,
    glDstAlpha: i32,
    glEqRGB: i32,
    glEqAlpha: i32,
);

//------------------------------------------------------------------------------------
// Functions Declaration - rlgl functionality
//------------------------------------------------------------------------------------
// rlgl initialization functions

/// Initialize rlgl (buffers, shaders, textures, states)
#[inline]
pub fn rlglInit(
    width: i32,
    height: i32,
);

/// De-initialize rlgl (buffers, shaders, textures)
#[inline]
pub fn rlglClose();

/// Load OpenGL extensions (loader function required)
#[inline]
pub fn rlLoadExtensions(
    loader: *mut ::std::os::raw::c_void,
);

/// Get current OpenGL version
#[inline]
pub fn rlGetVersion() -> i32;

/// Set current framebuffer width
#[inline]
pub fn rlSetFramebufferWidth(
    width: i32,
);

/// Get default framebuffer width
#[inline]
pub fn rlGetFramebufferWidth() -> i32;

/// Set current framebuffer height
#[inline]
pub fn rlSetFramebufferHeight(
    height: i32,
);

/// Get default framebuffer height
#[inline]
pub fn rlGetFramebufferHeight() -> i32;

/// Get default texture id
#[inline]
pub fn rlGetTextureIdDefault() -> u32;

/// Get default shader id
#[inline]
pub fn rlGetShaderIdDefault() -> u32;

/// Get default shader locations
#[inline]
pub fn rlGetShaderLocsDefault() -> *mut i32;

// Render batch management
// NOTE: rlgl provides a default render batch to behave like OpenGL 1.1 immediate mode
// but this render batch API is exposed in case of custom batches are required

/// Load a render batch system
#[inline]
pub fn rlLoadRenderBatch(
    numBuffers: i32,
    bufferElements: i32,
) -> rlRenderBatch;

/// Unload render batch system
#[inline]
pub fn rlUnloadRenderBatch(
    batch: rlRenderBatch,
);

/// Draw render batch data (Update->Draw->Reset)
#[inline]
pub fn rlDrawRenderBatch(
    batch: *mut rlRenderBatch,
);

/// Set the active render batch for rlgl (NULL for default internal)
#[inline]
pub fn rlSetRenderBatchActive(
    batch: *mut rlRenderBatch,
);

/// Update and draw internal render batch
#[inline]
pub fn rlDrawRenderBatchActive();

/// Check internal buffer overflow for a given number of vertex
#[inline]
pub fn rlCheckRenderBatchLimit(
    vCount: i32,
) -> bool;

/// Set current texture for render batch and check buffers limits
#[inline]
pub fn rlSetTexture(
    id: u32,
);

//------------------------------------------------------------------------------------------------------------------------

// Vertex buffers management

/// Load vertex array (vao) if supported
#[inline]
pub fn rlLoadVertexArray() -> u32;

/// Load a vertex buffer object
#[inline]
pub fn rlLoadVertexBuffer(
    buffer: *const ::std::os::raw::c_void,
    size: i32,
    dynamic: bool,
) -> u32;

/// Load vertex buffer elements object
#[inline]
pub fn rlLoadVertexBufferElement(
    buffer: *const ::std::os::raw::c_void,
    size: i32,
    dynamic: bool,
) -> u32;

/// Update vertex buffer object data on GPU buffer
#[inline]
pub fn rlUpdateVertexBuffer(
    bufferId: u32,
    data: *const ::std::os::raw::c_void,
    dataSize: i32,
    offset: i32,
);

/// Update vertex buffer elements data on GPU buffer
#[inline]
pub fn rlUpdateVertexBufferElements(
    id: u32,
    data: *const ::std::os::raw::c_void,
    dataSize: i32,
    offset: i32,
);

/// Unload vertex array (vao)
#[inline]
pub fn rlUnloadVertexArray(
    vaoId: u32,
);

/// Unload vertex buffer object
#[inline]
pub fn rlUnloadVertexBuffer(
    vboId: u32,
);

/// Set vertex attribute data configuration
#[inline]
pub fn rlSetVertexAttribute(
    index: u32,
    compSize: i32,
    type_: i32,
    normalized: bool,
    stride: i32,
    offset: i32,
);

/// Set vertex attribute data divisor
#[inline]
pub fn rlSetVertexAttributeDivisor(
    index: u32,
    divisor: i32,
);

/// Set vertex attribute default value, when attribute to provided
#[inline]
pub fn rlSetVertexAttributeDefault(
    locIndex: i32,
    value: *const ::std::os::raw::c_void,
    attribType: i32,
    count: i32,
);

/// Draw vertex array (currently active vao)
#[inline]
pub fn rlDrawVertexArray(
    offset: i32,
    count: i32,
);

/// Draw vertex array elements
#[inline]
pub fn rlDrawVertexArrayElements(
    offset: i32,
    count: i32,
    buffer: *const ::std::os::raw::c_void,
);

/// Draw vertex array (currently active vao) with instancing
#[inline]
pub fn rlDrawVertexArrayInstanced(
    offset: i32,
    count: i32,
    instances: i32,
);

/// Draw vertex array elements with instancing
#[inline]
pub fn rlDrawVertexArrayElementsInstanced(
    offset: i32,
    count: i32,
    buffer: *const ::std::os::raw::c_void,
    instances: i32,
);

// Textures management

/// Load texture data
#[inline]
pub fn rlLoadTexture(
    data: *const ::std::os::raw::c_void,
    width: i32,
    height: i32,
    format: i32,
    mipmapCount: i32,
) -> u32;

/// Load depth texture/renderbuffer (to be attached to fbo)
#[inline]
pub fn rlLoadTextureDepth(
    width: i32,
    height: i32,
    useRenderBuffer: bool,
) -> u32;

/// Load texture cubemap data
#[inline]
pub fn rlLoadTextureCubemap(
    data: *const ::std::os::raw::c_void,
    size: i32,
    format: i32,
    mipmapCount: i32,
) -> u32;

/// Update texture with new data on GPU
#[inline]
pub fn rlUpdateTexture(
    id: u32,
    offsetX: i32,
    offsetY: i32,
    width: i32,
    height: i32,
    format: i32,
    data: *const ::std::os::raw::c_void,
);

/// Get OpenGL internal formats
#[inline]
pub fn rlGetGlTextureFormats(
    format: i32,
    glInternalFormat: *mut u32,
    glFormat: *mut u32,
    glType: *mut u32,
);

/// Get name string for pixel format
#[inline]
pub fn rlGetPixelFormatName(
    format: u32,
) -> *const ::std::os::raw::c_char;

/// Unload texture from GPU memory
#[inline]
pub fn rlUnloadTexture(
    id: u32,
);

/// Generate mipmap data for selected texture
#[inline]
pub fn rlGenTextureMipmaps(
    id: u32,
    width: i32,
    height: i32,
    format: i32,
    mipmaps: *mut i32,
);

/// Read texture pixel data
#[inline]
pub fn rlReadTexturePixels(
    id: u32,
    width: i32,
    height: i32,
    format: i32,
) -> *mut ::std::os::raw::c_void;

/// Read screen pixel data (color buffer)
#[inline]
pub fn rlReadScreenPixels(
    width: i32,
    height: i32,
) -> *mut u8;

// Framebuffer management (fbo)

/// Load an empty framebuffer
#[inline]
pub fn rlLoadFramebuffer() -> u32;

/// Attach texture/renderbuffer to a framebuffer
#[inline]
pub fn rlFramebufferAttach(
    fboId: u32,
    texId: u32,
    attachType: i32,
    texType: i32,
    mipLevel: i32,
);

/// Verify framebuffer is complete
#[inline]
pub fn rlFramebufferComplete(
    id: u32,
) -> bool;

/// Delete framebuffer from GPU
#[inline]
pub fn rlUnloadFramebuffer(
    id: u32,
);

// Shaders management

/// Load shader from code strings
#[inline]
pub fn rlLoadShaderCode(
    vsCode: *const ::std::os::raw::c_char,
    fsCode: *const ::std::os::raw::c_char,
) -> u32;

/// Compile custom shader and return shader id (type: RL_VERTEX_SHADER, RL_FRAGMENT_SHADER, RL_COMPUTE_SHADER)
#[inline]
pub fn rlCompileShader(
    shaderCode: *const ::std::os::raw::c_char,
    type_: i32,
) -> u32;

/// Load custom shader program
#[inline]
pub fn rlLoadShaderProgram(
    vShaderId: u32,
    fShaderId: u32,
) -> u32;

/// Unload shader program
#[inline]
pub fn rlUnloadShaderProgram(
    id: u32,
);

/// Get shader location uniform
#[inline]
pub fn rlGetLocationUniform(
    shaderId: u32,
    uniformName: *const ::std::os::raw::c_char,
) -> i32;

/// Get shader location attribute
#[inline]
pub fn rlGetLocationAttrib(
    shaderId: u32,
    attribName: *const ::std::os::raw::c_char,
) -> i32;

/// Set shader value uniform
#[inline]
pub fn rlSetUniform(
    locIndex: i32,
    value: *const ::std::os::raw::c_void,
    uniformType: i32,
    count: i32,
);

/// Set shader value matrix
#[inline]
pub fn rlSetUniformMatrix(
    locIndex: i32,
    mat: Matrix,
);

/// Set shader value matrices
#[inline]
pub fn rlSetUniformMatrices(
    locIndex: i32,
    mat: *const Matrix,
    count: i32,
);

/// Set shader value sampler
#[inline]
pub fn rlSetUniformSampler(
    locIndex: i32,
    textureId: u32,
);

/// Set shader currently active (id and locations)
#[inline]
pub fn rlSetShader(
    id: u32,
    locs: *mut i32,
);

// Compute shader management

/// Load compute shader program
#[inline]
pub fn rlLoadComputeShaderProgram(
    shaderId: u32,
) -> u32;

/// Dispatch compute shader (equivalent to *draw* for graphics pipeline)
#[inline]
pub fn rlComputeShaderDispatch(
    groupX: u32,
    groupY: u32,
    groupZ: u32,
);

// Shader buffer storage object management (ssbo)

/// Load shader storage buffer object (SSBO)
#[inline]
pub fn rlLoadShaderBuffer(
    size: u32,
    data: *const ::std::os::raw::c_void,
    usageHint: i32,
) -> u32;

/// Unload shader storage buffer object (SSBO)
#[inline]
pub fn rlUnloadShaderBuffer(
    ssboId: u32,
);

/// Update SSBO buffer data
#[inline]
pub fn rlUpdateShaderBuffer(
    id: u32,
    data: *const ::std::os::raw::c_void,
    dataSize: u32,
    offset: u32,
);

/// Bind SSBO buffer
#[inline]
pub fn rlBindShaderBuffer(
    id: u32,
    index: u32,
);

/// Read SSBO buffer data (GPU->CPU)
#[inline]
pub fn rlReadShaderBuffer(
    id: u32,
    dest: *mut ::std::os::raw::c_void,
    count: u32,
    offset: u32,
);

/// Copy SSBO data between buffers
#[inline]
pub fn rlCopyShaderBuffer(
    destId: u32,
    srcId: u32,
    destOffset: u32,
    srcOffset: u32,
    count: u32,
);

/// Get SSBO buffer size
#[inline]
pub fn rlGetShaderBufferSize(
    id: u32,
) -> u32;

// Buffer management

/// Bind image texture
#[inline]
pub fn rlBindImageTexture(
    id: u32,
    index: u32,
    format: i32,
    readonly: bool,
);

// Matrix state management

/// Get internal modelview matrix
#[inline]
pub fn rlGetMatrixModelview() -> Matrix;

/// Get internal projection matrix
#[inline]
pub fn rlGetMatrixProjection() -> Matrix;

/// Get internal accumulated transform matrix
#[inline]
pub fn rlGetMatrixTransform() -> Matrix;

/// Get internal projection matrix for stereo render (selected eye)
#[inline]
pub fn rlGetMatrixProjectionStereo(
    eye: i32,
) -> Matrix;

/// Get internal view offset matrix for stereo render (selected eye)
#[inline]
pub fn rlGetMatrixViewOffsetStereo(
    eye: i32,
) -> Matrix;

/// Set a custom projection matrix (replaces internal projection matrix)
#[inline]
pub fn rlSetMatrixProjection(
    proj: Matrix,
);

/// Set a custom modelview matrix (replaces internal modelview matrix)
#[inline]
pub fn rlSetMatrixModelview(
    view: Matrix,
);

/// Set eyes projection matrices for stereo rendering
#[inline]
pub fn rlSetMatrixProjectionStereo(
    right: Matrix,
    left: Matrix,
);

/// Set eyes view offsets matrices for stereo rendering
#[inline]
pub fn rlSetMatrixViewOffsetStereo(
    right: Matrix,
    left: Matrix,
);

// Quick and dirty cube/quad buffers load->draw->unload

/// Load and draw a cube
#[inline]
pub fn rlLoadDrawCube();

/// Load and draw a quad
#[inline]
pub fn rlLoadDrawQuad();
