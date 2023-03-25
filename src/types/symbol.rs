use crate::types::{Cons, Error, Object};
use derive_more::{Constructor, Display};
use std::{rc::Rc, str::FromStr};

#[derive(Clone, PartialEq, Eq, Display, Constructor)]
pub struct Symbol {
    name: String,
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
                || c.is_ascii_digit()
                || matches!(c, '+' | '.' | '@' | '-')
        }

        if s == "+" || s == "-" || s == "..." || {
            let first_char = s.chars().next().ok_or(())?;
            char_is_symbol_initial(first_char)
                && s.chars().skip(1).all(char_is_symbol_subsequent)
        } {
            Ok(Self::new(String::from(s)))
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
                    Err(Error::new(format!("Unbound variable {symbol}").into()))
                }
                Cons::Some(first, rest) => match &*first.clone().car()? {
                    Object::Symbol(found_symbol) if symbol == found_symbol => {
                        first.clone().cdr()
                    }
                    _ => match &**rest {
                        Object::Cons(next_cons) => {
                            eval_symbol_internal(symbol, next_cons)
                        }
                        _ => Err(Error::new(
                            format!("Unbound variable {symbol}").into(),
                        )),
                    },
                },
            }
        }

        Ok((eval_symbol_internal(self, env)?, env.clone()))
    }
}
