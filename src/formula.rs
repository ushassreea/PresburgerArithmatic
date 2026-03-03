#![allow(dead_code)]

use crate::term::Term;
use crate::utils::bx;

pub enum For {
    Lt(Term, Term),
    Eql(Term, Term),
    And(Box<For>, Box<For>),
    Or(Box<For>, Box<For>),
    Not(Box<For>),
    Exst(Box<For>),
}

enum For2 {
    Lt(Term, Term),
    Eql(Term, Term),
    And(Box<For2>, Box<For2>),
    Or(Box<For2>, Box<For2>),
    Not(Box<For2>),
    Exst(Box<For2>),
    Eqv(u32, Term, Term),
}

impl For {
    fn to_for2(&self) -> For2 {
        match self {
            For::Lt(t, tn) => For2::Lt(t.clone(), tn.clone()),
            For::Eql(t, tn) => For2::Eql(t.clone(), tn.clone()),
            For::And(f, g) => For2::And(bx(f.to_for2()), bx(g.to_for2())),
            For::Or(f, g) => For2::Or(bx(f.to_for2()), bx(g.to_for2())),
            For::Not(f) => For2::Not(bx(f.to_for2())),
            For::Exst(f) => For2::Exst(bx(f.to_for2())),
        }
    }
}
