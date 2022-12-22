//! Kolmogorov–Smirnov test
//! Perform a Kolmogorov–Smirnov goodness of fit test on a data set
//! This can be used to find if a set of values comes from a given
//! distribution.
//! Because the primary purpose of this crate is analyzing checksum
//! and hash algorithms, the code is tailored to testing a uniform
//! distribution.
//! The data can be transformed to test other distributions if needed.
#![warn(missing_docs)]
#![warn(unsafe_code)]

use crate::analysis::{
    distribution::{normalize_variable, CriticalValue, DiscreteUniformDistributionParameters},
    experiment::Experiment,
};

/// Compute the Kolmogoro Smirnov distribution quantiles or tails
///
/// Values below 41 are not included due to precision concerns.
///
/// These are values for the two-sided Komogoro Smirnov distribution
///
/// The two-sided test is used when comparing an empirical
/// distribution to a target assumed cumulative distribution function.
/// We compute both minus and plus differences in this crate, so use
/// the two-sided test.
///
/// # Examples
///
/// ```
/// use tapestry_analysis::analysis::{distribution::CriticalValue, ks::critical_value};
///
/// let critical_value = critical_value(CriticalValue::FivePercent, 41).unwrap();
/// assert!(f32::abs(critical_value - 0.211) < 0.05);
/// ```
// For values above 40, the formulas can be found by doing
// regression on a table of quantiles for the KS distribution.
// See Kolmogorov-Smirnov and Mann-Whitney-Wilcoxon Tests
// https:///math.mit.edu/~rmd/465/edf-ks.pdf
// and
// Computing the Two-Sided Kolmogorov-Smirnov Distribution
// Richard Simard and Pierre L’Ecuyer
// https:///www.jstatsoft.org/article/download/v039i11/456
// Simard and L'Ecuyer provide a chart showing trade-offs between
// various approximation methods as a function of n and the
// critical value.
//
// The equivalent using the Python SciPy package is:
// stats.kstwo.ppf([1-0.01, 1-0.05, 1-0.10], n)
// Which is the percent point function (the inverse of the CDF)
pub fn critical_value(cv: CriticalValue, n: u32) -> Option<f32> {
    match n {
        0..=40 => None,
        41..=u32::MAX => match cv {
            CriticalValue::TenPercent => Some(1.07 / f32::sqrt(n as f32)),
            CriticalValue::FivePercent => Some(1.358 / f32::sqrt(n as f32)),
            CriticalValue::OnePercent => Some(1.52 / f32::sqrt(n as f32)),
        },
    }
}

/// Calculate the shifted expected uniform CDF distributions to
/// compare to the observed distribution.
/// This calculates "upper" and "lower" distributions.
///
/// # Examples
///
/// ```
/// use tapestry_analysis::analysis::ks::shifted_uniform_cdf_distribution;
///
/// let n = 5;
/// let (uniform_cdf_shifted_minus, uniform_cdf_shifted_plus) =
///     shifted_uniform_cdf_distribution(n as u32);
///
/// assert_eq!(uniform_cdf_shifted_minus, [0.0, 0.2, 0.4, 0.6, 0.8]);
/// assert_eq!(uniform_cdf_shifted_plus, [0.2, 0.4, 0.6, 0.8, 1.0]);
/// ```
pub fn shifted_uniform_cdf_distribution(n: u32) -> (Vec<f32>, Vec<f32>) {
    let mut uniform_cdf_shifted_minus: Vec<f32> = Vec::new();
    (0..n).for_each(|i| uniform_cdf_shifted_minus.push(i as f32 / n as f32));

    let mut uniform_cdf_shifted_plus: Vec<f32> = Vec::new();
    (1..=n).for_each(|i| uniform_cdf_shifted_plus.push(i as f32 / n as f32));

    (uniform_cdf_shifted_minus, uniform_cdf_shifted_plus)
}

