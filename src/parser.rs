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
        Some(first_token) if first_token.token_type == TokenType::Ident => {
            let num = first_token.string.parse().ok()?;
            Some((Integer { value: num }, tokens.get(1..).unwrap()))
        }
        _ => None,
    }
}

fn parse_symbol(tokens: &[Token]) -> Option<(Symbol, &[Token])> {
    match tokens.first() {
        Some(first_token) if first_token.token_type == TokenType::Ident => {
            if first_token.string == "+"
                || first_token.string == "-"
                || first_token.string == "..."
                || {
                    let first_char = first_token.string.chars().next()?;
                    char_is_symbol_initial(first_char)
                        && first_token
                            .string
                            .chars()
                            .skip(1)
                            .all(char_is_symbol_subsequent)
                }
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
    let remaining_tokens = parse_quote(tokens)?;
    let (expr, unconsumed_tokens) = parse_expression(remaining_tokens)?;
    Some((Quote { contained: expr }, unconsumed_tokens))
}

fn parse_cons(tokens: &[Token]) -> Option<(Cons, &[Token])> {
    let remaining_tokens = parse_lparen(tokens)?;
    parse_cons_helper(remaining_tokens)
}

fn parse_cons_helper(tokens: &[Token]) -> Option<(Cons, &[Token])> {
    match parse_rparen(tokens) {
        Some(unconsumed_tokens) => Some((Cons::Nil, unconsumed_tokens)),
        _ => {
            let (first_expr, remaining_tokens) = parse_expression(tokens)?;
            match parse_dot(remaining_tokens) {
                Some(remaining_tokens) => {
                    let (last_expr, remaining_tokens) =
                        parse_expression(remaining_tokens)?;
                    let unconsumed_tokens = parse_rparen(remaining_tokens)?;
                    Some((Cons::Some(first_expr, last_expr), unconsumed_tokens))
                }
                _ => {
                    let (rest, remaining_tokens) =
                        parse_cons_helper(remaining_tokens)?;
                    Some((
                        Cons::Some(first_expr, Rc::new(Object::Cons(rest))),
                        remaining_tokens,
                    ))
                }
            }
        }
    }
}

pub fn parse_expression(tokens: &[Token]) -> Option<(Rc<Object>, &[Token])> {
    if let Some((expr, unconsumed_tokens)) = parse_cons(tokens) {
        return Some((Rc::new(Object::Cons(expr)), unconsumed_tokens));
    }
    if let Some((expr, unconsumed_tokens)) = parse_quoted_expression(tokens) {
        return Some((Rc::new(Object::Quote(expr)), unconsumed_tokens));
    }
    if let Some((expr, unconsumed_tokens)) = parse_integer(tokens) {
        return Some((Rc::new(Object::Integer(expr)), unconsumed_tokens));
    }
    if let Some((expr, unconsumed_tokens)) = parse_symbol(tokens) {
        return Some((Rc::new(Object::Symbol(expr)), unconsumed_tokens));
    }

    None
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
