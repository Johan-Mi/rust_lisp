use std::num::ParseIntError;
use std::rc::Rc;
use std::str::FromStr;

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

impl FromStr for Integer {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Integer { value: s.parse()? })
    }
}

impl FromStr for Symbol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn char_is_symbol_initial(c: char) -> bool {
            c.is_alphabetic()
                || match c {
                    '!' | '$' | '%' | '&' | '*' | '/' | ':' | '<' | '='
                    | '>' | '?' | '^' | '_' | '~' => true,
                    _ => false,
                }
        }

        fn char_is_symbol_subsequent(c: char) -> bool {
            char_is_symbol_initial(c)
                || c.is_digit(10)
                || match c {
                    '+' | '.' | '@' | '-' => true,
                    _ => false,
                }
        }

        if s == "+" || s == "-" || s == "..." || {
            let first_char = s.chars().next().ok_or(())?;
            char_is_symbol_initial(first_char)
                && s.chars().skip(1).all(char_is_symbol_subsequent)
        } {
            Ok(Symbol {
                name: String::from(s),
            })
        } else {
            Err(())
        }
    }
}

impl Cons {
    pub fn len(&self) -> usize {
        match self {
            Cons::Nil => 0,
            Cons::Some(_, next) => match &**next {
                Object::Cons(rest) => rest.len() + 1,
                _ => 1,
            },
        }
    }
}
