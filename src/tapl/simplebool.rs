use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Arrow(Box<Type>, Box<Type>),
    Bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    True,
    False,
    Var(usize, usize),
    App(Box<Term>, Box<Term>),
    Abs(String, Type, Box<Term>),
    If(Box<Term>, Box<Term>, Box<Term>),
}

#[derive(Clone, PartialEq)]
pub enum BindingType {
    NameBind,
    VarBind(Type),
}


#[derive(Clone, PartialEq)]
pub struct Binding {
    label: String,
    binding: BindingType,
}

pub enum Command {
    Eval(Term),
    Bind(String, Binding),
}

#[derive(Clone)]
pub struct Context {
    bindings: Vec<Binding>,
}

struct ContextTerm<'a> {
    context: &'a Context,
    term: &'a Term,
}

use self::Term::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ContextError {
    ArrowExpected,
    ParameterTypeMismatch,
    VariableLookupFailure(usize, usize),
    WrongBindingForVariable,
    ConditionalWithGuardOfNotBoolean,
    ConditionalWithArmsOfDifferentTypes,
    UnboundIdentifier(String),
}

impl Context {
    pub fn new() -> Self {
        Context { bindings: Vec::new() }
    }

    fn get_type(&self, idx: usize) -> Result<Type, ContextError> {
        self.get_binding(idx).and_then(
            |b: Binding| match b.binding {
                BindingType::VarBind(t) => Ok(t),
                _ => Err(ContextError::WrongBindingForVariable),
            },
        )
    }

    fn type_of(&self, term: &Term) -> Result<Type, ContextError> {
        match *term {
            Var(i, _) => self.get_type(i),
            Abs(ref x, ref ty_t1, ref t2) => {
                let ctx1 = self.add_binding(Binding {
                    label: x.clone(),
                    binding: BindingType::VarBind(ty_t1.clone()),
                });
                ctx1.type_of(t2).and_then(|ty_t2: Type| {
                    Ok(Type::Arrow(Box::new(ty_t1.clone()), Box::new(ty_t2)))
                })
            }
            App(ref t1, ref t2) => {
                self.type_of(t1).and_then(|_: Type| {
                    self.type_of(t2).and_then(|ty_t2: Type| match ty_t2 {
                        Type::Arrow(_, ref ty_t12) => {
                            if ty_t2 == *ty_t12.clone() {
                                Ok(*ty_t12.clone())
                            } else {
                                Err(ContextError::ParameterTypeMismatch)
                            }
                        }
                        _ => Err(ContextError::ArrowExpected),
                    })
                })
            }
            If(ref t1, ref t2, ref t3) => {
                self.type_of(t1).and_then(
                    |ty_t1: Type| if ty_t1 == Type::Bool {
                        self.type_of(t2).and_then(|ty_t2: Type| {
                            self.type_of(t3).and_then(|ty_t3: Type| if ty_t2 == ty_t3 {
                                Ok(ty_t2)
                            } else {
                                Err(ContextError::ConditionalWithArmsOfDifferentTypes)
                            })
                        })
                    } else {
                        Err(ContextError::ConditionalWithGuardOfNotBoolean)
                    },
                )
            }
            True | False => Ok(Type::Bool),
        }
    }

    fn add_binding(&self, b: Binding) -> Context {
        let mut newc = (*self).clone();
        newc.bindings.insert(0, b);
        newc
    }

    fn get_binding(&self, idx: usize) -> Result<Binding, ContextError> {
        match self.bindings.get(idx) {
            Some(s) => Ok((*s).clone()),
            None => Err(ContextError::VariableLookupFailure(idx, self.len())),
        }
    }

    fn len(&self) -> usize {
        self.bindings.len()
    }

    fn add_name(&mut self, name: String) -> Context {
        self.add_binding(Binding {
            label: name,
            binding: BindingType::NameBind,
        })
    }

    fn is_name_bound(&self, binding: &Binding) -> bool {
        self.bindings.iter().position(|b| b == binding).is_some()
    }

    fn pick_fresh_name(&self, binding: &Binding) -> (Context, Binding) {
        if self.is_name_bound(binding) {
            let mut nb = (*binding).clone();
            nb.label = nb.label + "'";
            self.pick_fresh_name(&nb)
        } else {
            let mut c = (*self).clone();
            c.bindings.insert(0, (*binding).clone());
            (c, (*binding).clone())
        }
    }

    fn index_to_name(&self, idx: usize) -> Result<Binding, ContextError> {
        if idx as usize >= self.len() {
            Err(ContextError::VariableLookupFailure(idx, self.len()))
        } else {
            let a = &self.bindings[idx as usize];
            Ok(a.clone())
        }
    }

    fn name_to_index(&self, name: &str) -> Result<usize, ContextError> {
        match self.bindings.iter().position(|b| b.label == name) {
            Some(s) => Ok(s),
            None => Err(ContextError::UnboundIdentifier(name.to_string())),
        }
    }
}

