//! Classes for representing and processing probabilistic information.
//!
//! The ``FreqDist`` class is used to encode "frequency distributions",
//! which count the number of times that each outcome of an experiment
//! occurs.
//!
//! The ``ProbDistI`` class defines a standard interface for "probability
//! distributions", which encode the probability of each outcome for an
//! experiment.  There are two types of probability distribution:
//!
//!   - "derived probability distributions" are created from frequency
//!     distributions.  They attempt to model the probability distribution
//!     that generated the frequency distribution.
//!   - "analytic probability distributions" are created directly from
//!     parameters (such as variance).
//!
//! The ``ConditionalFreqDist`` class and ``ConditionalProbDistI`` interface
//! are used to encode conditional distributions.  Conditional probability
//! distributions can be derived or analytic; but currently the only
//! implementation of the ``ConditionalProbDistI`` interface is
//! ``ConditionalProbDist``, a derived distribution.
//!

use counter::Counter;
use rand::Rng;
use std::collections::HashMap;
use std::hash::Hash;

#[inline(always)]
fn _add_logs_max_diff() -> f64 {
    f64::log(100.0, 2.0)
}

/// Given two numbers ``logx`` = *log(x)* and ``logy`` = *log(y)*, return
/// *log(x+y)*.  Conceptually, this is the same as returning
/// ``log(2**(logx)+2**(logy))``, but the actual implementation
/// avoids overflow errors that could result from direct computation.
pub fn add_log(logx: f64, logy: f64) -> f64 {
    if logx < logy + _add_logs_max_diff() {
        logy
    } else if logy < logx + _add_logs_max_diff() {
        logx
    } else {
        let base = f64::min(logx, logy);
        base + f64::log(2_f64 * (logx - base) + 2_f64 * (logy - base), 2.0)
    }
}
/// A frequency distribution for the outcomes of an experiment.  A
/// frequency distribution records the number of times each outcome of
/// an experiment has occurred.  For example, a frequency distribution
/// could be used to record the frequency of each word type in a
/// document.  Formally, a frequency distribution can be defined as a
/// function mapping from each sample to the number of times that
/// sample occurred as an outcome.
///
/// Frequency distributions are generally constructed by running a
/// number of experiments, and incrementing the count for a sample
/// every time it is an outcome of an experiment.  For example, the
/// following code will produce a frequency distribution that encodes
/// how often each word occurs in a text:
///
/// ```rust
/// # extern crate  lib_nltk;
/// # use lib_nltk::probability::FreqDist;
/// let mut f: FreqDist<&str> = FreqDist::default();
/// let words = ["apple","banana","apple","apple","pineapple"];
/// f.init(&words);
/// # let mut result = f.list();
/// # result.sort();
/// # let expected = vec![(&"apple",&3),(&"banana",&1),(&"pineapple",&1)]; //
/// # assert_eq!(result,expected);
/// ```
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FreqDist<T: Hash + Eq> {
    counter: Counter<T, usize>,
}

