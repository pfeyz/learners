use learner::{Learner, Environment};
use domain::{Sentence, IllegalGrammar, LanguageDomain};
use hypothesis::{SimpleHypothesis, Theory};

use rand;

pub struct TriggerLearner {
    hypothesis: SimpleHypothesis,
    clean_parses: u32,
    rng: rand::XorShiftRng,
}

impl Learner for TriggerLearner {
    fn learn(&mut self, env: &Environment, sent: &Sentence){
        let parses = env.domain.parses(&self.hypothesis.grammar, sent);
        match parses {
            Ok(false) | Err(IllegalGrammar {..}) => {
                // TODO: should this only be guessing legal grammars?
                let new_grammar = *env.domain.random_grammar(&mut self.rng);
                if let Ok(true) = env.domain.parses(&new_grammar, sent){
                    // if the new grammar is a better hypothesis, adopt it.
                    self.hypothesis.grammar = new_grammar;
                }
                self.clean_parses = 0;
                // the new grammar also failed to parse the input. let's follow
                // the greediness princple and not change our minds.
            },
            _ => {
                self.clean_parses += 1;
                // our hypothesis worked, let's keep it.
            }
        }
    }
    fn theory<'a>(&'a self) -> Theory<'a> {
        Theory::Simple(&self.hypothesis)
    }
    fn converged(&mut self) -> bool {
        self.clean_parses > 1000
    }
}

impl TriggerLearner {
    pub fn new() -> Self {
        TriggerLearner { hypothesis: SimpleHypothesis {grammar: 0},
                         clean_parses: 0,
                         rng: rand::weak_rng()}
    }
    pub fn boxed() -> Box<Learner> {
        Box::new(TriggerLearner::new())
    }
}

mod bench {
    extern crate test;
    use self::test::Bencher;
    use learner::{TriggerLearner, Learner, Environment};
    use domain::{Colag};
    use speaker::{UniformRandomSpeaker};

    #[bench]
    fn trigger_learner_speaker(b: &mut Bencher) {
        let colag = Colag::default();
        let env = Environment { domain: colag };
        let mut speaker = UniformRandomSpeaker::new(&env.domain, 611);
        let mut learner = TriggerLearner::new();
        b.iter(|| learner.learn(&env, speaker.next().unwrap()));
    }

    #[bench]
    fn trigger_learner_constant(b: &mut Bencher) {
        let colag = Colag::default();
        let env = Environment { domain: colag };
        let ref sent = 400;
        let mut learner = TriggerLearner::new();
        b.iter(|| learner.learn(&env, sent));
    }
}
