mod formula;
mod term;
mod utils;

pub use crate::{formula::For, term::Term, utils::bx};

fn main() {
    use For::*;
    use Term::*;

    let _x: For = Exst(bx(Eql(A(bx(Var(1, 0)), bx(Var(1, 0))), Succ(bx(O)))));
    //∃x,x+x=1  | ∃ x           x           +          x    =     S(0)
}
