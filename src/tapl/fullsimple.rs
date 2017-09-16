#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Var(usize, usize),
    Id(String),
    Arrow(Box<Type>, Box<Type>),
    Unit,
    Record(Vec<(String, Box<Type>)>),
    Variant(Vec<(String, Box<Type>)>),
    Bool,
    Str,
    Float,
    Nat,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    True,
    False,
    If(Box<Term>, Box<Term>, Box<Term>),
    Case(Box<Term>, Vec<(String, (String, Box<Term>))>),
    Tag(String, Box<Term>, Type),
    Var(usize, usize),
    Abs(String, Type, Box<Term>),
    App(Box<Term>, Box<Term>),
    Let(String, Box<Term>, Box<Term>),
    Fix(Box<Term>),
    Str(String),
    Unit,
    Ascribe(Box<Term>, Type),
    Record(Vec<(String, Box<Term>)>),
    Projection(Box<Term>, String),
    Float(f32),
    TimesFloat(Box<Term>, Box<Term>),
    Zero,
    Succ(Box<Term>),
    Pred(Box<Term>),
    IsZero(Box<Term>),
    Inert(Type),
}

#[derive(Clone, PartialEq)]
pub enum BindingType {
    NameBind,
    TypeVarBind,
    VarBind(Type),
    TermAbbBind(Term, Option<Type>),
    TypeAbbBind(Type),
}

pub enum Command {
    Eval(Term),
    Bind(String, BindingType),
}

#[derive(Clone, PartialEq)]
pub struct Binding {
    label: String,
    binding: BindingType,
}


pub enum EvalError {
    NoRuleApplies(Term),
}

fn is_numeric_val(term: &Term) -> bool {
    use self::Term::*;

    match *term {
        Zero => true,
        Succ(ref t1) => is_numeric_val(t1),
        _ => false,
    }
}

fn isval(term: &Term) -> bool {
    use self::Term::*;

    match *term {
        True | False | Str(_) | Float(_) | Unit | Abs(_, _, _) => true,
        Tag(_, ref t, _) => isval(t),
        Record(ref fields) => fields.iter().all(|&(_, ref t)| isval(t)),
        ref t if is_numeric_val(t) => true,
        _ => false,
    }
}

