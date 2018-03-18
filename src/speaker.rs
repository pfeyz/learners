use rand;
use rand::{Rng, SeedableRng};
use mersenne_twister::MersenneTwister;

use domain::{Colag, LanguageDomain, Grammar, Sentence};

pub struct UniformRandomSpeaker<'a> {
    domain: &'a Colag,
    language: Grammar,
    sentences: &'a Vec<Sentence>,
    rng: MersenneTwister
}

impl<'a> UniformRandomSpeaker<'a> {
    pub fn new(domain: &'a Colag, language: Grammar) -> Self {
        let mut rng = rand::thread_rng();
        UniformRandomSpeaker {
            domain: domain,
            language: language,
            sentences: domain
                .language_vec(&language)
                .expect(&format!("Illegal grammar: {}", language)),
            rng: MersenneTwister::from_seed(rng.next_u64())
        }
    }
}

impl<'a> Iterator for UniformRandomSpeaker<'a> {
    type Item = &'a Sentence;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.rng.choose(&self.sentences).unwrap())
    }
}

mod bench {
    extern crate test;
    use self::test::Bencher;
    use rand::{Rng, thread_rng};
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
