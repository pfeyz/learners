use std::time::{Duration, SystemTime};

use hypothesis::Hypothesis;
use learner::{Learner, Environment};
use domain::{Grammar, Sentence};

pub struct ProfiledLearner<H: Hypothesis>{
    learner: Box<Learner<H>>,
    target: Grammar,
    consumed: u64,
    start_time: SystemTime,
}

impl<H: Hypothesis> ProfiledLearner<H> {
    fn new(learner: Box<Learner<H>>, target: Grammar) -> Self {
        ProfiledLearner {
            learner: learner,
            target: target,
            consumed: 0,
            start_time: SystemTime::now()
        }
    }
    fn consume(&mut self, env: &Environment, sent: &Sentence) {
        self.consumed += 1;
        self.learner.learn(env, sent);
    }
}