fn eval1(ctx: Context, term: &Term) -> Result<Term, EvalError> {
    use self::Term::*;

    match *term {
        If(box True, ref t2, _) => Ok(*t2.clone()),
        If(box False, _, ref t3) => Ok(*t3.clone()),
        If(ref t1, ref t2, ref t3) => Ok(If(
            Box::new(eval1(ctx.clone(), t1)?),
            t2.clone(),
            t3.clone(),
        )),
        Tag(ref l, ref t1, ref ty_t) => Ok(Tag(
            l.clone(),
            Box::new(eval1(ctx.clone(), t1)?),
            ty_t.clone(),
        )),
        Case(box Tag(ref li, ref v11, _), ref branches) if isval(v11) => {
            match branches.iter().find(|&x| li.clone() == (x.1).0) {
                Some(&(_, (_, ref body))) => Ok(term_subst_top(v11, body)),
                None => Err(EvalError::NoRuleApplies(*v11.clone())),
            }
        }
        Case(ref t1, ref branches) => {
            eval1(ctx.clone(), t1).and_then(|t| Ok(Case(Box::new(t), branches.clone())))
        }
        App(box Abs(_, _, ref t12), ref v2) if isval(v2) => Ok(term_subst_top(v2, t12)),
        App(ref v1, ref t2) if isval(v1) => {
            eval1(ctx.clone(), t2).and_then(|t| Ok(App(Box::new(*v1.clone()), Box::new(t))))
        }
        App(ref t1, ref t2) => {
            eval1(ctx.clone(), t1).and_then(|t| Ok(App(Box::new(t), Box::new(*t2.clone()))))
        }
        Let(_, ref v1, ref t2) if isval(v1) => Ok(term_subst_top(v1, t2)),
        Let(ref x, ref t1, ref t2) => {
            eval1(ctx.clone(), t1).and_then(|t| {
                Ok(Let(x.clone(), Box::new(t), Box::new(*t2.clone())))
            })
        }
        Fix(ref v1) if isval(v1) => {
            match *v1 {
                box Abs(_, _, ref t12) => Ok(term_subst_top(term, t12)),
                _ => Err(EvalError::NoRuleApplies(*v1.clone())),
            }
        }
        Fix(ref t1) => eval1(ctx.clone(), t1).and_then(|t| Ok(Fix(Box::new(t)))),
        Var(n, _) => {
            match get_binding(ctx.clone(), n) {
                Ok(binding) => {
                    match binding {
                        BindingType::TermAbbBind(t, _) => Ok(t),
                        _ => Err(EvalError::NoRuleApplies(term.clone())),
                    }
                }
                Err(_) => Err(EvalError::NoRuleApplies(term.clone())),
            }
        }
        Ascribe(ref v1, _) if isval(v1) => Ok(*v1.clone()),
        Ascribe(ref t1, ref ty_t) => {
            eval1(ctx.clone(), t1).and_then(|t| Ok(Ascribe(Box::new(t), ty_t.clone())))
        }
        Projection(ref v1 @ box Record(_), ref l) if isval(v1) => {
            match *v1.clone() {
                Record(fields) => {
                    match fields.iter().find(|&x| l.clone() == x.0) {
                        Some(&(_, ref body)) => Ok(*body.clone()),
                        None => Err(EvalError::NoRuleApplies(*v1.clone())),
                    }
                }
                _ => Err(EvalError::NoRuleApplies(*v1.clone())),
            }
        }
        Projection(ref t1, ref l) => {
            eval1(ctx.clone(), t1).and_then(|t| Ok(Projection(Box::new(t), l.clone())))
        }
        TimesFloat(box Float(ref f1), box Float(ref f2)) => Ok(Float(f1 * f2)),
        TimesFloat(ref t1 @ box Float(_), ref t2) => {
            eval1(ctx.clone(), t2).and_then(|t| Ok(TimesFloat(t1.clone(), Box::new(t))))
        }
        TimesFloat(ref t1, ref t2) => {
            eval1(ctx.clone(), t1).and_then(|t| Ok(TimesFloat(Box::new(t), t2.clone())))
        }
        Succ(ref t1) => eval1(ctx.clone(), t1).and_then(|t| Ok(Succ(Box::new(t)))),
        Pred(box Zero) => Ok(Zero),
        Pred(box Succ(ref nv1)) if is_numeric_val(nv1) => Ok(*nv1.clone()),
        Pred(ref t1) => eval1(ctx.clone(), t1).and_then(|t| Ok(Pred(Box::new(t)))),
        IsZero(box Zero) => Ok(True),
        IsZero(box Succ(ref nv1)) if is_numeric_val(nv1) => Ok(False),
        IsZero(ref t1) => eval1(ctx.clone(), t1).and_then(|t| Ok(IsZero(Box::new(t)))),
        _ => Err(EvalError::NoRuleApplies(term.clone())),
    }
}

fn eval(ctx: Context, t: &Term) -> Term {
    match eval1(ctx.clone(), t) {
        Ok(t) => eval(ctx, &t),
        Err(EvalError::NoRuleApplies(_)) => t.clone(),
    }
}

