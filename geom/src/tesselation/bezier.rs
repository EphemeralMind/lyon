use tesselation::{ Index };
use tesselation::vertex_builder::VertexBufferBuilder;
use tesselation::vectors::{ Vec2, vec2_mul, vec2_add, vec2_sub, vec2_cross };

pub fn triangulate_quadratic_bezier<Geometry: VertexBufferBuilder<Vec2>>(
    from: Vec2,
    ctrl: Vec2,
    to: Vec2,
    num_points: u32,
    output: &mut Geometry
) {
    output.begin_geometry();
    println!("triangulate quadratic {:?} {:?} {:?}", from, ctrl, to);
    if vec2_cross(vec2_sub(to, from), vec2_sub(ctrl, from)) < 0.0 {
        // ctrl is outside the shape
        for i in 1..((num_points-1) as Index) {
            output.push_indices(0, i, i+1);
        }
    } else {
        // ctrl is inside the shape
        output.push_vertex(ctrl);
        for i in 1..(num_points as Index) {
            if i == i {
                output.push_indices(0, i, i+1);
            }
        }
    }
    for i in 0..num_points {
        let t: f32 = i as f32 / ((num_points - 1) as f32);
        let t2 = t*t;
        let one_t = 1.0 - t;
        let one_t2 = one_t * one_t;
        let new_vertex = vec2_add(
            vec2_add(
                vec2_mul(from, one_t2),
                vec2_mul(ctrl, 2.0*one_t*t)
            ),
            vec2_mul(to, t2)
        );
        output.push_vertex(new_vertex);
    }
}

pub fn sample_quadratic_bezier(from: Vec2, ctrl: Vec2, to: Vec2, t: f32) -> Vec2 {
    let t2 = t*t;
    let one_t = 1.0 - t;
    let one_t2 = one_t * one_t;
    return vec2_add(
        vec2_add(vec2_mul(from, one_t2), vec2_mul(ctrl, 2.0*one_t*t)),
        vec2_mul(to, t2)
    );
}

pub fn sample_cubic_bezier(from: Vec2, ctrl1: Vec2, ctrl2: Vec2, to: Vec2, t: f32) -> Vec2 {
    let t2 = t*t;
    let t3 = t2*t;
    let one_t = 1.0 - t;
    let     one_t2 = one_t*one_t;
    let one_t3 = one_t2*one_t;
    return vec2_add(
        vec2_add(vec2_mul(from, one_t3), vec2_mul(ctrl1, 3.0*one_t2*t)),
        vec2_add(vec2_mul(ctrl2, 3.0*one_t*t2), vec2_mul(to, t3))
    );
}
