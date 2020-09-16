use std::rc::Rc;

pub enum Object {
    Integer(Integer),
    Symbol(Symbol),
    Error(Error),
    Function(Function),
    BuiltinFunction(BuiltinFunction),
    Quote(Quote),
    Cons(Cons),
    Bool(Bool),
}

pub struct Integer {
    pub value: i32,
}

#[derive(Clone, PartialEq)]
pub struct Symbol {
    pub name: String,
}

pub struct Error {
    pub message: String,
}

pub struct BuiltinFunction {
    pub func: fn(&Cons, &Cons) -> (Rc<Object>, Cons),
}

pub struct Function {
    pub parameters: Cons,
    pub body: Rc<Object>,
}

pub struct Quote {
    pub contained: Rc<Object>,
}

pub struct Bool {
    pub value: bool,
}

#[derive(Clone)]
pub enum Cons {
    Some(Rc<Object>, Rc<Object>),
    Nil,
}
