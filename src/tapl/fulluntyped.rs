use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    True,
    False,
    Var(u32, u32),
    Abs(String, Box<Term>),
    App(Box<Term>, Box<Term>),
    Let(String, Box<Term>, Box<Term>),
    If(Box<Term>, Box<Term>, Box<Term>),
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

#[derive(Debug, Default, Clone)]
pub struct Context {
    contexts: Vec<NameBinding>,
}

#[derive(Debug, PartialEq)]
pub enum ContextError {
    UnboundIdentifier(String),
    VariableLookupFailure(u32, usize),
}

impl Context {
    pub fn new() -> Self {
        Context { contexts: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.contexts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.contexts.is_empty()
    }

    fn is_name_bound(&self, name: &str) -> bool {
        fn walk(slc: &[NameBinding], name: String) -> bool {
            match *slc {
                [] => false,
                [ref h, ref t..] => if h.name == name { true } else { walk(t, name) },
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
            newc.contexts.insert(0, nb.clone());
            (newc, nb)
        }
    }

    fn add_binding(&self, x: &str) -> (Context, NameBinding) {
        let s = x.to_owned();
        let nb = NameBinding {
            name: s,
            binding: Binding::NameBind,
        };
        let mut newc = (*self).clone();
        newc.contexts.insert(0, nb.clone());
        (newc, nb)
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
                [ref h, ref t..] => {
                    if h.name == name {
                        Ok(i)
                    } else {
                        walk(t, name, i + 1)
                    }
                }
            }
        }

        walk(self.contexts.as_slice(), name.to_owned(), 0)
    }
}

use self::Term::*;

enum EvalError {
    NoRuleApplies(Term),
}

#[derive(Debug, PartialEq)]
pub enum RunError {
    ParseError(String),
    ContextError(ContextError),
}

fn term_subst_top(s: &Term, t: &Term) -> Term {
    t.term_subst(0, &s.term_shift(1)).term_shift(-1)
}

impl Term {
    fn term_shift(&self, d: i32) -> Term {
        fn walk(d: i32, c: i32, t: &Term) -> Term {
            match *t {
                Var(x, n) => {
                    if x as i32 >= c {
                        Var((x as i32 + d) as u32, (n as i32 + d) as u32)
                    } else {
                        Var(x, (n as i32 + d) as u32)
                    }
                }
                Abs(ref x, ref t2) => Abs(x.clone(), Box::new(walk(d, c + 1, t2))),
                App(ref t1, ref t2) => App(Box::new(walk(d, c, t1)), Box::new(walk(d, c, t2))),
                True => True,
                False => False,
                If(ref t1, ref t2, ref t3) => {
                    If(
                        Box::new(walk(d, c, t1)),
                        Box::new(walk(d, c, t2)),
                        Box::new(walk(d, c, t3)),
                    )
                }
                Let(ref x, ref t1, ref t2) => {
                    Let(
                        x.clone(),
                        Box::new(walk(d, c, t1)),
                        Box::new(walk(d, c + 1, t2)),
                    )
                }
            }
        }
        walk(d, 0, self)
    }

    fn term_subst(&self, j: i32, s: &Term) -> Term {
        fn walk(j: i32, s: &Term, c: i32, t: &Term) -> Term {
            match *t {
                Var(x, n) => {
                    if x as i32 == j + c {
                        s.term_shift(c)
                    } else {
                        Var(x, n)
                    }
                }
                Abs(ref x, ref t1) => Abs(x.clone(), Box::new(walk(j, s, c + 1, t1))),
                App(ref t1, ref t2) => {
                    App(Box::new(walk(j, s, c, t1)), Box::new(walk(j, s, c, t2)))
                }
                True => True,
                False => False,
                If(ref t1, ref t2, ref t3) => {
                    If(
                        Box::new(walk(j, s, c, t1)),
                        Box::new(walk(j, s, c, t2)),
                        Box::new(walk(j, s, c, t3)),
                    )
                }
                Let(ref x, ref t1, ref t2) => {
                    Let(
                        x.clone(),
                        Box::new(walk(j, s, c, t1)),
                        Box::new(walk(j, s, c + 1, t2)),
                    )
                }
            }
        }
        walk(j, s, 0, self)
    }

    fn is_val(&self) -> bool {
        match *self {
            True | False => true,
            Abs(_, _) => true,
            _ => false,
        }
    }

    fn eval1(&self, ctx: &Context) -> Result<Term, EvalError> {
        match *self {
            If(box True, ref t2, _) => Ok(*t2.clone()),
            If(box False, _, ref t3) => Ok(*t3.clone()),
            If(ref t1, ref t2, ref t3) => Ok(If(Box::new(t1.eval1(ctx)?), t2.clone(), t3.clone())),
            App(box Abs(_, ref t12), ref v2) if v2.is_val() => Ok(term_subst_top(v2, t12)),
            App(ref v1, ref t2) if v1.is_val() => Ok(App(v1.clone(), Box::new(t2.eval1(ctx)?))),
            App(ref t1, ref t2) => Ok(App(Box::new(t1.eval1(ctx)?), t2.clone())),
            Let(_, ref v1, ref t2) if v1.is_val() => Ok(term_subst_top(v1, t2)),
            Let(ref x, ref t1, ref t2) => Ok(Let(x.clone(), Box::new(t1.eval1(ctx)?), t2.clone())),
            _ => Err(EvalError::NoRuleApplies(self.clone())),
        }
    }