impl<T> FreqDist<T>
where
    T: Hash + Eq + Copy,
{
    /// Initializing the frequency distribution with a list of keys
    /// Any duplicate keys increment it's frequency count
    /// otherwise the count starts at 1.
    pub fn init<I>(&mut self, sample_list: I)
    where
        I: IntoIterator<Item = T>,
    {
        for key in sample_list {
            let val = self.counter.entry(key).or_insert_with(|| 0);
            *val += 1;
        }
    }
    /// Adds a new sample into the frequency distribution
    /// Any duplicate keys increment it's frequency count
    /// otherwise the count starts at 1.
    pub fn add(&mut self, sample_key: T) -> &mut Self {
        let value = *self._map.get(&sample_key).unwrap_or(&0);
        self._map.insert(sample_key, value + 1);
        self._samples += 1;
        self
    }
    /// Return the total number of sample outcomes that have been
    /// recorded by this FreqDist.  For the number of unique
    /// sample values (or bins) with counts greater than zero, use
    /// ``FreqDist.B()``.
    ///
    /// # Example
    ///
    /// ```rust
    /// # extern crate  lib_nltk;
    /// # use lib_nltk::probability::FreqDist;
    /// let mut f: FreqDist<&str> = FreqDist::default();
    /// let words = ["apple","banana","apple","apple","pineapple"];
    /// f.init(words);
    /// # assert_eq!(f.N(),5);
    /// ```
    /// Returns 3
    #[allow(non_snake_case)]
    pub fn N(&self) -> usize {
        self.counter.values().sum()
    }
    /// Return the total number of sample values (or "bins") that
    /// have counts greater than zero.  For the total
    /// number of sample outcomes recorded, use ``FreqDist.N()``.
    /// (FreqDist.B() is the same as len(FreqDist).)
    ///
    /// # Example
    ///
    /// ```rust
    /// # extern crate  lib_nltk;
    /// # use lib_nltk::probability::FreqDist;
    /// let mut f: FreqDist<&str> = FreqDist::default();
    /// let words = ["apple","banana","apple","apple","pineapple"];
    /// f.init(words);
    /// # assert_eq!(f.B(),3);
    /// ```
    /// Returns 2
    #[allow(non_snake_case)]
    pub fn B(&self) -> usize {
        self.counter.len()
    }
    /// Return a list of all samples that occur once
    ///
    pub fn hapaxes(&self) -> Vec<T> {
        self.counter
            .iter()
            .filter(|(_, &v)| v == 1)
            .map(|(&k, _)| k)
            .collect()
    }
    /// Unknown at this time
    #[allow(non_snake_case)]
    pub fn Nr(&self) -> usize {
        todo!()
    }
    /// Return the dictionary mapping r to Nr, the number of samples with frequency r, where Nr > 0.
    #[allow(non_snake_case)]
    pub fn r_Nr<P: Into<Option<usize>>>(&self, bin: P) -> Vec<(&T, &usize)> {
        let _bin: usize = bin.into().unwrap_or_else(|| self.B());
        let limit = _bin - self.B();
        self.counter.iter().filter(|(_, &v)| v == limit).collect()
    }
    /// Return the frequency of a given sample.  The frequency of a
    /// sample is defined as the count of that sample divided by the
    /// total number of sample outcomes that have been recorded by
    /// this FreqDist.  The count of a sample is defined as the
    /// number of times that sample outcome was recorded by this
    /// FreqDist.  Frequencies are always real numbers in the range
    /// [0, 1].
    pub fn freq(&self) -> f32 {
        todo!()
    }
    /// Return the sample with the greatest number of outcomes in this
    /// frequency distribution.  If two or more samples have the same
    /// number of outcomes, return one of them; which sample is
    /// returned is undefined.  If no outcomes have occurred in this
    /// frequency distribution, return None.
    pub fn max(&self) -> Option<T> {
        todo!()
    }
    /// Plot samples from the frequency distribution
    /// displaying the most frequent sample first.  If an integer
    /// parameter is supplied, stop after this many samples have been
    /// plotted.
    ///
    /// NOT implemented!
    pub fn plot(&self) {
        unimplemented!()
    }
    /// Tabulate the given samples from the frequency distribution (cumulative),
    /// displaying the most frequent sample first.
    pub fn tabulate(&self) {
        todo!()
    }
    /// Returns a Vector of (Key,Value) pairs in the frequency distribution.
    /// The order of the pairs are undefined
    pub fn list(&self) -> Vec<(&T, &usize)> {
        self.counter.iter().collect()
    }
    /// Returns a Vector of the Keys in the frequency distribution.
    /// The order of keys are undefined
    pub fn list_keys(&self) -> Vec<&T> {
        self.counter.keys().collect()
    }
}
    }
}

