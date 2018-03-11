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

mod bench {
    extern crate test;
    use self::test::Bencher;
    use rand::{Rng, thread_rng};
    use learner::{NonDefaultsLearner, Learner, Environment};
    use domain::{Colag, LanguageDomain, Sentence, Grammar, NUM_PARAMS};
    use speaker::{UniformRandomSpeaker};

    #[bench]
    fn speaker_iter(b: &mut Bencher) {
        let colag = Colag::default();
        let mut speaker = UniformRandomSpeaker::new(&colag, 611);
        b.iter(|| speaker.next().unwrap());
    }

    #[bench]
    fn speaker_vec(b: &mut Bencher) {
        let colag = Colag::default();
        let mut speaker = UniformRandomSpeaker::new(&colag, 611);
        let mut sentences: Vec<&Sentence> = speaker.take(20_000_000).collect();

        b.iter(|| test::black_box(sentences.pop()));
    }
}
