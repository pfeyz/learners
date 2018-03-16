use learner::{Learner, Environment};
use hypothesis::{WeightedHypothesis, Theory};
use sentence::{SurfaceForm, Illoc};
use domain::{LanguageDomain, Sentence};


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Param {
    SP,
    HIP,
    HCP,
    OPT,
    NS,
    NT,
    WHM,
    PI,
    TM,
    VtoI,
    ItoC,
    AH,
    QInv
}

#[derive(Debug)]
enum Rate {
    Normal,
    Conservative
}

#[derive(Debug)]
enum Op {
    Update(Param, Rate, bool),
    Update2((Param, Rate, bool), (Param, Rate, bool))
}

pub struct NonDefaultsLearner {
    hypothesis: WeightedHypothesis
}

impl Learner for NonDefaultsLearner {
    fn learn(&mut self, env: &Environment, sent: &Sentence){
        let surface_form = env.domain.surface_form(sent);
        let ops = self.run_triggers(surface_form);
        // let mut params: HashSet<Param> = HashSet::new();
        for op in ops {
            // println!("{:?}", op);
            match op {
                Some(Op::Update(param, rate, direction)) => {
                    // params.insert(param.clone());
                    self.update_weights(param, rate, direction);
                },
                Some(Op::Update2((p1, r1, d1), (p2, r2, d2))) => {
                    // params.insert(p1.clone());
                    // params.insert(p2.clone());
                    self.update_weights(p1, r1, d1);
                    self.update_weights(p2, r2, d2);
                },
                None => ()
            }
        }
        // println!("{}, {:?}", sent, params);
    }
    fn theory(&self) -> Theory {
        Theory::Weighted(&self.hypothesis)
    }
}

impl NonDefaultsLearner {
    pub fn new() -> Self {
        NonDefaultsLearner { hypothesis: WeightedHypothesis::new() }
    }
    pub fn boxed() -> Box<Learner> {
        Box::new(Self::new())
    }

    fn run_triggers(&self, form: &SurfaceForm) -> Vec<Option<Op>> {
        vec![
            self.subject_position(form),
            self.head_in_cp(form),
            self.head_ip(form),
            self.null_subject(form),
            self.wh_movement(form),
            self.prep_stranding(form)
            ]
    }

    fn update_weights(&mut self, param: Param, rate: Rate, direction: bool){
        let ref mut weights = self.hypothesis.weights;
        let param = param as usize;
        let rate = match rate {
            Rate::Conservative => 0.001,
            Rate::Normal => 0.001
        };
        match direction {
            true => {
                weights[param] += rate * weights[param];
            },
            false => {
                weights[param] -= rate * weights[param];
            }
        }
    }

    fn subject_position(&self, form: &SurfaceForm) -> Option<Op> {
        use sentence::*;
        if form.illoc != Illoc::Dec {
            None
        } else if !form.topicalized(&O1) && form.order(&O1, &S){
            Some(Op::Update(Param::SP, Rate::Normal, true))
        } else if !form.topicalized(&S) && form.order(&S, &O1) {
            Some(Op::Update(Param::SP, Rate::Normal, false))
        } else {
            None
        }
    }

    fn head_ip(&self, form: &SurfaceForm) -> Option<Op> {
        use sentence::*;
        use sentence::SurfaceSymbol::*;
        if form.contains(&O3) & form.contains(&P){
            if !form.topicalized(&O3) & form.adjacent(&O3, &P) {
                return Some(Op::Update(Param::HIP, Rate::Normal, true));
            } else if !form.topicalized(&O3) & form.adjacent(&P, &O3) {
                return Some(Op::Update(Param::HIP, Rate::Normal, false));
            }
        }
        else if (form.illoc == Illoc::Imp) & form.contains(&O1) & form.contains(&Verb){
            if form.adjacent(&O1, &Verb) {
                return Some(Op::Update(Param::HIP, Rate::Normal, true));
            } else if form.adjacent(&O1, &Verb){
                return Some(Op::Update(Param::HIP, Rate::Normal, false));
            }
        }
        None
    }

