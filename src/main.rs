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
    for (consumed, sent) in speaker.into_iter().take(num_sentences).enumerate() {
        learner.learn(env, sent);
        if learner.converged() {
            return (consumed + 1 as usize, learner)
        }
    }
    (num_sentences, learner)
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

fn to_secs(duration: Duration) -> f64 {
    duration.as_secs() as f64
        + duration.subsec_nanos() as f64 * 1e-9
}

fn main() {
    let env = Arc::new(Environment { domain: Colag::default() });
    let mut handles = Vec::new();
    let start = SystemTime::now();
    for _ in 0..4 {
        let env = env.clone();
        handles.push(thread::spawn(move|| {
            for target in vec![611, 3856, 2253, 584]{
                let mut speaker = UniformRandomSpeaker::new(&env.domain, target);
                    // for name in vec!["ndl", "vl", "rovl", "tla"]{
                    for name in vec!["rorvl", "rovl"]{
                        for iter in 0..1 {
                        if let Some(factory) = get_learner_factory(&name) {
                            // watch_learner(name, iter, &500_000, &env, &language[..], factory);
                            let start = SystemTime::now();
                            let (n, learner) = learn_language(2_000_000, &env, &mut speaker, factory);
                            match learner.theory() {
                                Theory::Simple(h) => println!("{}, {}, {}, {}", name, n, target, h),
                                Theory::Weighted(h) => println!("{}, {}, {}, {}", name, n, target, h)
                            }
                            println!("{:?}", to_secs(SystemTime::now().duration_since(start).unwrap()));
                        } else {
                            println!("`{}` is not a valid learner name", name);
                        }
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
