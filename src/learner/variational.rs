use learner::{Learner, Environment, Consumer};
use domain::{Grammar, Sentence, NUM_PARAMS};
use hypothesis::{WeightedHypothesis};

pub struct VariationalLearner {
    hypothesis: WeightedHypothesis,
}

impl VL for VariationalLearner {
    fn reward(&mut self, _: &Environment, gram: &Grammar, sent: &Sentence){
        unimplemented!();
    }
}

pub struct RewardOnlyVariationalLearner {
    hypothesis: WeightedHypothesis,
}

impl VL for RewardOnlyVariationalLearner {
    fn reward(&mut self, _: &Environment, gram: &Grammar, sent: &Sentence){
        unimplemented!();
    }
}

trait VL: Learner<WeightedHypothesis> {
    fn reward(&mut self, env: &Environment, gram: &Grammar, sent: &Sentence);
    fn punish(&mut self, env: &Environment, gram: &Grammar, sent: &Sentence){
    }
    fn vl_learn(&mut self, env: &Environment, sent: &Sentence){
        loop {
            let g = env.domain.random_weighted_grammar(self.hypothesis().weights);
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
}

impl Learner<WeightedHypothesis> for VariationalLearner {
    fn hypothesis(&self) -> &WeightedHypothesis { &self.hypothesis }
    fn hypothesis_mut(&mut self) -> &mut WeightedHypothesis { &mut self.hypothesis }
    fn learn(&mut self, env: &Environment, sent: &Sentence){
        self.vl_learn(env, sent);
    }
}

impl RewardOnlyVariationalLearner {
    pub fn new() -> RewardOnlyVariationalLearner {
        RewardOnlyVariationalLearner { hypothesis: WeightedHypothesis::new() }
    }
}

impl Learner<WeightedHypothesis> for RewardOnlyVariationalLearner {
    fn hypothesis(&self) -> &WeightedHypothesis { &self.hypothesis }
    fn hypothesis_mut(&mut self) -> &mut WeightedHypothesis { &mut self.hypothesis }
    fn learn(&mut self, env: &Environment, sent: &Sentence){
        self.vl_learn(env, sent);
    }
}

impl Consumer for VariationalLearner {
    fn consume(&mut self, env: &Environment, sent: &Sentence){
        self.learn(env, sent);
    }
}

impl Consumer for RewardOnlyVariationalLearner {
    fn consume(&mut self, env: &Environment, sent: &Sentence){
        self.learn(env, sent);
    }
}
