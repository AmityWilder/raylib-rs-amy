use super::*;

// Basic geometric 3D shapes drawing functions

/// Draw a line in 3D world space
#[inline]
pub fn DrawLine3D(
    startPos: Vector3,
    endPos: Vector3,
    color: Color,
) {
    unsafe {
        sys::
    }
}

/// Draw a point in 3D space, actually a small line
#[inline]
pub fn DrawPoint3D(
    position: Vector3,
    color: Color,
) {
    unsafe {
        sys::
    }
}

/// Draw a circle in 3D world space
#[inline]
pub fn DrawCircle3D(
    center: Vector3,
    radius: f32,
    rotationAxis: Vector3,
    rotationAngle: f32,
    color: Color,
) {
    unsafe {
        sys::
    }
}

/// Draw a color-filled triangle (vertex in counter-clockwise order!)
#[inline]
pub fn DrawTriangle3D(
    v1: Vector3,
    v2: Vector3,
    v3: Vector3,
    color: Color,
) {
    unsafe {
        sys::
    }
}

/// Draw a triangle strip defined by points
#[inline]
pub fn DrawTriangleStrip3D(
    points: *const Vector3,
    pointCount: i32,
    color: Color,
) {
    unsafe {
        sys::
    }
}

/// Draw cube
#[inline]
pub fn DrawCube(
    position: Vector3,
    width: f32,
    height: f32,
    length: f32,
    color: Color,
) {
    unsafe {
        sys::
    }
}

/// Draw cube (Vector version)
#[inline]
pub fn DrawCubeV(
    position: Vector3,
    size: Vector3,
    color: Color,
) {
    unsafe {
        sys::
    }
}

/// Draw cube wires
#[inline]
pub fn DrawCubeWires(
    position: Vector3,
    width: f32,
    height: f32,
    length: f32,
    color: Color,
) {
    unsafe {
        sys::
    }
}

/// Draw cube wires (Vector version)
#[inline]
pub fn DrawCubeWiresV(
    position: Vector3,
    size: Vector3,
    color: Color,
) {
    unsafe {
        sys::
    }
}

/// Draw sphere
#[inline]
pub fn DrawSphere(
    centerPos: Vector3,
    radius: f32,
    color: Color,
) {
    unsafe {
        sys::
    }
}

/// Draw sphere with extended parameters
#[inline]
pub fn DrawSphereEx(
    centerPos: Vector3,
    radius: f32,
    rings: i32,
    slices: i32,
    color: Color,
) {
    unsafe {
        sys::
    }
}

/// Draw sphere wires
#[inline]
pub fn DrawSphereWires(
    centerPos: Vector3,
    radius: f32,
    rings: i32,
    slices: i32,
    color: Color,
) {
    unsafe {
        sys::
    }
}

/// Draw a cylinder/cone
#[inline]
pub fn DrawCylinder(
    position: Vector3,
    radiusTop: f32,
    radiusBottom: f32,
    height: f32,
    slices: i32,
    color: Color,
) {
    unsafe {
        sys::
    }
}

/// Draw a cylinder with base at startPos and top at endPos
#[inline]
pub fn DrawCylinderEx(
    startPos: Vector3,
    endPos: Vector3,
    startRadius: f32,
    endRadius: f32,
    sides: i32,
    color: Color,
) {
    unsafe {
        sys::
    }
}

/// Draw a cylinder/cone wires
#[inline]
pub fn DrawCylinderWires(
    position: Vector3,
    radiusTop: f32,
    radiusBottom: f32,
    height: f32,
    slices: i32,
    color: Color,
) {
    unsafe {
        sys::
    }
}

/// Draw a cylinder wires with base at startPos and top at endPos
#[inline]
pub fn DrawCylinderWiresEx(
    startPos: Vector3,
    endPos: Vector3,
    startRadius: f32,
    endRadius: f32,
    sides: i32,
    color: Color,
) {
    unsafe {
        sys::
    }
}

/// Draw a capsule with the center of its sphere caps at startPos and endPos
#[inline]
pub fn DrawCapsule(
    startPos: Vector3,
    endPos: Vector3,
    radius: f32,
    slices: i32,
    rings: i32,
    color: Color,
) {
    unsafe {
        sys::
    }
}

/// Draw capsule wireframe with the center of its sphere caps at startPos and endPos
#[inline]
pub fn DrawCapsuleWires(
    startPos: Vector3,
    endPos: Vector3,
    radius: f32,
    slices: i32,
    rings: i32,
    color: Color,
) {
    unsafe {
        sys::
    }
}

