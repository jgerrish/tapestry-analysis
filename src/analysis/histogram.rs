//! Generate a histogram from a set of data
//! This crate provides a set of functions for generating histograms
//! This includes basic data structures and functions for binning data
//! and functions for plotting the data on a terminal.

use crate::analysis::experiment::Experiment;

/// A histogram that only contains the count of data in each bin
/// Contains experiment data in a set of bins
pub struct SimpleHistogram {
    /// The number of bins separate the data into
    pub num_bins: u8,
    /// The bins containining the data
    pub bins: Vec<u32>,
    /// Number of data points
    pub num_data_points: u32,
}

/// A more complicated histogram that contains the actual values in
/// each bin
pub struct FullHistogram {}

/// Functions a histogram should implement
pub trait Histogram {
    /// Bin an experiment
    ///
    /// # Examples
    /// ```
    /// use tapestry_analysis::analysis::{
    ///     distribution::DiscreteUniformDistribution,
    ///     experiment::Experiment,
    ///     histogram::{Histogram, SimpleHistogram},
    /// };
    /// use checksum_tapestry::adler32::Adler32;
    ///
    /// let mut dud = DiscreteUniformDistribution::new(0, u32::MAX);
    ///
    /// // Run an Adler-32 experiment, showing a histogram of values
    /// let mut adler32 = Adler32::default();
    /// let adler32_experiment =
    ///     Experiment::<u32>::run(&mut dud, &mut adler32, 50, 1000);
    ///
    /// println!("Adler32 Histogram");
    /// let histogram = SimpleHistogram::new(&adler32_experiment, 10);
    /// assert_eq!(histogram.bins.len(), 10);
    /// ```
    fn new(experiment: &Experiment<u32>, num_bins: u8) -> Self;

    /// Draw the histogram on a terminal
    /// This function has side effects
    ///
    /// # Examples
    /// ```
    /// use tapestry_analysis::analysis::{
    ///     distribution::DiscreteUniformDistribution,
    ///     experiment::Experiment,
    ///     histogram::{Histogram, SimpleHistogram},
    /// };
    /// use checksum_tapestry::adler32::Adler32;
    ///
    /// let mut dud = DiscreteUniformDistribution::new(0, u32::MAX);
    ///
    /// // Run an Adler-32 experiment, showing a histogram of values
    /// let mut adler32 = Adler32::default();
    /// let adler32_experiment =
    ///     Experiment::<u32>::run(&mut dud, &mut adler32, 50, 1000);
    ///
    /// println!("Adler32 Histogram");
    /// let histogram = SimpleHistogram::new(&adler32_experiment, 10);
    /// histogram.draw_terminal();
    fn draw_terminal(&self);
}

impl Histogram for SimpleHistogram {
    fn new(experiment: &Experiment<u32>, num_bins: u8) -> Self {
        let bin_boundary = num_bins as f32 / u32::MAX as f32;
        let mut bins: Vec<u32> = vec![0; num_bins.into()];

        for i in 0..experiment.samples.len() {
            let bin = (experiment.samples[i].sample as f32 * bin_boundary).floor();
            bins[bin as usize] += 1;
        }

        SimpleHistogram {
            num_bins,
            bins,
            num_data_points: experiment.samples.len().try_into().unwrap(),
        }
    }

    fn draw_terminal(&self) {
        // graph width in characters
        let width = 55;

        // The histogram code assumes the distribution is uniform for display purposes
        // For the adler-32 case, this isn't true, but still do the calculation
        let avg_stars_per_bin = self.num_data_points as f32 / self.num_bins as f32;
        // Set aside some extra space
        let avg_stars_per_bin = avg_stars_per_bin * 1.8;
        let line_div = avg_stars_per_bin / width as f32;

        for i in 0..self.num_bins {
            let total = self.bins[i as usize];
            let start = (u32::MAX as f32 / self.num_bins as f32) * i as f32;
            let end = (u32::MAX as f32 / self.num_bins as f32) * (i + 1) as f32;
            print!("0x{:08X} - 0x{:08X}: ", start as u32, end as u32);
            let stars_to_print: u32 = (total as f32 / line_div).floor() as u32;
            for _j in 0..stars_to_print {
                print!("*");
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::analysis::{
        distribution::DiscreteUniformDistribution,
        experiment::Experiment,
        histogram::{Histogram, SimpleHistogram},
    };
    use checksum_tapestry::adler32::Adler32;

    #[test]
    fn new_works() {
        let mut dud = DiscreteUniformDistribution::new(0, u32::MAX);

        // Run an Adler-32 experiment, binning the data
        let mut adler32 = Adler32::default();
        let adler32_experiment = Experiment::<u32>::run(&mut dud, &mut adler32, 50, 1000);

        let histogram = SimpleHistogram::new(&adler32_experiment, 10);

        assert_eq!(histogram.num_bins, 10);
        assert_eq!(histogram.bins.len(), 10);
        assert_eq!(histogram.num_data_points, 1000);
    }

    #[test]
    fn draw_terminal_works() {
        let mut dud = DiscreteUniformDistribution::new(0, u32::MAX);

        // Run an Adler-32 experiment, binning the data
        let mut adler32 = Adler32::default();
        let adler32_experiment = Experiment::<u32>::run(&mut dud, &mut adler32, 50, 1000);

        let histogram = SimpleHistogram::new(&adler32_experiment, 10);

        // draw the data
        histogram.draw_terminal();
    }
}
