use super::*;

// Basic geometric 3D shapes drawing functions

/// Draw a line in 3D world space
#[inline]
pub fn DrawLine3D(startPos: Vector3, endPos: Vector3, color: Color);

/// Draw a point in 3D space, actually a small line
#[inline]
pub fn DrawPoint3D(position: Vector3, color: Color);

/// Draw a circle in 3D world space
#[inline]
pub fn DrawCircle3D(
    center: Vector3,
    radius: ::std::os::raw::c_float,
    rotationAxis: Vector3,
    rotationAngle: ::std::os::raw::c_float,
    color: Color,
);

/// Draw a color-filled triangle (vertex in counter-clockwise order!)
#[inline]
pub fn DrawTriangle3D(v1: Vector3, v2: Vector3, v3: Vector3, color: Color);

/// Draw a triangle strip defined by points
#[inline]
pub fn DrawTriangleStrip3D(
    points: *const Vector3,
    pointCount: ::std::os::raw::c_int,
    color: Color,
);

/// Draw cube
#[inline]
pub fn DrawCube(
    position: Vector3,
    width: ::std::os::raw::c_float,
    height: ::std::os::raw::c_float,
    length: ::std::os::raw::c_float,
    color: Color,
);

/// Draw cube (Vector version)
#[inline]
pub fn DrawCubeV(position: Vector3, size: Vector3, color: Color);

/// Draw cube wires
#[inline]
pub fn DrawCubeWires(
    position: Vector3,
    width: ::std::os::raw::c_float,
    height: ::std::os::raw::c_float,
    length: ::std::os::raw::c_float,
    color: Color,
);

/// Draw cube wires (Vector version)
#[inline]
pub fn DrawCubeWiresV(position: Vector3, size: Vector3, color: Color);

/// Draw sphere
#[inline]
pub fn DrawSphere(centerPos: Vector3, radius: ::std::os::raw::c_float, color: Color);

/// Draw sphere with extended parameters
#[inline]
pub fn DrawSphereEx(
    centerPos: Vector3,
    radius: ::std::os::raw::c_float,
    rings: ::std::os::raw::c_int,
    slices: ::std::os::raw::c_int,
    color: Color,
);

/// Draw sphere wires
#[inline]
pub fn DrawSphereWires(
    centerPos: Vector3,
    radius: ::std::os::raw::c_float,
    rings: ::std::os::raw::c_int,
    slices: ::std::os::raw::c_int,
    color: Color,
);

/// Draw a cylinder/cone
#[inline]
pub fn DrawCylinder(
    position: Vector3,
    radiusTop: ::std::os::raw::c_float,
    radiusBottom: ::std::os::raw::c_float,
    height: ::std::os::raw::c_float,
    slices: ::std::os::raw::c_int,
    color: Color,
);

/// Draw a cylinder with base at startPos and top at endPos
#[inline]
pub fn DrawCylinderEx(
    startPos: Vector3,
    endPos: Vector3,
    startRadius: ::std::os::raw::c_float,
    endRadius: ::std::os::raw::c_float,
    sides: ::std::os::raw::c_int,
    color: Color,
);

/// Draw a cylinder/cone wires
#[inline]
pub fn DrawCylinderWires(
    position: Vector3,
    radiusTop: ::std::os::raw::c_float,
    radiusBottom: ::std::os::raw::c_float,
    height: ::std::os::raw::c_float,
    slices: ::std::os::raw::c_int,
    color: Color,
);

/// Draw a cylinder wires with base at startPos and top at endPos
#[inline]
pub fn DrawCylinderWiresEx(
    startPos: Vector3,
    endPos: Vector3,
    startRadius: ::std::os::raw::c_float,
    endRadius: ::std::os::raw::c_float,
    sides: ::std::os::raw::c_int,
    color: Color,
);

/// Draw a capsule with the center of its sphere caps at startPos and endPos
#[inline]
pub fn DrawCapsule(
    startPos: Vector3,
    endPos: Vector3,
    radius: ::std::os::raw::c_float,
    slices: ::std::os::raw::c_int,
    rings: ::std::os::raw::c_int,
    color: Color,
);

