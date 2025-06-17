use super::*;

// Basic geometric 3D shapes drawing functions

/// Draw a line in 3D world space
#[allow(non_snake_case)]
#[inline]
pub unsafe fn draw_line3D(
    start_pos: sys::Vector3,
    end_pos: sys::Vector3,
    color: sys::Color,
) {
    unsafe {
        sys::DrawLine3D(
            start_pos,
            end_pos,
            color,
        );
    }
}

/// Draw a point in 3D space, actually a small line
#[allow(non_snake_case)]
#[inline]
pub unsafe fn draw_point3D(
    position: sys::Vector3,
    color: sys::Color,
) {
    unsafe {
        sys::DrawPoint3D(
            position,
            color,
        );
    }
}

/// Draw a circle in 3D world space
#[allow(non_snake_case)]
#[inline]
pub unsafe fn draw_circle3D(
    center: sys::Vector3,
    radius: f32,
    rotation_axis: sys::Vector3,
    rotation_angle: f32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawCircle3D(
            center,
            radius,
            rotation_axis,
            rotation_angle,
            color,
        );
    }
}

/// Draw a color-filled triangle (vertex in counter-clockwise order!)
#[allow(non_snake_case)]
#[inline]
pub unsafe fn draw_triangle3D(
    v1: sys::Vector3,
    v2: sys::Vector3,
    v3: sys::Vector3,
    color: sys::Color,
) {
    unsafe {
        sys::DrawTriangle3D(
            v1,
            v2,
            v3,
            color,
        );
    }
}

/// Draw a triangle strip defined by points
#[allow(non_snake_case)]
#[inline]
pub unsafe fn draw_triangle_strip3D(
    points: &[sys::Vector3],
    color: sys::Color,
) {
    unsafe {
        sys::DrawTriangleStrip3D(
            points.as_ptr(),
            points.len().try_into().unwrap(),
            color,
        );
    }
}

/// Draw cube
#[inline]
pub unsafe fn draw_cube(
    position: sys::Vector3,
    width: f32,
    height: f32,
    length: f32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawCube(
            position,
            width,
            height,
            length,
            color,
        );
    }
}

/// Draw cube (Vector version)
#[inline]
pub unsafe fn draw_cube_v(
    position: sys::Vector3,
    size: sys::Vector3,
    color: sys::Color,
) {
    unsafe {
        sys::DrawCubeV(
            position,
            size,
            color,
        );
    }
}

/// Draw cube wires
#[inline]
pub unsafe fn draw_cube_wires(
    position: sys::Vector3,
    width: f32,
    height: f32,
    length: f32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawCubeWires(
            position,
            width,
            height,
            length,
            color,
        );
    }
}

/// Draw cube wires (Vector version)
#[inline]
pub unsafe fn draw_cube_wires_v(
    position: sys::Vector3,
    size: sys::Vector3,
    color: sys::Color,
) {
    unsafe {
        sys::DrawCubeWiresV(
            position,
            size,
            color,
        )
    }
}

/// Draw sphere
#[inline]
pub unsafe fn draw_sphere(
    center_pos: sys::Vector3,
    radius: f32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawSphere(
            center_pos,
            radius,
            color,
        )
    }
}

/// Draw sphere with extended parameters
#[inline]
pub unsafe fn draw_sphere_ex(
    center_pos: sys::Vector3,
    radius: f32,
    rings: u32,
    slices: u32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawSphereEx(
            center_pos,
            radius,
            rings.try_into().unwrap(),
            slices.try_into().unwrap(),
            color,
        );
    }
}

/// Draw sphere wires
#[inline]
pub unsafe fn draw_sphere_wires(
    center_pos: sys::Vector3,
    radius: f32,
    rings: u32,
    slices: u32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawSphereWires(
            center_pos,
            radius,
            rings.try_into().unwrap(),
            slices.try_into().unwrap(),
            color,
        );
    }
}

/// Draw a cylinder/cone
#[inline]
pub unsafe fn draw_cylinder(
    position: sys::Vector3,
    radius_top: f32,
    radius_bottom: f32,
    height: f32,
    slices: u32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawCylinder(
            position,
            radius_top,
            radius_bottom,
            height,
            slices.try_into().unwrap(),
            color,
        );
    }
}

