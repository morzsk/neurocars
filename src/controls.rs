use rapier2d::prelude::Real;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Controls {
    pub throttle: Real,
    pub steering: Real,
}

impl Controls {
    pub fn from_direction_inputs(forward: bool, backward: bool, left: bool, right: bool) -> Self {
        let throttle = match (forward, backward) {
            (true, false) => 1.0,
            (false, true) => -1.0,
            _ => 0.0,
        };
        let steering = match (left, right) {
            (true, false) => 1.0,
            (false, true) => -1.0,
            _ => 0.0,
        };

        Self { throttle, steering }
    }
}
