#[derive(Clone, Copy, Debug, Default)]
pub struct DcBlock {
    x1: f32,
    y1: f32,
}

impl DcBlock {
    pub fn process(&mut self, input: f32) -> f32 {
        let output = input - self.x1 + 0.995 * self.y1;
        self.x1 = input;
        self.y1 = flush_denormal(output);
        self.y1
    }
}

pub fn softclip(input: f32) -> f32 {
    let x = input.clamp(-3.0, 3.0);
    x - (x * x * x) / 3.0
}

#[derive(Clone, Copy, Debug)]
pub struct SafetyLimiter {
    envelope: f32,
    release_coeff: f32,
    ceiling: f32,
}

impl SafetyLimiter {
    pub fn new(ceiling: f32) -> Self {
        Self {
            envelope: 0.0,
            release_coeff: 0.9995,
            ceiling,
        }
    }

    pub fn prepare(&mut self, sample_rate: f32) {
        self.release_coeff = (-1.0 / (sample_rate * 0.080).max(1.0)).exp();
    }

    pub fn reset(&mut self) {
        self.envelope = 0.0;
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let peak = input.abs();
        if peak > self.envelope {
            self.envelope = peak;
        } else {
            self.envelope *= self.release_coeff;
        }

        if self.envelope <= self.ceiling {
            input
        } else {
            input * (self.ceiling / self.envelope)
        }
    }

    pub fn gain_reduction_db(&self) -> f32 {
        if self.envelope <= self.ceiling {
            0.0
        } else {
            20.0 * (self.ceiling / self.envelope).log10()
        }
    }
}

pub fn flush_denormal(value: f32) -> f32 {
    if value.abs() < 1.0e-20 { 0.0 } else { value }
}
