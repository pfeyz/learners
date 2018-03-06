use std::time::{Duration, SystemTime};

use hypothesis::{Hypothesis, Theory};
use domain::{NUM_PARAMS, LanguageDomain, Grammar, Sentence, Colag};

pub mod trigger;
// pub mod variational;

pub use self::trigger::TriggerLearner;
// pub use self::variational::{VariationalLearner, RewardOnlyVariationalLearner};

pub struct Environment {
    pub domain: Colag
}

pub trait Learner {
    fn learn(&mut self, &Environment, &Sentence);
    fn converged(&self) -> bool {
        false
    }
    fn theory(&self) -> Theory;
}

#[derive(Debug)]
pub struct LearnerReport {
    hypothesis: String,
}