/// Draw a cylinder with base at start_pos and top at end_pos
#[inline]
pub unsafe fn draw_cylinder_ex(
    start_pos: sys::Vector3,
    end_pos: sys::Vector3,
    start_radius: f32,
    end_radius: f32,
    sides: u32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawCylinderEx(
            start_pos,
            end_pos,
            start_radius,
            end_radius,
            sides.try_into().unwrap(),
            color,
        )
    }
}

/// Draw a cylinder/cone wires
#[inline]
pub unsafe fn draw_cylinder_wires(
    position: sys::Vector3,
    radius_top: f32,
    radius_bottom: f32,
    height: f32,
    slices: u32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawCylinderWires(
            position,
            radius_top,
            radius_bottom,
            height,
            slices.try_into().unwrap(),
            color,
        );
    }
}

/// Draw a cylinder wires with base at start_pos and top at end_pos
#[inline]
pub unsafe fn draw_cylinder_wires_ex(
    start_pos: sys::Vector3,
    end_pos: sys::Vector3,
    start_radius: f32,
    end_radius: f32,
    sides: u32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawCylinderWiresEx(
            start_pos,
            end_pos,
            start_radius,
            end_radius,
            sides.try_into().unwrap(),
            color,
        );
    }
}

/// Draw a capsule with the center of its sphere caps at start_pos and end_pos
#[inline]
pub unsafe fn draw_capsule(
    start_pos: sys::Vector3,
    end_pos: sys::Vector3,
    radius: f32,
    slices: u32,
    rings: u32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawCapsule(
            start_pos,
            end_pos,
            radius,
            slices.try_into().unwrap(),
            rings.try_into().unwrap(),
            color,
        );
    }
}

/// Draw capsule wireframe with the center of its sphere caps at start_pos and end_pos
#[inline]
pub unsafe fn draw_capsule_wires(
    start_pos: sys::Vector3,
    end_pos: sys::Vector3,
    radius: f32,
    slices: u32,
    rings: u32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawCapsuleWires(
            start_pos,
            end_pos,
            radius,
            slices.try_into().unwrap(),
            rings.try_into().unwrap(),
            color,
        );
    }
}

/// Draw a plane XZ
#[inline]
pub unsafe fn draw_plane(
    center_pos: sys::Vector3,
    size: sys::Vector2,
    color: sys::Color,
) {
    unsafe {
        sys::DrawPlane(
            center_pos,
            size,
            color,
        )
    }
}

/// Draw a ray line
#[inline]
pub unsafe fn draw_ray(
    ray: sys::Ray,
    color: sys::Color,
) {
    unsafe {
        sys::DrawRay(
            ray,
            color,
        );
    }
}

/// Draw a grid (centered at (0, 0, 0))
#[inline]
pub unsafe fn draw_grid(
    slices: u32,
    spacing: f32,
) {
    unsafe {
        sys::DrawGrid(
            slices.try_into().unwrap(),
            spacing,
        );
    }
}

//------------------------------------------------------------------------------------
// Model 3d Loading and Drawing Functions (Module: models)
//------------------------------------------------------------------------------------

// Model management functions

// /// Load model from files (meshes and materials)
// #[inline]
// pub unsafe fn LoadModel(
//     fileName: *const ::std::os::raw::c_char,
// ) -> sys::Model {
//     unsafe {
//         sys::
//     }
// }

// /// Load model from generated mesh (default material)
// #[inline]
// pub unsafe fn LoadModelFromMesh(
//     mesh: sys::Mesh,
// ) -> sys::Model {
//     unsafe {
//         sys::
//     }
// }

// /// Check if a model is valid (loaded in GPU, VAO/VBOs)
// #[inline]
// pub unsafe fn IsModelValid(
//     model: sys::Model,
// ) -> bool {
//     unsafe {
//         sys::
//     }
// }

// /// Unload model (including meshes) from memory (RAM and/or VRAM)
// #[inline]
// pub unsafe fn UnloadModel(
//     model: sys::Model,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Compute model bounding box limits (considers all meshes)
// #[inline]
// pub unsafe fn GetModelBoundingBox(
//     model: sys::Model,
// ) -> sys::BoundingBox {
//     unsafe {
//         sys::
//     }
// }

// // Model drawing functions

