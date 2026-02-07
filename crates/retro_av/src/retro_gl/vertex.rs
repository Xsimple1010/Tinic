use retro_core::core::Geometry;

use super::texture::TexturePosition;

pub type Pos = [f32; 2];
#[repr(C, packed)]
pub struct GlVertex(Pos, TexturePosition);

pub fn new_vertex(
    geo: &Geometry,
    window_w: f32,
    window_h: f32,
    origin_w: f32,
    origin_h: f32,
) -> [GlVertex; 4] {
    let (v_bottom, v_right) = resize_vertex_to_aspect(
        *geo.aspect_ratio.read().unwrap(),
        window_w,
        window_h,
        origin_w,
        origin_h,
    );
    let (t_bottom, t_right) = resize_texture(geo, origin_w, origin_h);

    let vertex: [GlVertex; 4] = [
        // vertex_position - texture_coordinate
        GlVertex([-v_bottom, -v_right], [0.0, t_bottom]), //left_bottom
        GlVertex([-v_bottom, v_right], [0.0, 0.0]),       //left_top
        GlVertex([v_bottom, -v_right], [t_right, t_bottom]), //right_bottom
        GlVertex([v_bottom, v_right], [t_right, 0.0]),    //right_top
    ];

    vertex
}

fn resize_texture(geo: &Geometry, origin_w: f32, origin_h: f32) -> (f32, f32) {
    let bottom = origin_h / *geo.max_height.read().unwrap() as f32;
    let right = origin_w / *geo.max_width.read().unwrap() as f32;

    (bottom, right)
}

fn resize_vertex_to_aspect(
    mut aspect: f32,
    window_w: f32,
    window_h: f32,
    origin_w: f32,
    origin_h: f32,
) -> (f32, f32) {
    let mut right: f32 = 1.0;
    let mut bottom: f32 = 1.0;

    if aspect <= 0.0 {
        aspect = origin_w / origin_h;
    }

    if window_h < window_w {
        right = (window_h * aspect) / window_w;
    } else if window_h > window_w {
        bottom = (window_w / aspect) / window_h;
    }

    if right > 1.0 {
        let rest = right - 1.0;
        right -= rest;
        bottom -= rest;
    }

    (right, bottom)
}