/// Draw a plane XZ
#[inline]
pub fn DrawPlane(
    centerPos: Vector3,
    size: Vector2,
    color: Color,
) {
    unsafe {
        sys::
    }
}

/// Draw a ray line
#[inline]
pub fn DrawRay(
    ray: Ray,
    color: Color,
) {
    unsafe {
        sys::
    }
}

/// Draw a grid (centered at (0, 0, 0))
#[inline]
pub fn DrawGrid(
    slices: i32,
    spacing: f32,
) {
    unsafe {
        sys::
    }
}

//------------------------------------------------------------------------------------
// Model 3d Loading and Drawing Functions (Module: models)
//------------------------------------------------------------------------------------

// Model management functions

/// Load model from files (meshes and materials)
#[inline]
pub fn LoadModel(
    fileName: *const ::std::os::raw::c_char,
) -> Model {
    unsafe {
        sys::
    }
}

/// Load model from generated mesh (default material)
#[inline]
pub fn LoadModelFromMesh(
    mesh: Mesh,
) -> Model {
    unsafe {
        sys::
    }
}

/// Check if a model is valid (loaded in GPU, VAO/VBOs)
#[inline]
pub fn IsModelValid(
    model: Model,
) -> bool {
    unsafe {
        sys::
    }
}

/// Unload model (including meshes) from memory (RAM and/or VRAM)
#[inline]
pub fn UnloadModel(
    model: Model,
) {
    unsafe {
        sys::
    }
}

/// Compute model bounding box limits (considers all meshes)
#[inline]
pub fn GetModelBoundingBox(
    model: Model,
) -> BoundingBox {
    unsafe {
        sys::
    }
}

// Model drawing functions

/// Draw a model (with texture if set)
#[inline]
pub fn DrawModel(
    model: Model,
    position: Vector3,
    scale: f32,
    tint: Color,
) {
    unsafe {
        sys::
    }
}

/// Draw a model with extended parameters
#[inline]
pub fn DrawModelEx(
    model: Model,
    position: Vector3,
    rotationAxis: Vector3,
    rotationAngle: f32,
    scale: Vector3,
    tint: Color,
) {
    unsafe {
        sys::
    }
}

/// Draw a model wires (with texture if set)
#[inline]
pub fn DrawModelWires(
    model: Model,
    position: Vector3,
    scale: f32,
    tint: Color,
) {
    unsafe {
        sys::
    }
}

/// Draw a model wires (with texture if set) with extended parameters
#[inline]
pub fn DrawModelWiresEx(
    model: Model,
    position: Vector3,
    rotationAxis: Vector3,
    rotationAngle: f32,
    scale: Vector3,
    tint: Color,
) {
    unsafe {
        sys::
    }
}

/// Draw a model as points
#[inline]
pub fn DrawModelPoints(
    model: Model,
    position: Vector3,
    scale: f32,
    tint: Color,
) {
    unsafe {
        sys::
    }
}

/// Draw a model as points with extended parameters
#[inline]
pub fn DrawModelPointsEx(
    model: Model,
    position: Vector3,
    rotationAxis: Vector3,
    rotationAngle: f32,
    scale: Vector3,
    tint: Color,
) {
    unsafe {
        sys::
    }
}

/// Draw bounding box (wires)
#[inline]
pub fn DrawBoundingBox(
    box_: BoundingBox,
    color: Color,
) {
    unsafe {
        sys::
    }
}

/// Draw a billboard texture
#[inline]
pub fn DrawBillboard(
    camera: Camera,
    texture: Texture2D,
    position: Vector3,
    scale: f32,
    tint: Color,
) {
    unsafe {
        sys::
    }
}

/// Draw a billboard texture defined by source
#[inline]
pub fn DrawBillboardRec(
    camera: Camera,
    texture: Texture2D,
    source: Rectangle,
    position: Vector3,
    size: Vector2,
    tint: Color,
) {
    unsafe {
        sys::
    }
}

/// Draw a billboard texture defined by source and rotation
#[inline]
pub fn DrawBillboardPro(
    camera: Camera,
    texture: Texture2D,
    source: Rectangle,
    position: Vector3,
    up: Vector3,
    size: Vector2,
    origin: Vector2,
    rotation: f32,
    tint: Color,
) {
    unsafe {
        sys::
    }
}

// Mesh management functions