// /// Draw a model (with texture if set)
// #[inline]
// pub unsafe fn DrawModel(
//     model: sys::Model,
//     position: sys::Vector3,
//     scale: f32,
//     tint: sys::Color,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Draw a model with extended parameters
// #[inline]
// pub unsafe fn DrawModelEx(
//     model: sys::Model,
//     position: sys::Vector3,
//     rotationAxis: sys::Vector3,
//     rotationAngle: f32,
//     scale: sys::Vector3,
//     tint: sys::Color,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Draw a model wires (with texture if set)
// #[inline]
// pub unsafe fn DrawModelWires(
//     model: sys::Model,
//     position: sys::Vector3,
//     scale: f32,
//     tint: sys::Color,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Draw a model wires (with texture if set) with extended parameters
// #[inline]
// pub unsafe fn DrawModelWiresEx(
//     model: sys::Model,
//     position: sys::Vector3,
//     rotationAxis: sys::Vector3,
//     rotationAngle: f32,
//     scale: sys::Vector3,
//     tint: sys::Color,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Draw a model as points
// #[inline]
// pub unsafe fn DrawModelPoints(
//     model: sys::Model,
//     position: sys::Vector3,
//     scale: f32,
//     tint: sys::Color,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Draw a model as points with extended parameters
// #[inline]
// pub unsafe fn DrawModelPointsEx(
//     model: sys::Model,
//     position: sys::Vector3,
//     rotationAxis: sys::Vector3,
//     rotationAngle: f32,
//     scale: sys::Vector3,
//     tint: sys::Color,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Draw bounding box (wires)
// #[inline]
// pub unsafe fn DrawBoundingBox(
//     box_: sys::BoundingBox,
//     color: sys::Color,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Draw a billboard texture
// #[inline]
// pub unsafe fn DrawBillboard(
//     camera: sys::Camera,
//     texture: sys::Texture2D,
//     position: sys::Vector3,
//     scale: f32,
//     tint: sys::Color,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Draw a billboard texture defined by source
// #[inline]
// pub unsafe fn DrawBillboardRec(
//     camera: sys::Camera,
//     texture: sys::Texture2D,
//     source: sys::Rectangle,
//     position: sys::Vector3,
//     size: sys::Vector2,
//     tint: sys::Color,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Draw a billboard texture defined by source and rotation
// #[inline]
// pub unsafe fn DrawBillboardPro(
//     camera: sys::Camera,
//     texture: sys::Texture2D,
//     source: sys::Rectangle,
//     position: sys::Vector3,
//     up: sys::Vector3,
//     size: sys::Vector2,
//     origin: sys::Vector2,
//     rotation: f32,
//     tint: sys::Color,
// ) {
//     unsafe {
//         sys::
//     }
// }

// // Mesh management functions

// /// Upload mesh vertex data in GPU and provide VAO/VBO ids
// #[inline]
// pub unsafe fn UploadMesh(
//     mesh: *mut sys::Mesh,
//     dynamic: bool,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Update mesh vertex data in GPU for a specific buffer index
// #[inline]
// pub unsafe fn UpdateMeshBuffer(
//     mesh: sys::Mesh,
//     index: i32,
//     data: *const ::std::os::raw::c_void,
//     dataSize: i32,
//     offset: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Unload mesh data from CPU and GPU
// #[inline]
// pub unsafe fn UnloadMesh(
//     mesh: sys::Mesh,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Draw a 3d mesh with material and transform
// #[inline]
// pub unsafe fn DrawMesh(
//     mesh: sys::Mesh,
//     material: sys::Material,
//     transform: sys::Matrix,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Draw multiple mesh instances with material and different transforms
// #[inline]
// pub unsafe fn DrawMeshInstanced(
//     mesh: sys::Mesh,
//     material: sys::Material,
//     transforms: *const sys::Matrix,
//     instances: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Compute mesh bounding box limits
// #[inline]
// pub unsafe fn GetMeshBoundingBox(
//     mesh: sys::Mesh,
// ) -> sys::BoundingBox {
//     unsafe {
//         sys::
//     }
// }

// /// Compute mesh tangents
// #[inline]
// pub unsafe fn GenMeshTangents(
//     mesh: *mut sys::Mesh,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Export mesh data to file, returns true on success
// #[inline]
// pub unsafe fn ExportMesh(
//     mesh: sys::Mesh,
//     fileName: *const ::std::os::raw::c_char,
// ) -> bool {
//     unsafe {
//         sys::
//     }
// }

