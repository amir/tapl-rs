#[derive(Debug, PartialEq)]
pub enum Term {
    Var(u32, u32),
    Abs(String, Box<Term>),
    App(Box<Term>, Box<Term>),
}

pub enum Binding {
    NameBind,
}

struct NameBinding {
    name: String,
    binding: Binding,
}

struct Context {
    contexts: Vec<NameBinding>,
}

impl Context {
    fn new() -> Context {
        Context { contexts: Vec::new() }
    }

    fn is_name_bound(&self, name: String) -> bool {
        fn walk(slc: &[NameBinding], name: String) -> bool {
            match *slc {
                [] => false,
                [ref h, ref t..] => if h.name == name { true } else { walk(t, name) },
            }
        }
        walk(self.contexts.as_slice(), name)
    }
}

use self::Term::*;

impl Term {
    fn term_shift(&self, d: u32) -> Term {
        fn walk(d: u32, c: u32, t: &Term) -> Term {
            match *t {
                Var(x, n) => {
                    if x >= c {
                        Var(x + d, n + d)
                    } else {
                        Var(x, n + d)
                    }
                }
                Abs(ref x, ref t1) => Abs(x.clone(), Box::new(walk(d, c + 1, t1))),
                App(ref t1, ref t2) => App(Box::new(walk(d, c, t1)), Box::new(walk(d, c, t2))),
            }
        }
        walk(d, 0, self)
    }

    fn term_subst(&self, j: u32, s: &Term) -> Term {
        fn walk(j: u32, s: &Term, c: u32, t: &Term) -> Term {
            match *t {
                Var(x, n) => {
                    if x == j + c {
                        s.term_shift(c)
                    } else {
                        Var(x, n)
                    }
                }
                Abs(ref x, ref t1) => Abs(x.clone(), Box::new(walk(j, s, c + 1, t1))),
                App(ref t1, ref t2) => {
                    App(Box::new(walk(j, s, c, t1)), Box::new(walk(j, s, c, t2)))
                }
            }
        }
        walk(j, s, 0, self)
    }
}

#[cfg(test)]
mod tests {
    use super::Term::*;
    use super::Context;
    use super::Binding::*;
    use super::NameBinding;

    #[test]
    fn term_shift_test() {
        assert_eq!(
            Abs("a".to_string(), Box::new(Var(0, 1))).term_shift(1),
            Abs("a".to_string(), Box::new(Var(0, 2)))
        );
    }

    #[test]
    fn term_subst_test() {
        assert_eq!(
            Var(0, 1).term_subst(1, &Abs("a".to_string(), Box::new(Var(0, 1)))),
            Var(0, 1)
        );
    }

    #[test]
    fn context_test() {
        let mut c = Context::new();
        c.contexts.push(NameBinding {
            name: "a".to_string(),
            binding: NameBind,
        });
        assert!(c.is_name_bound("a".to_string()));
    }
}
