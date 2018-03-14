use std::mem;
use std::fmt;

use domain::{Grammar, NUM_PARAMS};

type ParameterWeights = [f64; NUM_PARAMS];
type FuzzyGrammar = ParameterWeights;

// TODO: maybe add a second Grammar field. When we ask the learner for their
// theory they can just tell us a grammar, and we can optionally inspect their
// specific hypothesis.
pub enum Theory<'a> {
    Simple(&'a SimpleHypothesis),
    Weighted(&'a WeightedHypothesis)
}

pub trait Hypothesis {}

#[derive(Debug)]
pub struct SimpleHypothesis { pub grammar: Grammar }

impl fmt::Display for SimpleHypothesis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "SimpleHypothesis {{ grammar: {:013b} }}", self.grammar as i32)
        write!(f, "{}", self.grammar as i32)
    }
}

#[derive(Debug)]
pub struct WeightedHypothesis { pub weights: ParameterWeights }

impl fmt::Display for WeightedHypothesis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "WeightedHypothesis {{ weights: {:013b} }}",
        //        Colag::random_weighted_grammar(self.weights));
        // write!(f, "WeightedHypothesis {{ weights: [")?;
        // write!(f, "{}, ", Colag::random_weighted_grammar(self.weights))?;
        for i in 0..NUM_PARAMS {
            write!(f, "{:.6}, ", self.weights[i])?;
        }
        // write!(f, "]}}")?;
        Ok(())
    }
}

impl Hypothesis for WeightedHypothesis {}

impl WeightedHypothesis {
    pub fn new() -> Self {
        unsafe {
            let mut array: ParameterWeights = mem::uninitialized();
            for param in 0..NUM_PARAMS {
                array[param] = 0.5;
            }
            WeightedHypothesis { weights: array }
        }
    }
}
