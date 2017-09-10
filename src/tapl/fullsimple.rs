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

#[cfg(test)]
mod tests {
    use super::Term::*;

    #[test]
    fn isval_test() {
        assert!(super::isval(&True));
    }
}
