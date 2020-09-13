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

pub struct Symbol {
    pub name: String,
}

pub struct Error {
    pub message: String,
}

pub struct BuiltinFunction {
    pub func: fn(&Cons, &Cons) -> Rc<Object>,
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

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
