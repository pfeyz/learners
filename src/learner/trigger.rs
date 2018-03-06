use learner::{Learner, Environment};
use domain::{Sentence, IllegalGrammar, LanguageDomain};
use hypothesis::{SimpleHypothesis, Theory};

pub struct TriggerLearner {
    hypothesis: SimpleHypothesis,
    last_change: u32
}

impl Learner for TriggerLearner {
    fn learn(&mut self, env: &Environment, sent: &Sentence){
        let parses = env.domain.parses(&self.hypothesis.grammar, sent);
        match parses {
            Ok(false) | Err(IllegalGrammar {..}) => {
                // TODO should this only be guessing legal grammars?
                self.hypothesis.grammar = *env.domain.random_grammar();
                self.last_change = 0;
            },
            _ => {
                self.last_change += 1;
                // our hypothesis worked, let's keep it.
            }
        }
    }
    fn theory(&self) -> Theory {
        Theory::Simple(&self.hypothesis)
    }
    fn converged(&mut self) -> bool {
        self.last_change > 100
    }
}

impl TriggerLearner {
    pub fn new() -> Self {
        TriggerLearner { hypothesis: SimpleHypothesis {grammar: 0},
                         last_change: 0}
    }
    pub fn boxed() -> Box<Learner> {
        Box::new(TriggerLearner::new())
    }
}
