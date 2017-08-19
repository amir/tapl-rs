pub enum Term {
    Var(u32, u32),
    Abs(String, Box<Term>),
    App(Box<Term>, Box<Term>),
}

use self::Term::*;

type OnVar = fn(u32, u32, u32) -> Term;

impl Term {
    fn term_map(&self, on_var: OnVar) -> Term {
        fn walk(d: u32, c: u32, t: &Term) -> Term {
            match *t {
                Var(x, n) => on_var(0, x, n),
                Abs(ref x, ref t1) => Abs(x.clone(), Box::new(walk(d, c + 1, t1))),
                App(ref t1, ref t2) => App(Box::new(walk(d, c, t1)), Box::new(walk(d, c, t2))),
            }
        }
        walk(0, 0, self)
    }

    fn shift(c: u32, x: u32, n: u32, d: u32) -> Term {
        if x >= c {
            Var(x + d, n + d)
        } else {
            Var(x, n + d)
        }
    }

    fn subst(&self, j: u32, c: u32, s: u32, x: u32, n: u32, t: &Term) -> Term {
        if x == j + c {
            t.term_shift(c, s)
        } else {
            Var(x, n)
        }
    }


    fn term_shift(&self, c: u32, s: u32) -> Term {
        unimplemented!();
    }

    fn term_subst(&self) -> Term {
        unimplemented!();
    }
}
