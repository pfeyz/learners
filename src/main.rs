use std::env;
use std::thread;
use std::collections::HashMap;

mod domain;
mod learner;
mod hypothesis;
mod profiling;

use domain::{Colag, Grammar, LanguageDomain};
use learner::{Learner, TriggerLearner, // VariationalLearner, RewardOnlyVariationalLearner,
              Environment};
use profiling::ProfiledLearner;
use hypothesis::{Hypothesis, Theory};

type LearnerFactory = fn() -> Box<Learner>;

fn learn_language<'a>(target: &Grammar, num_sentences: &u64, env: &Environment, factory: LearnerFactory)
                  -> Box<Learner> {
    let mut learner = factory();
    for _ in 0..*num_sentences {
        let sent = env.domain.random_sentence(target);
        learner.learn(env, sent);
        if (learner.converged()){
            break;
        }
    }
    learner
}

fn get_learner(name: &str) -> Option<LearnerFactory> {
    match name {
        "tla" => Some(TriggerLearner::boxed),
        // "vl" => Some(VariationalLearner::boxed),
        // "rovl" => Some(RewardOnlyVariationalLearner::boxed),
        _ => None
    }
}

fn main() {
    let name = "vl";
    let colag = Colag {};
    let environment = Environment { domain: colag };
    if let Some(factory) = get_learner(&name) {
        let learner = learn_language(&611, &100, &environment, factory);
        match learner.theory() {
            Theory::Simple(h) => println!("{:?}", h),
            Theory::Weighted(h) => println!("{:?}", h)
        }
    } else {
        println!("`{}` is not a valid learner name", name);
    }
}
