use super::types::*;
use std::fmt;

fn to_cons_string(obj: &Object) -> String {
    match obj {
        Object::Cons(cons) => match cons {
            Cons::Nil => String::new(),
            Cons::Some(first, second) => {
                format!(" {}{}", first, to_cons_string(second))
            }
        },
        _ => format!(" . {}", obj),
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
        write!(formatter, "Function {} => {}", self.parameters, self.body)
    }
}

impl fmt::Display for Quote {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "(quote {})", self.contained)
    }
}

impl fmt::Display for Cons {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cons::Nil => write!(formatter, "()"),
            Cons::Some(first, second) => {
                write!(formatter, "({}{})", first, to_cons_string(second))
            }
        }
    }
}

impl fmt::Display for Bool {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.value)
    }
}

impl fmt::Display for Object {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Integer(contained) => contained.fmt(formatter),
            Object::Symbol(contained) => contained.fmt(formatter),
            Object::Error(contained) => contained.fmt(formatter),
            Object::Function(contained) => contained.fmt(formatter),
            Object::BuiltinFunction(contained) => contained.fmt(formatter),
            Object::Quote(contained) => contained.fmt(formatter),
            Object::Cons(contained) => contained.fmt(formatter),
            Object::Bool(contained) => contained.fmt(formatter),
        }
    }
}