// /// Export mesh as code file (.h) defining multiple arrays of vertex attributes
// #[inline]
// pub unsafe fn ExportMeshAsCode(
//     mesh: sys::Mesh,
//     fileName: *const ::std::os::raw::c_char,
// ) -> bool {
//     unsafe {
//         sys::
//     }
// }

// // Mesh generation functions

// /// Generate polygonal mesh
// #[inline]
// pub unsafe fn GenMeshPoly(
//     sides: i32,
//     radius: f32,
// ) -> sys::Mesh {
//     unsafe {
//         sys::
//     }
// }

// /// Generate plane mesh (with subdivisions)
// #[inline]
// pub unsafe fn GenMeshPlane(
//     width: f32,
//     length: f32,
//     resX: i32,
//     resZ: i32,
// ) -> sys::Mesh {
//     unsafe {
//         sys::
//     }
// }

// /// Generate cuboid mesh
// #[inline]
// pub unsafe fn GenMeshCube(
//     width: f32,
//     height: f32,
//     length: f32,
// ) -> sys::Mesh {
//     unsafe {
//         sys::
//     }
// }

// /// Generate sphere mesh (standard sphere)
// #[inline]
// pub unsafe fn GenMeshSphere(
//     radius: f32,
//     rings: i32,
//     slices: i32,
// ) -> sys::Mesh {
//     unsafe {
//         sys::
//     }
// }

// /// Generate half-sphere mesh (no bottom cap)
// #[inline]
// pub unsafe fn GenMeshHemiSphere(
//     radius: f32,
//     rings: i32,
//     slices: i32,
// ) -> sys::Mesh {
//     unsafe {
//         sys::
//     }
// }

// /// Generate cylinder mesh
// #[inline]
// pub unsafe fn GenMeshCylinder(
//     radius: f32,
//     height: f32,
//     slices: i32,
// ) -> sys::Mesh {
//     unsafe {
//         sys::
//     }
// }

// /// Generate cone/pyramid mesh
// #[inline]
// pub unsafe fn GenMeshCone(
//     radius: f32,
//     height: f32,
//     slices: i32,
// ) -> sys::Mesh {
//     unsafe {
//         sys::
//     }
// }

// /// Generate torus mesh
// #[inline]
// pub unsafe fn GenMeshTorus(
//     radius: f32,
//     size: f32,
//     radSeg: i32,
//     sides: i32,
// ) -> sys::Mesh {
//     unsafe {
//         sys::
//     }
// }

// /// Generate trefoil knot mesh
// #[inline]
// pub unsafe fn GenMeshKnot(
//     radius: f32,
//     size: f32,
//     radSeg: i32,
//     sides: i32,
// ) -> sys::Mesh {
//     unsafe {
//         sys::
//     }
// }

// /// Generate heightmap mesh from image data
// #[inline]
// pub unsafe fn GenMeshHeightmap(
//     heightmap: sys::Image,
//     size: sys::Vector3,
// ) -> sys::Mesh {
//     unsafe {
//         sys::
//     }
// }

// /// Generate cubes-based map mesh from image data
// #[inline]
// pub unsafe fn GenMeshCubicmap(
//     cubicmap: sys::Image,
//     cubeSize: sys::Vector3,
// ) -> sys::Mesh {
//     unsafe {
//         sys::
//     }
// }

// // Material loading/unloading functions

// /// Load materials from model file
// #[inline]
// pub unsafe fn LoadMaterials(
//     fileName: *const ::std::os::raw::c_char,
//     materialCount: *mut i32,
// ) -> *mut sys::Material {
//     unsafe {
//         sys::
//     }
// }

// /// Load default material (Supports: DIFFUSE, SPECULAR, NORMAL maps)
// #[inline]
// pub unsafe fn LoadMaterialDefault() -> sys::Material {
//     unsafe {
//         sys::
//     }
// }

// /// Check if a material is valid (shader assigned, map textures loaded in GPU)
// #[inline]
// pub unsafe fn IsMaterialValid(
//     material: sys::Material,
// ) -> bool {
//     unsafe {
//         sys::
//     }
// }

