//! A basic Sample type for sampling from distributions
use std::{
    cmp::Ordering,
    fmt::{Display, Formatter, Result},
};

/// A single sample from a distribution
///
/// The implementation of f32 for Sample included in this crate does
/// not follow IEEE 758
/// See the comments for Ord for details:
///
/// See [`impl Ord for Sample<f32>`](Sample#impl-Ord-for-Sample<f32>)
#[derive(Debug)]
pub struct Sample<T> {
    /// The sample item itself
    pub sample: T,
}

/// Format a Sample for display
impl Display for Sample<f32> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let number: f64 = self.sample.into();
        let width: usize = 3;
        write!(f, "{number:>width$}")
    }
}

/// Format a Sample for display
impl<T> Default for Samples<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// A vector of samples
pub struct Samples<T>(Vec<Sample<T>>);

impl From<Vec<f32>> for Samples<f32> {
    fn from(v: Vec<f32>) -> Self {
        Samples(v.iter().map(|d| Sample { sample: *d }).collect())
    }
}

/// Format Samples for display
impl Display for Samples<f32> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for s in &self.0 {
            write!(f, "{}, ", s).unwrap();
        }
        writeln!(f)
    }
}

impl<T> Samples<T> {
    /// Create a new Samples object
    pub fn new() -> Samples<T> {
        Samples::<T>(Vec::new())
    }

    /// Add a sample to the samples
    pub fn add(&mut self, sample: T) {
        self.0.push(Sample { sample });
    }
}

impl PartialOrd for Sample<f32> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.sample.partial_cmp(&other.sample)
    }
    fn lt(&self, other: &Self) -> bool {
        self.sample < other.sample
    }
    fn le(&self, other: &Self) -> bool {
        self.sample <= other.sample
    }
    fn gt(&self, other: &Self) -> bool {
        self.sample > other.sample
    }
    fn ge(&self, other: &Self) -> bool {
        self.sample >= other.sample
    }
}

impl PartialEq for Sample<f32> {
    fn eq(&self, other: &Self) -> bool {
        self.sample == other.sample
    }
}

impl Eq for Sample<f32> {}

/// A non-IEEE 754 implementation of Ord for Sample<f32>
///
/// We want to be able to sort samples for algorithms like Kolmogorov–Smirnov
/// So we'll provide an implementation for our Sample type based on total_cmp
///
/// f32 doesn't have an Ord implementation by default.
/// But it does have a total ordering.
///
/// The normal definition of a total ordering on a set is when any two
/// elements of a set are comparable.  Below some of the details
/// of comparisons between floating point values are outlined.
///
/// Positive and negative zero
/// The common IEEE standard for floating point, IEEE 754, has a
/// signed zero.  There exist two zeros, +0 and -0.
/// It's unclear how these should be ordered for this use case.
///
/// Positive and negative infinity
/// IEEE 754 has both positive and negative infinity
/// Defining negative infinity as less than positive infinity usually
/// makes sense.
/// But there can be issues, because Rust total_cmp defines INFINITY
/// as greater than MAX.
///
/// NaN
/// IEEE 754 has a NaN result or element.  It may not make sense to compare this
/// element to other elements for ordering, depending on your application.
///
/// When we sample from a distribution, normally NaN and infinity are not considered valid
/// values or elements.  So we can ignore how those values are compared.
/// We may generate infinity when normalizing or for distributions
/// with large sample sizes.
/// Zero does matter, because it can be sampled.  But we only consider
/// one of the zeros usually.
/// We could add additional type constraints to the Sample type to make this more clear.
///
/// total_cmp does order NaN \
/// total_cmp orders -0.0 as less than 0.0 \
/// total_cmp orders -infinity as less than infinity
impl Ord for Sample<f32> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sample.total_cmp(&other.sample)
    }
}

impl<T> FromIterator<T> for Samples<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut c = Samples::<T>::new();

        for i in iter {
            c.add(i);
        }

        c
    }
}

#[cfg(test)]
mod tests {
    use crate::analysis::sample::Sample;

    /// Test ordering of f32 samples
    #[test]
    fn test_f32_sample_ord_works() {
        let data: [f32; 8] = [1.23, 0.85, 1.62, 0.31, 0.55, 0.26, 1.91, 1.18];
        assert_eq!(data, [1.23, 0.85, 1.62, 0.31, 0.55, 0.26, 1.91, 1.18]);
        let mut samples: Vec<Sample<f32>> = data.iter().map(|d| Sample { sample: *d }).collect();
        samples.sort();
        let sorted_data: Vec<f32> = samples.iter().map(|s| s.sample).collect();

        assert_eq!(
            sorted_data,
            [0.26, 0.31, 0.55, 0.85, 1.18, 1.23, 1.62, 1.91]
        );
    }

    /// Test PartialEq and Eq for f32 Samples
    #[test]
    fn test_f32_sample_eq_works() {
        let sample_1 = Sample { sample: 0.01 };
        let sample_2 = Sample { sample: 0.01 };
        let sample_3 = Sample { sample: 0.03 };

        assert_eq!(sample_1, sample_2);
        assert_eq!(sample_2, sample_1);
        assert_ne!(sample_1, sample_3);
        assert_ne!(sample_3, sample_1);

        assert!(sample_1.eq(&sample_2));
        assert!(!sample_1.eq(&sample_3));
        assert!(sample_2.eq(&sample_1));
        assert!(!sample_3.eq(&sample_1));
    }

    /// Test PartialOrd for f32 Samples
    #[test]
    fn test_f32_sample_partial_ord_works() {
        let sample_1 = Sample { sample: 0.01 };
        let sample_2 = Sample { sample: 0.01 };
        let sample_3 = Sample { sample: 0.03 };

        assert_eq!(
            sample_1.partial_cmp(&sample_2),
            Some(std::cmp::Ordering::Equal)
        );
        assert_eq!(
            sample_1.partial_cmp(&sample_3),
            Some(std::cmp::Ordering::Less)
        );
        assert_eq!(
            sample_3.partial_cmp(&sample_1),
            Some(std::cmp::Ordering::Greater)
        );

        assert!(sample_1 < sample_3);
        assert!(!(sample_1 < sample_2));
        assert!(!(sample_3 < sample_1));

        assert!(sample_1 <= sample_2);
        assert!(sample_1 <= sample_3);
        assert!(!(sample_3 <= sample_1));

        assert!(sample_3 > sample_1);
        assert!(!(sample_1 > sample_2));
        assert!(!(sample_1 > sample_3));

        assert!(sample_1 >= sample_2);
        assert!(sample_3 >= sample_1);
        assert!(!(sample_1 >= sample_3));
    }
}
