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

use domain::{Colag, LanguageDomain, Sentence};
use learner::{Learner, Environment};
use hypothesis::{Theory};

mod macrotest;

type LearnerFactory = fn() -> Box<Learner>;

fn learn_language<'a>(num_sentences: &u64, env: &Environment, language: &[&Sentence],
                      factory: LearnerFactory)
                  -> (u64, Box<Learner>) {
    let mut learner = factory();
    let mut rng = rand::thread_rng();
    let mut consumed = *num_sentences;
    for n in 0..*num_sentences {
        let sent = rng.choose(language).unwrap();
        learner.learn(env, sent);
        if learner.converged() {
            consumed = n;
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
    let colag = Colag::from_file("./data/COLAG_2011_ids.txt")
        .unwrap()
        .read_triggers("./data/irrelevance-output.txt")
        .unwrap()
        .read_surface_forms("./data/COLAG_2011_sents.txt")
        .unwrap();

    let env = Arc::new(Environment { domain: colag });
    let mut handles = Vec::new();
    for _ in 0..1 {
        let env = env.clone();
        handles.push(thread::spawn(move|| {
            let target = 611;
            let language: Vec<&Sentence> = env
                .domain
                .language(&target)
                .expect(&format!("Illegal grammar: {}", target))
                .iter()
                .collect();
            for iter in 0..33 {
                // for name in vec!["ndl", "vl", "rovl", "tla"]{
                for name in vec!["ndl"]{
                    if let Some(factory) = get_learner_factory(&name) {
                        // watch_learner(name, iter, &500_000, &env, &language[..], factory);
                        let start = SystemTime::now();
                        let (n, learner) = learn_language(&500_000, &env, &language[..], factory);
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
    let start = SystemTime::now();
    for h in handles {
        h.join();
    }
    println!("{:?}", SystemTime::now().duration_since(start));
}
