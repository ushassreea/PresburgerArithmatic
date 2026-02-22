fn main() {
    let _x: For = For::Exst(Box::new(For::Eql(
        Term::A(Box::new(Term::Var(1, 0)), Box::new(Term::Var(1, 0))),
        Term::Succ(Box::new(Term::O)),
    )));
}

enum Term {
    O,
    Succ(Box<Term>),
    Var(u32, u32),
    A(Box<Term>, Box<Term>),
}

enum For {
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
    fn toFor2(self) -> For2 {
        match self {
            For::Lt(t, tn) => For2::Lt(t, tn),
            For::Eql(t, tn) => For2::Eql(t, tn),
            For::And(f, g) => For2::And(Box::new(f.toFor2()), Box::new(g.toFor2())),
            For::Or(f, g) => For2::Or(Box::new(f.toFor2()), Box::new(g.toFor2())),
            For::Not(f) => For2::Not(Box::new(f.toFor2())),
            For::Exst(f) => For2::Exst(Box::new(f.toFor2())),
        }
    }
}

impl Term {
    fn fltn(self) -> Term {
        match self {
            Term::Succ(t) => match t.fltn() {
                Term::A(tr, tn) => Term::A(Box::new((Term::Succ(tr)).fltn()), tn),
                Term::Var(i, j) => Term::A(
                    Box::new(Term::Succ(Box::new(Term::O))),
                    Box::new(Term::Var(i, j)),
                ),
                x => Term::Succ(Box::new(x)),
            },
            Term::A(t, tn) => Term::A(Box::new(t.fltn()), Box::new(tn.fltn())),
            x => x,
        }
    }

    fn csum(a: Term, b: Term) -> Term {
        match a {
            Term::Succ(a1) => Term::Succ(Box::new(Term::csum(*a1, b))),
            _ => b,
        }
    }

    fn cmbn(t1: Term, t2: Term) -> Term {
        match (t1, t2) {
            (Term::A(e1, e2), Term::A(f1, f2)) => match (e1, f1) {
                (Term::Var(i1, x1), Term::Var(i2, x2)) => {
                    if x1 < x2 {
                        Term::A(e1, Box::new(Term::cmbn(e2, t2)))
                    } else if x1 == x2 {
                        Term::A(
                            Box::new(Term::Var(i1 + i2, x1)),
                            Box::new(Term::cmbn(e2, f2)),
                        )
                    } else {
                        Term::A(f1, Box::new(Term::cmbn(t1, f2)))
                    }
                }
                (Term::Var(_, _), Term::Succ(_)) => Term::A(f1, Box::new(Term::cmbn(t1, f2))),
                (Term::Var(_, _), Term::O) => Term::A(f1, Box::new(Term::cmbn(t1, f2))),
                (Term::Succ(_), Term::Var(_, _)) => Term::A(e1, Box::new(Term::cmbn(e2, t2))),
                (Term::O, Term::Var(_, _)) => Term::A(e1, Box::new(Term::cmbn(e2, t2))),
                (a, b) => Term::A(Box::new(Term::csum(a, b)), Box::new(Term::cmbn(e2, f2))),
            },
            (Term::A(e1, e2), Term::Var(i2, x2)) => match *e1 {
                Term::Var(i1, x1) => {
                    if x1 < x2 {
                        Term::A(e1, Box::new(Term::cmbn(e2, t2)))
                    } else if x1 == x2 {
                        Term::A(Box::new(Term::Var(i1 + i2, x1)), e2)
                    } else {
                        Term::A(Box::new(t2), Box::new(t1))
                    }
                }
                _ => Term::A(e1, Box::new(Term::cmbn(*e2, t2))),
            },
            (Term::A(e1, e2), b) => match *e1 {
                Term::Var(_, _) => Term::A(Box::new(t2), Box::new(t1)),
                a => Term::A(Box::new(Term::csum(a, b)), e2),
            },

            (Term::Var(i1, x1), Term::A(f1, f2)) => match *f1 {
                Term::Var(i2, x2) => {
                    if x2 < x1 {
                        Term::A(f1, Box::new(Term::cmbn(*f2, t1)))
                    } else if x1 == x2 {
                        Term::A(Box::new(Term::Var(i1 + i2, x1)), f2)
                    } else {
                        Term::A(Box::new(t1), Box::new(t2))
                    }
                }
                _ => Term::A(f1, Box::new(Term::cmbn(*f2, t1))),
            },
            (Term::Var(i1, x1), Term::Var(i2, x2)) => {
                if x1 < x2 {
                    Term::A(Box::new(t1), Box::new(t2))
                } else if x1 == x2 {
                    Term::Var(i1 + i2, x1)
                } else {
                    Term::A(Box::new(t2), Box::new(t1))
                }
            }
            (Term::Var(_, _), _) => Term::A(Box::new(t2), Box::new(t1)),
            (a, Term::A(f1, f2)) => match *f1 {
                Term::Var(_, _) => Term::A(Box::new(t1), Box::new(t2)),
                b => Term::A(Box::new(Term::csum(a, b)), f2),
            },
            (_, Term::Var(_, _)) => Term::A(Box::new(t1), Box::new(t2)),
            (a, b) => Term::csum(a, b),
        }
    }
    fn ordr(self) -> Term {
        match self {
            Term::A(t1, t2) => Term::cmbn(t1.ordr(), t2.ordr()),
            x => x,
        }
    }
}
