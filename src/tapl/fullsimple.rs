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
                Some(&(ref x, (_, ref body))) => Ok(term_subst_top(v11, body)),
                None => Err(EvalError::NoRuleApplies(*v11.clone())),
            }
        }
        Case(ref t1, ref branches) => {
            eval1(ctx.clone(), t1).and_then(|t| Ok(Case(Box::new(t), branches.clone())))
        }
        App(box Abs(ref x, ref ty_t11, ref t12), ref v2) if isval(v2) => Ok(
            term_subst_top(v2, t12),
        ),
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
        Ascribe(ref v1, ref ty_t) if isval(v1) => Ok(*v1.clone()),
        Ascribe(ref t1, ref ty_t) => {
            eval1(ctx.clone(), t1).and_then(|t| Ok(Ascribe(Box::new(t), ty_t.clone())))
        }
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
            _ => term.clone(),
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

type Context = Vec<Binding>;

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
}
