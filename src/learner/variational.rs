use learner::{Learner, Environment};
use domain::{Grammar, Sentence, NUM_PARAMS, LanguageDomain};
use hypothesis::{WeightedHypothesis};

pub struct VariationalLearner {
    hypothesis: WeightedHypothesis,
}

impl VL for VariationalLearner {
    fn vl_hypothesis(&self) -> &WeightedHypothesis {
        &self.hypothesis
    }
}

pub struct RewardOnlyVariationalLearner {
    hypothesis: WeightedHypothesis,
}

impl VL for RewardOnlyVariationalLearner {
    fn vl_hypothesis(&self) -> &WeightedHypothesis {
        &self.hypothesis
    }
    fn reward(&mut self, _: &Environment, gram: &Grammar, sent: &Sentence){
        unimplemented!();
    }
}

trait VL: Learner {
    fn vl_hypothesis(&self) -> &WeightedHypothesis;
    fn reward(&mut self, _: &Environment, gram: &Grammar, sent: &Sentence){
        unimplemented!();
    }
    fn punish(&mut self, env: &Environment, gram: &Grammar, sent: &Sentence){
    }
    fn vl_learn(&mut self, env: &Environment, sent: &Sentence){
        loop {
            let g = env.domain.random_weighted_grammar(self.vl_hypothesis().weights);
            match env.domain.parses(&g, sent) {
                Ok(parsed) => {
                    if parsed {
                        self.reward(env, &g, sent);
                    } else {
                        self.punish(env, &g, sent);
                    }
                    break;
                },
                Err(_) => ()  // wasn't a legal grammar, try again.
            }
        }

    }
}

impl VariationalLearner {
    pub fn new() -> VariationalLearner {
        VariationalLearner { hypothesis: WeightedHypothesis::new() }
    }
    pub fn boxed() -> Box<Learner> {
        Box::new(VariationalLearner::new())
    }
}

impl Learner for VariationalLearner {
    fn learn(&mut self, env: &Environment, sent: &Sentence){
        self.vl_learn(env, sent);
    }
}

impl RewardOnlyVariationalLearner {
    pub fn new() -> RewardOnlyVariationalLearner {
        RewardOnlyVariationalLearner { hypothesis: WeightedHypothesis::new() }
    }
    pub fn boxed() -> Box<Learner> {
        Box::new(RewardOnlyVariationalLearner::new())
    }
}

impl Learner for RewardOnlyVariationalLearner {
    fn learn(&mut self, env: &Environment, sent: &Sentence){
        self.vl_learn(env, sent);
    }
}
