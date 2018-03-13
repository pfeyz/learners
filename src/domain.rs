extern crate csv;
extern crate rand;
use std::mem;
use rand::{XorShiftRng, Rng};
use rand::distributions::{Range, Sample};

use std::error::Error;
use std::collections::{HashSet, HashMap};

use sentence::{SurfaceForm, Illoc};

pub const NUM_PARAMS: usize = 13;
pub type Grammar = u16;
pub type Sentence = u32;
pub type TriggerVec = [Trigger; NUM_PARAMS];


pub trait LanguageDomain {
    fn language(&self, g: &Grammar) -> Result<&HashSet<Sentence>, IllegalGrammar>;
    fn surface_form(&self, g: &Sentence) -> &SurfaceForm;
    fn triggers(&self, &Sentence) -> &TriggerVec;
    fn parses(&self, &Grammar, &Sentence) -> Result<bool, IllegalGrammar>;
    fn random_grammar(&self, rng: &mut XorShiftRng) -> &Grammar;
}

#[derive(Debug)]
pub struct IllegalGrammar {
    grammar: Grammar
}

pub enum Trigger {
    On,
    Off,
    Ambiguous,
    Irrelevant
}

type ColagTsvLine = (u16, u32, u32);

pub struct Colag {
    language: HashMap<Grammar, HashSet<u32>>,
    grammars: Vec<Grammar>,
    trigger: HashMap<Sentence, TriggerVec>,
    surface_form: HashMap<Sentence, SurfaceForm>
}

impl LanguageDomain for Colag {
    fn language(&self, g: &Grammar) -> Result<&HashSet<Sentence>, IllegalGrammar> {
        self.language.get(g).ok_or_else({|| IllegalGrammar {grammar: *g } })
    }
    fn triggers(&self, s: &Sentence) -> &TriggerVec {
        self.trigger.get(s).unwrap()
    }
    fn parses(&self, g: &Grammar, s: &Sentence) -> Result<bool, IllegalGrammar> {
        match self.language.get(g) {
            None => Err(IllegalGrammar{ grammar: *g }),
            Some(sents) => Ok(sents.contains(s))
        }
    }
    fn surface_form(&self, s: &Sentence) -> &SurfaceForm {
        self.surface_form.get(s).unwrap()
    }
    fn random_grammar(&self, rng: &mut XorShiftRng) -> &Grammar {
        rng.choose(&self.grammars).unwrap()
    }
}

impl Colag {
    pub fn new() -> Colag {
        let lang = HashMap::new();
        Colag { language: lang,
                grammars: Vec::new(),
                trigger: HashMap::new(),
                surface_form: HashMap::new()
        }
    }

    pub fn default() -> Colag {
        Colag::from_file("./data/COLAG_2011_ids.txt")
            .unwrap()
            .read_triggers("./data/irrelevance-output.txt")
            .unwrap()
            .read_surface_forms("./data/COLAG_2011_sents.txt")
            .unwrap()
    }

    pub fn random_weighted_grammar(rng: &mut XorShiftRng,
                                   weights: &[f64; NUM_PARAMS]) -> Grammar {
        let mut grammar = 0;
        for param in 0..NUM_PARAMS {
            if weighted_coin_flip(rng, weights[param]) {
                grammar = set_param(grammar, param);
            }
        }
        grammar
    }

    pub fn from_file(filename: &str) -> Result<Colag, Box<Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .from_path(filename)
            .expect(filename);
        let mut domain = Colag::new();