/// Draw capsule wireframe with the center of its sphere caps at startPos and endPos
#[inline]
pub fn DrawCapsuleWires(
    startPos: Vector3,
    endPos: Vector3,
    radius: ::std::os::raw::c_float,
    slices: ::std::os::raw::c_int,
    rings: ::std::os::raw::c_int,
    color: Color,
);

/// Draw a plane XZ
#[inline]
pub fn DrawPlane(centerPos: Vector3, size: Vector2, color: Color);

/// Draw a ray line
#[inline]
pub fn DrawRay(ray: Ray, color: Color);

/// Draw a grid (centered at (0, 0, 0))
#[inline]
pub fn DrawGrid(slices: ::std::os::raw::c_int, spacing: ::std::os::raw::c_float);

//------------------------------------------------------------------------------------
// Model 3d Loading and Drawing Functions (Module: models)
//------------------------------------------------------------------------------------

// Model management functions

/// Load model from files (meshes and materials)
#[inline]
pub fn LoadModel(fileName: *const ::std::os::raw::c_char) -> Model;

/// Load model from generated mesh (default material)
#[inline]
pub fn LoadModelFromMesh(mesh: Mesh) -> Model;

/// Check if a model is valid (loaded in GPU, VAO/VBOs)
#[inline]
pub fn IsModelValid(model: Model) -> bool;

/// Unload model (including meshes) from memory (RAM and/or VRAM)
#[inline]
pub fn UnloadModel(model: Model);

/// Compute model bounding box limits (considers all meshes)
#[inline]
pub fn GetModelBoundingBox(model: Model) -> BoundingBox;

// Model drawing functions

/// Draw a model (with texture if set)
#[inline]
pub fn DrawModel(model: Model, position: Vector3, scale: ::std::os::raw::c_float, tint: Color);

/// Draw a model with extended parameters
#[inline]
pub fn DrawModelEx(
    model: Model,
    position: Vector3,
    rotationAxis: Vector3,
    rotationAngle: ::std::os::raw::c_float,
    scale: Vector3,
    tint: Color,
);

/// Draw a model wires (with texture if set)
#[inline]
pub fn DrawModelWires(
    model: Model,
    position: Vector3,
    scale: ::std::os::raw::c_float,
    tint: Color,
);

/// Draw a model wires (with texture if set) with extended parameters
#[inline]
pub fn DrawModelWiresEx(
    model: Model,
    position: Vector3,
    rotationAxis: Vector3,
    rotationAngle: ::std::os::raw::c_float,
    scale: Vector3,
    tint: Color,
);

/// Draw a model as points
#[inline]
pub fn DrawModelPoints(
    model: Model,
    position: Vector3,
    scale: ::std::os::raw::c_float,
    tint: Color,
);

/// Draw a model as points with extended parameters
#[inline]
pub fn DrawModelPointsEx(
    model: Model,
    position: Vector3,
    rotationAxis: Vector3,
    rotationAngle: ::std::os::raw::c_float,
    scale: Vector3,
    tint: Color,
);

/// Draw bounding box (wires)
#[inline]
pub fn DrawBoundingBox(box_: BoundingBox, color: Color);

/// Draw a billboard texture
#[inline]
pub fn DrawBillboard(
    camera: Camera,
    texture: Texture2D,
    position: Vector3,
    scale: ::std::os::raw::c_float,
    tint: Color,
);

/// Draw a billboard texture defined by source
#[inline]
pub fn DrawBillboardRec(
    camera: Camera,
    texture: Texture2D,
    source: Rectangle,
    position: Vector3,
    size: Vector2,
    tint: Color,
);

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
    rotation: ::std::os::raw::c_float,
    tint: Color,
);

// Mesh management functions

/// Upload mesh vertex data in GPU and provide VAO/VBO ids
#[inline]
pub fn UploadMesh(mesh: *mut Mesh, dynamic: bool);

