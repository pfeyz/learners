use std::mem;

use domain::{Grammar, NUM_PARAMS};

type ParameterWeights = [f64; NUM_PARAMS];
type FuzzyGrammar = ParameterWeights;

pub trait Hypothesis {
    fn reset(&mut self);
}

pub struct SimpleHypothesis { pub grammar: Grammar }
pub struct WeightedHypothesis { pub weights: ParameterWeights }

impl Hypothesis for SimpleHypothesis {
    fn reset(&mut self){
        self.grammar = 0;
    }
}
impl Hypothesis for WeightedHypothesis {
    fn reset(&mut self){
        *self = WeightedHypothesis::new();
    }
}

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
