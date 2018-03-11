use rand;
use rand::{Rng, ThreadRng};
use rand::distributions::{Range, Sample};

use domain::{Colag, LanguageDomain, Grammar, Sentence};

pub struct UniformRandomSpeaker<'a> {
    domain: &'a Colag,
    language: Grammar,
    sentences: Vec<&'a Sentence>,
    rng: ThreadRng
}

impl<'a> UniformRandomSpeaker<'a> {
    pub fn new(domain: &'a Colag, language: Grammar) -> Self {
        UniformRandomSpeaker {
            domain: domain,
            language: language,
            sentences: domain
                .language(&language)
                .expect(&format!("Illegal grammar: {}", language))
                .iter()
                .collect(),
            rng: rand::thread_rng()
        }
    }
}

impl<'a> Iterator for UniformRandomSpeaker<'a> {
    type Item = &'a Sentence;
    fn next(&mut self) -> Option<Self::Item> {
        Some(*self.rng.choose(&self.sentences).unwrap())
    }
}
