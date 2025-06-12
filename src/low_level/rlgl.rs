pub fn rlMatrixMode(mode: ::std::os::raw::c_int);
pub fn rlPushMatrix();
pub fn rlPopMatrix();
pub fn rlLoadIdentity();
pub fn rlTranslatef(
    x: ::std::os::raw::c_float,
    y: ::std::os::raw::c_float,
    z: ::std::os::raw::c_float,
);
pub fn rlRotatef(
    angle: ::std::os::raw::c_float,
    x: ::std::os::raw::c_float,
    y: ::std::os::raw::c_float,
    z: ::std::os::raw::c_float,
);
pub fn rlScalef(
    x: ::std::os::raw::c_float,
    y: ::std::os::raw::c_float,
    z: ::std::os::raw::c_float,
);
pub fn rlMultMatrixf(matf: *const ::std::os::raw::c_float);
pub fn rlFrustum(
    left: ::std::os::raw::c_double,
    right: ::std::os::raw::c_double,
    bottom: ::std::os::raw::c_double,
    top: ::std::os::raw::c_double,
    znear: ::std::os::raw::c_double,
    zfar: ::std::os::raw::c_double,
);
pub fn rlOrtho(
    left: ::std::os::raw::c_double,
    right: ::std::os::raw::c_double,
    bottom: ::std::os::raw::c_double,
    top: ::std::os::raw::c_double,
    znear: ::std::os::raw::c_double,
    zfar: ::std::os::raw::c_double,
);
pub fn rlViewport(
    x: ::std::os::raw::c_int,
    y: ::std::os::raw::c_int,
    width: ::std::os::raw::c_int,
    height: ::std::os::raw::c_int,
);
pub fn rlSetClipPlanes(nearPlane: ::std::os::raw::c_double, farPlane: ::std::os::raw::c_double);
pub fn rlGetCullDistanceNear() -> ::std::os::raw::c_double;
pub fn rlGetCullDistanceFar() -> ::std::os::raw::c_double;
pub fn rlBegin(mode: ::std::os::raw::c_int);
pub fn rlEnd();
pub fn rlVertex2i(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int);
pub fn rlVertex2f(x: ::std::os::raw::c_float, y: ::std::os::raw::c_float);
pub fn rlVertex3f(
    x: ::std::os::raw::c_float,
    y: ::std::os::raw::c_float,
    z: ::std::os::raw::c_float,
);
pub fn rlTexCoord2f(x: ::std::os::raw::c_float, y: ::std::os::raw::c_float);
pub fn rlNormal3f(
    x: ::std::os::raw::c_float,
    y: ::std::os::raw::c_float,
    z: ::std::os::raw::c_float,
);
pub fn rlColor4ub(
    r: ::std::os::raw::c_uchar,
    g: ::std::os::raw::c_uchar,
    b: ::std::os::raw::c_uchar,
    a: ::std::os::raw::c_uchar,
);
pub fn rlColor3f(
    x: ::std::os::raw::c_float,
    y: ::std::os::raw::c_float,
    z: ::std::os::raw::c_float,
);
pub fn rlColor4f(
    x: ::std::os::raw::c_float,
    y: ::std::os::raw::c_float,
    z: ::std::os::raw::c_float,
    w: ::std::os::raw::c_float,
);
pub fn rlEnableVertexArray(vaoId: ::std::os::raw::c_uint) -> bool;
pub fn rlDisableVertexArray();
pub fn rlEnableVertexBuffer(id: ::std::os::raw::c_uint);
pub fn rlDisableVertexBuffer();
pub fn rlEnableVertexBufferElement(id: ::std::os::raw::c_uint);
pub fn rlDisableVertexBufferElement();
pub fn rlEnableVertexAttribute(index: ::std::os::raw::c_uint);
pub fn rlDisableVertexAttribute(index: ::std::os::raw::c_uint);
pub fn rlActiveTextureSlot(slot: ::std::os::raw::c_int);
pub fn rlEnableTexture(id: ::std::os::raw::c_uint);
pub fn rlDisableTexture();
pub fn rlEnableTextureCubemap(id: ::std::os::raw::c_uint);
pub fn rlDisableTextureCubemap();
pub fn rlTextureParameters(
    id: ::std::os::raw::c_uint,
    param: ::std::os::raw::c_int,
    value: ::std::os::raw::c_int,
);
pub fn rlCubemapParameters(
    id: ::std::os::raw::c_uint,
    param: ::std::os::raw::c_int,
    value: ::std::os::raw::c_int,
);
pub fn rlEnableShader(id: ::std::os::raw::c_uint);
pub fn rlDisableShader();
pub fn rlEnableFramebuffer(id: ::std::os::raw::c_uint);
pub fn rlDisableFramebuffer();
pub fn rlGetActiveFramebuffer() -> ::std::os::raw::c_uint;
pub fn rlActiveDrawBuffers(count: ::std::os::raw::c_int);
pub fn rlBlitFramebuffer(
    srcX: ::std::os::raw::c_int,
    srcY: ::std::os::raw::c_int,
    srcWidth: ::std::os::raw::c_int,
    srcHeight: ::std::os::raw::c_int,
    dstX: ::std::os::raw::c_int,
    dstY: ::std::os::raw::c_int,
    dstWidth: ::std::os::raw::c_int,
    dstHeight: ::std::os::raw::c_int,
    bufferMask: ::std::os::raw::c_int,
);
pub fn rlBindFramebuffer(target: ::std::os::raw::c_uint, framebuffer: ::std::os::raw::c_uint);
pub fn rlEnableColorBlend();
pub fn rlDisableColorBlend();
pub fn rlEnableDepthTest();
pub fn rlDisableDepthTest();
pub fn rlEnableDepthMask();
pub fn rlDisableDepthMask();
pub fn rlEnableBackfaceCulling();
pub fn rlDisableBackfaceCulling();
pub fn rlColorMask(r: bool, g: bool, b: bool, a: bool);
pub fn rlSetCullFace(mode: ::std::os::raw::c_int);
pub fn rlEnableScissorTest();
pub fn rlDisableScissorTest();
pub fn rlScissor(
    x: ::std::os::raw::c_int,
    y: ::std::os::raw::c_int,
    width: ::std::os::raw::c_int,
    height: ::std::os::raw::c_int,
);
pub fn rlEnablePointMode();
pub fn rlDisablePointMode();
pub fn rlEnableWireMode();
pub fn rlDisableWireMode();
pub fn rlSetLineWidth(width: ::std::os::raw::c_float);
pub fn rlGetLineWidth() -> ::std::os::raw::c_float;
pub fn rlEnableSmoothLines();
pub fn rlDisableSmoothLines();
pub fn rlEnableStereoRender();
pub fn rlDisableStereoRender();
pub fn rlIsStereoRenderEnabled() -> bool;
pub fn rlClearColor(
    r: ::std::os::raw::c_uchar,
    g: ::std::os::raw::c_uchar,
    b: ::std::os::raw::c_uchar,
    a: ::std::os::raw::c_uchar,
);
pub fn rlClearScreenBuffers();
pub fn rlCheckErrors();
pub fn rlSetBlendMode(mode: ::std::os::raw::c_int);
pub fn rlSetBlendFactors(
    glSrcFactor: ::std::os::raw::c_int,
    glDstFactor: ::std::os::raw::c_int,
    glEquation: ::std::os::raw::c_int,
);
pub fn rlSetBlendFactorsSeparate(
    glSrcRGB: ::std::os::raw::c_int,
    glDstRGB: ::std::os::raw::c_int,
    glSrcAlpha: ::std::os::raw::c_int,
    glDstAlpha: ::std::os::raw::c_int,
    glEqRGB: ::std::os::raw::c_int,
    glEqAlpha: ::std::os::raw::c_int,
);
pub fn rlglInit(width: ::std::os::raw::c_int, height: ::std::os::raw::c_int);
pub fn rlglClose();
pub fn rlLoadExtensions(loader: *mut ::std::os::raw::c_void);
pub fn rlGetVersion() -> ::std::os::raw::c_int;
pub fn rlSetFramebufferWidth(width: ::std::os::raw::c_int);
pub fn rlGetFramebufferWidth() -> ::std::os::raw::c_int;
pub fn rlSetFramebufferHeight(height: ::std::os::raw::c_int);
pub fn rlGetFramebufferHeight() -> ::std::os::raw::c_int;
pub fn rlGetTextureIdDefault() -> ::std::os::raw::c_uint;
pub fn rlGetShaderIdDefault() -> ::std::os::raw::c_uint;
pub fn rlGetShaderLocsDefault() -> *mut ::std::os::raw::c_int;
pub fn rlLoadRenderBatch(
    numBuffers: ::std::os::raw::c_int,
    bufferElements: ::std::os::raw::c_int,
) -> rlRenderBatch;
pub fn rlUnloadRenderBatch(batch: rlRenderBatch);
pub fn rlDrawRenderBatch(batch: *mut rlRenderBatch);
pub fn rlSetRenderBatchActive(batch: *mut rlRenderBatch);
pub fn rlDrawRenderBatchActive();
pub fn rlCheckRenderBatchLimit(vCount: ::std::os::raw::c_int) -> bool;
pub fn rlSetTexture(id: ::std::os::raw::c_uint);
pub fn rlLoadVertexArray() -> ::std::os::raw::c_uint;
pub fn rlLoadVertexBuffer(
    buffer: *const ::std::os::raw::c_void,
    size: ::std::os::raw::c_int,
    dynamic: bool,
) -> ::std::os::raw::c_uint;
pub fn rlLoadVertexBufferElement(
    buffer: *const ::std::os::raw::c_void,
    size: ::std::os::raw::c_int,
    dynamic: bool,
) -> ::std::os::raw::c_uint;
pub fn rlUpdateVertexBuffer(
    bufferId: ::std::os::raw::c_uint,
    data: *const ::std::os::raw::c_void,
    dataSize: ::std::os::raw::c_int,
    offset: ::std::os::raw::c_int,
);
pub fn rlUpdateVertexBufferElements(
    id: ::std::os::raw::c_uint,
    data: *const ::std::os::raw::c_void,
    dataSize: ::std::os::raw::c_int,
    offset: ::std::os::raw::c_int,
);
pub fn rlUnloadVertexArray(vaoId: ::std::os::raw::c_uint);
pub fn rlUnloadVertexBuffer(vboId: ::std::os::raw::c_uint);
pub fn rlSetVertexAttribute(
    index: ::std::os::raw::c_uint,
    compSize: ::std::os::raw::c_int,
    type_: ::std::os::raw::c_int,
    normalized: bool,
    stride: ::std::os::raw::c_int,
    offset: ::std::os::raw::c_int,
);
pub fn rlSetVertexAttributeDivisor(
    index: ::std::os::raw::c_uint,
    divisor: ::std::os::raw::c_int,
);
pub fn rlSetVertexAttributeDefault(
    locIndex: ::std::os::raw::c_int,
    value: *const ::std::os::raw::c_void,
    attribType: ::std::os::raw::c_int,
    count: ::std::os::raw::c_int,
);
pub fn rlDrawVertexArray(offset: ::std::os::raw::c_int, count: ::std::os::raw::c_int);
pub fn rlDrawVertexArrayElements(
    offset: ::std::os::raw::c_int,
    count: ::std::os::raw::c_int,
    buffer: *const ::std::os::raw::c_void,
);
pub fn rlDrawVertexArrayInstanced(
    offset: ::std::os::raw::c_int,
    count: ::std::os::raw::c_int,
    instances: ::std::os::raw::c_int,
);
pub fn rlDrawVertexArrayElementsInstanced(
    offset: ::std::os::raw::c_int,
    count: ::std::os::raw::c_int,
    buffer: *const ::std::os::raw::c_void,
    instances: ::std::os::raw::c_int,
);
pub fn rlLoadTexture(
    data: *const ::std::os::raw::c_void,
    width: ::std::os::raw::c_int,
    height: ::std::os::raw::c_int,
    format: ::std::os::raw::c_int,
    mipmapCount: ::std::os::raw::c_int,
) -> ::std::os::raw::c_uint;
pub fn rlLoadTextureDepth(
    width: ::std::os::raw::c_int,
    height: ::std::os::raw::c_int,
    useRenderBuffer: bool,
) -> ::std::os::raw::c_uint;
pub fn rlLoadTextureCubemap(
    data: *const ::std::os::raw::c_void,
    size: ::std::os::raw::c_int,
    format: ::std::os::raw::c_int,
    mipmapCount: ::std::os::raw::c_int,
) -> ::std::os::raw::c_uint;
pub fn rlUpdateTexture(
    id: ::std::os::raw::c_uint,
    offsetX: ::std::os::raw::c_int,
    offsetY: ::std::os::raw::c_int,
    width: ::std::os::raw::c_int,
    height: ::std::os::raw::c_int,
    format: ::std::os::raw::c_int,
    data: *const ::std::os::raw::c_void,
);
pub fn rlGetGlTextureFormats(
    format: ::std::os::raw::c_int,
    glInternalFormat: *mut ::std::os::raw::c_uint,
    glFormat: *mut ::std::os::raw::c_uint,
    glType: *mut ::std::os::raw::c_uint,
);
pub fn rlGetPixelFormatName(format: ::std::os::raw::c_uint) -> *const ::std::os::raw::c_char;
pub fn rlUnloadTexture(id: ::std::os::raw::c_uint);
pub fn rlGenTextureMipmaps(
    id: ::std::os::raw::c_uint,
    width: ::std::os::raw::c_int,
    height: ::std::os::raw::c_int,
    format: ::std::os::raw::c_int,
    mipmaps: *mut ::std::os::raw::c_int,
);
pub fn rlReadTexturePixels(
    id: ::std::os::raw::c_uint,
    width: ::std::os::raw::c_int,
    height: ::std::os::raw::c_int,
    format: ::std::os::raw::c_int,
) -> *mut ::std::os::raw::c_void;
pub fn rlReadScreenPixels(
    width: ::std::os::raw::c_int,
    height: ::std::os::raw::c_int,
) -> *mut ::std::os::raw::c_uchar;
pub fn rlLoadFramebuffer() -> ::std::os::raw::c_uint;
pub fn rlFramebufferAttach(
    fboId: ::std::os::raw::c_uint,
    texId: ::std::os::raw::c_uint,
    attachType: ::std::os::raw::c_int,
    texType: ::std::os::raw::c_int,
    mipLevel: ::std::os::raw::c_int,
);
pub fn rlFramebufferComplete(id: ::std::os::raw::c_uint) -> bool;
pub fn rlUnloadFramebuffer(id: ::std::os::raw::c_uint);
pub fn rlLoadShaderCode(
    vsCode: *const ::std::os::raw::c_char,
    fsCode: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_uint;
pub fn rlCompileShader(
    shaderCode: *const ::std::os::raw::c_char,
    type_: ::std::os::raw::c_int,
) -> ::std::os::raw::c_uint;
pub fn rlLoadShaderProgram(
    vShaderId: ::std::os::raw::c_uint,
    fShaderId: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_uint;
pub fn rlUnloadShaderProgram(id: ::std::os::raw::c_uint);
pub fn rlGetLocationUniform(
    shaderId: ::std::os::raw::c_uint,
    uniformName: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int;
pub fn rlGetLocationAttrib(
    shaderId: ::std::os::raw::c_uint,
    attribName: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int;
pub fn rlSetUniform(
    locIndex: ::std::os::raw::c_int,
    value: *const ::std::os::raw::c_void,
    uniformType: ::std::os::raw::c_int,
    count: ::std::os::raw::c_int,
);
pub fn rlSetUniformMatrix(locIndex: ::std::os::raw::c_int, mat: Matrix);
pub fn rlSetUniformMatrices(
    locIndex: ::std::os::raw::c_int,
    mat: *const Matrix,
    count: ::std::os::raw::c_int,
);
pub fn rlSetUniformSampler(locIndex: ::std::os::raw::c_int, textureId: ::std::os::raw::c_uint);
pub fn rlSetShader(id: ::std::os::raw::c_uint, locs: *mut ::std::os::raw::c_int);
pub fn rlLoadComputeShaderProgram(shaderId: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;
pub fn rlComputeShaderDispatch(
    groupX: ::std::os::raw::c_uint,
    groupY: ::std::os::raw::c_uint,
    groupZ: ::std::os::raw::c_uint,
);
pub fn rlLoadShaderBuffer(
    size: ::std::os::raw::c_uint,
    data: *const ::std::os::raw::c_void,
    usageHint: ::std::os::raw::c_int,
) -> ::std::os::raw::c_uint;
pub fn rlUnloadShaderBuffer(ssboId: ::std::os::raw::c_uint);
pub fn rlUpdateShaderBuffer(
    id: ::std::os::raw::c_uint,
    data: *const ::std::os::raw::c_void,
    dataSize: ::std::os::raw::c_uint,
    offset: ::std::os::raw::c_uint,
);
pub fn rlBindShaderBuffer(id: ::std::os::raw::c_uint, index: ::std::os::raw::c_uint);
pub fn rlReadShaderBuffer(
    id: ::std::os::raw::c_uint,
    dest: *mut ::std::os::raw::c_void,
    count: ::std::os::raw::c_uint,
    offset: ::std::os::raw::c_uint,
);
pub fn rlCopyShaderBuffer(
    destId: ::std::os::raw::c_uint,
    srcId: ::std::os::raw::c_uint,
    destOffset: ::std::os::raw::c_uint,
    srcOffset: ::std::os::raw::c_uint,
    count: ::std::os::raw::c_uint,
);
pub fn rlGetShaderBufferSize(id: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;
pub fn rlBindImageTexture(
    id: ::std::os::raw::c_uint,
    index: ::std::os::raw::c_uint,
    format: ::std::os::raw::c_int,
    readonly: bool,
);
pub fn rlGetMatrixModelview() -> Matrix;
pub fn rlGetMatrixProjection() -> Matrix;
pub fn rlGetMatrixTransform() -> Matrix;
pub fn rlGetMatrixProjectionStereo(eye: ::std::os::raw::c_int) -> Matrix;
pub fn rlGetMatrixViewOffsetStereo(eye: ::std::os::raw::c_int) -> Matrix;
pub fn rlSetMatrixProjection(proj: Matrix);
pub fn rlSetMatrixModelview(view: Matrix);
pub fn rlSetMatrixProjectionStereo(right: Matrix, left: Matrix);
pub fn rlSetMatrixViewOffsetStereo(right: Matrix, left: Matrix);
pub fn rlLoadDrawCube();
pub fn rlLoadDrawQuad();
