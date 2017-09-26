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
