use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    True,
    False,
    If(Box<Term>, Box<Term>, Box<Term>),
    Zero,
    Succ(Box<Term>),
    Pred(Box<Term>),
    IsZero(Box<Term>),
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Bool,
    Nat,
}

use self::Term::*;
use self::Type::*;

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            True => write!(f, "true"),
            False => write!(f, "false"),
            If(ref t0, ref t1, ref t2) => write!(f, "if {} then {} else {}", t0, t1, t2),
            Zero => write!(f, "0"),
            Succ(ref t0) => {
                fn go(n: u32, t: &Term, f: &mut fmt::Formatter) -> fmt::Result {
                    match *t {
                        Zero => write!(f, "{}", n),
                        Succ(ref s) => go(n + 1, s, f),
                        _ => write!(f, "(succ {})", t),
                    }
                }
                go(1, t0, f)
            }
            Pred(ref t0) => write!(f, "pred {}", t0),
            IsZero(ref t0) => write!(f, "iszero {}", t0),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Nat => write!(f, "Nat"),
            Bool => write!(f, "Bool"),
        }
    }
}

fn is_numeric_val(t: &Term) -> bool {
    match *t {
        Zero => true,
        Succ(ref t1) => is_numeric_val(t1),
        _ => false,
    }
}

enum EvalError {
    NoRuleApplies(Term),
}


#[derive(Debug, PartialEq)]
pub enum RunError {
    ParseError(String),
    TypeError(String),
}

fn eval1(t: Term) -> Result<Term, EvalError> {
    match t {
        If(box True, t1, _) => Ok(*t1),
        If(box False, _, t2) => Ok(*t2),
        If(t0, t1, t2) => Ok(If(Box::new(eval1(*t0)?), t1, t2)),
        Succ(t1) => Ok(Succ(Box::new(eval1(*t1)?))),
        Pred(box Zero) => Ok(Zero),
        Pred(box Succ(box ref nv)) if is_numeric_val(nv) => Ok((*nv).clone()),
        Pred(t0) => Ok(Pred(Box::new(eval1(*t0)?))),
        IsZero(box Zero) => Ok(True),
        IsZero(box Succ(box ref nv)) if is_numeric_val(nv) => Ok(False),
        IsZero(t0) => Ok(IsZero(Box::new(eval1(*t0)?))),
        t0 => Err(EvalError::NoRuleApplies(t0)),
    }
}

fn eval(t: Term) -> Term {
    match eval1(t.clone()) {
        Ok(t1) => eval(t1),
        Err(EvalError::NoRuleApplies(_)) => t,
    }
}

fn type_of(t: Term) -> Result<Type, RunError> {
    match t {
        True | False => Ok(Bool),
        Zero => Ok(Nat),
        If(t0, t1, t2) => {
            type_of((*t0).clone()).and_then(|tyt0: Type| if tyt0 == Bool {
                type_of((*t1).clone()).and_then(|tyt1: Type| {
                    type_of((*t2).clone()).and_then(|tyt2: Type| if tyt1 == tyt2 {
                        Ok(tyt2)
                    } else {
                        Err(RunError::TypeError(
                            "arms of conditional have different types".to_string(),
                        ))
                    })
                })
            } else {
                Err(RunError::TypeError(
                    "guard of conditional not a boolean".to_string(),
                ))
            })
        }
        Succ(t0) => {
            type_of(*t0).and_then(|t: Type| if t == Nat {
                Ok(Nat)
            } else {
                Err(RunError::TypeError(
                    "argument of succ is not a number".to_string(),
                ))
            })
        }
        Pred(t0) => {
            type_of(*t0).and_then(|t: Type| if t == Nat {
                Ok(Nat)
            } else {
                Err(RunError::TypeError(
                    "argument of pred is not a number".to_string(),
                ))
            })
        }
        IsZero(t0) => {
            type_of(*t0).and_then(|t: Type| if t == Nat {
                Ok(Bool)
            } else {
                Err(RunError::TypeError(
                    "argument of iszero is not a number".to_string(),
                ))
            })
        }
    }
}