/// Upload mesh vertex data in GPU and provide VAO/VBO ids
#[inline]
pub fn UploadMesh(
    mesh: *mut Mesh,
    dynamic: bool,
) {
    unsafe {
        sys::
    }
}

/// Update mesh vertex data in GPU for a specific buffer index
#[inline]
pub fn UpdateMeshBuffer(
    mesh: Mesh,
    index: i32,
    data: *const ::std::os::raw::c_void,
    dataSize: i32,
    offset: i32,
) {
    unsafe {
        sys::
    }
}

/// Unload mesh data from CPU and GPU
#[inline]
pub fn UnloadMesh(
    mesh: Mesh,
) {
    unsafe {
        sys::
    }
}

/// Draw a 3d mesh with material and transform
#[inline]
pub fn DrawMesh(
    mesh: Mesh,
    material: Material,
    transform: Matrix,
) {
    unsafe {
        sys::
    }
}

/// Draw multiple mesh instances with material and different transforms
#[inline]
pub fn DrawMeshInstanced(
    mesh: Mesh,
    material: Material,
    transforms: *const Matrix,
    instances: i32,
) {
    unsafe {
        sys::
    }
}

/// Compute mesh bounding box limits
#[inline]
pub fn GetMeshBoundingBox(
    mesh: Mesh,
) -> BoundingBox {
    unsafe {
        sys::
    }
}

/// Compute mesh tangents
#[inline]
pub fn GenMeshTangents(
    mesh: *mut Mesh,
) {
    unsafe {
        sys::
    }
}

/// Export mesh data to file, returns true on success
#[inline]
pub fn ExportMesh(
    mesh: Mesh,
    fileName: *const ::std::os::raw::c_char,
) -> bool {
    unsafe {
        sys::
    }
}

/// Export mesh as code file (.h) defining multiple arrays of vertex attributes
#[inline]
pub fn ExportMeshAsCode(
    mesh: Mesh,
    fileName: *const ::std::os::raw::c_char,
) -> bool {
    unsafe {
        sys::
    }
}

// Mesh generation functions

/// Generate polygonal mesh
#[inline]
pub fn GenMeshPoly(
    sides: i32,
    radius: f32,
) -> Mesh {
    unsafe {
        sys::
    }
}

/// Generate plane mesh (with subdivisions)
#[inline]
pub fn GenMeshPlane(
    width: f32,
    length: f32,
    resX: i32,
    resZ: i32,
) -> Mesh {
    unsafe {
        sys::
    }
}

/// Generate cuboid mesh
#[inline]
pub fn GenMeshCube(
    width: f32,
    height: f32,
    length: f32,
) -> Mesh {
    unsafe {
        sys::
    }
}

/// Generate sphere mesh (standard sphere)
#[inline]
pub fn GenMeshSphere(
    radius: f32,
    rings: i32,
    slices: i32,
) -> Mesh {
    unsafe {
        sys::
    }
}

/// Generate half-sphere mesh (no bottom cap)
#[inline]
pub fn GenMeshHemiSphere(
    radius: f32,
    rings: i32,
    slices: i32,
) -> Mesh {
    unsafe {
        sys::
    }
}

/// Generate cylinder mesh
#[inline]
pub fn GenMeshCylinder(
    radius: f32,
    height: f32,
    slices: i32,
) -> Mesh {
    unsafe {
        sys::
    }
}

/// Generate cone/pyramid mesh
#[inline]
pub fn GenMeshCone(
    radius: f32,
    height: f32,
    slices: i32,
) -> Mesh {
    unsafe {
        sys::
    }
}

/// Generate torus mesh
#[inline]
pub fn GenMeshTorus(
    radius: f32,
    size: f32,
    radSeg: i32,
    sides: i32,
) -> Mesh {
    unsafe {
        sys::
    }
}

/// Generate trefoil knot mesh
#[inline]
pub fn GenMeshKnot(
    radius: f32,
    size: f32,
    radSeg: i32,
    sides: i32,
) -> Mesh {
    unsafe {
        sys::
    }
}

/// Generate heightmap mesh from image data
#[inline]
pub fn GenMeshHeightmap(
    heightmap: Image,
    size: Vector3,
) -> Mesh {
    unsafe {
        sys::
    }
}

/// Generate cubes-based map mesh from image data
#[inline]
pub fn GenMeshCubicmap(
    cubicmap: Image,
    cubeSize: Vector3,
) -> Mesh {
    unsafe {
        sys::
    }
}

// Material loading/unloading functions