fn term_map(
    onvar: &Fn(usize, usize) -> Term,
    ontype: &Fn(&Type) -> Type,
    c: usize,
    t: &Term,
) -> Term {
    use self::Term::*;

    fn walk(
        onvar: &Fn(usize, usize) -> Term,
        ontype: &Fn(&Type) -> Type,
        c: usize,
        term: &Term,
    ) -> Term {
        match *term {
            Inert(ref ty_t) => Inert(ontype(ty_t)),
            Var(x, n) => onvar(x, n),
            True | False | Str(_) | Unit | Float(_) | Zero => term.clone(),
            Abs(ref x, ref ty_t1, ref t2) => {
                Abs(
                    x.clone(),
                    ontype(ty_t1),
                    Box::new(walk(onvar, ontype, c + 1, t2)),
                )
            }
            App(ref t1, ref t2) => {
                App(
                    Box::new(walk(onvar, ontype, c, t1)),
                    Box::new(walk(onvar, ontype, c, t2)),
                )
            }
            Let(ref x, ref t1, ref t2) => {
                Let(
                    x.clone(),
                    Box::new(walk(onvar, ontype, c, t1)),
                    Box::new(walk(onvar, ontype, c + 1, t2)),
                )
            }
            Fix(ref t1) => Fix(Box::new(walk(onvar, ontype, c, t1))),
            If(ref t1, ref t2, ref t3) => {
                If(
                    Box::new(walk(onvar, ontype, c, t1)),
                    Box::new(walk(onvar, ontype, c, t2)),
                    Box::new(walk(onvar, ontype, c, t3)),
                )
            }
            Projection(ref t1, ref l) => {
                Projection(Box::new(walk(onvar, ontype, c, t1)), l.clone())
            }
            Record(ref fields) => Record(
                fields
                    .iter()
                    .map(|&(ref li, ref ti)| {
                        (li.clone(), Box::new(walk(onvar, ontype, c, ti)))
                    })
                    .collect::<Vec<(String, Box<Term>)>>(),
            ),
            Ascribe(ref t1, ref ty_t1) => {
                Ascribe(Box::new(walk(onvar, ontype, c, t1)), ontype(ty_t1))
            }
            TimesFloat(ref t1, ref t2) => {
                TimesFloat(
                    Box::new(walk(onvar, ontype, c, t1)),
                    Box::new(walk(onvar, ontype, c, t2)),
                )
            }
            Succ(ref t1) => Succ(Box::new(walk(onvar, ontype, c, t1))),
            Pred(ref t1) => Pred(Box::new(walk(onvar, ontype, c, t1))),
            IsZero(ref t1) => IsZero(Box::new(walk(onvar, ontype, c, t1))),
            Tag(ref l, ref t1, ref ty_t) => {
                Tag(
                    l.clone(),
                    Box::new(walk(onvar, ontype, c, t1)),
                    ontype(ty_t),
                )
            }
            Case(ref t, ref cases) => {
                Case(
                    Box::new(walk(onvar, ontype, c, t)),
                    cases
                        .iter()
                        .map(|&(ref li, (ref xi, ref ti))| {
                            (li.clone(), (
                                xi.clone(),
                                Box::new(walk(onvar, ontype, c + 1, ti)),
                            ))
                        })
                        .collect::<Vec<(String, (String, Box<Term>))>>(),
                )
            }
        }
    }
    walk(onvar, ontype, 0, t)
}

fn term_shift_above(d: i32, c: usize, t: &Term) -> Term {
    use self::Term::*;

    fn _onvar(d: i32, c: usize, x: usize, n: usize) -> Term {
        if x >= c {
            Var(((x as i32) + d) as usize, ((n as i32) + d) as usize)
        } else {
            Var(x, ((n as i32) + d) as usize)
        }
    }

    let onvar = |x, n| _onvar(d, c, x, n);

    term_map(&onvar, &|t| type_shift_above(d, c, t), c, t)
}

fn term_shift(d: i32, t: &Term) -> Term {
    term_shift_above(d, 0, t)
}

fn type_map(onvar: &Fn(usize, usize) -> Type, ty_t: &Type) -> Type {
    use self::Type::*;

    fn walk(onvar: &Fn(usize, usize) -> Type, ty_t: &Type) -> Type {
        match *ty_t {
            Var(x, n) => onvar(x, n),
            Id(_) => ty_t.clone(),
            Str => Str,
            Unit => Unit,
            Float => Float,
            Bool => Bool,
            Nat => Nat,
            Arrow(ref ty_t1, ref ty_t2) => {
                Arrow(Box::new(walk(onvar, ty_t1)), Box::new(walk(onvar, ty_t2)))
            }
            Record(ref field_tys) => Record(
                field_tys
                    .iter()
                    .map(|&(ref li, ref ty_ti)| {
                        (li.clone(), Box::new(walk(onvar, ty_ti)))
                    })
                    .collect::<Vec<(String, Box<Type>)>>(),
            ),
            Variant(ref field_tys) => Variant(
                field_tys
                    .iter()
                    .map(|&(ref li, ref ty_ti)| {
                        (li.clone(), Box::new(walk(onvar, ty_ti)))
                    })
                    .collect::<Vec<(String, Box<Type>)>>(),
            ),
        }
    }

    walk(onvar, ty_t)
}

