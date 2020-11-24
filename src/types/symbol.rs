use super::cons::*;
use super::error::*;
use super::object::*;
use derive_more::Display;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Clone, PartialEq, Display)]
pub struct Symbol {
    pub name: String,
}

impl FromStr for Symbol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn char_is_symbol_initial(c: char) -> bool {
            c.is_alphabetic()
                || matches!(
                    c,
                    '!' | '$'
                        | '%'
                        | '&'
                        | '*'
                        | '/'
                        | ':'
                        | '<'
                        | '='
                        | '>'
                        | '?'
                        | '^'
                        | '_'
                        | '~'
                )
        }

        fn char_is_symbol_subsequent(c: char) -> bool {
            char_is_symbol_initial(c)
                || c.is_digit(10)
                || matches!(c, '+' | '.' | '@' | '-')
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

impl Symbol {
    pub fn eval(&self, env: &Cons) -> Result<(Rc<Object>, Cons), Error> {
        fn eval_symbol_internal(
            symbol: &Symbol,
            env: &Cons,
        ) -> Result<Rc<Object>, Error> {
            match env {
                Cons::Nil => {
                    Err(Error::new(format!("Unbound variable {}", symbol)))
                }
                Cons::Some(first, rest) => match &*car_obj(first.clone())? {
                    Object::Symbol(found_symbol) if symbol == found_symbol => {
                        cdr_obj(first.clone())
                    }
                    _ => match &**rest {
                        Object::Cons(next_cons) => {
                            eval_symbol_internal(symbol, &next_cons)
                        }
                        _ => Err(Error::new(format!(
                            "Unbound variable {}",
                            symbol
                        ))),
                    },
                },
            }
        }

        Ok((eval_symbol_internal(self, env)?, env.clone()))
    }
}
