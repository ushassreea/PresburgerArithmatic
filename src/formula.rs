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


#[derive(Clone)]
enum For2 {
    Lt(Term, Term),
    Eql(Term, Term),
    And(Box<For2>, Box<For2>),
    Or(Box<For2>, Box<For2>),
    Not(Box<For2>),
    Exst(Box<For2>),
    Eqv(u32, Term, Term),
}
use For2::*;

impl For {
    fn to_for2(&self) -> For2 {
        match self {
            For::Lt(t, tn)  => Lt(t.clone(), tn.clone()),
            For::Eql(t, tn) => Eql(t.clone(), tn.clone()),
            For::And(f, g)  => And(bx(f.to_for2()), bx(g.to_for2())),
            For::Or(f, g)   => Or(bx(f.to_for2()), bx(g.to_for2())),
            For::Not(f)     => Not(bx(f.to_for2())),
            For::Exst(f)    => Exst(bx(f.to_for2())),
        }
    }
}

impl For2 {
    fn unknot(self) -> Self {
        match self {
            And(t1, t2) => And(bx(t1.unknot()), bx(t2.unknot())),
            Or(t1, t2)  => Or(bx(t1.unknot()), bx(t2.unknot())),
            Not(f)      => {
                match *f {
                    Lt(t1,t2)      => Or(bx(Eql(t1.clone(),t2.clone())),bx(Lt(t2,t1))),
                    Eql(t1,t2)     => Or(bx(Lt(t1.clone(),t2.clone())),bx(Lt(t2,t1))),
                    And(f1,f2)     => Or(bx(Not(f1).unknot()),bx(Not(f2).unknot())),
                    Or(f1,f2)      => And(bx(Not(f1).unknot()),bx(Not(f2).unknot())),
                    Not(f1)        => f1.unknot(),
                    Exst(_)        => panic!(),
                    Eqv(0, _, _)   => panic!(),
                    Eqv(1, _, _)   => panic!(),
                    Eqv(n, t1, t2) => (2..n).map(|n| Eqv(n, t1.clone(), Term::cmbn(t2.clone(), Term::num_to_term(n))))
                                            .fold(Eqv(n, t1.clone(), Term::cmbn(t2.clone(), Term::num_to_term(1))),
                                                 |acc, sn| Or(bx(acc), bx(sn)))
                }},
            x           => x
        }
    }
    
    fn to_dnf(self) -> Self {
        match self {
            Or(t1, t2)  => Or(bx(t1.to_dnf()), bx(t2.to_dnf())),
            And(t1, t2) => match (t1.to_dnf(), t2.to_dnf()) {
                (Or(e1, e2), Or(f1, f2)) => Or(bx(Or(bx(And(e1.clone(), f1.clone())), bx(And(e1, f2.clone())))),
                                               bx(Or(bx(And(e2.clone(), f1)), bx(And(e2, f2))))),
                (Or(e1, e2), f)          => Or(bx(And(e1, bx(f.clone()))), bx(And(e2, bx(f)))),
                (f, Or(e1, e2))          => Or(bx(And(e1, bx(f.clone()))), bx(And(e2, bx(f)))),
                (x1, x2)                 => And(bx(x1), bx(x2))
            }
            Exst(_)     => panic!(),
            Not(_)     => panic!(),
            x           => x
        }
    }
}