fn type_shift_above(d: i32, c: usize, ty_t: &Type) -> Type {
    use self::Type::*;

    fn _onvar(d: i32, c: usize, x: usize, n: usize) -> Type {
        if x >= c {
            Var(((x as i32) + d) as usize, ((n as i32) + d) as usize)
        } else {
            Var(x, ((n as i32) + d) as usize)
        }
    }

    let onvar = |x, n| _onvar(d, c, x, n);
    type_map(&onvar, ty_t)
}

fn type_shift(d: i32, ty_t: &Type) -> Type {
    type_shift_above(d, 0, ty_t)
}

fn type_subst(ty_s: &Type, j: i32, ty_t: &Type) -> Type {
    use self::Type::*;

    fn _onvar(j: i32, x: usize, n: usize, ty_s: &Type) -> Type {
        if x as i32 == j {
            type_shift(j, ty_s)
        } else {
            Var(x, n)
        }
    }

    let onvar = |x, n| _onvar(j, x, n, ty_s);
    type_map(&onvar, ty_t)
}

fn type_subst_top(ty_s: &Type, ty_t: &Type) -> Type {
    type_shift(-1, &type_subst(&type_shift(1, ty_s), 0, ty_t))
}

fn binding_shift(d: i32, bind: &BindingType) -> BindingType {
    use self::BindingType::*;

    match *bind {
        NameBind => NameBind,
        TypeVarBind => TypeVarBind,
        TermAbbBind(ref t, ref opt_ty_t) => {
            let ts_opt = match *opt_ty_t {
                None => None,
                Some(ref ty_t) => Some(type_shift(d, ty_t)),
            };
            TermAbbBind(term_shift(d, t), ts_opt)
        }
        VarBind(ref ty_t) => VarBind(type_shift(d, ty_t)),
        TypeAbbBind(ref ty_t) => TypeAbbBind(type_shift(d, ty_t)),
    }
}

fn term_subst(j: usize, s: &Term, t: &Term) -> Term {
    use self::Term::Var;

    fn _onvar(j: usize, s: &Term, x: usize, n: usize) -> Term {
        if x == j {
            term_shift(j as i32, s)
        } else {
            Var(x, n)
        }
    }
    let onvar = |x, n| _onvar(j, s, x, n);
    fn ontype(t: &Type) -> Type {
        t.clone()
    }

    term_map(&onvar, &ontype, j, t)
}

fn term_subst_top(s: &Term, t: &Term) -> Term {
    term_shift(-1, &term_subst(0, &term_shift(1, s), t))
}

fn type_term_subst(ty_s: &Type, j: usize, t: &Term) -> Term {
    use self::Term::Var;

    fn onvar(x: usize, n: usize) -> Term {
        Var(x, n)
    }

    fn _ontype(ty_s: &Type, j: i32, ty_t: &Type) -> Type {
        type_subst(ty_s, j, ty_t)
    }

    term_map(&onvar, &|t| _ontype(ty_s, j as i32, t), j, t)
}

fn type_term_subst_top(ty_s: &Type, t: &Term) -> Term {
    term_shift(-1, &type_term_subst(&type_shift(1, ty_s), 0, t))
}

pub type Context = Vec<Binding>;

#[derive(Clone, Debug, PartialEq)]
pub enum ContextError {
    ArrowExpected,
    ParameterTypeMismatch,
    VariableLookupFailure(usize, usize),
    WrongBindingForVariable,
    ConditionalWithGuardOfNotBoolean,
    ConditionalWithArmsOfDifferentTypes,
    UnboundIdentifier(String),
    NoRecordedType(usize),
}

fn is_name_bound(ctx: Context, binding: &Binding) -> bool {
    ctx.iter().position(|b| b == binding).is_some()
}

fn add_binding(ctx: Context, binding: &Binding) -> Context {
    let mut newc = ctx.clone();
    newc.insert(0, binding.clone());

    newc
}

fn add_name(ctx: Context, name: String) -> Context {
    add_binding(
        ctx,
        &Binding {
            label: name,
            binding: BindingType::NameBind,
        },
    )
}

fn pick_fresh_name(ctx: Context, binding: &Binding) -> (Context, Binding) {
    if is_name_bound(ctx.clone(), binding) {
        let mut nb = (*binding).clone();
        nb.label = nb.label + "'";
        pick_fresh_name(ctx.clone(), &nb)
    } else {
        let newc = add_binding(ctx.clone(), binding);
        (newc, (*binding).clone())
    }
}