// /// Unload material from GPU memory (VRAM)
// #[inline]
// pub unsafe fn UnloadMaterial(
//     material: sys::Material,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Set texture for a material map type (MATERIAL_MAP_DIFFUSE, MATERIAL_MAP_SPECULAR...)
// #[inline]
// pub unsafe fn SetMaterialTexture(
//     material: *mut sys::Material,
//     mapType: i32,
//     texture: sys::Texture2D,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Set material for a mesh
// #[inline]
// pub unsafe fn SetModelMeshMaterial(
//     model: *mut sys::Model,
//     meshId: i32,
//     materialId: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// // Model animations loading/unloading functions

// /// Load model animations from file
// #[inline]
// pub unsafe fn LoadModelAnimations(
//     fileName: *const ::std::os::raw::c_char,
//     animCount: *mut i32,
// ) -> *mut sys::ModelAnimation {
//     unsafe {
//         sys::
//     }
// }

// /// Update model animation pose (CPU)
// #[inline]
// pub unsafe fn UpdateModelAnimation(
//     model: sys::Model,
//     anim: sys::ModelAnimation,
//     frame: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Update model animation mesh bone matrices (GPU skinning)
// #[inline]
// pub unsafe fn UpdateModelAnimationBones(
//     model: sys::Model,
//     anim: sys::ModelAnimation,
//     frame: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Unload animation data
// #[inline]
// pub unsafe fn UnloadModelAnimation(
//     anim: sys::ModelAnimation,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Unload animation array data
// #[inline]
// pub unsafe fn UnloadModelAnimations(
//     animations: *mut sys::ModelAnimation,
//     animCount: i32,
// ) {
//     unsafe {
//         sys::
//     }
// }

// /// Check model animation skeleton match
// #[inline]
// pub unsafe fn IsModelAnimationValid(
//     model: sys::Model,
//     anim: sys::ModelAnimation,
// ) -> bool {
//     unsafe {
//         sys::
//     }
// }

// // Collision detection functions

// /// Check collision between two spheres
// #[inline]
// pub unsafe fn CheckCollisionSpheres(
//     center1: sys::Vector3,
//     radius1: f32,
//     center2: sys::Vector3,
//     radius2: f32,
// ) -> bool {
//     unsafe {
//         sys::
//     }
// }

// /// Check collision between two bounding boxes
// #[inline]
// pub unsafe fn CheckCollisionBoxes(
//     box1: sys::BoundingBox,
//     box2: sys::BoundingBox,
// ) -> bool {
//     unsafe {
//         sys::
//     }
// }

// /// Check collision between box and sphere
// #[inline]
// pub unsafe fn CheckCollisionBoxSphere(
//     box_: sys::BoundingBox,
//     center: sys::Vector3,
//     radius: f32,
// ) -> bool {
//     unsafe {
//         sys::
//     }
// }

// /// Get collision info between ray and sphere
// #[inline]
// pub unsafe fn GetRayCollisionSphere(
//     ray: sys::Ray,
//     center: sys::Vector3,
//     radius: f32,
// ) -> sys::RayCollision {
//     unsafe {
//         sys::
//     }
// }

// /// Get collision info between ray and box
// #[inline]
// pub unsafe fn GetRayCollisionBox(
//     ray: sys::Ray,
//     box_: sys::BoundingBox,
// ) -> sys::RayCollision {
//     unsafe {
//         sys::
//     }
// }

// /// Get collision info between ray and mesh
// #[inline]
// pub unsafe fn GetRayCollisionMesh(
//     ray: sys::Ray,
//     mesh: sys::Mesh,
//     transform: sys::Matrix,
// ) -> sys::RayCollision {
//     unsafe {
//         sys::
//     }
// }

// /// Get collision info between ray and triangle
// #[inline]
// pub unsafe fn GetRayCollisionTriangle(
//     ray: sys::Ray,
//     p1: sys::Vector3,
//     p2: sys::Vector3,
//     p3: sys::Vector3,
// ) -> sys::RayCollision {
//     unsafe {
//         sys::
//     }
// }

// /// Get collision info between ray and quad
// #[inline]
// pub unsafe fn GetRayCollisionQuad(
//     ray: sys::Ray,
//     p1: sys::Vector3,
//     p2: sys::Vector3,
//     p3: sys::Vector3,
//     p4: sys::Vector3,
// ) -> sys::RayCollision {
//     unsafe {
//         sys::
//     }
// }
