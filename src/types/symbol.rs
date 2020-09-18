use std::fmt;
use std::str::FromStr;

#[derive(Clone, PartialEq)]
pub struct Symbol {
    pub name: String,
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

impl fmt::Display for Symbol {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.name)
    }
}