fn index_to_name(ctx: Context, idx: usize) -> Result<Binding, ContextError> {
    if idx >= ctx.len() {
        Err(ContextError::VariableLookupFailure(idx, ctx.len()))
    } else {
        let a = &ctx[idx];
        Ok(a.clone())
    }
}

fn name_to_index(ctx: Context, name: &str) -> Result<usize, ContextError> {
    match ctx.iter().position(|b| b.label == name) {
        Some(s) => Ok(s),
        None => Err(ContextError::UnboundIdentifier(name.to_string())),
    }
}

fn get_binding(ctx: Context, idx: usize) -> Result<BindingType, ContextError> {
    index_to_name(ctx.clone(), idx).and_then(|b: Binding| {
        Ok(binding_shift((idx + 1) as i32, &b.binding))
    })
}

fn get_type_from_context(ctx: Context, idx: usize) -> Result<Type, ContextError> {
    get_binding(ctx.clone(), idx).and_then(|bt: BindingType| match bt {
        BindingType::VarBind(t) => Ok(t),
        BindingType::TermAbbBind(_, Some(t)) => Ok(t),
        BindingType::TermAbbBind(_, None) => Err(ContextError::NoRecordedType(idx)),
        _ => Err(ContextError::WrongBindingForVariable),
    })
}

#[derive(Debug, PartialEq)]
pub enum RunError {
    ParseError(String),
    ContextError(ContextError),
}

mod parser {
    extern crate nom;

    use std::str;

    use super::Term;
    use super::Type;
    use super::Binding;
    use super::Context;
    use super::RunError;
    use super::BindingType;
    use super::{add_binding, name_to_index};

