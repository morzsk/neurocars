use rapier2d::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RacerGeometry {
    pub length: Real,
    pub width: Real,
    pub wheelbase: Real,
    pub track_width: Real,
    wheel_positions: [Vector; 4],
}

impl RacerGeometry {
    pub fn new(length: Real, width: Real, wheelbase: Real, track_width: Real) -> Self {
        assert!(
            length.is_finite() && length > 0.0,
            "racer length must be positive"
        );
        assert!(
            width.is_finite() && width > 0.0,
            "racer width must be positive"
        );
        assert!(
            wheelbase.is_finite() && wheelbase > 0.0,
            "racer wheelbase must be positive"
        );
        assert!(
            track_width.is_finite() && track_width > 0.0,
            "racer track width must be positive"
        );

        let half_wheelbase = wheelbase / 2.0;
        let half_track_width = track_width / 2.0;

        Self {
            length,
            width,
            wheelbase,
            track_width,
            wheel_positions: [
                Vector::new(-half_track_width, half_wheelbase),
                Vector::new(half_track_width, half_wheelbase),
                Vector::new(-half_track_width, -half_wheelbase),
                Vector::new(half_track_width, -half_wheelbase),
            ],
        }
    }

    pub fn wheel_positions(&self) -> &[Vector; 4] {
        &self.wheel_positions
    }
}

#[derive(Clone, Debug)]
pub struct Racer {
    pub geometry: RacerGeometry,
    pub physics_handle: RigidBodyHandle,
}

impl Racer {
    pub(crate) fn new(geometry: RacerGeometry, physics_handle: RigidBodyHandle) -> Self {
        Self {
            geometry,
            physics_handle,
        }
    }
}