enum EvalError {
    NoRuleApplies(Term),
}

fn term_subst_top(s: &Term, t: &Term) -> Term {
    t.term_subst(0, &s.term_shift(1)).term_shift(-1)
}

impl Term {
    fn term_shift(&self, d: i32) -> Term {
        fn walk(d: i32, c: usize, t: &Term) -> Term {
            match *t {
                Var(x, n) => {
                    if x >= c {
                        Var(((x as i32) + d) as usize, ((n as i32) + d) as usize)
                    } else {
                        Var(x, ((n as i32) + d) as usize)
                    }
                }
                Abs(ref x, ref ty_t1, ref t2) => {
                    Abs(x.clone(), ty_t1.clone(), Box::new(walk(d, c + 1, t2)))
                }
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
                Abs(ref x, ref ty_t1, ref t1) => {
                    Abs(x.clone(), ty_t1.clone(), Box::new(walk(j, s, c + 1, t1)))
                }
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
            }
        }
        walk(j, s, 0, self)
    }

    fn is_val(&self) -> bool {
        match *self {
            True | False => true,
            Abs(_, _, _) => true,
            _ => false,
        }
    }

    fn eval1(&self, ctx: &Context) -> Result<Term, EvalError> {
        match *self {
            If(box True, ref t2, _) => Ok(*t2.clone()),
            If(box False, _, ref t3) => Ok(*t3.clone()),
            If(ref t1, ref t2, ref t3) => Ok(If(Box::new(t1.eval1(ctx)?), t2.clone(), t3.clone())),
            App(box Abs(_, _, ref t12), ref v2) if v2.is_val() => Ok(term_subst_top(v2, t12)),
            App(ref v1, ref t2) if v1.is_val() => Ok(App(v1.clone(), Box::new(t2.eval1(ctx)?))),
            App(ref t1, ref t2) => Ok(App(Box::new(t1.eval1(ctx)?), t2.clone())),
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

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Bool => write!(f, "Bool"),
            Type::Arrow(ref t1, ref t2) => write!(f, "{}->{}", t1, t2),
        }
    }
}

impl fmt::Display for Binding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.label)
    }
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
            True => write!(f, "true"),
            False => write!(f, "false"),
            Abs(ref x, ref ty_t1, ref t1) => {
                let (ctx1, x1) = self.context.pick_fresh_name(&Binding {
                    label: x.clone(),
                    binding: BindingType::VarBind((*ty_t1).clone()),
                });
                write!(
                    f,
                    "(lambda {}:{}. {})",
                    x1,
                    ty_t1,
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
            If(ref t0, ref t1, ref t2) => write!(f, "if {} then {} else {}", t0, t1, t2),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RunError {
    ParseError(String),
    ContextError(ContextError),
}

mod parser {
    extern crate nom;

    use std::str;

    use nom::multispace;
    use nom::IResult;

    use super::Type;
    use super::Term;
    use super::Binding;
    use super::Context;
    use super::Term::*;
    use super::RunError;
    use super::BindingType;

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
                    Ok(n1) => Ok(Var(n1, ctx.len())),
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

    named!(type_non_arrow <&[u8], Type>, alt!(
        tag!("Bool") => { |_| Type::Bool }
    ));

    named!(types <&[u8], Type>, alt!(
            do_parse!(
                t1: type_non_arrow >>
                tag!("->")         >>
                t2: type_non_arrow >>
                (Type::Arrow(Box::new(t1), Box::new(t2)))
            )                                   |
            type_non_arrow
        )
    );

    named!(term_abs <&[u8], ContextTermResult>,
        do_parse!(
            char!('(')       >>
            opt!(multispace) >>
            tag!("lambda")   >>
            multispace       >>
            x: identifier    >>
            char!(':')       >>
            xt: types        >>
            char!('.')       >>
            multispace       >>
            t1: term         >>
            opt!(multispace) >>
            char!(')')       >>
            ({
                let s = tos(x);
                Box::new(move |ctx: Context| -> Result<Term, RunError> {
                    let b = Binding  {
                        label: s.clone(),
                        binding: BindingType::VarBind(xt.clone()),
                    };
                    let c2 = ctx.add_binding(b);
                    t1(c2).and_then(|t: Term| Ok(Abs(s.clone(), xt.clone(), Box::new(t))))
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

    named!(term <&[u8], ContextTermResult>, alt!(
        term_if | term_bools | term_var | term_app | term_abs
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
    parser::parse(s.as_bytes()).map(|t| t.eval(ctx)).and_then(
        |t| {
            match ctx.type_of(&t) {
                Ok(ty_t) => Ok(format!("{} : {}", t, ty_t)),
                Err(e) => Err(RunError::ContextError(e)),
            }
        },
    )
}
