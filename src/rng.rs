/// RNG wrapper providing deterministic seed-based randomness
/// Uses ChaCha8Rng for consistent generation across platforms
use rand_chacha::ChaCha8Rng;
use rand::{Rng, SeedableRng};

pub struct SeededRng {
    rng: ChaCha8Rng,
}

impl SeededRng {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }

    /// Roll a single D6 (1-6)
    pub fn d6(&mut self) -> u32 {
        self.rng.gen_range(1..=6)
    }

    /// Roll D66: two D6 dice returned as (primary, secondary)
    /// Used for room dimensions (primary=X, secondary=Y)
    pub fn d66(&mut self) -> (u32, u32) {
        (self.d6(), self.d6())
    }
    
    /// Generate a random number in range [min, max)
    pub fn range(&mut self, min: usize, max: usize) -> usize {
        self.rng.gen_range(min..max)
    }
}
