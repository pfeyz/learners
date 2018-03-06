use hypothesis::{Theory};
use domain::{Sentence, Colag};

pub mod trigger;
pub mod variational;
pub mod ndl;

pub use self::trigger::TriggerLearner;
pub use self::variational::{VariationalLearner, RewardOnlyVariationalLearner};
pub use self::ndl::NonDefaultsLearner;

pub struct Environment {
    pub domain: Colag
}

pub trait Learner {
    fn learn(&mut self, &Environment, &Sentence);
    fn converged(&mut self) -> bool {
        false
    }
    fn theory(&self) -> Theory;
}
