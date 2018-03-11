use std::mem;
use learner::{Learner, Environment};
use domain::{Grammar, Sentence, NUM_PARAMS, LanguageDomain, Colag, get_param, Trigger};
use hypothesis::{WeightedHypothesis, Theory};

const LEARNING_RATE: f64 = 0.001;
const THRESHOLD: f64 = 0.02;

type Mask = [bool; NUM_PARAMS];

// reward only VL

pub struct RewardOnlyVL {
    hypothesis: WeightedHypothesis,
}

impl VL for RewardOnlyVL {
    fn vl_hypothesis(&mut self) -> &mut WeightedHypothesis {
        &mut self.hypothesis
    }
}

// reward only relevant VL

pub struct RewardOnlyRelevantVL {
    hypothesis: WeightedHypothesis,
}

impl VL for RewardOnlyRelevantVL {
    fn vl_hypothesis(&mut self) -> &mut WeightedHypothesis {
        &mut self.hypothesis
    }

    fn reward(&mut self, env: &Environment, gram: &Grammar, sent: &Sentence){
        let triggers = env.domain.triggers(&sent);
        let ref mut weights = self.hypothesis.weights;
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

// masked VL

pub struct MaskedVL {
    hypothesis: WeightedHypothesis,
    mask: Mask
}

const TEMP: [bool; NUM_PARAMS] = [false; NUM_PARAMS];

impl VL for MaskedVL {
    fn vl_hypothesis(&mut self) -> &mut WeightedHypothesis {
        &mut self.hypothesis
    }
    fn reward(&mut self, env: &Environment, gram: &Grammar, sent: &Sentence){
        let triggers = env.domain.triggers(&sent);
        let mask = mem::replace(&mut self.mask, TEMP);
        {
            let ref mut weights = self.vl_hypothesis().weights;
            for param in 0..NUM_PARAMS {
                if mask[param]{
                    continue;
                }
                if get_param(gram, param) == 0 {
                    weights[param] -= LEARNING_RATE * weights[param]
                } else {
                    weights[param] += LEARNING_RATE * (1. - weights[param])
                }
            }
        }
        mem::swap(&mut TEMP, &mut self.mask);
    }
}
// boilerplate

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
    fn reward(&mut self, _: &Environment, gram: &Grammar, _: &Sentence){
        let ref mut hyp = self.vl_hypothesis();
        let ref mut weights = hyp.weights;
        for param in 0..NUM_PARAMS {
            if get_param(gram, param) == 0 {
                weights[param] -= LEARNING_RATE * weights[param];
            } else {
                weights[param] += LEARNING_RATE * (1. - weights[param]);
            }
        }
    }
    fn punish(&mut self, _env: &Environment, _gram: &Grammar, _sent: &Sentence){
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

impl RewardOnlyVL {
    pub fn new() -> RewardOnlyVL {
        RewardOnlyVL { hypothesis: WeightedHypothesis::new() }
    }
    pub fn boxed() -> Box<Learner> {
        Box::new(RewardOnlyVL::new())
    }
}

impl Learner for RewardOnlyVL {
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

impl RewardOnlyRelevantVL {
    pub fn new() -> RewardOnlyRelevantVL {
        RewardOnlyRelevantVL { hypothesis: WeightedHypothesis::new() }
    }
    pub fn boxed() -> Box<Learner> {
        Box::new(RewardOnlyRelevantVL::new())
    }
}

impl Learner for RewardOnlyRelevantVL {
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

impl MaskedVL {
    pub fn new(mask: Mask) -> MaskedVL {
        MaskedVL { hypothesis: WeightedHypothesis::new(), mask: mask }
    }
    pub fn boxed(mask: Mask) -> Box<Learner> {
        Box::new(MaskedVL::new(mask))
    }
}

impl Learner for MaskedVL {
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

mod bench {
    extern crate test;
    use self::test::Bencher;
    use rand::{Rng, thread_rng};
    use learner::{RewardOnlyVL, RewardOnlyRelevantVL, Learner, Environment};
    use domain::{Colag, LanguageDomain, Sentence, Grammar};
    use speaker::{UniformRandomSpeaker};

    #[bench]
    fn reward_only_vl(b: &mut Bencher) {
        let colag = Colag::default();
        let env = Environment { domain: colag };
        let mut speaker = UniformRandomSpeaker::new(&env.domain, 611);
        let mut learner = RewardOnlyVL::new();
        let mut sentences: Vec<&Sentence> = speaker.take(5_000_000).collect();
        b.iter(|| learner.learn(&env, sentences.pop().unwrap()));
    }

    #[bench]
    fn reward_only_relevant_vl(b: &mut Bencher) {
        let colag = Colag::default();
        let env = Environment { domain: colag };
        let mut speaker = UniformRandomSpeaker::new(&env.domain, 611);
        let mut learner = RewardOnlyRelevantVL::new();
        let mut sentences: Vec<&Sentence> = speaker.take(5_000_000).collect();
        b.iter(|| learner.learn(&env, sentences.pop().unwrap()));
    }
}
