#[derive(Clone, Copy, Debug)]
pub struct SmoothedValue {
    current: f32,
    target: f32,
    coeff: f32,
}

impl SmoothedValue {
    pub fn new(value: f32) -> Self {
        Self {
            current: value,
            target: value,
            coeff: 1.0,
        }
    }

    pub fn set_smoothing_time(&mut self, sample_rate: f32, seconds: f32) {
        let samples = (sample_rate * seconds).max(1.0);
        self.coeff = 1.0 - (-1.0 / samples).exp();
    }

    pub fn set_target(&mut self, target: f32) {
        self.target = target;
    }

    pub fn reset(&mut self, value: f32) {
        self.current = value;
        self.target = value;
    }

    pub fn next(&mut self) -> f32 {
        self.current += (self.target - self.current) * self.coeff;
        if self.current.abs() < 1.0e-20 {
            self.current = 0.0;
        }
        self.current
    }
}
