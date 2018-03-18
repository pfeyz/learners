use std::fmt::Display;

use hypothesis::{Theory};
use domain::{Sentence, Colag, Grammar};

// pub mod trigger;
pub mod variational;
// pub mod ndl;

// pub use self::trigger::TriggerLearner;
pub use self::variational::{RewardOnlyVL, RewardOnlyRelevantVL};
// pub use self::ndl::NonDefaultsLearner;

pub struct Environment {
    pub domain: Colag
}

// Represents a language learner. They learn from sentences as their input, and
// update their internal hypothesis about which grammar generated the language
// they're seeing. After they have tested/updated this hypothesis for a while
// they will have a Theory as to which grammar generated their language.
pub trait Learner: Display {
    fn learn(&mut self, &Environment, &Sentence);
    fn converged(&mut self) -> bool {
        false
    }
    fn theory(&self) -> Theory;
    fn guess(&mut self) -> Grammar;
}
