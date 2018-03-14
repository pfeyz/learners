extern crate csv;
use std::mem;
use std::collections::{HashSet, HashMap};
use std::error::Error;

use domain::{TriggerVec, Trigger, Sentence, NUM_PARAMS};

pub struct TriggerMap(HashMap<Sentence, TriggerVec>);

impl TriggerMap {
    pub fn sentence(&self, sent: &Sentence) -> Option<&TriggerVec> {
        self.0.get(sent)
    }

    pub fn from_file(filename: &str) -> Result<Self, Box<Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b' ')
            .has_headers(false)
            .from_path(filename)
            .expect(filename);

        let mut triggers = HashMap::new();
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
                triggers.insert(sentence, array);
            }

        }
        assert!(triggers.len() == 48077,
                format!("expected 48077 sentences, saw {}",
                triggers.len()));
        Ok(TriggerMap(triggers))
    }
}
