use super::*;

/// Set texture and rectangle to be used on shapes drawing
/// NOTE: It can be useful when using basic shapes and one single font,
/// defining a font char white rectangle would allow drawing everything in a single draw call
#[inline]
pub fn set_shapes_texture(
    texture: sys::Texture2D,
    source: sys::Rectangle,
) {
    unsafe {
        sys::SetShapesTexture(
            texture,
            source,
        );
    }
}

/// Get texture that is used for shapes drawing
#[inline]
pub fn get_shapes_texture() -> sys::Texture2D {
    unsafe {
        sys::GetShapesTexture()
    }
}

/// Get texture source rectangle that is used for shapes drawing
#[inline]
pub fn get_shapes_texture_rectangle() -> sys::Rectangle {
    unsafe {
        sys::GetShapesTextureRectangle()
    }
}

// Basic shapes drawing functions

/// Draw a pixel using geometry [Can be slow, use with care]
#[inline]
pub fn draw_pixel(
    pos_x: i32,
    pos_y: i32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawPixel(
            pos_x,
            pos_y,
            color,
        );
    }
}

/// Draw a pixel using geometry (Vector version) [Can be slow, use with care]
#[inline]
pub fn draw_pixel_v(
    position: sys::Vector2,
    color: sys::Color,
) {
    unsafe {
        sys::DrawPixelV(
            position,
            color,
        );
    }
}

/// Draw a line
#[inline]
pub fn draw_line(
    start_pos_x: i32,
    start_pos_y: i32,
    end_pos_x: i32,
    end_pos_y: i32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawLine(
            start_pos_x,
            start_pos_y,
            end_pos_x,
            end_pos_y,
            color,
        );
    }
}

/// Draw a line (using gl lines)
#[inline]
pub fn draw_line_v(
    start_pos: sys::Vector2,
    end_pos: sys::Vector2,
    color: sys::Color,
) {
    unsafe {
        sys::DrawLineV(
            start_pos,
            end_pos,
            color,
        );
    }
}

/// Draw a line (using triangles/quads)
#[inline]
pub fn draw_line_ex(
    start_pos: sys::Vector2,
    end_pos: sys::Vector2,
    thick: f32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawLineEx(
            start_pos,
            end_pos,
            thick,
            color,
        );
    }
}

/// Draw lines sequence (using gl lines)
#[inline]
pub fn draw_line_strip(
    points: &[sys::Vector2],
    color: sys::Color,
) {
    unsafe {
        sys::DrawLineStrip(
            points.as_ptr(),
            points.len().try_into().unwrap(),
            color,
        );
    }
}

/// Draw line segment cubic-bezier in-out interpolation
#[inline]
pub fn draw_line_bezier(
    start_pos: sys::Vector2,
    end_pos: sys::Vector2,
    thick: f32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawLineBezier(
            start_pos,
            end_pos,
            thick,
            color,
        );
    }
}

/// Draw a color-filled circle
#[inline]
pub fn draw_circle(
    center_x: i32,
    center_y: i32,
    radius: f32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawCircle(
            center_x,
            center_y,
            radius,
            color,
        );
    }
}

/// Draw a piece of a circle
#[inline]
pub fn draw_circle_sector(
    center: sys::Vector2,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    segments: u32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawCircleSector(
            center,
            radius,
            start_angle,
            end_angle,
            segments.try_into().unwrap(),
            color,
        );
    }
}

/// Draw circle sector outline
#[inline]
pub fn draw_circle_sector_lines(
    center: sys::Vector2,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    segments: u32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawCircleSectorLines(
            center,
            radius,
            start_angle,
            end_angle,
            segments.try_into().unwrap(),
            color,
        );
    }
}

/// Draw a gradient-filled circle
#[inline]
pub fn draw_circle_gradient(
    center_x: i32,
    center_y: i32,
    radius: f32,
    inner: sys::Color,
    outer: sys::Color,
) {
    unsafe {
        sys::DrawCircleGradient(
            center_x,
            center_y,
            radius,
            inner,
            outer,
        );
    }
}

