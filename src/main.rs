#![feature(test)]

extern crate rand;
extern crate mersenne_twister;

use std::thread;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, Duration};

use rand::Rng;

mod domain;
mod hypothesis;
mod learner;
mod sentence;
mod speaker;
mod triggers;

use domain::{Colag, LanguageDomain, Sentence, Grammar};
use learner::{Learner, Environment};
use hypothesis::{Theory};
use speaker::{UniformRandomSpeaker};
use triggers::{TriggerMap};

type LearnerFactory = fn() -> Box<Learner>;

fn learn_language<'a>(num_sentences: usize, env: &Environment, speaker: &mut UniformRandomSpeaker, learner: &mut Learner) -> usize {
    for (consumed, sent) in speaker.into_iter().take(num_sentences).enumerate() {
        learner.learn(env, sent);
        if learner.converged() {
            return consumed + 1 as usize;
        }
    }
    num_sentences
}

fn learner_report(learner: &mut learner::Learner, target: &Grammar, name: &str, consumed: usize) {
    let guess = learner.guess();
    match learner.theory() {
        Theory::Simple(h) => println!("{}, {}, {}, {}, {}, {}", learner, target, guess, name, consumed, h),
        Theory::Weighted(h) => println!("{}, {}, {}, {}, {}, {}", learner, target, guess, name, consumed, h)
    }
}

fn watch_language<'a>(name: &str, num_sentences: usize, target: u16, env: &Environment, speaker: &mut UniformRandomSpeaker, learner: &mut learner::Learner) {
    for (consumed, sent) in speaker.into_iter().take(num_sentences).enumerate() {
        learner.learn(env, sent);
        if learner.converged() || consumed == num_sentences - 1 {
            learner_report(learner, &target, name, consumed);
            break;
        }
        if consumed % 5000 != 0 {
            continue;
        }
        learner_report(learner, &target, name, consumed);
    }
}

fn to_secs(duration: Duration) -> f64 {
    duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9
}
static LANGUAGES: [u16; 1] = [611];
// static LANGUAGES: [u16; 1] = [611];

fn vl_simulation(){
    let env = Arc::new(Environment { domain: Colag::default() });
    let maps = [
        ("normal", TriggerMap::from_file("data/irrelevance-output.txt").unwrap()),
        // ("equiv", TriggerMap::from_file("data/irrelevance-output-no-equiv.txt").unwrap()),
        // ("super", TriggerMap::from_file("data/irrelevance-output-no-superset.txt").unwrap()),
    ];
    let mut handles = Vec::new();
    let maps = Arc::new(maps);
    let mut languages: Vec<Grammar> = LANGUAGES.iter().cloned() //env.domain.language.keys() .cloned()
        .flat_map(|x| vec![x; 100])
        .collect();
    let languages = Arc::new(Mutex::new(languages));
    for thread_id in 0..4 {
        let maps = maps.clone();
        let env = env.clone();
        let languages = languages.clone();
        let mut iteration = 0;
        handles.push(thread::spawn(move|| {
            // for target in env.domain.language.keys() {
            // for target in LANGUAGES.iter() {
            loop {
                let mut target = {
                    if let Some(v) = languages.lock().unwrap().pop(){
                        v
                    } else {
                        break;
                    }
                };
                let mut speaker = UniformRandomSpeaker::new(&env.domain, target);
                // let mut learner = learner::RewardOnlyVL::new();
                // watch_language(&format!("{}:{}", &thread_id.to_string(), &iteration.to_string()),
                //                10_000_000, target, &env, &mut speaker, &mut learner);
                // iteration += 1;
                for &(ref name, ref trigger_map) in maps.iter() {
                    for rate in [0., 0.1, 0.25, 0.3, 0.33, 0.4, 0.45, 0.5, 0.75, 1.0].iter().cloned() {
                        let mut learner = learner::RewardOnlyRelevantVL::new(name, &trigger_map, rate);
                        // watch_language(&format!("{}:{}", &thread_id.to_string(), &iteration.to_string()),
                        //                10_000_000, target, &env, &mut speaker, &mut learner);
                        let consumed = learn_language(10_000_000, &env, &mut speaker, &mut learner);
                        learner_report(&mut learner, &target, "", consumed);
                        iteration += 1;
                    }
                }
                // let mut learner = learner::RewardOnlyRelevantVL::new("super", &map);
                // let consumed = learn_language(5_000_000, &env, &mut speaker, &mut learner);
                // let guess = learner.guess();
                // if let Theory::Weighted(weights) = learner.theory(){
                //     println!("{}, {}, {}, {}, {}", learner, target, consumed, guess, weights);
                // }
            }
        }));

    }
    for h in handles {
        h.join();
    }
}

fn main(){
    vl_simulation();
}
