use std::time::{SystemTime, UNIX_EPOCH};

const KX: u32 = 123456789;
const KY: u32 = 362436069;
const KZ: u32 = 521288629;
const KW: u32 = 88675123;

/// Simple Random Number Generator
///
/// I can't find the StackOverflow post I copied this from
/// but I still want to express my gratitude for one of the
/// best answers on that website
pub struct Rand {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl Rand {
    pub const fn new(seed: u32) -> Self {
        Self {
            x: KX ^ seed,
            y: KY ^ seed,
            z: KZ,
            w: KW,
        }
    }

    /// seed is picked feom SystemTime
    pub fn with_random_seed() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        Self::new(seed)
    }

    // Xorshift 128, taken from German Wikipedia
    pub const fn rand(&mut self) -> u32 {
        let t = self.x ^ self.x.wrapping_shl(11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w ^= self.w.wrapping_shr(19) ^ t ^ t.wrapping_shr(8);
        self.w
    }

    /// rand f64 in [0, 1]
    /// aka: sampling from the Uniform distribution
    pub fn rand_float(&mut self) -> f64 {
        (self.rand() as f64) / (<u32>::MAX as f64)
    }

    pub const fn shuffle<T>(&mut self, a: &mut [T]) {
        if a.is_empty() {
            return;
        }
        let mut i = a.len() - 1;
        while i > 0 {
            let j = (self.rand() as usize) % (i + 1);
            a.swap(i, j);
            i -= 1;
        }
    }

    pub const fn rand_in_range(&mut self, low: i32, high: i32) -> i32 {
        let m = (high - low + 1) as u32;
        low + (self.rand() % m) as i32
    }

    // pub fn normal_standard(&mut self) -> f64 {
    //     todo!("implement algo for generating a standard random normal")
    // }
    //
    // pub fn normal(&mut self, mu: f64, sigma: f64) -> f64 {
    //     todo!("implement algo for generating a random normal")
    // }
}