/// Draw a color-filled circle (Vector version)
#[inline]
pub fn draw_circle_v(
    center: sys::Vector2,
    radius: f32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawCircleV(
            center,
            radius,
            color,
        );
    }
}

/// Draw circle outline
#[inline]
pub fn draw_circle_lines(
    center_x: i32,
    center_y: i32,
    radius: f32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawCircleLines(
            center_x,
            center_y,
            radius,
            color,
        );
    }
}

/// Draw circle outline (Vector version)
#[inline]
pub fn draw_circle_lines_v(
    center: sys::Vector2,
    radius: f32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawCircleLinesV(
            center,
            radius,
            color,
        );
    }
}

/// Draw ellipse
#[inline]
pub fn draw_ellipse(
    center_x: i32,
    center_y: i32,
    radius_h: f32,
    radius_v: f32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawEllipse(
            center_x,
            center_y,
            radius_h,
            radius_v,
            color,
        );
    }
}

/// Draw ellipse (Vector version)
#[inline]
pub fn draw_ellipse_v(
    center: sys::Vector2,
    radius_h: f32,
    radius_v: f32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawEllipseV(
            center,
            radius_h,
            radius_v,
            color,
        );
    }
}

/// Draw ellipse outline
#[inline]
pub fn draw_ellipse_lines(
    center_x: i32,
    center_y: i32,
    radius_h: f32,
    radius_v: f32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawEllipseLines(
            center_x,
            center_y,
            radius_h,
            radius_v,
            color,
        );
    }
}

/// Draw ellipse outline (Vector version)
#[inline]
pub fn draw_ellipse_lines_v(
    center: sys::Vector2,
    radius_h: f32,
    radius_v: f32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawEllipseLinesV(
            center,
            radius_h,
            radius_v,
            color,
        );
    }
}

/// Draw ring
#[inline]
pub fn draw_ring(
    center: sys::Vector2,
    inner_radius: f32,
    outer_radius: f32,
    start_angle: f32,
    end_angle: f32,
    segments: u32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawRing(
            center,
            inner_radius,
            outer_radius,
            start_angle,
            end_angle,
            segments.try_into().unwrap(),
            color,
        );
    }
}

/// Draw ring outline
#[inline]
pub fn draw_ring_lines(
    center: sys::Vector2,
    inner_radius: f32,
    outer_radius: f32,
    start_angle: f32,
    end_angle: f32,
    segments: u32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawRingLines(
            center,
            inner_radius,
            outer_radius,
            start_angle,
            end_angle,
            segments.try_into().unwrap(),
            color,
        );
    }
}

/// Draw a color-filled rectangle
#[inline]
pub fn draw_rectangle(
    pos_x: i32,
    pos_y: i32,
    width: i32,
    height: i32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawRectangle(
            pos_x,
            pos_y,
            width,
            height,
            color,
        );
    }
}

/// Draw a color-filled rectangle (Vector version)
#[inline]
pub fn draw_rectangle_v(
    position: sys::Vector2,
    size: sys::Vector2,
    color: sys::Color,
) {
    unsafe {
        sys::DrawRectangleV(
            position,
            size,
            color,
        );
    }
}

/// Draw a color-filled rectangle
#[inline]
pub fn draw_rectangle_rec(
    rec: sys::Rectangle,
    color: sys::Color,
) {
    unsafe {
        sys::DrawRectangleRec(
            rec,
            color,
        );
    }
}

/// Draw a color-filled rectangle with pro parameters
#[inline]
pub fn draw_rectangle_pro(
    rec: sys::Rectangle,
    origin: sys::Vector2,
    rotation: f32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawRectanglePro(
            rec,
            origin,
            rotation,
            color,
        );
    }
}

