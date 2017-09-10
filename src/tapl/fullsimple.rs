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

pub struct Binding {
    label: String,
    binding: BindingType,
}

pub struct Context {
    bindings: Vec<Binding>,
}

impl Context {
    pub fn new() -> Self {
        Context { bindings: Vec::new() }
    }
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

fn eval1(term: &Term) -> Result<Term, EvalError> {
    use self::Term::*;

    match *term {
        If(box True, ref t2, _) => Ok(*t2.clone()),
        If(box False, _, ref t3) => Ok(*t3.clone()),
        If(ref t1, ref t2, ref t3) => Ok(If(Box::new(eval1(t1)?), t2.clone(), t3.clone())),
        Tag(ref l, ref t1, ref ty_t) => Ok(Tag(l.clone(), Box::new(eval1(t1)?), ty_t.clone())),
        _ => Err(EvalError::NoRuleApplies(term.clone())),
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
    let ontype = |t| type_shift_above(d, c, t);

    term_map(&onvar, &ontype, c, t)
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

    let ontype = |t| _ontype(ty_s, j as i32, t);

    term_map(&onvar, &ontype, j, t)
}

fn type_term_subst_top(ty_s: &Type, t: &Term) -> Term {
    term_shift(-1, &type_term_subst(&type_shift(1, ty_s), 0, t))
}

#[cfg(test)]
mod tests {
    use super::Term::*;

    #[test]
    fn isval_test() {
        assert!(super::isval(&True));
    }
}
