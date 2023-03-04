pub struct RandomGenerator {
    seed: u64,
}

impl RandomGenerator {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }
    pub fn rand(&mut self) -> f64 {
        // https://en.wikipedia.org/wiki/Linear_congruential_generator
        self.seed = self.seed.wrapping_mul(25214903917) + 11;
        (self.seed & 0xFFFF) as f64 / 0xFFFF as f64
    }
}