        for result in rdr.deserialize() {
            let (grammar, sentence, _tree): ColagTsvLine = result?;
            if domain.language.contains_key(&grammar){
                domain.language.get_mut(&grammar).map(|set| set.insert(sentence));
            } else {
                let mut set = HashSet::new();
                set.insert(sentence);
                domain.language.insert(grammar, set);
            }
        }
        domain.grammars = domain.language.keys().map(|x| *x).collect();
        assert!(domain.language.len() == 3072, "Expected 3072 languages in Colag");
        {
            let english = domain.language.get(&611).unwrap();
            assert!(english.len() == 360, "Expected 360 sentences in Colag English");
            for s in vec![3138, 1970, 5871, 6923, 1969].iter() {
                assert!(english.contains(&s), format!("Expected sentence {} in Colag English", &s))
            }
        }
        Ok(domain)
    }

    pub fn read_triggers(mut self, filename: &str) -> Result<Self, Box<Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b' ')
            .from_path(filename)
            .expect(filename);

        for result in rdr.deserialize() {
            let (sentence, trigger_str): (Sentence, String) = result?;
            assert!(trigger_str.len() == NUM_PARAMS);
            let mut trigger_vec: Vec<Trigger> = trigger_str
                .as_bytes()
                .iter()
                .map(|b| {
                    match *b {
                        b'0' => Trigger::Off,
                        b'1' => Trigger::On,
                        b'*' => Trigger::Ambiguous,
                        b'~' => Trigger::Irrelevant,
                        _ => panic!("illegal char in irrel str")
                    }
                }).collect();

            unsafe {
                let mut array: TriggerVec = mem::uninitialized();
                for i in 0..NUM_PARAMS {
                    array[i] = trigger_vec.remove(0);
                }
                self.trigger.insert(sentence, array);
            }
        }
        Ok(self)
    }

    fn sentence_generators(&self) -> HashMap<&Sentence, &Grammar> {
        unimplemented!();
    }

    fn all_sentences(&self) -> HashSet<Sentence> {
        let mut all_sents: HashSet<Sentence> = HashSet::new();
        for sents in self.language.values() {
            all_sents.extend(sents);
        }
        all_sents
    }

    fn unambiguous_trigger(&self, sent: &Sentence, param: usize) -> Result<bool, Vec<Grammar>> {
        unimplemented!();
    }

    fn illegal_grammar(&self, g: &Grammar) -> bool {
        unimplemented!();
    }

    fn ambig_or_irrel(&self, generators: Vec<Grammar>, param: usize) -> Trigger {
        for generator in generators.iter() {
            let min_pair = toggled(&generator, param);
            if !generators.contains(&&min_pair) && !self.illegal_grammar(&&min_pair){
                return Trigger::Ambiguous
            }
        }
        Trigger::Irrelevant
    }

    pub fn gen_triggers(&mut self) {
        let sentences = self.all_sentences().into_iter();
        for sentence in sentences {
            let mut triggers = unsafe {
                let mut triggers: TriggerVec = mem::uninitialized();
                for param in 0..NUM_PARAMS {
                    triggers[param] = match self.unambiguous_trigger(&sentence, param) {
                        Ok(true) => Trigger::On,
                        Ok(false) => Trigger::Off,
                        Err(generators) => self.ambig_or_irrel(generators, param)
                    };
                }
                triggers
            };
            self.trigger.insert(sentence, triggers);
        }
    }

    pub fn read_surface_forms(mut self, filename: &str) -> Result<Self, Box<Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .from_path(filename)
            .expect(filename);

        for result in rdr.deserialize() {
            let (sentence, illoc, form): (Sentence, String, String) = result?;
            let illoc: Illoc = illoc.trim().into();
            let mut form: SurfaceForm = form.trim().into();
            form.illoc = illoc;
            self.surface_form.insert(sentence, form);
        }
        Ok(self)
    }
}

fn toggled(grammar: &Grammar, param_num: usize) -> Grammar {
    unimplemented!();
}

/// Returns parameter # `param_num` from `grammar`.
pub fn get_param(grammar: &Grammar, param_num: usize) -> Grammar {
    (grammar >> (NUM_PARAMS - param_num - 1)) & 1
}

/// Returns `grammar` with `param_num` turned on.
fn set_param(grammar: Grammar, param_num: usize) -> Grammar {
    grammar + (1 << (NUM_PARAMS - param_num - 1))
}

/// Returns true `weight` percent of the time
fn weighted_coin_flip(rng: &mut XorShiftRng, weight: f64) -> bool {
    debug_assert!((weight >= 0.) & (weight <= 1.));
    let mut range = Range::new(0., 1.);
    range.sample(rng) < weight
}
mod bench {
    extern crate test;
    use self::test::Bencher;
    use rand;
    use rand::{Rng, thread_rng};
    use learner::{NonDefaultsLearner, Learner, Environment};
    use domain::{Colag, LanguageDomain, Sentence, Grammar, NUM_PARAMS};
    use speaker::{UniformRandomSpeaker};

    #[bench]
    fn random_grammar(b: &mut Bencher) {
        let colag = Colag::default();
        let ref mut rng = rand::weak_rng();
        b.iter(|| colag.random_grammar(rng));
    }


    #[bench]
    fn random_weighted_grammar(b: &mut Bencher) {
        let colag = Colag::default();
        let ref mut rng = rand::weak_rng();
        let ref weights = [0.5; NUM_PARAMS];
        b.iter(|| Colag::random_weighted_grammar(rng, weights));
    }

}
