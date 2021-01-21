
// https://www.pcg-random.org/download.html

pub trait Rng {
    fn uniform(&mut self, a: u32, b: u32) -> u32;
}

pub struct Pcg {
    state: u64,
    inc: u64,
}

impl Pcg {
    pub fn new(state: u64, inc: u64) -> Self {
        Pcg { state, inc }
    }

    pub fn next(&mut self) -> u32 {
        let oldstate = self.state;
        self.state = oldstate.wrapping_mul(6364136223846793005) + (self.inc | 1);
        let xorshifted = ((oldstate >> 18) ^ oldstate) >> 27;
        let rot = oldstate >> 59;
        let res = (xorshifted >> rot) | (xorshifted << ((-(rot as i64)) & 31));
        res as u32
    }
}

impl Rng for Pcg {
    fn uniform(&mut self, a: u32, b: u32) -> u32 {
        let res = self.next();
        a + res % (1+b-a)
    }
}
