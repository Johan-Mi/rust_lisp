use super::lexer::*;
use super::types::*;
use std::rc::Rc;

fn char_is_symbol_initial(c: char) -> bool {
    if c.is_alphabetic() {
        true
    } else {
        match c {
            '!' | '$' | '%' | '&' | '*' | '/' | ':' | '<' | '=' | '>' | '?'
            | '^' | '_' | '~' => true,
            _ => false,
        }
    }
}

fn char_is_symbol_subsequent(c: char) -> bool {
    if char_is_symbol_initial(c) || c.is_digit(10) {
        true
    } else {
        match c {
            '+' | '.' | '@' | '-' => true,
            _ => false,
        }
    }
}

fn parse_integer(tokens: &[Token]) -> Option<(Integer, &[Token])> {
    match tokens.first() {
        Some(first_token) => {
            if first_token.token_type != TokenType::Ident {
                None
            } else {
                match first_token.string.parse() {
                    Ok(num) => {
                        Some((Integer { value: num }, tokens.get(1..).unwrap()))
                    }
                    _ => None,
                }
            }
        }
        _ => None,
    }
}

fn parse_symbol(tokens: &[Token]) -> Option<(Symbol, &[Token])> {
    match tokens.first() {
        Some(first_token) => {
            if first_token.token_type != TokenType::Ident {
                None
            } else if first_token.string == "+"
                || first_token.string == "-"
                || first_token.string == "..."
            {
                Some((
                    Symbol {
                        name: first_token.string.clone(),
                    },
                    tokens.get(1..).unwrap(),
                ))
            } else {
                let maybe_first_char = first_token.string.chars().next();
                match maybe_first_char {
                    Some(first_char) => {
                        if char_is_symbol_initial(first_char)
                            && first_token
                                .string
                                .chars()
                                .skip(1)
                                .all(char_is_symbol_subsequent)
                        {
                            Some((
                                Symbol {
                                    name: first_token.string.clone(),
                                },
                                tokens.get(1..).unwrap(),
                            ))
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
        }
        _ => None,
    }
}

fn parse_lparen(tokens: &[Token]) -> Option<&[Token]> {
    match tokens.first() {
        Some(first_token) if first_token.token_type == TokenType::LParen => {
            tokens.get(1..)
        }
        _ => None,
    }
}

fn parse_rparen(tokens: &[Token]) -> Option<&[Token]> {
    match tokens.first() {
        Some(first_token) if first_token.token_type == TokenType::RParen => {
            tokens.get(1..)
        }
        _ => None,
    }
}

fn parse_quote(tokens: &[Token]) -> Option<&[Token]> {
    match tokens.first() {
        Some(first_token) if first_token.token_type == TokenType::Quote => {
            tokens.get(1..)
        }
        _ => None,
    }
}

fn parse_dot(tokens: &[Token]) -> Option<&[Token]> {
    match tokens.first() {
        Some(first_token)
            if first_token.token_type == TokenType::Ident
                && first_token.string == "." =>
        {
            tokens.get(1..)
        }
        _ => None,
    }
}

fn parse_quoted_expression(tokens: &[Token]) -> Option<(Quote, &[Token])> {
    match parse_quote(tokens) {
        Some(remaining_tokens) => match parse_expression(remaining_tokens) {
            Some((expr, unconsumed_tokens)) => {
                Some((Quote { contained: expr }, unconsumed_tokens))
            }
            _ => None,
        },
        _ => None,
    }
}

fn parse_cons(tokens: &[Token]) -> Option<(Cons, &[Token])> {
    match parse_lparen(tokens) {
        Some(remaining_tokens) => parse_cons_helper(remaining_tokens),
        _ => None,
    }
}

fn parse_cons_helper(tokens: &[Token]) -> Option<(Cons, &[Token])> {
    match parse_rparen(tokens) {
        Some(unconsumed_tokens) => Some((Cons::Nil, unconsumed_tokens)),
        _ => match parse_expression(tokens) {
            Some((first_expr, remaining_tokens)) => {
                match parse_dot(remaining_tokens) {
                    Some(remaining_tokens) => {
                        match parse_expression(remaining_tokens) {
                            Some((last_expr, remaining_tokens)) => {
                                match parse_rparen(remaining_tokens) {
                                    Some(unconsumed_tokens) => Some((
                                        Cons::Some(first_expr, last_expr),
                                        unconsumed_tokens,
                                    )),
                                    _ => None,
                                }
                            }
                            _ => None,
                        }
                    }
                    _ => match parse_cons_helper(remaining_tokens) {
                        Some((rest, remaining_tokens)) => Some((
                            Cons::Some(first_expr, Rc::new(Object::Cons(rest))),
                            remaining_tokens,
                        )),
                        _ => None,
                    },
                }
            }
            _ => None,
        },
    }
}

pub fn parse_expression(tokens: &[Token]) -> Option<(Rc<Object>, &[Token])> {
    match parse_cons(tokens) {
        Some((expr, unconsumed_tokens)) => {
            Some((Rc::new(Object::Cons(expr)), unconsumed_tokens))
        }
        _ => match parse_quoted_expression(tokens) {
            Some((expr, unconsumed_tokens)) => {
                Some((Rc::new(Object::Quote(expr)), unconsumed_tokens))
            }
            _ => match parse_integer(tokens) {
                Some((expr, unconsumed_tokens)) => {
                    Some((Rc::new(Object::Integer(expr)), unconsumed_tokens))
                }
                _ => match parse_symbol(tokens) {
                    Some((expr, unconsumed_tokens)) => {
                        Some((Rc::new(Object::Symbol(expr)), unconsumed_tokens))
                    }
                    _ => None,
                },
            },
        },
    }
}

pub fn parse_expressions(
    mut tokens: &[Token],
) -> Option<(Vec<Rc<Object>>, &[Token])> {
    let mut ret = Vec::<Rc<Object>>::new();

    loop {
        match parse_expression(tokens) {
            Some((expr, remaining_tokens)) => {
                ret.push(expr);
                tokens = remaining_tokens;
            }
            _ => {
                return if ret.is_empty() {
                    None
                } else {
                    Some((ret, tokens))
                }
            }
        }
    }
}
