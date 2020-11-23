use super::lexer::*;
use super::types::*;
use std::rc::Rc;

fn parse_integer(tokens: &[Token]) -> Option<(Integer, &[Token])> {
    if let Some(Token::Ident(num_str)) = tokens.first() {
        let num = num_str.parse().ok()?;
        Some((num, tokens.get(1..)?))
    } else {
        None
    }
}

fn parse_symbol(tokens: &[Token]) -> Option<(Symbol, &[Token])> {
    if let Some(Token::Ident(symbol_str)) = tokens.first() {
        let symbol = symbol_str.parse().ok()?;
        Some((symbol, tokens.get(1..)?))
    } else {
        None
    }
}

fn parse_lparen(tokens: &[Token]) -> Option<&[Token]> {
    match tokens.first() {
        Some(Token::LParen) => tokens.get(1..),
        _ => None,
    }
}

fn parse_rparen(tokens: &[Token]) -> Option<&[Token]> {
    match tokens.first() {
        Some(Token::RParen) => tokens.get(1..),
        _ => None,
    }
}

fn parse_quote(tokens: &[Token]) -> Option<&[Token]> {
    match tokens.first() {
        Some(Token::Quote) => tokens.get(1..),
        _ => None,
    }
}

fn parse_dot(tokens: &[Token]) -> Option<&[Token]> {
    match tokens.first() {
        Some(Token::Ident(s)) if s == "." => tokens.get(1..),
        _ => None,
    }
}

fn parse_quoted_expression(tokens: &[Token]) -> Option<(Quote, &[Token])> {
    let remaining_tokens = parse_quote(tokens)?;
    let (expr, unconsumed_tokens) = parse_expression(remaining_tokens)?;
    Some((Quote { contained: expr }, unconsumed_tokens))
}

fn parse_cons(tokens: &[Token]) -> Option<(Cons, &[Token])> {
    fn parse_cons_helper(tokens: &[Token]) -> Option<(Cons, &[Token])> {
        if let Some(unconsumed_tokens) = parse_rparen(tokens) {
            Some((Cons::Nil, unconsumed_tokens))
        } else {
            let (first_expr, remaining_tokens) = parse_expression(tokens)?;
            if let Some(remaining_tokens) = parse_dot(remaining_tokens) {
                let (last_expr, remaining_tokens) =
                    parse_expression(remaining_tokens)?;
                let unconsumed_tokens = parse_rparen(remaining_tokens)?;
                Some((Cons::Some(first_expr, last_expr), unconsumed_tokens))
            } else {
                let (rest, remaining_tokens) =
                    parse_cons_helper(remaining_tokens)?;
                Some((
                    Cons::Some(first_expr, Rc::new(Object::Cons(rest))),
                    remaining_tokens,
                ))
            }
        }
    }

    let remaining_tokens = parse_lparen(tokens)?;
    parse_cons_helper(remaining_tokens)
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
    let mut ret = Vec::new();

    while let Some((expr, remaining_tokens)) = parse_expression(tokens) {
        ret.push(expr);
        tokens = remaining_tokens;
    }

    if ret.is_empty() {
        None
    } else {
        Some((ret, tokens))
    }
}
