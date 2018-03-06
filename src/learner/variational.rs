use learner::{Learner, Environment};
use domain::{Grammar, Sentence, NUM_PARAMS, LanguageDomain, Colag, get_param, Trigger};
use hypothesis::{WeightedHypothesis, Theory};

const LEARNING_RATE: f64 = 0.001;
const THRESHOLD: f64 = 0.02;

pub struct VariationalLearner {
    hypothesis: WeightedHypothesis,
}

impl VL for VariationalLearner {
    fn vl_hypothesis(&mut self) -> &mut WeightedHypothesis {
        &mut self.hypothesis
    }
}

pub struct RewardOnlyVariationalLearner {
    hypothesis: WeightedHypothesis,
}

impl VL for RewardOnlyVariationalLearner {
    fn vl_hypothesis(&mut self) -> &mut WeightedHypothesis {
        &mut self.hypothesis
    }
    fn reward(&mut self, env: &Environment, gram: &Grammar, sent: &Sentence){
        let triggers = env.domain.trigger.get(&sent).unwrap();
        let ref mut hyp = self.vl_hypothesis();
        let ref mut weights = hyp.weights;
        for param in 0..NUM_PARAMS {
            let rate = match triggers[param] {
                Trigger::On | Trigger::Off => { LEARNING_RATE },
                Trigger::Ambiguous         => { LEARNING_RATE }
                Trigger::Irrelevant        => { 0. },
            };
            if get_param(gram, param) == 0 {
                weights[param] -= rate * weights[param]
            } else {
                weights[param] += rate * (1. - weights[param])
            }
        }
    }
}

trait VL: Learner {
    fn vl_hypothesis(&mut self) -> &mut WeightedHypothesis;
    fn vl_converged(&mut self) -> bool {
        for weight in self.vl_hypothesis().weights.iter() {
            if (weight > &THRESHOLD) & (weight < &(1.0 - THRESHOLD)) {
                return false;
            }
        }
        true
    }
    fn reward(&mut self, _: &Environment, gram: &Grammar, sent: &Sentence){
        let ref mut hyp = self.vl_hypothesis();
        let ref mut weights = hyp.weights;
        for param in 0..NUM_PARAMS {
            if get_param(gram, param) == 0 {
                weights[param] -= LEARNING_RATE * weights[param]
            } else {
                weights[param] += LEARNING_RATE * (1. - weights[param])
            }
        }
    }
    fn punish(&mut self, env: &Environment, gram: &Grammar, sent: &Sentence){
    }
    fn vl_learn(&mut self, env: &Environment, sent: &Sentence){
        loop {
            let g = Colag::random_weighted_grammar(self.vl_hypothesis().weights);
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
    fn theory(&self) -> Theory {
        Theory::Weighted(&self.hypothesis)
    }
    fn converged(&mut self) -> bool {
        self.vl_converged()
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
    fn theory(&self) -> Theory {
        Theory::Weighted(&self.hypothesis)
    }
    fn converged(&mut self) -> bool {
        self.vl_converged()
    }
}
