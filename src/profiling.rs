use std::time::{Duration, SystemTime};

use hypothesis::Hypothesis;
use learner::{Learner, Environment};
use domain::{Grammar, Sentence};

pub struct ProfiledLearner{
    learner: Box<Learner>,
    target: Grammar,
    consumed: u64,
    start_time: SystemTime,
}

impl ProfiledLearner {
    fn new(learner: Box<Learner>, target: Grammar) -> Self {
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