/// Load materials from model file
#[inline]
pub fn LoadMaterials(
    fileName: *const ::std::os::raw::c_char,
    materialCount: *mut i32,
) -> *mut Material {
    unsafe {
        sys::
    }
}

/// Load default material (Supports: DIFFUSE, SPECULAR, NORMAL maps)
#[inline]
pub fn LoadMaterialDefault() -> Material {
    unsafe {
        sys::
    }
}

/// Check if a material is valid (shader assigned, map textures loaded in GPU)
#[inline]
pub fn IsMaterialValid(
    material: Material,
) -> bool {
    unsafe {
        sys::
    }
}

/// Unload material from GPU memory (VRAM)
#[inline]
pub fn UnloadMaterial(
    material: Material,
) {
    unsafe {
        sys::
    }
}

/// Set texture for a material map type (MATERIAL_MAP_DIFFUSE, MATERIAL_MAP_SPECULAR...)
#[inline]
pub fn SetMaterialTexture(
    material: *mut Material,
    mapType: i32,
    texture: Texture2D,
) {
    unsafe {
        sys::
    }
}

/// Set material for a mesh
#[inline]
pub fn SetModelMeshMaterial(
    model: *mut Model,
    meshId: i32,
    materialId: i32,
) {
    unsafe {
        sys::
    }
}

// Model animations loading/unloading functions

/// Load model animations from file
#[inline]
pub fn LoadModelAnimations(
    fileName: *const ::std::os::raw::c_char,
    animCount: *mut i32,
) -> *mut ModelAnimation {
    unsafe {
        sys::
    }
}

/// Update model animation pose (CPU)
#[inline]
pub fn UpdateModelAnimation(
    model: Model,
    anim: ModelAnimation,
    frame: i32,
) {
    unsafe {
        sys::
    }
}

/// Update model animation mesh bone matrices (GPU skinning)
#[inline]
pub fn UpdateModelAnimationBones(
    model: Model,
    anim: ModelAnimation,
    frame: i32,
) {
    unsafe {
        sys::
    }
}

/// Unload animation data
#[inline]
pub fn UnloadModelAnimation(
    anim: ModelAnimation,
) {
    unsafe {
        sys::
    }
}

/// Unload animation array data
#[inline]
pub fn UnloadModelAnimations(
    animations: *mut ModelAnimation,
    animCount: i32,
) {
    unsafe {
        sys::
    }
}

/// Check model animation skeleton match
#[inline]
pub fn IsModelAnimationValid(
    model: Model,
    anim: ModelAnimation,
) -> bool {
    unsafe {
        sys::
    }
}

// Collision detection functions

/// Check collision between two spheres
#[inline]
pub fn CheckCollisionSpheres(
    center1: Vector3,
    radius1: f32,
    center2: Vector3,
    radius2: f32,
) -> bool {
    unsafe {
        sys::
    }
}

/// Check collision between two bounding boxes
#[inline]
pub fn CheckCollisionBoxes(
    box1: BoundingBox,
    box2: BoundingBox,
) -> bool {
    unsafe {
        sys::
    }
}

/// Check collision between box and sphere
#[inline]
pub fn CheckCollisionBoxSphere(
    box_: BoundingBox,
    center: Vector3,
    radius: f32,
) -> bool {
    unsafe {
        sys::
    }
}

/// Get collision info between ray and sphere
#[inline]
pub fn GetRayCollisionSphere(
    ray: Ray,
    center: Vector3,
    radius: f32,
) -> RayCollision {
    unsafe {
        sys::
    }
}

/// Get collision info between ray and box
#[inline]
pub fn GetRayCollisionBox(
    ray: Ray,
    box_: BoundingBox,
) -> RayCollision {
    unsafe {
        sys::
    }
}

/// Get collision info between ray and mesh
#[inline]
pub fn GetRayCollisionMesh(
    ray: Ray,
    mesh: Mesh,
    transform: Matrix,
) -> RayCollision {
    unsafe {
        sys::
    }
}

/// Get collision info between ray and triangle
#[inline]
pub fn GetRayCollisionTriangle(
    ray: Ray,
    p1: Vector3,
    p2: Vector3,
    p3: Vector3,
)
-> RayCollision;

/// Get collision info between ray and quad
#[inline]
pub fn GetRayCollisionQuad(
    ray: Ray,
    p1: Vector3,
    p2: Vector3,
    p3: Vector3,
    p4: Vector3,
) -> RayCollision {
    unsafe {
        sys::
    }
}
