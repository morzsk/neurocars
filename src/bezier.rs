use macroquad::prelude::Vec2;

/// https://pomax.github.io/bezierinfo/#explanation
pub fn quadratic(start: Vec2, control: Vec2, end: Vec2, t: f32) -> Vec2 {
    let remaining = 1.0 - t;

    remaining * remaining * start + 2.0 * remaining * t * control + t * t * end
}

/// https://pomax.github.io/bezierinfo/#derivatives
pub fn quadratic_tangent(start: Vec2, control: Vec2, end: Vec2, t: f32) -> Vec2 {
    2.0 * (1.0 - t) * (control - start) + 2.0 * t * (end - control)
}
