use std::mem;
use learner::{Learner, Environment};
use domain::{Grammar, Sentence, NUM_PARAMS, LanguageDomain, Colag, get_param, Trigger};
use hypothesis::{WeightedHypothesis, Theory};

use rand;
use rand::XorShiftRng;

const LEARNING_RATE: f64 = 0.001;
const THRESHOLD: f64 = 0.02;

// reward only VL

pub struct RewardOnlyVL {
    hypothesis: WeightedHypothesis,
    rng: rand::XorShiftRng,
}

impl RewardOnlyVL {
    pub fn new() -> RewardOnlyVL {
        RewardOnlyVL { hypothesis: WeightedHypothesis::new(),
                       rng: rand::weak_rng() }
    }
    pub fn boxed() -> Box<Learner> {
        Box::new(RewardOnlyVL::new())
    }

    fn reward(&mut self, _: &Environment, gram: &Grammar, _: &Sentence){
        let ref mut hyp = self.hypothesis;
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
}

impl Learner for RewardOnlyVL {
    fn learn(&mut self, env: &Environment, sent: &Sentence){
        loop {
            let g = Colag::random_weighted_grammar(&mut self.rng,
                                                   &self.hypothesis.weights);
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

        fn converged(&mut self) -> bool {
            for weight in self.hypothesis.weights.iter() {
                if (weight > &THRESHOLD) & (weight < &(1.0 - THRESHOLD)) {
                    return false;
                }
            }
            true
        }
    fn theory(&self) -> Theory {
        Theory::Weighted(&self.hypothesis)
    }
}

pub struct RewardOnlyRelevantVL {
    hypothesis: WeightedHypothesis,
    rng: rand::XorShiftRng,
}

impl RewardOnlyRelevantVL {
    pub fn new() -> RewardOnlyRelevantVL {
        RewardOnlyRelevantVL { hypothesis: WeightedHypothesis::new(),
                       rng: rand::weak_rng() }
    }
    pub fn boxed() -> Box<Learner> {
        Box::new(RewardOnlyRelevantVL::new())
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

    fn punish(&mut self, _env: &Environment, _gram: &Grammar, _sent: &Sentence){
    }
}

impl Learner for RewardOnlyRelevantVL {
    fn learn(&mut self, env: &Environment, sent: &Sentence){
        loop {
            let g = Colag::random_weighted_grammar(&mut self.rng,
                                                   &self.hypothesis.weights);
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

        fn converged(&mut self) -> bool {
            for weight in self.hypothesis.weights.iter() {
                if (weight > &THRESHOLD) & (weight < &(1.0 - THRESHOLD)) {
                    return false;
                }
            }
            true
        }
    fn theory(&self) -> Theory {
        Theory::Weighted(&self.hypothesis)
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
        // let mut sentences: Vec<&Sentence> = speaker.take(5_000_000).collect();
        b.iter(|| learner.learn(&env, speaker.next().unwrap()));
    }

    #[bench]
    fn reward_only_relevant_vl(b: &mut Bencher) {
        let colag = Colag::default();
        let env = Environment { domain: colag };
        let mut speaker = UniformRandomSpeaker::new(&env.domain, 611);
        let mut learner = RewardOnlyRelevantVL::new();
        // let mut sentences: Vec<&Sentence> = speaker.take(5_000_000).collect();
        b.iter(|| learner.learn(&env, speaker.next().unwrap()));
    }

}
