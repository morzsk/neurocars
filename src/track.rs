use macroquad::prelude::{Color, GRAY, Vec2, WHITE, draw_line};

use crate::bezier::{quadratic, quadratic_tangent};

pub struct Track {
    pub width: f32,
    pub vertices: Vec<Vec2>,
    controls: Vec<Vec2>,
}

pub fn init_track(width: f32, start: Vec2) -> Track {
    Track {
        width,
        vertices: vec![start],
        controls: Vec::new(),
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
}

pub fn remove_from_track(track: &mut Track) {
    track.vertices.pop();
    track.controls.pop();
}

pub fn draw_track(track: &Track, segments: usize) {
    for (anchors, control) in track.vertices.windows(2).zip(&track.controls) {
        let [start, end] = anchors else {
            unreachable!();
        };

        draw_segment(*start, *control, *end, track.width, segments, WHITE);
    }
}

pub fn draw_track_preview(track: &Track, end: Vec2, segments: usize) {
    if let Some(start) = track.vertices.last() {
        let control = infer_control(track.controls.last().copied(), *start, end);

        draw_segment(*start, control, end, track.width, segments, GRAY);
    }
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

fn draw_segment(start: Vec2, control: Vec2, end: Vec2, width: f32, segments: usize, color: Color) {
    let segments = segments.max(1);
    let (mut previous_left, mut previous_right) = track_edges_at(start, control, end, width, 0.0);

    for step in 1..=segments {
        let t = step as f32 / segments as f32;
        let (left, right) = track_edges_at(start, control, end, width, t);

        draw_line(previous_left.x, previous_left.y, left.x, left.y, 2.0, color);
        draw_line(
            previous_right.x,
            previous_right.y,
            right.x,
            right.y,
            2.0,
            color,
        );

        previous_left = left;
        previous_right = right;
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
