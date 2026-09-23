#[derive(Clone, Copy, Default)]
pub struct AxisInput(f32);

impl AxisInput {
    pub const NEGATIVE: Self = Self(-1.0);
    pub const NEUTRAL: Self = Self(0.0);
    pub const POSITIVE: Self = Self(1.0);

    pub fn new(value: f32) -> Option<Self> {
        (-1.0..=1.0).contains(&value).then_some(Self(value))
    }

    pub fn value(self) -> f32 {
        self.0
    }
}
