use learner::{Learner, Environment};
use domain::{Grammar, Sentence, IllegalGrammar, LanguageDomain};
use hypothesis::{SimpleHypothesis};

pub struct TriggerLearner {
    hypothesis: SimpleHypothesis,
}

impl Learner for TriggerLearner {
    fn learn(&mut self, env: &Environment, sent: &Sentence){
        let parses = env.domain.parses(&self.hypothesis.grammar, sent);
        match parses {
            Ok(false) | Err(IllegalGrammar {..}) => {
                // TODO should this only be guessing legal grammars?
                self.hypothesis.grammar = env.domain.random_grammar();
            },
            _ => ()  // our hypothesis worked, let's keep it.
        }
    }
}

impl TriggerLearner {
    pub fn new() -> Self {
        TriggerLearner { hypothesis: SimpleHypothesis {grammar: 0} }
    }
    pub fn boxed() -> Box<Learner> {
        Box::new(TriggerLearner { hypothesis: SimpleHypothesis {grammar: 0} })
    }
}
