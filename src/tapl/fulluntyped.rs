use std::fmt;

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Debug, Clone)]
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
        if idx as usize >= self.contexts.len() {
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

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            ContextTerm {
                context: &Context::new(),
                term: self,
            }
        )
    }
}

impl fmt::Display for NameBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

struct ContextTerm<'a> {
    context: &'a Context,
    term: &'a Term,
}

impl<'a> fmt::Display for ContextTerm<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.term {
            Var(x, _) => match self.context.index_to_name(x) {
                Ok(n2) => write!(f, "{}", n2),
                Err(e) => write!(f, "{:?}", e),
            },
            Abs(ref x, ref t1) => {
                let (ctx1, x1) = self.context.pick_fresh_name(x);
                write!(
                    f,
                    "(lambda {}. {})",
                    x1,
                    ContextTerm {
                        context: &ctx1,
                        term: t1,
                    }
                )
            }
            App(ref t1, ref t2) => write!(
                f,
                "({} {})",
                ContextTerm {
                    context: self.context,
                    term: t1,
                },
                ContextTerm {
                    context: self.context,
                    term: t2,
                }
            ),
        }
    }
}

mod parser {
    extern crate nom;

    use std::str;

    use nom::multispace;
    use nom::IResult;

    use super::Term;
    use super::Term::*;
    use super::Context;

    type Result = Box<Fn(Context) -> Term>;

    fn tos(s: &[u8]) -> String {
        str::from_utf8(s).unwrap().to_owned()
    }

    fn is_alphabetic(chr: u8) -> bool {
        (chr >= 0x41 && chr <= 0x5A) || (chr >= 0x61 && chr <= 0x7A)
    }

    fn is_digit(chr: u8) -> bool {
        chr >= 0x30 && chr <= 0x39
    }

    fn is_prime(chr: u8) -> bool {
        chr == 0x27
    }

    fn is_identifier(chr: u8) -> bool {
        is_alphabetic(chr) || is_digit(chr) || is_prime(chr)
    }

    named!(identifier, take_while1!(is_identifier));

    named!(term_var <&[u8], Result>,
           map!(identifier, |x| {
               let s = tos(x);
               Box::new(move |ctx: Context| -> Term {
                   match ctx.name_to_index(&s) {
                       Ok(n1) => Var(n1, ctx.contexts.len() as u32),
                       Err(e) => {
                           panic!(e)
                       }
                   }
               })
           }));

    named!(term_app <&[u8], Result>,
           do_parse!(
               char!('(')       >>
               opt!(multispace) >>
               t1: term         >>
               multispace       >>
               t2: term         >>
               opt!(multispace) >>
               char!(')')       >>
               (Box::new(move |ctx: Context| -> Term {
                    App(Box::new(Var(0, 2)), Box::new(Var(1, 2)))
               }))
           ));

    named!(term_abs <&[u8], Result>,
        do_parse!(
            char!('(')       >>
            opt!(multispace) >>
            tag!("lambda")   >>
            multispace       >>
            x: identifier    >>
            opt!(multispace) >>
            char!('.')       >>
            multispace       >>
            t1: term         >>
            opt!(multispace) >>
            char!(')')       >>
            ({
                let s = tos(x);
                Box::new(move |ctx: Context| -> Term {
                    let (c2, x2) = ctx.pick_fresh_name(&s);
                    Abs(x2.name, Box::new(t1(c2)))
                })
            })
        )
    );

    named!(term <&[u8], Result>, alt!(term_var | term_app | term_abs));

    pub fn parse(s: &[u8]) -> Option<Term> {
        match term(s) {
            IResult::Done(_, res) => Some((*res)(Context::new())),
            _ => None,
        }
    }
}

pub fn run(s: &str) -> Option<Term> {
    parser::parse(s.as_bytes())
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

    #[test]
    fn display_test() {
        let app = Abs(
            "a".to_string(),
            Box::new(Abs(
                "a".to_string(),
                Box::new(App(Box::new(Var(0, 2)), Box::new(Var(1, 2)))),
            )),
        );
        assert_eq!(app.to_string(), "(lambda a. (lambda a'. (a a')))");
    }

    #[test]
    fn parse_test() {
        use super::parser::parse;

        assert_eq!(
            parse(b"(lambda a. (lambda a'. (a a')))"),
            Some(Abs(
                "a".to_string(),
                Box::new(Abs(
                    "a'".to_string(),
                    Box::new(App(Box::new(Var(0, 2)), Box::new(Var(1, 2))))
                ))
            ))
        );
    }
}