/// Draw a vertical-gradient-filled rectangle
#[inline]
pub fn draw_rectangle_gradient_v(
    pos_x: i32,
    pos_y: i32,
    width: i32,
    height: i32,
    top: sys::Color,
    bottom: sys::Color,
) {
    unsafe {
        sys::DrawRectangleGradientV(
            pos_x,
            pos_y,
            width,
            height,
            top,
            bottom,
        );
    }
}

/// Draw a horizontal-gradient-filled rectangle
#[inline]
pub fn draw_rectangle_gradient_h(
    pos_x: i32,
    pos_y: i32,
    width: i32,
    height: i32,
    left: sys::Color,
    right: sys::Color,
) {
    unsafe {
        sys::DrawRectangleGradientH(
            pos_x,
            pos_y,
            width,
            height,
            left,
            right,
        );
    }
}

/// Draw a gradient-filled rectangle with custom vertex colors
#[inline]
pub fn draw_rectangle_gradient_ex(
    rec: sys::Rectangle,
    top_left: sys::Color,
    bottom_left: sys::Color,
    bottom_right: sys::Color,
    top_right: sys::Color,
) {
    unsafe {
        sys::DrawRectangleGradientEx(
            rec,
            top_left,
            bottom_left,
            bottom_right,
            top_right,
        );
    }
}

/// Draw rectangle outline
#[inline]
pub fn draw_rectangle_lines(
    pos_x: i32,
    pos_y: i32,
    width: i32,
    height: i32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawRectangleLines(
            pos_x,
            pos_y,
            width,
            height,
            color,
        );
    }
}

/// Draw rectangle outline with extended parameters
#[inline]
pub fn draw_rectangle_lines_ex(
    rec: sys::Rectangle,
    line_thick: f32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawRectangleLinesEx(
            rec,
            line_thick,
            color,
        );
    }
}

/// Draw rectangle with rounded edges
#[inline]
pub fn draw_rectangle_rounded(
    rec: sys::Rectangle,
    roundness: f32,
    segments: u32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawRectangleRounded(
            rec,
            roundness,
            segments.try_into().unwrap(),
            color,
        );
    }
}

/// Draw rectangle lines with rounded edges
#[inline]
pub fn draw_rectangle_rounded_lines(
    rec: sys::Rectangle,
    roundness: f32,
    segments: u32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawRectangleRoundedLines(
            rec,
            roundness,
            segments.try_into().unwrap(),
            color,
        );
    }
}

/// Draw rectangle with rounded edges outline
#[inline]
pub fn draw_rectangle_rounded_lines_ex(
    rec: sys::Rectangle,
    roundness: f32,
    segments: u32,
    line_thick: f32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawRectangleRoundedLinesEx(
            rec,
            roundness,
            segments.try_into().unwrap(),
            line_thick,
            color,
        );
    }
}

/// Draw a color-filled triangle (vertex in counter-clockwise order!)
#[inline]
pub fn draw_triangle(
    v1: sys::Vector2,
    v2: sys::Vector2,
    v3: sys::Vector2,
    color: sys::Color,
) {
    unsafe {
        sys::DrawTriangle(
            v1,
            v2,
            v3,
            color,
        );
    }
}

/// Draw triangle outline (vertex in counter-clockwise order!)
#[inline]
pub fn draw_triangle_lines(
    v1: sys::Vector2,
    v2: sys::Vector2,
    v3: sys::Vector2,
    color: sys::Color,
) {
    unsafe {
        sys::DrawTriangleLines(
            v1,
            v2,
            v3,
            color,
        );
    }
}

/// Draw a triangle fan defined by points (first vertex is the center)
#[inline]
pub fn draw_triangle_fan(
    points: &[sys::Vector2],
    color: sys::Color,
) {
    unsafe {
        sys::DrawTriangleFan(
            points.as_ptr(),
            points.len().try_into().unwrap(),
            color,
        );
    }
}

/// Draw a triangle strip defined by points
#[inline]
pub fn draw_triangle_strip(
    points: &[sys::Vector2],
    color: sys::Color,
) {
    unsafe {
        sys::DrawTriangleStrip(
            points.as_ptr(),
            points.len().try_into().unwrap(),
            color,
        );
    }
}

