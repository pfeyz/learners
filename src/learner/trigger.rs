use learner::{Learner, Consumer, Environment};
use domain::{Grammar, Sentence, IllegalGrammar};
use hypothesis::{SimpleHypothesis};

pub struct TriggerLearner {
    hypothesis: SimpleHypothesis,
}

impl Learner<SimpleHypothesis> for TriggerLearner {
    fn hypothesis(&self) -> &SimpleHypothesis { &self.hypothesis }
    fn hypothesis_mut(&mut self) -> &mut SimpleHypothesis { &mut self.hypothesis }
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

impl Consumer for TriggerLearner {
    fn consume(&mut self, env: &Environment, sent: &Sentence){
        self.learn(env, sent);
    }
}

impl TriggerLearner {
    pub fn new() -> Self {
        TriggerLearner { hypothesis: SimpleHypothesis {grammar: 0} }
    }
}
