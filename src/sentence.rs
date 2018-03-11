pub enum FeatureType {
    WH,
    WA
}

#[derive(Eq, Debug)]
pub enum FeatureVal {
    True,
    False,
    Any
}

impl PartialEq for FeatureVal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&FeatureVal::Any, _) => true,
            (_, &FeatureVal::Any) => true,
            (&FeatureVal::True,  &FeatureVal::True) => true,
            (&FeatureVal::False, &FeatureVal::False) => true,
            _ => false
        }
    }
}

use std::cmp::Ordering;

impl PartialOrd for FeatureVal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FeatureVal {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (&FeatureVal::Any, _) => Ordering::Equal,
            (_, &FeatureVal::Any) => Ordering::Equal,
            (&FeatureVal::True,  &FeatureVal::True) => Ordering::Equal,
            (&FeatureVal::False, &FeatureVal::False) => Ordering::Equal,
            (&FeatureVal::True,  &FeatureVal::False) => Ordering::Greater,
            (&FeatureVal::False,  &FeatureVal::True) => Ordering::Less
        }
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum SurfaceSymbol {
    Adv { wa: FeatureVal, wh: FeatureVal },
    Aux,
    Never,
    Not,
    O1 { wa: FeatureVal, wh: FeatureVal },
    O2 { wa: FeatureVal, wh: FeatureVal },
    O3 { wa: FeatureVal, wh: FeatureVal },
    P { wa: FeatureVal},
    S  { wa: FeatureVal, wh: FeatureVal },
    Verb,
    Ka
}

struct FeatureSetting2 {
    wa: FeatureVal,
    wh: FeatureVal
}

enum SS {
    Any(FeatureSetting2),
    O1(FeatureSetting2),
    Adv(FeatureSetting2)
}

pub static adv: SurfaceSymbol = SurfaceSymbol::Adv { wh: FeatureVal::Any, wa: FeatureVal::Any };
pub static o1: SurfaceSymbol = SurfaceSymbol::O1 { wh: FeatureVal::Any, wa: FeatureVal::Any };
pub static o2: SurfaceSymbol = SurfaceSymbol::O2 { wh: FeatureVal::Any, wa: FeatureVal::Any };
pub static o3: SurfaceSymbol = SurfaceSymbol::O3 { wh: FeatureVal::Any, wa: FeatureVal::Any };
pub static pro: SurfaceSymbol = SurfaceSymbol::P { wa: FeatureVal::Any };
pub static sub: SurfaceSymbol = SurfaceSymbol::S { wh: FeatureVal::Any, wa: FeatureVal::Any };

use self::SurfaceSymbol::*;

impl SurfaceSymbol {
    pub fn has_feature(&self, feature: &FeatureType) -> bool {
        match self {
            &Adv  { ref wa, ref wh}
            | &O1 { ref wa, ref wh}
            | &O2 { ref wa, ref wh}
            | &O3 { ref wa, ref wh}
            | &S  { ref wa, ref wh} => match feature {
                &FeatureType::WH => wh == &FeatureVal::True,
                &FeatureType::WA => wa == &FeatureVal::True
            },
            &P { ref wa } => match feature {
                &FeatureType::WA => wa == &FeatureVal::True,
                _ => false
            }
            _ => false
        }
    }
}

