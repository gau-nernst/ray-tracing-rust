pub struct PCG32State {
    state: u64,
    inc: u64,
}

impl PCG32State {
    pub fn new(init_state: u64, init_seq: u64) -> PCG32State {
        let mut rng = PCG32State { state: 0, inc: 0 };
        rng.seed(init_state, init_seq);
        rng
    }
    pub fn seed(&mut self, init_state: u64, init_seq: u64) {
        self.state = 0;
        self.inc = (init_seq << 1) | 1;
        self.u32();
        self.state += init_state;
        self.u32();
    }
    pub fn u32(&mut self) -> u32 {
        let old_state = self.state;
        self.state = old_state * 6364136223846793005 + self.inc;
        let xorshifted = (((old_state >> 18) ^ old_state) >> 27) as u32;
        let rot = (old_state >> 59) as u32;
        (xorshifted >> rot) | (xorshifted << (0u32.wrapping_sub(rot) & 31))
    }
    pub fn u32_between(&mut self, lo: u32, hi: u32) -> u32 {
        lo + self.u32() % (hi - lo) // not accurate
    }
    pub fn f32(&mut self) -> f32 {
        (self.u32() >> 8) as f32 / (1 << 24) as f32
    }
    pub fn f32_between(&mut self, lo: f32, hi: f32) -> f32 {
        lo + self.f32() * (hi - lo)
    }
}
