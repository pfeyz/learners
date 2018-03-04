use std::env;

mod domain;
mod learner;
mod hypothesis;
mod profiling;

use learner::{Consumer, Learner, TriggerLearner, VariationalLearner, RewardOnlyVariationalLearner};
use profiling::ProfiledLearner;
use hypothesis::{Hypothesis, SimpleHypothesis};

fn main() {
    let args: Vec<String> = env::args().collect();
    if (args.len() < 3){
        println!("takes 3 arguments: num_sentences target_langs learners, for example: 2_000_000 611,2 VL,ROVL,TLA")
    }
    else {
    }
}