pub mod parser {
    extern crate nom;

    use nom::multispace;
    use nom::IResult;

    use super::Term;
    use super::Term::*;
    use super::RunError;

    named!(term <&[u8], Term>, alt!(
        tag!("true")  => { |_| True  }     |
        tag!("false") => { |_| False }     |
        tag!("zero")  => { |_| Zero  }     |
        do_parse!(
            opt!(complete!(char!('('))) >>
            tag!("iszero")              >>
            multispace                  >>
            t0: term                    >>
            opt!(complete!(char!(')'))) >>
            (IsZero(Box::new(t0)))
        )                                  |
        do_parse!(
            opt!(complete!(char!('('))) >>
            tag!("succ")                >>
            multispace                  >>
            t0: term                    >>
            opt!(complete!(char!(')'))) >>
            (Succ(Box::new(t0)))
        )                                  |
        do_parse!(
            opt!(complete!(char!('('))) >>
            tag!("pred")                >>
            multispace                  >>
            t0: term                    >>
            opt!(complete!(char!(')'))) >>
            (Pred(Box::new(t0)))
        )                                  |
        do_parse!(
            opt!(complete!(char!('('))) >>
            tag!("if")   >> multispace >> t0: term >> multispace >>
            tag!("then") >> multispace >> t1: term >> multispace >>
            tag!("else") >> multispace >> t2: term >>
            opt!(complete!(char!(')'))) >>
            (If(Box::new(t0), Box::new(t1), Box::new(t2)))
        )
    ));

    pub fn parse(s: &[u8]) -> Result<Term, RunError> {
        match term(s) {
            IResult::Done(_, o) => Ok(o),
            IResult::Error(e) => Err(RunError::ParseError(e.description().to_string())),
            IResult::Incomplete(e) => Err(RunError::ParseError(format!("Incomplete {:?}", e))),
        }
    }
}

pub fn run(s: &str) -> Result<String, RunError> {
    parser::parse(s.as_bytes()).and_then(|t: Term| {
        type_of(t.clone()).map(|tt: Type| format!("{}: {}", eval(t), tt))
    })
}

#[cfg(test)]
mod tests {
    use super::run;
    use super::Term::*;
    use super::Type::*;
    use super::RunError;
    use super::is_numeric_val;
    use super::type_of;
    use super::parser::parse;

    #[test]
    fn eval_test() {
        assert!(is_numeric_val(&Zero));
    }

    #[test]
    fn run_test() {
        assert_eq!(run("succ (succ zero)").ok().unwrap(), "2: Nat");
        assert_eq!(run("succ (pred zero)").ok().unwrap(), "1: Nat");
        assert_eq!(run("pred (succ zero)").ok().unwrap(), "0: Nat");
        assert_eq!(
            run("iszero (pred (succ (succ zero)))").ok().unwrap(),
            "false: Bool"
        );
        assert_eq!(
            run("if (iszero (succ zero)) then (succ zero) else (pred zero)")
                .ok()
                .unwrap(),
            "0: Nat"
        );
        assert_eq!(
            run("pred (succ (succ (succ zero)))").ok().unwrap(),
            "2: Nat"
        );
    }

    #[test]
    fn parse_test() {
        assert_eq!(parse(b"false").ok(), Some(False));
        assert_eq!(parse(b"iszero false").ok(), Some(IsZero(Box::new(False))));
        assert_eq!(
            parse(b"iszero (pred (succ (succ zero)))").ok(),
            Some(IsZero(Box::new(
                Pred(Box::new(Succ(Box::new(Succ(Box::new(Zero)))))),
            )))
        );
    }

    #[test]
    fn type_of_test() {
        assert_eq!(type_of(False).ok(), Some(Bool));
        assert_eq!(type_of(Zero).ok(), Some(Nat));
    }
}
