//! Statistical experiment structures and implementations
//!
use crate::analysis::{distribution::Distribution, sample::Sample};
use checksum_tapestry::Checksum;

/// A single experiment
/// An experiment is a collection of samples, usually from the same
/// distribution.
pub struct Experiment<T> {
    /// Samples in this experiment
    pub samples: Vec<Sample<T>>,
}

/// Run an experiment for a given checksum algorithm.
/// Generates a set of random byte strings, and then calculates the checksum for that data.
/// Repeats this several times and returns the data.
impl<T> Experiment<T> {
    /// Run an experiment
    pub fn run(
        prng: &mut dyn Distribution<u32>,
        checksum: &mut dyn Checksum<u32>,
        message_size: u32,
        num_experiments: u32,
    ) -> Experiment<u32> {
        let mut random_buffer: Vec<u8> = vec![0; message_size.try_into().unwrap()];
        let mut experiments: Vec<u32> = vec![0; num_experiments.try_into().unwrap()];

        // Run the experiment
        for i in 0..num_experiments {
            // Generate a random message string
            let mut last: u32;
            for item in &mut random_buffer {
                last = prng.sample().sample;
                *item = (last >> 24) as u8;
            }

            let result = checksum.compute(&random_buffer);
            checksum.reset();

            experiments[i as usize] = result;
        }

        Experiment {
            samples: experiments.iter().map(|d| Sample { sample: *d }).collect(),
        }
    }
}
