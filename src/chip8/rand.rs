pub struct Rand {
    pub seed: u32,
}

impl Rand {
    pub fn range(&mut self, min: u32, max: u32) -> u32 {
        self.seed = self.seed.wrapping_add(0xe120fc15);
        let mut tmp: u64 = self.seed as u64 * 0x4a39b70d;
        let m1: u32 = ((tmp >> 32) ^ tmp) as u32;
        tmp = m1 as u64 * 0x12fad5c9;
        let m2: u32 = ((tmp >> 32) ^ tmp) as u32;

        min.saturating_add(m2 % ((max.saturating_add(1)).saturating_sub(min)))
    }
}
