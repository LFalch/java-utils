use std::num::Wrapping;

/// A `java.util.Random` implementation that implements the logic from Java in pure Rust.
pub struct Random {
    seed: Wrapping<u64>,
}

const DEECE: Wrapping<u64> = Wrapping(0x0005_DEEC_E66D);
const BL: Wrapping<u64> = Wrapping(0xB);

impl Random {
    /// Create a new `Random` object from the given seed
    pub fn new(seed: u64) -> Self {
        Self {
            seed: Wrapping((seed ^ 0x0005_DEEC_E66D) & ((1 << 48) - 1)),
        }
    }
    /// Return a new number with the given amount of bits
    pub fn next(&mut self, bits: u32) -> u32 {
        self.seed = Wrapping((self.seed * DEECE + BL).0 & ((1 << 48) - 1));

        (self.seed.0 >> (48 - bits)) as u32
    }
    #[allow(clippy::cast_lossless)]
    /// Return a new int
    pub fn next_int(&mut self, bound: u32) -> u32 {
        assert!(bound > 0, "bound must be positive");
        if bound.count_ones() == 1 {
            ((bound as u64 * self.next(31) as u64) >> 31) as u32
        } else {
            let mut bits = self.next(31);
            let mut val = bits % bound;
            while bits + bound < val + 1 {
                bits = self.next(31);
                val = bits % bound;
            }
            val
        }
    }
}
