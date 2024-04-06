use crate::peg::*;
use egg::{rewrite as rw, Rewrite};

pub type RewriteSystem = [Rewrite<Peg, PegAnalysis>];

#[allow(unused_parens)]
pub fn rw_rules() -> Box<RewriteSystem> {
    let rules = [
        // Arithmetic
        rw!("commute-add";   "(+ ?a ?b)"         => "(+ ?b ?a)"),
        rw!("commute-mul";   "(* ?a ?b)"         => "(* ?b ?a)"),
        rw!("associate-add"; "(+ ?a (+ ?b ?c))"  => "(+ (+ ?a ?b) ?c)"),
        rw!("associate-mul"; "(* ?a (* ?b ?c))"  => "(* (* ?a ?b) ?c)"),
        rw!("add-ident";     "(+ ?a 0)"          => "?a" if is_not_const("?a")),
        rw!("mul-bot";       "(* ?a 0)"          => "0" if is_not_const("?a")),
        rw!("mul-bot-long";  "(* ?a 0l)"         => "0l" if is_not_const("?a")),
        rw!("mul-ident";     "(* ?a 1)"          => "?a" if is_not_const("?a")),
        rw!("neg-zero";      "(--- 0)"           => "0"),
        rw!("add-inv";       "(+ ?a (--- ?a))"   => "0" if is_not_const("?a")),
        rw!("sub-to-add";    "(- ?a ?b)"         => "(+ ?a (--- ?b))"),
        //Division
        rw!("commute-div"; "(/ ?a ?b)" => "(/ ?a ?b)"),
        rw!("div-ident"; "(/ ?a 1)" => "?a"),
        rw!("div-by-self"; "(/ ?a ?a)" => "1"),
        rw!("div-zero"; "(/ ?a 0)" => "error"),
        rw!("mul-div-cancel"; "(* ?a (/ 1 ?a))" => "1"),
        rw!("associate-div"; "(/ (/ ?a ?b) ?c)" => "(/ ?a (* ?b ?c))"), 
        
        //Remainder
        rw!("rem-zero-divisor"; "(% ?a 1)" => "0"),
        rw!("rem-same-num"; "(% ?a ?a)" => "0"),
        rw!("rem-zero-numerator"; "(% 0 ?a)" => "0" if is_not_const("?a")),
        rw!("rem-additive"; "(% (+ ?a ?n) ?n)" => "(% ?a ?n)"),
        rw!("rem-subtractive"; "(% (- ?a ?n) ?n)" => "(% ?a ?n)"),
        rw!("rem-multiplicative"; "(% (* ?k ?a) ?a)" => "0"),
        rw!("rem-distrib-add"; "(% (+ ?a ?b) ?n)" => "(% (+ (% ?a ?n) (% ?b ?n)) ?n)"),
        rw!("rem-distrib-sub"; "(% (- ?a ?b) ?n)" => "(% (+ (- (% ?a ?n) (% ?b ?n)) ?n) ?n)"),
        rw!("rem-negation"; "(% (--- ?a) ?n)" => "(% (+ ?n (--- (% ?a ?n))) ?n)"),


        // Ordering
        rw!("lt-comp";      "(< ?a ?b)"     => "(! (>= ?a ?b))"),
        rw!("gt-comp";      "(> ?a ?b)"     => "(! (<= ?a ?b))"),
        rw!("lte-comp";     "(<= ?a ?b)"    => "(! (> ?a ?b))"),
        rw!("gte-comp";     "(>= ?a ?b)"    => "(! (< ?a ?b))"),
        rw!("gte-split";    "(>= ?a ?b)"    => "(|| (> ?a ?b) (== ?a ?b))"),
        rw!("lte-split";    "(<= ?a ?b)"    => "(|| (< ?a ?b) (== ?a ?b))"),
        
    ];
    Box::new(rules)
}
