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

#[cfg(test)]
mod tests {
    use super::Term::*;

    #[test]
    fn isval_test() {
        assert!(super::isval(&True));
    }
}