/// Draw a regular polygon (Vector version)
#[inline]
pub fn draw_poly(
    center: sys::Vector2,
    sides: u32,
    radius: f32,
    rotation: f32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawPoly(
            center,
            sides.try_into().unwrap(),
            radius,
            rotation,
            color,
        );
    }
}

/// Draw a polygon outline of n sides
#[inline]
pub fn draw_poly_lines(
    center: sys::Vector2,
    sides: u32,
    radius: f32,
    rotation: f32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawPolyLines(
            center,
            sides.try_into().unwrap(),
            radius,
            rotation,
            color,
        );
    }
}

/// Draw a polygon outline of n sides with extended parameters
#[inline]
pub fn draw_poly_lines_ex(
    center: sys::Vector2,
    sides: u32,
    radius: f32,
    rotation: f32,
    line_thick: f32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawPolyLinesEx(
            center,
            sides.try_into().unwrap(),
            radius,
            rotation,
            line_thick,
            color,
        );
    }
}

// Splines drawing functions

/// Draw spline: Linear, minimum 2 points
#[inline]
pub fn draw_spline_linear(
    points: &[sys::Vector2],
    thick: f32,
    color: sys::Color,
) {
    if cfg!(feature = "assert_spline_point_count") {
        debug_assert!(points.len() >= 2);
    }
    unsafe {
        sys::DrawSplineLinear(
            points.as_ptr(),
            points.len().try_into().unwrap(),
            thick,
            color,
        );
    }
}

/// Draw spline: B-Spline, minimum 4 points
#[inline]
pub fn draw_spline_basis(
    points: &[sys::Vector2],
    thick: f32,
    color: sys::Color,
) {
    if cfg!(feature = "assert_spline_point_count") {
        debug_assert!(points.len() >= 4);
    }
    unsafe {
        sys::DrawSplineBasis(
            points.as_ptr(),
            points.len().try_into().unwrap(),
            thick,
            color,
        );
    }
}

/// Draw spline: Catmull-Rom, minimum 4 points
#[inline]
pub fn draw_spline_catmull_rom(
    points: &[sys::Vector2],
    thick: f32,
    color: sys::Color,
) {
    if cfg!(feature = "assert_spline_point_count") {
        debug_assert!(points.len() >= 4);
    }
    unsafe {
        sys::DrawSplineCatmullRom(
            points.as_ptr(),
            points.len().try_into().unwrap(),
            thick,
            color,
        );
    }
}

/// Draw spline: Quadratic Bezier, minimum 3 points (1 control point): [p1, c2, p3, c4...]
#[inline]
pub fn draw_spline_bezier_quadratic(
    points: &[sys::Vector2],
    thick: f32,
    color: sys::Color,
) {
    if cfg!(feature = "assert_spline_point_count") {
        debug_assert!(points.len().checked_sub(3).is_some_and(|n| n % 2 == 0));
    }
    unsafe {
        sys::DrawSplineBezierQuadratic(
            points.as_ptr(),
            points.len().try_into().unwrap(),
            thick,
            color,
        );
    }
}

/// Draw spline: Cubic Bezier, minimum 4 points (2 control points): [p1, c2, c3, p4, c5, c6...]
#[inline]
pub fn draw_spline_bezier_cubic(
    points: &[sys::Vector2],
    thick: f32,
    color: sys::Color,
) {
    if cfg!(feature = "assert_spline_point_count") {
        debug_assert!(points.len().checked_sub(4).is_some_and(|n| n % 3 == 0));
    }
    unsafe {
        sys::DrawSplineBezierCubic(
            points.as_ptr(),
            points.len().try_into().unwrap(),
            thick,
            color,
        );
    }
}

/// Draw spline segment: Linear, 2 points
#[inline]
pub fn draw_spline_segment_linear(
    p1: sys::Vector2,
    p2: sys::Vector2,
    thick: f32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawSplineSegmentLinear(
            p1,
            p2,
            thick,
            color,
        );
    }
}