/// Update mesh vertex data in GPU for a specific buffer index
#[inline]
pub fn UpdateMeshBuffer(
    mesh: Mesh,
    index: ::std::os::raw::c_int,
    data: *const ::std::os::raw::c_void,
    dataSize: ::std::os::raw::c_int,
    offset: ::std::os::raw::c_int,
);

/// Unload mesh data from CPU and GPU
#[inline]
pub fn UnloadMesh(mesh: Mesh);

/// Draw a 3d mesh with material and transform
#[inline]
pub fn DrawMesh(mesh: Mesh, material: Material, transform: Matrix);

/// Draw multiple mesh instances with material and different transforms
#[inline]
pub fn DrawMeshInstanced(
    mesh: Mesh,
    material: Material,
    transforms: *const Matrix,
    instances: ::std::os::raw::c_int,
);

/// Compute mesh bounding box limits
#[inline]
pub fn GetMeshBoundingBox(mesh: Mesh) -> BoundingBox;

/// Compute mesh tangents
#[inline]
pub fn GenMeshTangents(mesh: *mut Mesh);

/// Export mesh data to file, returns true on success
#[inline]
pub fn ExportMesh(mesh: Mesh, fileName: *const ::std::os::raw::c_char) -> bool;

/// Export mesh as code file (.h) defining multiple arrays of vertex attributes
#[inline]
pub fn ExportMeshAsCode(mesh: Mesh, fileName: *const ::std::os::raw::c_char) -> bool;

// Mesh generation functions

/// Generate polygonal mesh
#[inline]
pub fn GenMeshPoly(sides: ::std::os::raw::c_int, radius: ::std::os::raw::c_float) -> Mesh;

/// Generate plane mesh (with subdivisions)
#[inline]
pub fn GenMeshPlane(
    width: ::std::os::raw::c_float,
    length: ::std::os::raw::c_float,
    resX: ::std::os::raw::c_int,
    resZ: ::std::os::raw::c_int,
) -> Mesh;

/// Generate cuboid mesh
#[inline]
pub fn GenMeshCube(
    width: ::std::os::raw::c_float,
    height: ::std::os::raw::c_float,
    length: ::std::os::raw::c_float,
) -> Mesh;

/// Generate sphere mesh (standard sphere)
#[inline]
pub fn GenMeshSphere(
    radius: ::std::os::raw::c_float,
    rings: ::std::os::raw::c_int,
    slices: ::std::os::raw::c_int,
) -> Mesh;

/// Generate half-sphere mesh (no bottom cap)
#[inline]
pub fn GenMeshHemiSphere(
    radius: ::std::os::raw::c_float,
    rings: ::std::os::raw::c_int,
    slices: ::std::os::raw::c_int,
) -> Mesh;

/// Generate cylinder mesh
#[inline]
pub fn GenMeshCylinder(
    radius: ::std::os::raw::c_float,
    height: ::std::os::raw::c_float,
    slices: ::std::os::raw::c_int,
) -> Mesh;

/// Generate cone/pyramid mesh
#[inline]
pub fn GenMeshCone(
    radius: ::std::os::raw::c_float,
    height: ::std::os::raw::c_float,
    slices: ::std::os::raw::c_int,
) -> Mesh;

/// Generate torus mesh
#[inline]
pub fn GenMeshTorus(
    radius: ::std::os::raw::c_float,
    size: ::std::os::raw::c_float,
    radSeg: ::std::os::raw::c_int,
    sides: ::std::os::raw::c_int,
) -> Mesh;

/// Generate trefoil knot mesh
#[inline]
pub fn GenMeshKnot(
    radius: ::std::os::raw::c_float,
    size: ::std::os::raw::c_float,
    radSeg: ::std::os::raw::c_int,
    sides: ::std::os::raw::c_int,
) -> Mesh;

/// Generate heightmap mesh from image data
#[inline]
pub fn GenMeshHeightmap(heightmap: Image, size: Vector3) -> Mesh;

