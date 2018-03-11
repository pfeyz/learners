#![feature(test)]

extern crate rand;
use rand::Rng;

use std::thread;
use std::sync::Arc;

use std::time::{SystemTime, Duration};

mod domain;
mod learner;
mod hypothesis;
mod sentence;

mod speaker;

use domain::{Colag, LanguageDomain, Sentence};
use learner::{Learner, Environment};
use hypothesis::{Theory};
use speaker::{UniformRandomSpeaker};

mod macrotest;

type LearnerFactory = fn() -> Box<Learner>;

fn learn_language<'a>(num_sentences: usize, env: &Environment, speaker: &mut UniformRandomSpeaker,
                      factory: LearnerFactory)
                  -> (usize, Box<Learner>) {
    let mut learner = factory();
    let mut consumed = 0;
    for (n, sent) in speaker.into_iter().take(num_sentences).enumerate() {
        learner.learn(env, sent);
        if learner.converged() {
            consumed = n as usize;
            break;
        }
    }
    (consumed, learner)
}
fn watch_learner<'a>(name: &str, id: u64, num_sentences: &u64, env: &Environment, language: &[&Sentence], factory: LearnerFactory) {
    let mut learner = factory();
    let mut rng = rand::thread_rng();
    for n in 0..*num_sentences {
        let sent = rng.choose(language).unwrap();
        learner.learn(env, sent);
        if n % (num_sentences / 100) == 0 || learner.converged() {
            match learner.theory() {
                Theory::Simple(h) => println!("{}, {}, {}, {}, ", name, id, n, h),
                Theory::Weighted(h) => println!("{}, {}, {}, {},", name, id, n, h)
            }
        }
        if learner.converged() {
            break;
        }
    }
}

fn get_learner_factory(name: &str) -> Option<LearnerFactory> {
    match name {
        "tla" => Some(learner::TriggerLearner::boxed),
        "rovl" => Some(learner::RewardOnlyVL::boxed),
        "rorvl" => Some(learner::RewardOnlyRelevantVL::boxed),
        "ndl" => Some(learner::NonDefaultsLearner::boxed),
        _ => None
    }
}



fn main() {
    let env = Arc::new(Environment { domain: Colag::default() });
    let mut handles = Vec::new();
    let start = SystemTime::now();
    for _ in 0..1 {
        let env = env.clone();
        handles.push(thread::spawn(move|| {
            let target = 611;
            let mut speaker = UniformRandomSpeaker::new(&env.domain, 611);
            for iter in 0..100 {
                // for name in vec!["ndl", "vl", "rovl", "tla"]{
                for name in vec!["rovl"]{
                    if let Some(factory) = get_learner_factory(&name) {
                        // watch_learner(name, iter, &500_000, &env, &language[..], factory);
                        let start = SystemTime::now();
                        let (n, learner) = learn_language(2_000_000, &env, &mut speaker, factory);
                        match learner.theory() {
                            Theory::Simple(h) => println!("{}, {}, {}, {}", name, n, target, h),
                            Theory::Weighted(h) => println!("{}, {}, {}, {}", name, n, target, h)
                        }
                        println!("{:?}", SystemTime::now().duration_since(start));
                    } else {
                        println!("`{}` is not a valid learner name", name);
                    }
                }
            }
        }));
    }
    for h in handles {
        h.join();
    }
    println!("{:?}", SystemTime::now().duration_since(start));
}
