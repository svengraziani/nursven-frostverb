#[derive(Clone, Debug)]
pub struct DelayLine {
    buffer: Vec<f32>,
    write: usize,
}

impl DelayLine {
    pub fn new() -> Self {
        Self {
            buffer: vec![0.0; 1],
            write: 0,
        }
    }

    pub fn resize(&mut self, samples: usize) {
        self.buffer.clear();
        self.buffer.resize(samples.max(1), 0.0);
        self.write = 0;
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0.0);
        self.write = 0;
    }

    pub fn push_read(&mut self, input: f32, delay_samples: usize) -> f32 {
        let len = self.buffer.len();
        let delay = delay_samples.min(len - 1);
        let read = (self.write + len - delay) % len;
        let output = self.buffer[read];
        self.buffer[self.write] = input;
        self.write = (self.write + 1) % len;
        output
    }
}
