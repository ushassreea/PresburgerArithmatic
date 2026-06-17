#![allow(dead_code)]

use crate::utils::bx;

#[derive(Clone)]
pub enum Term {
    O,
    Succ(Box<Term>),
    Var(u32, u32), // Var (Coefficient, Quantifier depth)
    A(Box<Term>, Box<Term>),
}

use Term::*;

impl Term {
    // Case 1 : S (a + b) -> S a + b
    // Case 2 : S x -> 1 + x
    fn fltn(self) -> Term {
        match self {
            Succ(t) => match t.fltn() {
                A(tr, tn) => A(bx((Succ(tr)).fltn()), tn),      // Case 1
                Var(i, j) => A(bx(Succ(bx(O))), bx(Var(i, j))), // Case 2
                x => Succ(bx(x)),
            },
            A(t, tn) => A(bx(t.fltn()), bx(tn.fltn())),
            x => x,
        }
    }

    // combining sum of 2 constants to a single constant
    fn csum(a: Term, b: Term) -> Term {
        match a {
            Succ(a1) => Succ(bx(Term::csum(*a1, b))),
            _ => b,
        }
    }

    pub fn cmbn(t1: Term, t2: Term) -> Term {
        match (t1.clone(), t2.clone()) {
            // Add and Add
            (A(e1, e2), A(f1, f2)) => match (*e1.clone(), *f1.clone()) {
                // Order variables
                (Var(i1, x1), Var(i2, x2)) => {
                    if x1 < x2 {
                        A(e1, bx(Term::cmbn(*e2, t2)))
                    } else if x1 == x2 {
                        A(bx(Var(i1 + i2, x1)), bx(Term::cmbn(*e2, *f2)))
                    } else {
                        A(f1, bx(Term::cmbn(t1, *f2)))
                    }
                }

                // Kill 0s
                (Var(_, _), O) => A(f1, bx(Term::cmbn(t1, *f2))),
                (O, Var(_, _)) => A(e1, bx(Term::cmbn(*e2, t2))),

                // Constants before variables
                (Var(_, _), Succ(_)) => A(f1, bx(Term::cmbn(t1, *f2))),
                (Succ(_), Var(_, _)) => A(e1, bx(Term::cmbn(*e2, t2))),

                // Combine Constants
                (a, b) => A(bx(Term::csum(a, b)), bx(Term::cmbn(*e2, *f2))),
            },

            // Add and Var
            (A(e1, e2), Var(i2, x2)) => match *e1 {
                Var(i1, x1) => {
                    if x1 < x2 {
                        A(e1, bx(Term::cmbn(*e2, t2)))
                    } else if x1 == x2 {
                        A(bx(Var(i1 + i2, x1)), e2)
                    } else {
                        A(bx(t2), bx(t1))
                    }
                }
                _ => A(e1, bx(Term::cmbn(*e2, t2))),
            },

            // Add and Const
            (A(e1, e2), b) => match *e1 {
                Var(_, _) => A(bx(t2), bx(t1)), // constant before vars
                a => A(bx(Term::csum(a, b)), e2), // combine constants
            },

            // Var and Sum
            (Var(i1, x1), A(f1, f2)) => match *f1 {
                Var(i2, x2) => {
                    if x2 < x1 {
                        A(f1, bx(Term::cmbn(*f2, t1)))
                    } else if x1 == x2 {
                        A(bx(Var(i1 + i2, x1)), f2)
                    } else {
                        A(bx(t1), bx(t2))
                    }
                }
                _ => A(f1, bx(Term::cmbn(*f2, t1))),
            },

            // Order the vars
            (Var(i1, x1), Var(i2, x2)) => {
                if x1 < x2 {
                    A(bx(t1), bx(t2))
                } else if x1 == x2 {
                    Var(i1 + i2, x1)
                } else {
                    A(bx(t2), bx(t1))
                }
            }

            // Var and Const
            (Var(_, _), _) => A(bx(t2), bx(t1)),

            // Const and addition
            (a, A(f1, f2)) => match *f1 {
                Var(_, _) => A(bx(t1), bx(t2)),
                b => A(bx(Term::csum(a, b)), f2),
            },

            // Const and Var
            (_, Var(_, _)) => A(bx(t1), bx(t2)),

            // Const and Const
            (a, b) => Term::csum(a, b),
        }
    }

    // This is like the split function in merge sort.
    fn ordr(self) -> Term {
        match self {
            A(t1, t2) => Term::cmbn(t1.ordr(), t2.ordr()),
            x => x,
        }
    }

    pub fn num_to_term(num : u32) -> Term {
        if num == 0 { O } else { Succ(bx(Term::num_to_term(num-1))) }
    }
}