    fn eval(&self, ctx: &Context) -> Term {
        match self.eval1(ctx) {
            Ok(t) => t.eval(ctx),
            Err(EvalError::NoRuleApplies(_)) => self.clone(),
        }
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
            Var(x, _) => {
                match self.context.index_to_name(x) {
                    Ok(n2) => write!(f, "{}", n2),
                    Err(e) => write!(f, "{:?}", e),
                }
            }
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
            App(ref t1, ref t2) => {
                write!(
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
                )
            }
            True => write!(f, "true"),
            False => write!(f, "false"),
            If(ref t0, ref t1, ref t2) => write!(f, "if {} then {} else {}", t0, t1, t2),
            Let(ref x, ref t0, ref t1) => write!(f, "let {} = {} in {}", x, t0, t1),
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
    use super::RunError;

    type ContextTermResult = Box<Fn(Context) -> Result<Term, RunError>>;

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

    named!(term_var <&[u8], ContextTermResult>,
        map!(identifier, |x| {
            let s = tos(x);
            Box::new(move |ctx: Context| -> Result<Term, RunError> {
                match ctx.name_to_index(&s) {
                    Ok(n1) => Ok(Var(n1, ctx.contexts.len() as u32)),
                    Err(e) => Err(RunError::ContextError(e)),
                }
            })
        })
    );

    named!(term_app <&[u8], ContextTermResult>,
        do_parse!(
            char!('(')       >>
            opt!(multispace) >>
            t1: term         >>
            multispace       >>
            t2: term         >>
            opt!(multispace) >>
            char!(')')       >>
            (Box::new(move |ctx: Context| -> Result<Term, RunError> {
                let r: Result<(Term, Term), RunError> = t1(ctx.clone()).and_then(|f: Term| {
                    t2(ctx.clone()).map(|g: Term| (f,g))
                });

                r.map(|(f,g)| App(Box::new(f), Box::new(g)))
            }))
        )
    );

    named!(term_abs <&[u8], ContextTermResult>,
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
                Box::new(move |ctx: Context| -> Result<Term, RunError> {
                    let (c2, x2) = ctx.add_binding(&s);
                    t1(c2).and_then(|t: Term| Ok(Abs(s.clone(), Box::new(t))))
                })
            })
        )
    );

    named!(term_if <&[u8], ContextTermResult>,
        do_parse!(
            opt!(complete!(char!('('))) >>
            tag!("if")   >> multispace >> t0: term >> multispace >>
            tag!("then") >> multispace >> t1: term >> multispace >>
            tag!("else") >> multispace >> t2: term >>
            opt!(complete!(char!(')'))) >>
            (Box::new(move |ctx: Context| -> Result<Term, RunError> {
                let r: Result<(Term, Term, Term), RunError> = t0(ctx.clone()).and_then(|f: Term| {
                    t1(ctx.clone()).and_then(|g: Term| {
                        t2(ctx.clone()).map(|h: Term| (f,g,h))
                    })
                });

                r.map(|(f,g,h)| If(Box::new(f), Box::new(g), Box::new(h)))
            }))
        )
    );

    named!(term_bools1 <&[u8], Term>, alt!(
            tag!("true")  => { |_| True  } |
            tag!("false") => { |_| False }
        )
    );

    named!(term_bools <&[u8], ContextTermResult>, map!(term_bools1, |x| {
        Box::new(move |ctx: Context| -> Result<Term, RunError> {
            Ok(x.clone())
        })
    }));

    named!(term_let <&[u8], ContextTermResult>,
        do_parse!(
            tag!("let") >> multispace >> x: identifier >> multispace >>
            char!('=') >> multispace >> t0: term >> multispace >>
            tag!("in") >> multispace >> t1: term >>
            ({
                let s = tos(x);
                Box::new(move |ctx: Context| -> Result<Term, RunError> {
                    let (c2, _) = ctx.add_binding(&s);

                    let r: Result<(Term, Term), RunError> = t0(c2.clone()).and_then(|f: Term| {
                        t1(c2.clone()).map(|g: Term| (f,g))
                    });

                    r.map(|(f,g)| Let(s.clone(), Box::new(f), Box::new(g)))
                })
            })
        )
    );

    named!(term <&[u8], ContextTermResult>, alt!(
        term_if | term_bools | term_let | term_var | term_app | term_abs
    ));

    pub fn parse(s: &[u8]) -> Result<Term, RunError> {
        match term(s) {
            IResult::Done(_, res) => (*res)(Context::new()),
            IResult::Error(e) => Err(RunError::ParseError(e.description().to_string())),
            IResult::Incomplete(e) => Err(RunError::ParseError(format!("Incomplete {:?}", e))),
        }
    }
}

pub fn run(s: &str) -> Result<String, RunError> {
    parser::parse(s.as_bytes())
        .map(|t| t.eval(&Context::new()))
        .map(|t| t.to_string())
}

pub fn repl(s: &str, ctx: &Context) -> Result<String, RunError> {
    parser::parse(s.as_bytes()).map(|t| t.eval(ctx)).map(|t| {
        t.to_string()
    })
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
        assert_eq!(app.to_string(), "(lambda a. (lambda a'. (a' a)))");
    }

    #[test]
    fn parse_test() {
        use super::parser::parse;

        assert_eq!(
            parse(b"(lambda a. (lambda a'. (a a')))"),
            Ok(Abs(
                "a".to_string(),
                Box::new(Abs(
                    "a'".to_string(),
                    Box::new(App(Box::new(Var(1, 2)), Box::new(Var(0, 2)))),
                )),
            ))
        );
    }
}
