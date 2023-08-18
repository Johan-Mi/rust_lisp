use crate::types::{Cons, Error, Object};
use std::{fmt, rc::Rc, str::FromStr};

#[derive(Clone, PartialEq, Eq)]
pub struct Symbol {
    name: String,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name.fmt(f)
    }
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

        if matches!(s, "+" | "-" | "...") || {
            let mut chars = s.chars();
            let first_char = chars.next().ok_or(())?;
            char_is_symbol_initial(first_char)
                && chars.all(char_is_symbol_subsequent)
        } {
            Ok(Self {
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
