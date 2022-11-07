//! Example of using the Adler-32 checksum and a simple visualization
//! of it's weaknesses with small message sizes.
//!
//! This also performs a Kolmogorov–Smirnov test on the data
#[cfg(feature = "external-rand")]
use tapestry_analysis::analysis::rand_distribution::RandDiscreteUniformDistribution;

#[cfg(not(feature = "external-rand"))]
use tapestry_analysis::analysis::distribution::DiscreteUniformDistribution;

use tapestry_analysis::analysis::{
    distribution::{CriticalValue, DiscreteUniformDistributionParameters},
    experiment::Experiment,
    histogram::{Histogram, SimpleHistogram},
    ks::{critical_value, statistic},
    sample::Sample,
};

use checksum_tapestry::adler32::Adler32;
use checksum_tapestry::crc::{BitWidth, CRCConfiguration, CRCEndianness, CRC};

const NUM_EXPERIMENTS: u32 = 1000;
const MESSAGE_SIZE: u32 = 50;
const NUM_BINS: u8 = 10;

/// Perform a Kolmogorov–Smirnov test on an experiment
fn perform_ks_test(name: &str, experiment: &Experiment<u32>) {
    let experiment_f32: Experiment<f32> = Experiment {
        samples: experiment
            .samples
            .iter()
            .map(|s| Sample {
                sample: (*s).sample as f32,
            })
            .collect(),
    };

    let parameters = DiscreteUniformDistributionParameters { a: 0, b: u32::MAX };
    let statistic = statistic(experiment_f32, &parameters);

    let n: u32 = experiment.samples.len().try_into().unwrap();
    let crit_val = critical_value(CriticalValue::FivePercent, n);
    if let Some(cv) = crit_val {
        println!(
            "{0:>1$}{2:>3$}{4:>5$}     reject the null hypothesis?",
            "name", 8, "statistic", 15, "critical value", 18
        );
        print!("{0:>1$}{2:>3$}{4:>5$}     ", name, 8, statistic, 15, cv, 18);
        if statistic < cv {
            println!("no,  data follows a uniform distribution");
        } else {
            println!("yes, data does not follow a uniform distribution");
        }
    } else {
        println!("Invalid sample size");
    }
}

fn main() {
    #[cfg(not(feature = "external-rand"))]
    let mut dud = DiscreteUniformDistribution::new(0, u32::MAX);
    #[cfg(feature = "external-rand")]
    let mut dud = RandDiscreteUniformDistribution::new(0, u32::MAX);

    // Run an Adler-32 experiment, showing a histogram of values
    let mut adler32 = Adler32::default();
    let adler32_experiment =
        Experiment::<u32>::run(&mut dud, &mut adler32, MESSAGE_SIZE, NUM_EXPERIMENTS);

    println!("Adler32 Histogram");
    let histogram = SimpleHistogram::new(&adler32_experiment, NUM_BINS);
    histogram.draw_terminal();

    println!();

    // Run a CRC32 experiment, showing a histogram of values
    let mut crc32 = CRC::<u32>::new(
        CRCConfiguration::<u32>::new(
            "CRC-32/ISO-HDLC",
            BitWidth::ThirtyTwo,
            CRCEndianness::LSB,
            0x04C11DB7,
            true,
            Some(0xFFFFFFFF),
            Some(0xFFFFFFFF),
        ),
        true,
    );
    let crc_experiment =
        Experiment::<u32>::run(&mut dud, &mut crc32, MESSAGE_SIZE, NUM_EXPERIMENTS);

    println!("CRC32 Histogram");
    let histogram = SimpleHistogram::new(&crc_experiment, NUM_BINS);
    histogram.draw_terminal();

    println!();

    perform_ks_test("Adler32", &adler32_experiment);
    perform_ks_test("CRC32", &crc_experiment);
}