impl<'a> From<&'a str> for SurfaceSymbol {
    fn from(s: &'a str) -> SurfaceSymbol {
        match s {
            "Aux" => Aux,
            "Never" => Never,
            "Not" => Not,
            "Verb" => Verb,
            "ka" => Ka,

            "Adv" => Adv { wa: FeatureVal::False, wh: FeatureVal::False },
            "Adv[+WA]" => Adv { wa: FeatureVal::True, wh: FeatureVal::False },
            "Adv[+WH]" => Adv { wa: FeatureVal::False, wh: FeatureVal::True },
            "Adv[+WH][+WA]" => Adv { wa: FeatureVal::True, wh: FeatureVal::True },

            "O1" => O1 { wa: FeatureVal::False, wh: FeatureVal::False },
            "O1[+WA]" => O1 { wa: FeatureVal::True, wh: FeatureVal::False },
            "O1[+WH]" => O1 { wa: FeatureVal::False, wh: FeatureVal::True },
            "O1[+WH][+WA]" => O1 { wa: FeatureVal::True, wh: FeatureVal::True },

            "O2" => O2 { wa: FeatureVal::False, wh: FeatureVal::False },
            "O2[+WA]" => O2 { wa: FeatureVal::True, wh: FeatureVal::False },
            "O2[+WH]" => O2 { wa: FeatureVal::False, wh: FeatureVal::True },
            "O2[+WH][+WA]" => O2 { wa: FeatureVal::True, wh: FeatureVal::True },

            "O3" => O3 { wa: FeatureVal::False, wh: FeatureVal::False },
            "O3[+WA]" => O3 { wa: FeatureVal::True, wh: FeatureVal::False },
            "O3[+WH]" => O3 { wa: FeatureVal::False, wh: FeatureVal::True },
            "O3[+WH][+WA]" => O3 { wa: FeatureVal::True, wh: FeatureVal::True },

            "S" => S { wa: FeatureVal::False, wh: FeatureVal::False },
            "S[+WA]" => S { wa: FeatureVal::True, wh: FeatureVal::False },
            "S[+WH]" => S { wa: FeatureVal::False, wh: FeatureVal::True },
            "S[+WH][+WA]" => S { wa: FeatureVal::True, wh: FeatureVal::True },

            "P" => P { wa: FeatureVal::False },
            "P[+WA]" => P { wa: FeatureVal::True },
            _ => panic!(format!("Illegal surface form: {}", s))
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Illoc {
    Dec,
    Q,
    Imp
}

impl<'a> From <&'a str> for Illoc {
    fn from(s: &'a str) -> Illoc {
        match s {
            "Q" => Illoc::Q,
            "DEC" => Illoc::Dec,
            "IMP" => Illoc::Imp,
            _ => panic!(format!("Illegal illoc: {}", s))
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct SurfaceForm {pub illoc: Illoc, pub words: Vec<SurfaceSymbol> }

impl<'a> From<&'a str> for SurfaceForm {
    fn from(s: &'a str) -> Self {
        let v: Vec<SurfaceSymbol> = s.split(" ")
            .map({|s| s.into()})
            .collect();
        SurfaceForm {illoc: Illoc::Dec, words: v}
    }
}

impl SurfaceForm {
    pub fn contains(&self, sym: &SurfaceSymbol) -> bool {
        return self.words.contains(sym);
    }

    pub fn contains_feature(&self, feat: &FeatureType) -> bool {
        for item in self.words.iter() {
            if item.has_feature(feat){
                return true;
            }
        }
        false
    }

    pub fn topicalized(&self, sym: &SurfaceSymbol) -> bool{
        if self.words.len() == 0 {
            return false;
        } else {
            return self.words[0] == *sym;
        }
    }

    pub fn ends_with(&self, sym: &SurfaceSymbol) -> bool{
        if self.words.len() == 0 {
            return false;
        } else {
            return self.words[self.words.len()-1] == *sym;
        }
    }

    fn index(&self, item: &SurfaceSymbol) -> Option<usize> {
        for (n, word) in self.words.iter().enumerate() {
            if word == item {
                return Some(n)
            }
        }
        None
    }

    pub fn order(&self, a: &SurfaceSymbol, b: &SurfaceSymbol) -> bool {
        match (self.index(a), self.index(b)){
            (Some(x), Some(y)) => x < y,
            _ => false
        }
    }

    pub fn adjacent(&self, a: &SurfaceSymbol, b: &SurfaceSymbol) -> bool {
        match (self.index(a), self.index(b)){
            (Some(x), Some(y)) => x == y - 1,
            _ => false
        }
    }

    pub fn starts_with(&self, words: &[&SurfaceSymbol]) -> bool{
        let ws: Vec<&SurfaceSymbol> = self.words.iter().collect();
        ws.starts_with(words)
    }

    // pub fn wh_topicalized(&self) -> bool {
    //     match self self.words[0] {
    //         Any { ref wh, ..},
    //         | Adv { ref wh, ..},
    //         | O1 { ref wh, ..},
    //         | O2 { ref wh, ..},
    //         | O3 { ref wh, ..},
    //         | S  { ref wh, ..} => wh == FeatureVal::True,
    //         _ => false
    //     }
    // }

    pub fn out_oblique(&self) -> bool {
        if let Some(o1_index) = self.index(&o1){
            if let Some(o2_index) = self.index(&o2){
                if let Some(o3_index) = self.index(&o3){
                    if let Some(pro_index) = self.index(&pro){
                        if o1_index < o2_index && o2_index < pro_index && pro_index == o3_index - 1 {
                            return false
                        } else if o3_index < o2_index && o2_index < o1_index && o3_index == pro_index - 1 {
                            return false
                        } else {
                            return true
                        }
                    }
                }
            }
        }
        return false
    }
        // let o1_index = self.index(&o1);
        // let o2_index = self.index(&o2);
        // let o3_index = self.index(&o3);
        // let pro_index = self.index(&pro);
        // match (o1_index, o2_index, o3_index, pro_index){
        //     (Some(o1_index), Some(o2_index), Some(o3_index), Some(pro_index)) => {
        //         if o1_index < o2_index && o2_index < pro_index && pro_index == o3_index - 1 {
        //             false
        //         } else if o3_index < o2_index && o2_index < o1_index && o3_index == pro_index - 1 {
        //             false
        //         } else {
        //             true
        //         }
        //     },
        //     _ => return false
        // }
    // }
}

mod bench {
    extern crate test;
    use self::test::Bencher;

    use sentence::{SurfaceSymbol::*, SurfaceForm, FeatureVal::*, FeatureType::*};
    use sentence::*;


    #[bench]
    fn topicalized(b: &mut Bencher){
        let x = SurfaceForm {illoc: Illoc::Dec, words: vec![Aux, O1 {wh: True, wa: False}]};
        // b.iter(|| assert!(x.contains_feature(&WH)));
        let string =  "Aux Never Never Never O2[+WH][+WA] O1[+WH]";
        let mut s: SurfaceForm = string.into();
        b.iter(|| assert!(s.contains(&o2)));
    }

    #[bench]
    fn out_oblique_fail(b: &mut Bencher){
        let x = SurfaceForm {illoc: Illoc::Dec, words: vec![Aux, O1 {wh: True, wa: False}]};
        // b.iter(|| assert!(x.contains_feature(&WH)));
        let string =  "Aux Never Never Never O2[+WH][+WA] O1[+WH]";
        let mut s: SurfaceForm = string.into();
        b.iter(|| assert!(!s.out_oblique()))
    }
    #[bench]
    fn out_oblique_succeed(b: &mut Bencher){
        let x = SurfaceForm {illoc: Illoc::Dec, words: vec![Aux, O1 {wh: True, wa: False}]};
        // b.iter(|| assert!(x.contains_feature(&WH)));
        let string =  "P O2[+WH][+WA] O3 O1[+WH]";
        let mut s: SurfaceForm = string.into();
        b.iter(|| assert!(s.out_oblique()))
    }
}
