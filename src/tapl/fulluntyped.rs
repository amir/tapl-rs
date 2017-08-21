#[derive(Debug, PartialEq)]
pub enum Term {
    Var(u32, u32),
    Abs(String, Box<Term>),
    App(Box<Term>, Box<Term>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Binding {
    NameBind,
}

#[derive(Debug, Clone, PartialEq)]
struct NameBinding {
    name: String,
    binding: Binding,
}

#[derive(Clone)]
struct Context {
    contexts: Vec<NameBinding>,
}

#[derive(Debug)]
enum ContextError {
    UnboundIdentifier(String),
    VariableLookupFailure(u32, usize),
}

impl Context {
    fn new() -> Context {
        Context {
            contexts: Vec::new(),
        }
    }

    fn is_name_bound(&self, name: &str) -> bool {
        fn walk(slc: &[NameBinding], name: String) -> bool {
            match *slc {
                [] => false,
                [ref h, ref t..] => if h.name == name {
                    true
                } else {
                    walk(t, name)
                },
            }
        }
        walk(self.contexts.as_slice(), name.to_owned())
    }

    fn pick_fresh_name(&self, x: &str) -> (Context, NameBinding) {
        let s = x.to_owned();
        if self.is_name_bound(x) {
            self.pick_fresh_name(&(s + "'"))
        } else {
            let nb = NameBinding {
                name: s,
                binding: Binding::NameBind,
            };
            let mut newc = (*self).clone();
            newc.contexts.push(nb.clone());
            (newc, nb)
        }
    }

    fn index_to_name(&self, idx: u32) -> Result<NameBinding, ContextError> {
        if idx as usize > self.contexts.len() {
            Err(ContextError::VariableLookupFailure(
                idx,
                self.contexts.len(),
            ))
        } else {
            let a = &self.contexts[idx as usize];
            Ok(a.clone())
        }
    }

    fn name_to_index(&self, name: &str) -> Result<u32, ContextError> {
        fn walk(slc: &[NameBinding], name: String, i: u32) -> Result<u32, ContextError> {
            match *slc {
                [] => Err(ContextError::UnboundIdentifier(name)),
                [ref h, ref t..] => if h.name == name {
                    Ok(i)
                } else {
                    walk(t, name, i + 1)
                },
            }
        }

        walk(self.contexts.as_slice(), name.to_owned(), 0)
    }
}

use self::Term::*;

impl Term {
    fn term_shift(&self, d: u32) -> Term {
        fn walk(d: u32, c: u32, t: &Term) -> Term {
            match *t {
                Var(x, n) => if x >= c {
                    Var(x + d, n + d)
                } else {
                    Var(x, n + d)
                },
                Abs(ref x, ref t1) => Abs(x.clone(), Box::new(walk(d, c + 1, t1))),
                App(ref t1, ref t2) => App(Box::new(walk(d, c, t1)), Box::new(walk(d, c, t2))),
            }
        }
        walk(d, 0, self)
    }

    fn term_subst(&self, j: u32, s: &Term) -> Term {
        fn walk(j: u32, s: &Term, c: u32, t: &Term) -> Term {
            match *t {
                Var(x, n) => if x == j + c {
                    s.term_shift(c)
                } else {
                    Var(x, n)
                },
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
    fn is_name_bound_test() {
        let mut c = Context::new();
        c.contexts.push(NameBinding {
            name: "a".to_string(),
            binding: NameBind,
        });
        assert!(c.is_name_bound("a"));
        assert!(!c.is_name_bound("b"));
    }

    #[test]
    fn pick_fresh_name_test() {
        let mut c = Context::new();
        c.contexts.push(NameBinding {
            name: "a".to_string(),
            binding: NameBind,
        });
        c.contexts.push(NameBinding {
            name: "b".to_string(),
            binding: NameBind,
        });
        let (_, name) = c.pick_fresh_name("a");
        assert_eq!(
            name,
            NameBinding {
                name: "a'".to_string(),
                binding: NameBind,
            }
        );
        let (_, name) = c.pick_fresh_name("c");
        assert_eq!(
            name,
            NameBinding {
                name: "c".to_string(),
                binding: NameBind,
            }
        );
        assert_eq!(c.name_to_index("b").ok().unwrap(), 1);
        assert_eq!(
            c.index_to_name(0).ok().unwrap(),
            NameBinding {
                name: "a".to_string(),
                binding: NameBind,
            }
        );
    }
}