    fn head_in_cp(&self, form: &SurfaceForm) -> Option<Op> {
        use sentence::*;
        use sentence::SurfaceSymbol::*;
        if form.illoc != Illoc::Q {
            None
        }
        else if form.ends_with(&Ka) || (form.ends_with(&Aux) && !form.contains(&Ka)){
            Some(Op::Update(Param::HCP, Rate::Normal, true))
        } else if form.topicalized(&Ka) || (form.topicalized(&Aux) && !form.contains(&Ka)) {
            Some(Op::Update(Param::HCP, Rate::Normal, false))
        } else {
            None
        }
    }

    fn optional_topic(&self, form: &SurfaceForm) -> Option<Op> {
        use sentence::FeatureType;
        if !form.contains_feature(&FeatureType::WA) & (self.hypothesis.weights[Param::TM as usize] > 0.5) {
            Some(Op::Update(Param::OPT, Rate::Normal, true))
        } else {
            None
        }
    }

    // TODO: the python checks for membership in sentence string, not list
    fn null_subject(&self, form: &SurfaceForm) -> Option<Op> {
        use sentence::{S};

        if (form.illoc == Illoc::Dec) & !form.contains(&S) & form.out_oblique(){
            Some(Op::Update2((Param::NS, Rate::Normal, true),
                             (Param::OPT, Rate::Normal, true)))
        } else if (form.illoc == Illoc::Dec) & form.contains(&S) & form.out_oblique() {
            Some(Op::Update(Param::NS, Rate::Conservative, false))
        } else {
            None
        }
    }

    // // todo: the python checks for membership in sentence string, not list
    fn null_topic(&self, form: &SurfaceForm) -> Option<Op> {
        use sentence::{O1, O2, O3, S, Adv};

        if (form.illoc == Illoc::Dec) && form.contains(&O2) && !form.contains(&O1) {
            Some(Op::Update2((Param::NT, Rate::Normal, true), (Param::OPT, Rate::Normal, false)))
        }
        else if (form.illoc == Illoc::Dec)
                 && form.contains(&O1)
                 && form.contains(&O2)
                 && form.contains(&O3)
                 && form.contains(&S)
                 && form.contains(&Adv) {
            Some(Op::Update(Param::NT, Rate::Conservative, false))
        } else {
            None
        }
    }

    fn wh_movement(&self, form: &SurfaceForm) -> Option<Op> {
        use sentence::FeatureType::WH;
        use sentence::FeatureVal::*;
        use sentence::SurfaceSymbol::{O3_};
        use sentence::P;
        let has_wh = form.words.iter().any(|w| w.has_feature(&WH));

        if form.illoc == Illoc::Q && has_wh {
            if form.words[0].has_feature(&WH)

                | form.starts_with(&[&P, &O3_ {wh: True, wa: Any}]) {
                    Some(Op::Update(Param::WHM, Rate::Conservative, true))
                } else {
                    Some(Op::Update(Param::WHM, Rate::Normal, false))
                }
        } else {
            None
        }
    }

    fn prep_stranding(&self, form: &SurfaceForm) -> Option<Op> {
        None
    }
}

mod bench {
    extern crate test;
    use self::test::Bencher;
    use learner::{NonDefaultsLearner, Learner, Environment};
    use domain::{Colag};
    use speaker::{UniformRandomSpeaker};

    #[bench]
    fn non_defaults_learner(b: &mut Bencher) {
        let colag = Colag::default();
        let env = Environment { domain: colag };
        let mut speaker = UniformRandomSpeaker::new(&env.domain, 611);
        let mut learner = NonDefaultsLearner::new();
        b.iter(|| learner.learn(&env, speaker.next().unwrap()));
    }
}