/// Calculate the Kolmogorov–Smirnov test statistic
/// This finds the maximum absolute difference between a model or
/// expected CDF and an empirical CDF
///
/// It's assumed the data comes from a uniform distribution with
/// values between a and b inclusive: U[a; b]
///
/// # Examples
///
/// ```
/// use tapestry_analysis::analysis::{
///     distribution::DiscreteUniformDistributionParameters,
///     experiment::Experiment,
///     ks::statistic,
///     sample::Sample,
/// };
///
/// let data: [f32; 8] = [1.88, 0.10, 1.55, 0.89, 0.62, 1.30, 1.20, 1.01];
///
/// let samples: Vec<Sample<f32>> = data.iter().map(|d| Sample { sample: *d }).collect();
///
/// let experiment: Experiment<f32> = Experiment { samples };
/// let parameters = DiscreteUniformDistributionParameters { a: 0, b: 2 };
/// let statistic = statistic(experiment, &parameters);
/// assert!(f32::abs(statistic - 0.195) < 0.0001);
/// ```
pub fn statistic(
    experiment: Experiment<f32>,
    parameters: &DiscreteUniformDistributionParameters,
) -> f32 {
    let mut samples = experiment.samples;

    samples.sort();
    let sorted_data: Vec<f32> = samples.iter().map(|s| s.sample).collect();

    let n = sorted_data.len();

    let (uniform_cdf_shifted_minus, uniform_cdf_shifted_plus) =
        shifted_uniform_cdf_distribution(n as u32);

    let mut interpolated_values: Vec<f32> = Vec::new();

    for item in sorted_data {
        interpolated_values.push(normalize_variable(item, parameters));
    }

    let mut minus_max: f32 = 0.0;
    let mut plus_max: f32 = 0.0;

    for i in 0..n {
        let res = f32::abs(uniform_cdf_shifted_minus[i] - interpolated_values[i]);
        if res > minus_max {
            minus_max = res;
        }
    }

    for i in 0..n {
        let res = f32::abs(uniform_cdf_shifted_plus[i] - interpolated_values[i]);
        if res > plus_max {
            plus_max = res;
        }
    }

    f32::max(minus_max, plus_max)
}

/// Test examples comes from several sources, including:
/// PennState STAT 415 Introduction to Mathematical Statistics
/// https://online.stat.psu.edu/stat415/
#[cfg(test)]
mod tests {
    use crate::analysis::{
        distribution::{CriticalValue, DiscreteUniformDistributionParameters},
        experiment::Experiment,
        ks::{critical_value, shifted_uniform_cdf_distribution, statistic},
        sample::Sample,
    };

    /// PennState STAT 415 Introduction to Mathematical Statistics
    /// Example problem
    /// https://online.stat.psu.edu/stat415/
    #[test]
    fn statistic_works_psu() {
        let data: [f32; 8] = [1.41, 0.26, 1.97, 0.33, 0.55, 0.77, 1.46, 1.18];
        assert_eq!(data, [1.41, 0.26, 1.97, 0.33, 0.55, 0.77, 1.46, 1.18]);

        let samples: Vec<Sample<f32>> = data.iter().map(|d| Sample { sample: *d }).collect();

        let experiment: Experiment<f32> = Experiment { samples };
        let parameters = DiscreteUniformDistributionParameters { a: 0, b: 2 };
        let statistic = statistic(experiment, &parameters);

        assert!(f32::abs(statistic - 0.145) < 0.00001);
    }

    /// Test critical value calculations.
    /// Any approximation of the distribution should meet these
    /// specifications.
    #[test]
    fn test_critical_value_n_below_41_works() {
        let n: u32 = 0;
        let crit_val = critical_value(CriticalValue::TenPercent, n);
        assert!(crit_val.is_none());
        let crit_val = critical_value(CriticalValue::FivePercent, n);
        assert!(crit_val.is_none());
        let crit_val = critical_value(CriticalValue::OnePercent, n);
        assert!(crit_val.is_none());
    }

    /// Test critical value calculations.
    /// Any approximation of the distribution should meet these
    /// specifications.
    #[test]
    fn test_critical_value_n_41_works() {
        let n: u32 = 41;

        let crit_val = critical_value(CriticalValue::TenPercent, n).unwrap();
        assert!(f32::abs(crit_val - 0.163) < 0.03);

        let crit_val = critical_value(CriticalValue::FivePercent, n).unwrap();
        assert!(f32::abs(crit_val - 0.187) < 0.03);

        let crit_val = critical_value(CriticalValue::OnePercent, n).unwrap();
        assert!(f32::abs(crit_val - 0.232) < 0.03);
    }

