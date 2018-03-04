use std::env;
use std::thread;
use std::collections::HashMap;

mod domain;
mod learner;
mod hypothesis;
mod profiling;

use domain::{Colag, Grammar};
use learner::{Learner, TriggerLearner, VariationalLearner, RewardOnlyVariationalLearner, Environment};
use profiling::ProfiledLearner;
use hypothesis::{Hypothesis, SimpleHypothesis};

type LearnerFactory = fn() -> Box<Learner>;

fn learner_table() -> HashMap<String, LearnerFactory> {
    let mut learners: HashMap<String, LearnerFactory> = HashMap::new();
    learners.insert("tla".to_string(), TriggerLearner::boxed);
    learners.insert("vl".to_string(), VariationalLearner::boxed);
    learners.insert("rovl".to_string(), RewardOnlyVariationalLearner::boxed);
    learners
}

fn learn_language(target: &Grammar, num_sentences: &u64, env: &Environment, factory: LearnerFactory){
    let mut learner = factory();
    for _ in 0..*num_sentences {
        learner.learn(env, &24);
    }
}

fn main() {
    let learners = learner_table();
    let name = "vl";
    let colag = Colag {};
    let environment = Environment { domain: colag };
    match learners.get(name) {
        Some(factory) => learn_language(&611, &100, &environment, *factory),
        _ => {
            println!("`{}` is not a valid learner name", name);
        }
    }
}
