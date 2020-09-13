use super::types::*;
use std::fmt;
use std::rc::Rc;

pub fn to_string_obj(obj: Rc<Object>) -> String {
    match &*obj {
        Object::Integer(contained) => contained.to_string(),
        Object::Symbol(contained) => contained.to_string(),
        Object::Error(contained) => contained.to_string(),
        Object::Function(contained) => contained.to_string(),
        Object::BuiltinFunction(contained) => contained.to_string(),
        Object::Quote(contained) => contained.to_string(),
        Object::Cons(contained) => contained.to_string(),
        Object::Bool(contained) => contained.to_string(),
    }
}

fn to_cons_string(obj: Rc<Object>) -> String {
    match &*obj {
        Object::Cons(cons) => match cons {
            Cons::Nil => String::from(')'),
            Cons::Some(first, second) => format!(
                " {}{}",
                to_string_obj(first.clone()),
                to_cons_string(second.clone())
            ),
        },
        _ => format!(" . {})", to_string_obj(obj)),
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.value)
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.name)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Error: {}", self.message)
    }
}

impl fmt::Display for BuiltinFunction {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Builtin function")
    }
}

impl fmt::Display for Function {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Function {} => {}",
            self.parameters,
            to_string_obj(self.body.clone())
        )
    }
}

impl fmt::Display for Quote {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "(quote {})",
            to_string_obj(self.contained.clone())
        )
    }
}

impl fmt::Display for Cons {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cons::Nil => write!(formatter, "()"),
            Cons::Some(first, second) => write!(
                formatter,
                "({}{}",
                to_string_obj(first.clone()),
                to_cons_string(second.clone())
            ),
        }
    }
}

impl fmt::Display for Bool {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.value)
    }
}
