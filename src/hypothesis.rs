use std::mem;

use domain::{Grammar, NUM_PARAMS};

type ParameterWeights = [f64; NUM_PARAMS];
type FuzzyGrammar = ParameterWeights;

pub enum Theory<'a> {
    Simple(&'a SimpleHypothesis),
    Weighted(&'a WeightedHypothesis)
}

pub trait Hypothesis {}

#[derive(Debug)]
pub struct SimpleHypothesis { pub grammar: Grammar }

#[derive(Debug)]
pub struct WeightedHypothesis { pub weights: ParameterWeights }

impl Hypothesis for SimpleHypothesis {}
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
