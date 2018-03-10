#[derive(PartialOrd, Eq, Ord)]
enum FeatureVal {
    True,
    False,
    Any
}

enum FeatureType {
    WH,
    WA
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

#[derive(PartialEq, Eq, Ord, PartialOrd)]
enum SurfaceSymbol {
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

static adv: SurfaceSymbol = SurfaceSymbol::Adv { wh: FeatureVal::Any, wa: FeatureVal::Any };
static o1: SurfaceSymbol = SurfaceSymbol::O1 { wh: FeatureVal::Any, wa: FeatureVal::Any };
static o2: SurfaceSymbol = SurfaceSymbol::O2 { wh: FeatureVal::Any, wa: FeatureVal::Any };
static o3: SurfaceSymbol = SurfaceSymbol::O3 { wh: FeatureVal::Any, wa: FeatureVal::Any };
static pro: SurfaceSymbol = SurfaceSymbol::P { wa: FeatureVal::Any };
static sub: SurfaceSymbol = SurfaceSymbol::S { wh: FeatureVal::Any, wa: FeatureVal::Any };

use self::SurfaceSymbol::*;

impl SurfaceSymbol {
    fn has_feature(&self, feature: &FeatureType) -> bool {
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

            "S[+WA]" => S { wa: FeatureVal::True, wh: FeatureVal::False },
            "S[+WH]" => S { wa: FeatureVal::False, wh: FeatureVal::True },
            "S[+WH][+WA]" => S { wa: FeatureVal::True, wh: FeatureVal::True },

            "P" => P { wa: FeatureVal::False },
            "P[+WA]" => P { wa: FeatureVal::True },
            _ => panic!(format!("Illegal surface form: {}", s))
        }
    }
}

#[derive(PartialEq)]
enum Illoc {
    Dec,
    Q
}

#[derive(PartialEq)]
struct SurfaceForm {illoc: Illoc, words: Vec<SurfaceSymbol> }

impl<'a> From<&'a str> for SurfaceForm {
    fn from(s: &'a str) -> Self {
        let v: Vec<SurfaceSymbol> = s.split(" ")
            .map({|s| s.into()})
            .collect();
        SurfaceForm {illoc: Illoc::Dec, words: v}
    }
}

impl SurfaceForm {
    fn contains(&self, sym: &SurfaceSymbol) -> bool {
        return self.words.contains(sym);
    }

    fn contains_feature(&self, feat: &FeatureType) -> bool {
        for item in self.words.iter() {
            if item.has_feature(feat){
                return true;
            }
        }
        false
    }

    fn topicalized(&self, sym: SurfaceSymbol) -> bool{
        if self.words.len() == 0 {
            return false;
        } else {
            return self.words[0] == sym;
        }
    }

    fn order(&self, a: &SurfaceSymbol, b: &SurfaceSymbol) -> bool {
        match (self.words.binary_search(a), self.words.binary_search(b)){
            (Ok(x), Ok(y)) => x < y,
            _ => false
        }
    }

    fn adjacent(&self, a: &SurfaceSymbol, b: &SurfaceSymbol) -> bool {
        match (self.words.binary_search(a), self.words.binary_search(b)){
            (Ok(x), Ok(y)) => x == y - 1,
            _ => false
        }
    }

    fn out_oblique(&self) -> bool {
        if self.contains(&o1)
            & self.order(&o1, &o2) & self.order(&o2, &pro) & self.adjacent(&pro, &o3) {
                false
            }
        else if self.contains(&o3)
            & self.order(&o3, &o2) & self.order(&o2, &o1) & self.adjacent(&o3, &pro) {
                false
            }
        else if self.contains(&o1) & self.contains(&o2) & self.contains(&o3) & self.contains(&pro) {
            true
        }
        else {
            false
        }
    }
}

mod bench {
    use sentence::{SurfaceSymbol::*, SurfaceForm, FeatureVal::*, FeatureType::*};
    use sentence::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn topicalized(b: &mut Bencher){
        let x = SurfaceForm {illoc: Illoc::Dec, words: vec![Aux, O1 {wh: True, wa: False}]};
        // b.iter(|| assert!(x.contains_feature(&WH)));
        let string =  "Aux Never Never Never O2[+WH][+WA] O1[+WH]";
        let mut s: SurfaceForm = string.into();
        b.iter(|| assert!(s.contains(&o2)));
    }
}