/// Draw spline segment: B-Spline, 4 points
#[inline]
pub fn draw_spline_segment_basis(
    p1: sys::Vector2,
    p2: sys::Vector2,
    p3: sys::Vector2,
    p4: sys::Vector2,
    thick: f32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawSplineSegmentBasis(
            p1,
            p2,
            p3,
            p4,
            thick,
            color,
        );
    }
}

/// Draw spline segment: Catmull-Rom, 4 points
#[inline]
pub fn draw_spline_segment_catmull_rom(
    p1: sys::Vector2,
    p2: sys::Vector2,
    p3: sys::Vector2,
    p4: sys::Vector2,
    thick: f32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawSplineSegmentCatmullRom(
            p1,
            p2,
            p3,
            p4,
            thick,
            color,
        );
    }
}

/// Draw spline segment: Quadratic Bezier, 2 points, 1 control point
#[inline]
pub fn draw_spline_segment_bezier_quadratic(
    p1: sys::Vector2,
    c2: sys::Vector2,
    p3: sys::Vector2,
    thick: f32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawSplineSegmentBezierQuadratic(
            p1,
            c2,
            p3,
            thick,
            color,
        );
    }
}

/// Draw spline segment: Cubic Bezier, 2 points, 2 control points
#[inline]
pub fn draw_spline_segment_bezier_cubic(
    p1: sys::Vector2,
    c2: sys::Vector2,
    c3: sys::Vector2,
    p4: sys::Vector2,
    thick: f32,
    color: sys::Color,
) {
    unsafe {
        sys::DrawSplineSegmentBezierCubic(
            p1,
            c2,
            c3,
            p4,
            thick,
            color,
        );
    }
}

// Spline segment point evaluation functions, for a given t [0.0 .. 1.0]

/// Get (evaluate) spline point: Linear
#[inline]
pub fn get_spline_point_linear(
    start_pos: sys::Vector2,
    end_pos: sys::Vector2,
    t: f32,
) -> sys::Vector2 {
    unsafe {
        sys::GetSplinePointLinear(
            start_pos,
            end_pos,
            t,
        )
    }
}

/// Get (evaluate) spline point: B-Spline
#[inline]
pub fn get_spline_point_basis(
    p1: sys::Vector2,
    p2: sys::Vector2,
    p3: sys::Vector2,
    p4: sys::Vector2,
    t: f32,
) -> sys::Vector2 {
    unsafe {
        sys::GetSplinePointBasis(
            p1,
            p2,
            p3,
            p4,
            t,
        )
    }
}

/// Get (evaluate) spline point: Catmull-Rom
#[inline]
pub fn get_spline_point_catmull_rom(
    p1: sys::Vector2,
    p2: sys::Vector2,
    p3: sys::Vector2,
    p4: sys::Vector2,
    t: f32,
) -> sys::Vector2 {
    unsafe {
        sys::GetSplinePointCatmullRom(
            p1,
            p2,
            p3,
            p4,
            t,
        )
    }
}

/// Get (evaluate) spline point: Quadratic Bezier
#[inline]
pub fn get_spline_point_bezier_quad(
    p1: sys::Vector2,
    c2: sys::Vector2,
    p3: sys::Vector2,
    t: f32,
) -> sys::Vector2 {
    unsafe {
        sys::GetSplinePointBezierQuad(
            p1,
            c2,
            p3,
            t,
        )
    }
}

/// Get (evaluate) spline point: Cubic Bezier
#[inline]
pub fn get_spline_point_bezier_cubic(
    p1: sys::Vector2,
    c2: sys::Vector2,
    c3: sys::Vector2,
    p4: sys::Vector2,
    t: f32,
) -> sys::Vector2 {
    unsafe {
        sys::GetSplinePointBezierCubic(
            p1,
            c2,
            c3,
            p4,
            t,
        )
    }
}

// Basic shapes collision detection functions