    /// Test critical value calculations.
    /// Any approximation of the distribution should meet these
    /// specifications.
    #[test]
    fn test_critical_value_n_1000_works() {
        let n: u32 = 1000;

        let crit_val = critical_value(CriticalValue::TenPercent, n).unwrap();
        assert!(f32::abs(crit_val - 0.0338) < 0.01);

        let crit_val = critical_value(CriticalValue::FivePercent, n).unwrap();
        assert!(f32::abs(crit_val - 0.0385) < 0.01);

        let crit_val = critical_value(CriticalValue::OnePercent, n).unwrap();
        assert!(f32::abs(crit_val - 0.0478) < 0.01);
    }

    /// Test full pipeline
    #[test]
    fn test_works() {
        let data: [f32; 8] = [1.41, 0.26, 1.97, 0.33, 0.55, 0.77, 1.46, 1.18];
        assert_eq!(data, [1.41, 0.26, 1.97, 0.33, 0.55, 0.77, 1.46, 1.18]);

        let samples: Vec<Sample<f32>> = data.iter().map(|d| Sample { sample: *d }).collect();

        let experiment: Experiment<f32> = Experiment { samples };
        let parameters = DiscreteUniformDistributionParameters { a: 0, b: 2 };
        let statistic = statistic(experiment, &parameters);

        assert!(f32::abs(statistic - 0.145) < 0.0001);

        // We don't have critical values for n < 41 due to precision concerns
        // This is from a statistical table for n = 8, alpha = 0.05
        // An Introduction to Probability and Statistics, Third Edition,
        // Vijay K. Rohatgi and A.K. Md. Ehsanes Saleh.
        let critical_value = Some(0.410);

        if let Some(cv) = critical_value {
            assert!(statistic < cv);
            if statistic < cv {
                println!("Data follows a uniform distribution");
            } else {
                println!("Data does not follow a uniform distribution");
            }
        } else {
            assert!(false);
        }
    }

    /// Test issues with error propagation.
    /// There was a an issue with floating point calcluations in a previous version
    /// when generating plus and minus shifted CDFs.
    /// This tests for some of those errors.
    #[test]
    fn test_shifted_uniform_cdf_distribution_fp_expected_mean() {
        // Use a larger sample size, the issue wasn't being exposed in the
        // small n=8 test case
        let n: usize = 100000;

        let (uniform_cdf_minus, uniform_cdf_plus) = shifted_uniform_cdf_distribution(n as u32);

        let mut total: f32 = 0.0;
        for s in &uniform_cdf_minus {
            total += *s as f32;
        }
        let ucm_mean: f32 = total / n as f32;

        let mut total: f32 = 0.0;
        for s in &uniform_cdf_plus {
            total += *s as f32;
        }
        let ucp_mean: f32 = total / n as f32;

        // Test for expected distribution behavior.
        // The mean of the mean of uniform_cdf_minus and uniform_cdf_plus should
        // be the mean of the expected distribution, (e.g. 0.5 for normalized uniform)
        assert!(f32::abs(((ucm_mean + ucp_mean) / 2.0) - 0.5) < 0.000001);
    }

    /// Test issues with error propagation.
    /// There was a an issue with floating point calcluations in a previous version
    /// when generating plus and minus shifted CDFs.
    /// This tests for some of those errors.
    #[test]
    fn test_shifted_uniform_cdf_distribution_fp_error_propagation() {
        // Use a larger sample size, the issue wasn't being exposed in the
        // small n=8 test case
        let n: usize = 100000;

        let (uniform_cdf_minus, uniform_cdf_plus) = shifted_uniform_cdf_distribution(n as u32);

        // Test for error propagation, the last value should be exact
        // with 100000 items.
        let ucm_last = uniform_cdf_minus[n - 1];
        let ucp_last = uniform_cdf_plus[n - 1];

        assert_eq!(ucm_last, 0.99999);
        assert_eq!(ucp_last, 1.0);
    }
}
