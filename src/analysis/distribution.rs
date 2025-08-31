//! Statistical distribution structures and some implementations for
//! common distributions.
use checksum_tapestry::crc::{BitOrder, BitWidth, CRCConfiguration, CRC};
use checksum_tapestry::Checksum;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::analysis::sample::Sample;

/// A generic distribution interface
pub trait Distribution<T> {
    /// Sample a single item from a distribution
    fn sample(&mut self) -> Sample<T>;
}

/// Critical values for tails of distributions
pub enum CriticalValue {
    /// Ten percent or 0.10 for one-sided test
    TenPercent,
    /// Five percent or 0.05 for one-sided test
    FivePercent,
    /// One percent or 0.01 for one-sided test
    OnePercent,
}

/// The parameters of the discrete uniform distribution
pub struct DiscreteUniformDistributionParameters {
    /// The a parameter of the distribution
    /// This is the lowest-possible value a value can take from the
    /// distribution
    pub a: u32,
    /// The b parameter of the distribution
    /// This is the highest-possible value a value can take from the
    /// distribution
    pub b: u32,
}

/// A discrete uniform distribution that takes on values from a to b
/// inclusive: U[a; b]
pub struct DiscreteUniformDistribution<'a> {
    /// The parameters of the distribution
    pub parameters: DiscreteUniformDistributionParameters,
    /// The current state of the random number generator
    pub state: CRC<'a, u32>,
}

impl<'a> Distribution<u32> for DiscreteUniformDistribution<'a> {
    fn sample(&mut self) -> Sample<u32> {
        Sample {
            sample: self.state.update((self.state.state() >> 24) as u8),
        }
    }
}

impl<'a> DiscreteUniformDistribution<'a> {
    /// Use the CRC code as a crude PRNG
    /// It's not secure, but it works for the purposes here as an example
    /// If you want a more tested PRNG library, enable the external-rand feature with:
    /// cargo run --example adler32_output_space_use --features external-rand
    ///
    /// I'm in the progress of reviewing this PRNG.  One particular
    /// weakness is that CRCs don't cycle through zero, so the
    /// generated random distribution will be biased towards larger
    /// values without proper weighting.
    pub fn new(a: u32, b: u32) -> Self {
        let t = SystemTime::now();
        let t = t.duration_since(UNIX_EPOCH).unwrap().as_millis();
        let seed: u32 = (t % (u32::MAX as u128 + 1)) as u32;
        let prng_crc = CRC::<u32>::new(
            CRCConfiguration::<u32>::new(
                "CRC-32/ISO-HDLC",
                BitWidth::ThirtyTwo,
                BitOrder::LSBFirst,
                0x04C11DB7,
                true,
                Some(seed),
                Some(0xFFFFFFFF),
            ),
            true,
        );

        Self {
            parameters: DiscreteUniformDistributionParameters { a, b },
            state: prng_crc,
        }
    }
}

/// Normalize random variables from a discrete uniform distribution with values
/// between a and b to a uniform distribution with values between 0
/// and 1.
/// We can optimize this by caching the normalization factor.
pub fn normalize_variable(item: f32, parameters: &DiscreteUniformDistributionParameters) -> f32 {
    let width: f32 = 1.0 / (parameters.b - parameters.a) as f32;

    (item - parameters.a as f32) * width
}

#[cfg(test)]
mod tests {
    use super::{normalize_variable, DiscreteUniformDistributionParameters};

    /// Test normalizing discrete uniform distrubution variables works
    #[test]
    fn normalize_variable_a_zero_works() {
        let parameters = DiscreteUniformDistributionParameters { a: 0, b: 2 };

        let var_1 = 1.5;
        let var_2 = 0.0;

        let normalized_var_1 = normalize_variable(var_1, &parameters);
        let normalized_var_2 = normalize_variable(var_2, &parameters);

        assert_eq!(normalized_var_1, 0.75);
        assert_eq!(normalized_var_2, 0.00);
    }

    /// Test normalizing discrete uniform distrubution variables offset works
    #[test]
    fn normalize_variable_a_nonzero_works() {
        let parameters = DiscreteUniformDistributionParameters { a: 1, b: 3 };

        let var_1 = 2.5;
        let var_2 = 1.0;

        let normalized_var_1 = normalize_variable(var_1, &parameters);
        let normalized_var_2 = normalize_variable(var_2, &parameters);

        assert_eq!(normalized_var_1, 0.75);
        assert_eq!(normalized_var_2, 0.00);
    }
}
