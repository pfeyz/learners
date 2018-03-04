use std::time::{Duration, SystemTime};

use hypothesis::{Hypothesis, SimpleHypothesis, WeightedHypothesis};
use domain::{NUM_PARAMS, LanguageDomain, Grammar, Sentence};

pub mod trigger;
pub mod variational;

pub use self::trigger::TriggerLearner;
pub use self::variational::{VariationalLearner, RewardOnlyVariationalLearner};

pub struct Environment {
    domain: LanguageDomain,
}

pub trait Consumer {
    fn consume(&mut self, &Environment, &Sentence);
}

pub trait Learner<H: Hypothesis>: Consumer {
    fn learn(&mut self, &Environment, &Sentence);
    fn hypothesis(&self) -> &H;
    fn hypothesis_mut(&mut self) -> &mut H;
    fn reset(&mut self){
        self.hypothesis_mut().reset();
    }
}

#[derive(Debug)]
pub struct LearnerReport {
    hypothesis: String,
}

pub struct VL {             hypothesis: WeightedHypothesis }
pub struct RewardOnlyVL {   hypothesis: WeightedHypothesis }

// impl Learner<WeightedHypothesis> for VL {
//     fn hypothesis(&self) -> &WeightedHypothesis { &self.hypothesis }
//     pub fn new() -> Self {
//         VL { hypothesis: SimpleHypothesis {grammar: 0} }
//     }
// }

// impl Learner<WeightedHypothesis> for RewardOnlyVL {
//   fn hypothesis(&self) -> &WeightedHypothesis { &self.hypothesis }
// }