/// Check collision between two rectangles
#[inline]
pub fn check_collision_recs(
    rec1: sys::Rectangle,
    rec2: sys::Rectangle,
) -> bool {
    unsafe {
        sys::CheckCollisionRecs(
            rec1,
            rec2,
        )
    }
}

/// Check collision between two circles
#[inline]
pub fn check_collision_circles(
    center1: sys::Vector2,
    radius1: f32,
    center2: sys::Vector2,
    radius2: f32,
) -> bool {
    unsafe {
        sys::CheckCollisionCircles(
            center1,
            radius1,
            center2,
            radius2,
        )
    }
}

/// Check collision between circle and rectangle
#[inline]
pub fn check_collision_circle_rec(
    center: sys::Vector2,
    radius: f32,
    rec: sys::Rectangle,
) -> bool {
    unsafe {
        sys::CheckCollisionCircleRec(
            center,
            radius,
            rec,
        )
    }
}

/// Check if circle collides with a line created betweeen two points [p1] and [p2]
#[inline]
pub fn check_collision_circle_line(
    center: sys::Vector2,
    radius: f32,
    p1: sys::Vector2,
    p2: sys::Vector2,
) -> bool {
    unsafe {
        sys::CheckCollisionCircleLine(
            center,
            radius,
            p1,
            p2,
        )
    }
}

/// Check if point is inside rectangle
#[inline]
pub fn check_collision_point_rec(
    point: sys::Vector2,
    rec: sys::Rectangle,
) -> bool {
    unsafe {
        sys::CheckCollisionPointRec(
            point,
            rec,
        )
    }
}

/// Check if point is inside circle
#[inline]
pub fn check_collision_point_circle(
    point: sys::Vector2,
    center: sys::Vector2,
    radius: f32,
) -> bool {
    unsafe {
        sys::CheckCollisionPointCircle(
            point,
            center,
            radius,
        )
    }
}

/// Check if point is inside a triangle
#[inline]
pub fn check_collision_point_triangle(
    point: sys::Vector2,
    p1: sys::Vector2,
    p2: sys::Vector2,
    p3: sys::Vector2,
) -> bool {
    unsafe {
        sys::CheckCollisionPointTriangle(
            point,
            p1,
            p2,
            p3,
        )
    }
}

/// Check if point belongs to line created between two points [p1] and [p2] with defined margin in pixels [threshold]
#[inline]
pub fn check_collision_point_line(
    point: sys::Vector2,
    p1: sys::Vector2,
    p2: sys::Vector2,
    threshold: i32,
) -> bool {
    unsafe {
        sys::CheckCollisionPointLine(
            point,
            p1,
            p2,
            threshold,
        )
    }
}

/// Check if point is within a polygon described by array of vertices
#[inline]
pub fn check_collision_point_poly(
    point: sys::Vector2,
    points: &[sys::Vector2],
) -> bool {
    unsafe {
        sys::CheckCollisionPointPoly(
            point,
            points.as_ptr(),
            points.len().try_into().unwrap(),
        )
    }
}

/// Check the collision between two lines defined by two points each, returns collision point by reference
#[inline]
pub fn check_collision_lines(
    start_pos1: sys::Vector2,
    end_pos1: sys::Vector2,
    start_pos2: sys::Vector2,
    end_pos2: sys::Vector2,
) -> Option<sys::Vector2> {
    let mut collision_point = MaybeUninit::uninit();
    unsafe {
        if sys::CheckCollisionLines(
            start_pos1,
            end_pos1,
            start_pos2,
            end_pos2,
            collision_point.as_mut_ptr(),
        ) {
            Some(collision_point.assume_init())
        } else {
            None
        }
    }
}

/// Get collision rectangle for two rectangles collision
#[inline]
pub fn get_collision_rec(
    rec1: sys::Rectangle,
    rec2: sys::Rectangle,
) -> sys::Rectangle {
    unsafe {
        sys::GetCollisionRec(
            rec1,
            rec2,
        )
    }
}