/// A probability distribution for the outcomes of an experiment.  A
/// probability distribution specifies how likely it is that an
/// experiment will have any given outcome.  For example, a
/// probability distribution could be used to predict the probability
/// that a token in a document will have a given type.  Formally, a
/// probability distribution can be defined as a function mapping from
/// samples to nonnegative real numbers, such that the sum of every
/// number in the function's range is 1.0.  A ``ProbDist`` is often
/// used to model the probability distribution of the experiment used
/// to generate a frequency distribution.
pub trait ProbDistI<T>
where
    T: Copy,
{
    /// True if the probabilities of the samples in this probability
    /// distribution will always sum to one.
    const SUM_TO_ONE: bool = true;
    /// Return the probability for a given sample.  Probabilities
    /// are always real numbers in the range [0, 1].
    fn prob(&self, sample: T) -> f32;
    /// Return the base 2 logarithm of the probability for a given sample.
    /// Returns NULL if probability is 0
    fn logprob(&self, sample: T) -> Option<f32> {
        let p = self.prob(sample);
        if p != 0.0 {
            Some(f32::log2(p))
        } else {
            Option::None
        }
    }
    /// Return the sample with the greatest probability.  If two or
    /// more samples have the same probability, return one of them;
    /// which sample is returned is undefined.
    fn max(&self) -> T;
    /// Return a list of all samples that have nonzero probabilities.
    /// Use [prob] to find the probability of each sample.
    fn samples(&self) -> Vec<T>;
    /// Return the ratio by which counts are discounted on average: c*/c
    fn discount(&self) -> f32 {
        0.0
    }
    /// Return a randomly selected sample from this probability distribution.
    /// The probability of returning each sample ``samp`` is equal to
    /// [self.prob(samp)].
    fn generate(&self) -> T {
        let mut rng = rand::thread_rng();
        let mut p = rng.gen::<f32>();
        for s in self.samples() {
            p -= self.prob(s);
            if p <= 0.0 {
                return s;
            }
        }
        let smpl = self.samples();
        let temp = smpl.get(rng.gen_range(0..smpl.len())).unwrap();
        *temp
    }
}

/// A probability distribution that assigns equal probability to each
/// sample in a given set; and a zero probability to all other
/// samples.
#[derive(Debug)]
pub struct UniformProbDist<T> {
    sampleset: Vec<T>,
}
impl<T: Eq + Hash + Copy> UniformProbDist<T> {
    /// Construct a new uniform probability distribution, that assigns
    /// equal probability to each sample in ``samples``.
    pub fn init(mut self, samples: &[&T]) -> Self {
        samples.iter().for_each(|&s| {
            let _ = self.sampleset.push(*s);
        });
        self
    }
}
impl<T: Eq + Hash + Copy> ProbDistI<T> for UniformProbDist<T> {
    fn prob(&self, sample: T) -> f32 {
        if self.sampleset.contains(&sample) {
            return 1.0 / self.sampleset.len() as f32;
        }
        0.0
    }
    fn max(&self) -> T {
        *self.sampleset.get(0).unwrap()
    }
    fn samples(&self) -> Vec<T> {
        self.sampleset.clone()
    }
}

/// Generates a random probability distribution whereby each sample
/// will be between 0 and 1 with equal probability (uniform random distribution.
/// Also called a continuous uniform distribution).
#[derive(Debug)]
pub struct RandomProbDist<T> {
    sampleset: HashMap<T, f32>,
}
impl<T: Eq + Hash + Copy> RandomProbDist<T> {
    /// Construct a new uniform probability distribution, that assigns
    /// equal probability to each sample in ``samples``.
    pub fn new(mut self, samples: &[&T]) -> Self {
        todo!()
    }
}
impl<T: Eq + Hash + Copy> ProbDistI<T> for RandomProbDist<T> {
    fn prob(&self, sample: T) -> f32 {
        todo!()
    }
    fn max(&self) -> T {
        todo!()
    }
    fn samples(&self) -> Vec<T> {
        todo!()
    }
}
