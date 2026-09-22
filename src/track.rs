use macroquad::prelude::{Color, GRAY, Vec2, WHITE, draw_line};
use rapier2d::prelude::{ColliderBuilder, ColliderHandle, Vector};

use crate::bezier::{quadratic, quadratic_tangent};
use crate::physics::Physics;

pub struct Track {
    pub width: f32,
    pub vertices: Vec<Vec2>,
    controls: Vec<Vec2>,
    samples: TrackSamples,
    t_step: f32,
    collider_handles: Vec<ColliderHandle>,
}

struct TrackSamples {
    left: Vec<Vec2>,
    right: Vec<Vec2>,
}

pub fn init_track(width: f32, start: Vec2, t_step: f32) -> Track {
    assert!(t_step.is_finite() && t_step > 0.0);

    Track {
        width,
        vertices: vec![start],
        controls: Vec::new(),
        samples: TrackSamples {
            left: Vec::new(),
            right: Vec::new(),
        },
        t_step,
        collider_handles: Vec::new(),
    }
}

pub fn add_to_track(track: &mut Track, end: Vec2) {
    let start = *track
        .vertices
        .last()
        .expect("a track always has a starting vertex");
    let control = infer_control(track.controls.last().copied(), start, end);

    track.controls.push(control);
    track.vertices.push(end);
    track.samples = sample_track(track);
}

pub fn remove_from_track(track: &mut Track) {
    track.vertices.pop();
    track.controls.pop();
    track.samples = sample_track(track);
}

pub fn update_collider(track: &mut Track, physics: &mut Physics) {
    for handle in track.collider_handles.drain(..) {
        physics.colliders.remove(
            handle,
            &mut physics.island_manager,
            &mut physics.bodies,
            true,
        );
    }

    if track.samples.left.len() < 2 {
        return;
    }

    let left = track
        .samples
        .left
        .iter()
        .map(|point| Vector::new(point.x, point.y))
        .collect();
    let right = track
        .samples
        .right
        .iter()
        .map(|point| Vector::new(point.x, point.y))
        .collect();

    let left_collider = ColliderBuilder::polyline(left, None).build();
    let right_collider = ColliderBuilder::polyline(right, None).build();

    track
        .collider_handles
        .push(physics.colliders.insert(left_collider));
    track
        .collider_handles
        .push(physics.colliders.insert(right_collider));
}

pub fn draw_track(track: &Track) {
    draw_path(&track.samples.left, WHITE);
    draw_path(&track.samples.right, WHITE);
}

pub fn draw_track_preview(track: &Track, end: Vec2) {
    if let Some(start) = track.vertices.last() {
        let control = infer_control(track.controls.last().copied(), *start, end);
        let mut samples = sample_segment_edges(*start, control, end, track.width, track.t_step);
        let (mut previous_left, mut previous_right) =
            samples.next().expect("a segment always has samples");

        for (left, right) in samples {
            draw_line(previous_left.x, previous_left.y, left.x, left.y, 2.0, GRAY);
            draw_line(
                previous_right.x,
                previous_right.y,
                right.x,
                right.y,
                2.0,
                GRAY,
            );

            previous_left = left;
            previous_right = right;
        }
    }
}

fn sample_track(track: &Track) -> TrackSamples {
    let segments_per_curve = segments_for_step(track.t_step);
    let samples_per_edge = track.controls.len() * segments_per_curve + 1;
    let mut left = Vec::with_capacity(samples_per_edge);
    let mut right = Vec::with_capacity(samples_per_edge);

    for (curve_index, (anchors, control)) in
        track.vertices.windows(2).zip(&track.controls).enumerate()
    {
        let [start, end] = anchors else {
            unreachable!();
        };
        let first_sample = usize::from(curve_index > 0);

        for (left_sample, right_sample) in
            sample_segment_edges(*start, *control, *end, track.width, track.t_step)
                .skip(first_sample)
        {
            left.push(left_sample);
            right.push(right_sample);
        }
    }

    TrackSamples { left, right }
}

fn sample_segment_edges(
    start: Vec2,
    control: Vec2,
    end: Vec2,
    width: f32,
    t_step: f32,
) -> impl Iterator<Item = (Vec2, Vec2)> {
    let segments = segments_for_step(t_step);

    (0..=segments).map(move |step| {
        let t = step as f32 / segments as f32;
        track_edges_at(start, control, end, width, t)
    })
}

fn segments_for_step(t_step: f32) -> usize {
    assert!(t_step.is_finite() && t_step > 0.0);
    (1.0 / t_step).ceil() as usize
}

fn infer_control(previous_control: Option<Vec2>, start: Vec2, end: Vec2) -> Vec2 {
    let chord = end - start;

    if chord.length_squared() <= f32::EPSILON {
        return start;
    }

    let Some(previous_control) = previous_control else {
        return start + chord * 0.5;
    };

    let exit_tangent = start - previous_control;
    if exit_tangent.length_squared() <= f32::EPSILON {
        return start + chord * 0.5;
    }

    start + exit_tangent.normalize() * chord.length() * 0.5
}

fn draw_path(points: &[Vec2], color: Color) {
    for points in points.windows(2) {
        let [start, end] = points else {
            unreachable!();
        };

        draw_line(start.x, start.y, end.x, end.y, 2.0, color);
    }
}

fn track_edges_at(start: Vec2, control: Vec2, end: Vec2, width: f32, t: f32) -> (Vec2, Vec2) {
    // https://pomax.github.io/bezierinfo/#offsetting
    let center = quadratic(start, control, end, t);
    let tangent = quadratic_tangent(start, control, end, t);
    let tangent = if tangent.length_squared() > f32::EPSILON {
        tangent
    } else {
        end - start
    };

    let normal = if tangent.length_squared() > f32::EPSILON {
        Vec2::new(-tangent.y, tangent.x).normalize()
    } else {
        Vec2::Y
    };

    let offset = normal * width / 2.0;
    (center + offset, center - offset)
}
