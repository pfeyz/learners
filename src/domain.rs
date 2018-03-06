pub const NUM_PARAMS: usize = 13;
pub type Grammar = u16;
pub type Sentence = u32;
pub type TriggerVec = [Trigger; NUM_PARAMS];

pub trait LanguageDomain {
    fn language(&self, &Grammar) -> &Sentence;
    fn triggers(&self, &Sentence) -> &TriggerVec;
    fn parses(&self, &Grammar, &Sentence) -> Result<bool, IllegalGrammar>;
    fn random_sentence(&self, target: &Grammar) -> &Sentence;
    fn random_grammar(&self) -> Grammar;
    fn random_weighted_grammar(&self, [f64; NUM_PARAMS]) -> Grammar;
}

pub struct IllegalGrammar {
    grammar: Grammar
}

pub enum Trigger {
    On,
    Off,
    Ambiguous,
    Irrelevant
}

pub struct Colag {}
impl LanguageDomain for Colag {
    fn language(&self, g: &Grammar) -> &Sentence {
        unimplemented!();
    }
    fn triggers(&self, s: &Sentence) -> &TriggerVec {
        unimplemented!();
    }
    fn parses(&self, g: &Grammar, s: &Sentence) -> Result<bool, IllegalGrammar> {
        unimplemented!();
    }
    fn random_grammar(&self) -> Grammar {
        unimplemented!();
    }
    fn random_weighted_grammar(&self, weights: [f64; NUM_PARAMS])
                               -> Grammar {
        unimplemented!();
    }
    fn random_sentence(&self, target: &Grammar) -> &Sentence {
        unimplemented!();
    }
}

pub fn param_state(g: &Grammar, num: u8){
    unimplemented!();
}
