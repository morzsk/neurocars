use std::f32::consts::TAU;

use macroquad::rand::gen_range;
use rapier2d::prelude::{Real, Vector};

use crate::bezier::QuadraticBezier;

const RENDER_SEGMENTS_PER_CURVE: usize = 40;
const COLLISION_SEGMENTS_PER_CURVE: usize = 5;
const MIN_ANGLE_WEIGHT: Real = 0.85;
const MAX_ANGLE_WEIGHT: Real = 1.15;

#[derive(Clone, Debug)]
pub struct Track {
    render_boundary: TrackBoundary,
    collision_boundary: TrackBoundary,
}

#[derive(Clone, Debug)]
pub struct TrackBoundary {
    inner_points: Vec<Vector>,
    outer_points: Vec<Vector>,
}

impl Track {
    pub fn new(vertex_count: usize, size: Real, width: Real) -> Self {
        assert!(vertex_count >= 5, "a track needs at least five vertices");
        assert!(
            size.is_finite() && size > 0.0,
            "track size must be finite and positive"
        );
        assert!(
            width.is_finite() && width > 0.0 && width < size * 2.0,
            "track width must be finite, positive, and less than the diameter"
        );

        let inner_radius = size - width / 2.0;
        let outer_radius = size + width / 2.0;

        let mut angle_steps: Vec<Real> = (0..vertex_count)
            .map(|_| gen_range(MIN_ANGLE_WEIGHT, MAX_ANGLE_WEIGHT))
            .collect();
        let angle_scale = TAU / angle_steps.iter().sum::<Real>();
        for angle_step in &mut angle_steps {
            *angle_step *= angle_scale;
        }

        Self {
            render_boundary: TrackBoundary::sample(
                &angle_steps,
                inner_radius,
                outer_radius,
                RENDER_SEGMENTS_PER_CURVE,
            ),
            collision_boundary: TrackBoundary::sample(
                &angle_steps,
                inner_radius,
                outer_radius,
                COLLISION_SEGMENTS_PER_CURVE,
            ),
        }
    }

    pub fn render_boundary(&self) -> &TrackBoundary {
        &self.render_boundary
    }

    pub fn collision_boundary(&self) -> &TrackBoundary {
        &self.collision_boundary
    }

    pub fn spawn_position(&self, lane_position: Real) -> Vector {
        self.render_boundary.inner_points[0].lerp(
            self.render_boundary.outer_points[0],
            lane_position.clamp(0.0, 1.0),
        )
    }
}

impl TrackBoundary {
    fn sample(
        angle_steps: &[Real],
        inner_radius: Real,
        outer_radius: Real,
        segments_per_curve: usize,
    ) -> Self {
        Self {
            inner_points: sample_loop(angle_steps, inner_radius, segments_per_curve),
            outer_points: sample_loop(angle_steps, outer_radius, segments_per_curve),
        }
    }

    pub fn inner_points(&self) -> &[Vector] {
        &self.inner_points
    }

    pub fn outer_points(&self) -> &[Vector] {
        &self.outer_points
    }
}

fn sample_loop(angle_steps: &[Real], radius: Real, segments_per_curve: usize) -> Vec<Vector> {
    let mut angle: Real = 0.0;
    let vertices: Vec<Vector> = angle_steps
        .iter()
        .map(|angle_step| {
            let vertex = Vector::new(angle.cos(), angle.sin()) * radius;
            angle += angle_step;
            vertex
        })
        .collect();

    let mut points = Vec::with_capacity(vertices.len() * segments_per_curve);
    for index in 0..vertices.len() {
        let previous = vertices[(index + vertices.len() - 1) % vertices.len()];
        let control = vertices[index];
        let next = vertices[(index + 1) % vertices.len()];
        let curve =
            QuadraticBezier::new((previous + control) / 2.0, control, (control + next) / 2.0);
        let curve_points = curve.sample(segments_per_curve);

        points.extend_from_slice(&curve_points[..segments_per_curve]);
    }

    points
}