/// Generate cubes-based map mesh from image data
#[inline]
pub fn GenMeshCubicmap(cubicmap: Image, cubeSize: Vector3) -> Mesh;

// Material loading/unloading functions

/// Load materials from model file
#[inline]
pub fn LoadMaterials(
    fileName: *const ::std::os::raw::c_char,
    materialCount: *mut ::std::os::raw::c_int,
) -> *mut Material;

/// Load default material (Supports: DIFFUSE, SPECULAR, NORMAL maps)
#[inline]
pub fn LoadMaterialDefault() -> Material;

/// Check if a material is valid (shader assigned, map textures loaded in GPU)
#[inline]
pub fn IsMaterialValid(material: Material) -> bool;

/// Unload material from GPU memory (VRAM)
#[inline]
pub fn UnloadMaterial(material: Material);

/// Set texture for a material map type (MATERIAL_MAP_DIFFUSE, MATERIAL_MAP_SPECULAR...)
#[inline]
pub fn SetMaterialTexture(
    material: *mut Material,
    mapType: ::std::os::raw::c_int,
    texture: Texture2D,
);

/// Set material for a mesh
#[inline]
pub fn SetModelMeshMaterial(
    model: *mut Model,
    meshId: ::std::os::raw::c_int,
    materialId: ::std::os::raw::c_int,
);

// Model animations loading/unloading functions

/// Load model animations from file
#[inline]
pub fn LoadModelAnimations(
    fileName: *const ::std::os::raw::c_char,
    animCount: *mut ::std::os::raw::c_int,
) -> *mut ModelAnimation;

/// Update model animation pose (CPU)
#[inline]
pub fn UpdateModelAnimation(model: Model, anim: ModelAnimation, frame: ::std::os::raw::c_int);

/// Update model animation mesh bone matrices (GPU skinning)
#[inline]
pub fn UpdateModelAnimationBones(
    model: Model,
    anim: ModelAnimation,
    frame: ::std::os::raw::c_int,
);

/// Unload animation data
#[inline]
pub fn UnloadModelAnimation(anim: ModelAnimation);

/// Unload animation array data
#[inline]
pub fn UnloadModelAnimations(animations: *mut ModelAnimation, animCount: ::std::os::raw::c_int);

/// Check model animation skeleton match
#[inline]
pub fn IsModelAnimationValid(model: Model, anim: ModelAnimation) -> bool;

// Collision detection functions

/// Check collision between two spheres
#[inline]
pub fn CheckCollisionSpheres(
    center1: Vector3,
    radius1: ::std::os::raw::c_float,
    center2: Vector3,
    radius2: ::std::os::raw::c_float,
) -> bool;

/// Check collision between two bounding boxes
#[inline]
pub fn CheckCollisionBoxes(box1: BoundingBox, box2: BoundingBox) -> bool;

/// Check collision between box and sphere
#[inline]
pub fn CheckCollisionBoxSphere(
    box_: BoundingBox,
    center: Vector3,
    radius: ::std::os::raw::c_float,
) -> bool;

/// Get collision info between ray and sphere
#[inline]
pub fn GetRayCollisionSphere(
    ray: Ray,
    center: Vector3,
    radius: ::std::os::raw::c_float,
) -> RayCollision;

/// Get collision info between ray and box
#[inline]
pub fn GetRayCollisionBox(ray: Ray, box_: BoundingBox) -> RayCollision;

/// Get collision info between ray and mesh
#[inline]
pub fn GetRayCollisionMesh(ray: Ray, mesh: Mesh, transform: Matrix) -> RayCollision;

/// Get collision info between ray and triangle
#[inline]
pub fn GetRayCollisionTriangle(ray: Ray, p1: Vector3, p2: Vector3, p3: Vector3)
-> RayCollision;

/// Get collision info between ray and quad
#[inline]
pub fn GetRayCollisionQuad(
    ray: Ray,
    p1: Vector3,
    p2: Vector3,
    p3: Vector3,
    p4: Vector3,
) -> RayCollision;
