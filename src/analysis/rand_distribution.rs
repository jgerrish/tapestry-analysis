//! A set of random distribution implementations using the rand crate
use rand::{rngs::ThreadRng, Rng};

use crate::analysis::{distribution::Distribution, sample::Sample};

/// A discrete uniform distribution taking on values from a to b inclusive
pub struct RandDiscreteUniformDistribution {
    /// The a parameter of the distribution
    /// This is the lowest-possible value a value can take from the
    /// distribution
    pub a: u32,
    /// The b parameter of the distribution
    /// This is the highest-possible value a value can take from the
    /// distribution
    pub b: u32,

    /// The current state of the random number generator
    pub state: ThreadRng,
}

impl Distribution<u32> for RandDiscreteUniformDistribution {
    fn sample(&mut self) -> Sample<u32> {
        Sample {
            sample: self.state.gen_range(self.a..=self.b),
        }
    }
}

impl RandDiscreteUniformDistribution {
    /// Create a new RandDiscreteUniformDistribution with the given
    /// parameters
    pub fn new(a: u32, b: u32) -> Self {
        let rng = rand::thread_rng();

        Self { a, b, state: rng }
    }
}
