use rapier2d::prelude::{Real, Vector};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuadraticBezier {
    start: Vector,
    control: Vector,
    end: Vector,
}

impl QuadraticBezier {
    pub fn new(start: Vector, control: Vector, end: Vector) -> Self {
        Self {
            start,
            control,
            end,
        }
    }

    pub fn point(&self, progress: Real) -> Vector {
        let progress = progress.clamp(0.0, 1.0);
        let remaining = 1.0 - progress;

        self.start * remaining.powi(2)
            + self.control * (2.0 * remaining * progress)
            + self.end * progress.powi(2)
    }

    pub fn sample(&self, segment_count: usize) -> Vec<Vector> {
        assert!(segment_count > 0, "a bezier needs at least one segment");

        (0..=segment_count)
            .map(|segment| self.point(segment as Real / segment_count as Real))
            .collect()
    }
}