    use nom::IResult;
    use nom::multispace;

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
                match name_to_index(ctx.clone(), &s) {
                    Ok(n1) => Ok(Term::Var(n1, ctx.len())),
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

                r.map(|(f,g)| Term::App(Box::new(f), Box::new(g)))
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
                    let b = &Binding {
                        label: s.clone(),
                        binding: BindingType::VarBind(xt.clone()),
                    };
                    let c2 = add_binding(ctx, b);
                    t1(c2).and_then(|t: Term| Ok(Term::Abs(s.clone(), xt.clone(), Box::new(t))))
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

                r.map(|(f,g,h)| Term::If(Box::new(f), Box::new(g), Box::new(h)))
            }))
        )
    );

    named!(term_bools1 <&[u8], Term>, alt!(
            tag!("true")  => { |_| Term::True  } |
            tag!("false") => { |_| Term::False }
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

fn is_type_abb(c: Context, i: usize) -> bool {
    match get_binding(c, i) {
        Ok(self::BindingType::TypeAbbBind(_)) => true,
        _ => false,
    }
}

fn get_type_abb(c: Context, i: usize) -> Result<Type, EvalError> {
    match get_binding(c, i) {
        Ok(self::BindingType::TypeAbbBind(ref t)) => Ok(t.clone()),
        _ => Err(EvalError::NoRuleApplies(Term::False)), // introduce a dummy term
    }
}

fn compute_type(ctx: Context, ty_t: Type) -> Result<Type, EvalError> {
    match ty_t {
        Type::Var(ref i, _) if is_type_abb(ctx.clone(), i.clone()) => {
            get_type_abb(ctx.clone(), i.clone())
        }
        _ => Err(EvalError::NoRuleApplies(Term::False)), // introduce a dummy term
    }
}

fn simplify_type(ctx: Context, ty_t: Type) -> Type {
    match compute_type(ctx.clone(), ty_t.clone()) {
        Ok(t) => simplify_type(ctx.clone(), t),
        _ => ty_t,
    }
}

fn equivalent(ctx: Context, ty_s: Type, ty_t: Type) -> bool {
    use self::Type::*;

    let ty_s = simplify_type(ctx.clone(), ty_s);
    let ty_t = simplify_type(ctx.clone(), ty_t);
    match (ty_s.clone(), ty_t.clone()) {
        (Nat, Nat) => true,
        (Str, Str) => true,
        (Bool, Bool) => true,
        (Unit, Unit) => true,
        (Float, Float) => true,
        (Id(b1), Id(b2)) => b1 == b2,
        (Var(i, _), _) if is_type_abb(ctx.clone(), i) => {
            match get_type_abb(ctx.clone(), i) {
                Ok(t) => equivalent(ctx.clone(), t, ty_t.clone()),
                _ => false,
            }
        }
        (_, Var(i, _)) if is_type_abb(ctx.clone(), i) => {
            match get_type_abb(ctx.clone(), i) {
                Ok(t) => equivalent(ctx.clone(), ty_s.clone(), t),
                _ => false,
            }
        }
        (Var(i, _), Var(j, _)) => i == j,
        (Arrow(ref ty_s1, ref ty_s2), Arrow(ref ty_t1, ref ty_t2)) => {
            equivalent(ctx.clone(), *ty_s1.clone(), *ty_t1.clone()) &&
                equivalent(ctx.clone(), *ty_s2.clone(), *ty_t2.clone())
        }
        _ => false,
    }
}

pub fn type_of(c: Context, t: &Term) -> Result<Type, ContextError> {
    use self::Term::*;

    match *t {
        True | False => Ok(Type::Bool),
        Inert(ref t) => Ok(t.clone()),
        Var(i, _) => get_type_from_context(c.clone(), i),
        Abs(ref x, ref ty_t1, ref t2) => {
            let ctx1 = add_binding(
                c.clone(),
                &Binding {
                    label: x.clone(),
                    binding: BindingType::VarBind(ty_t1.clone()),
                },
            );
            type_of(ctx1, t2).and_then(|ty_t2: Type| {
                Ok(Type::Arrow(Box::new(ty_t1.clone()), Box::new(ty_t2)))
            })
        }
        App(ref t1, ref t2) => {
            type_of(c.clone(), t1).and_then(|_: Type| {
                type_of(c.clone(), t2).and_then(|ty_t2: Type| match ty_t2 {
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
            type_of(c.clone(), t1).and_then(|ty_t1: Type| if ty_t1 == Type::Bool {
                type_of(c.clone(), t2).and_then(|ty_t2: Type| {
                    type_of(c.clone(), t3).and_then(|ty_t3: Type| if ty_t2 == ty_t3 {
                        Ok(ty_t2)
                    } else {
                        Err(ContextError::ConditionalWithArmsOfDifferentTypes)
                    })
                })
            } else {
                Err(ContextError::ConditionalWithGuardOfNotBoolean)
            })
        }
        _ => Err(ContextError::ParameterTypeMismatch),
    }
}

pub fn repl(s: &str, ctx: Context) -> Result<String, RunError> {
    parser::parse(s.as_bytes())
        .map(|t| eval(ctx.clone(), &t))
        .and_then(|t| match type_of(ctx, &t) {
            Ok(ty_t) => Ok(format!("{:?} : {:?}", t, ty_t)),
            Err(e) => Err(RunError::ContextError(e)),
        })
}

#[cfg(test)]
mod tests {
    use super::Binding;
    use super::Context;
    use super::Term::*;
    use super::BindingType::*;

    #[test]
    fn isval_test() {
        assert!(super::isval(&True));
    }

    #[test]
    fn is_name_bound_test() {
        let ctx = Context::new();
        let b = &Binding {
            label: "test".to_string(),
            binding: NameBind,
        };
        let ctx = super::add_name(ctx, "test".to_string());
        assert!(super::is_name_bound(ctx, b))
    }

    #[test]
    fn pick_fresh_name_test() {
        let ctx = Context::new();
        let ctx = super::add_name(ctx, "a".to_string());
        let b = &Binding {
            label: "a".to_string(),
            binding: NameBind,
        };
        let (newc, newb) = super::pick_fresh_name(ctx, b);
        assert_eq!(newb.label, "a'".to_string());
    }


    #[test]
    fn index_to_name_to_index_test() {
        let ctx = Context::new();
        let ctx = super::add_name(ctx, "a".to_string());
        let ctx = super::add_name(ctx, "b".to_string());

        assert_eq!(1, super::name_to_index(ctx.clone(), "a").unwrap());
        assert_eq!("b".to_string(), super::index_to_name(ctx, 0).unwrap().label);
    }

    #[test]
    fn eval1_test() {
        let ctx = Context::new();
        let t = &TimesFloat(Box::new(Float(2.5)), Box::new(Float(2.5)));

        assert_eq!(Float(6.25), super::eval(ctx, t));
    }
}
