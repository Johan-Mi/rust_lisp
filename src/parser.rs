use crate::{
    lexer::Token,
    types::{Cons, Object, Quote, Symbol},
};
use std::rc::Rc;

fn parse_integer(tokens: &[Token]) -> Option<(i32, &[Token])> {
    if let [Token::Ident(num_str), tokens @ ..] = tokens {
        let num = num_str.parse().ok()?;
        Some((num, tokens))
    } else {
        None
    }
}

fn parse_symbol(tokens: &[Token]) -> Option<(Symbol, &[Token])> {
    if let [Token::Ident(symbol_str), tokens @ ..] = tokens {
        let symbol = symbol_str.parse().ok()?;
        Some((symbol, tokens))
    } else {
        None
    }
}

fn parse_lparen(tokens: &[Token]) -> Option<&[Token]> {
    tokens.strip_prefix(&[Token::LParen])
}

fn parse_rparen(tokens: &[Token]) -> Option<&[Token]> {
    tokens.strip_prefix(&[Token::RParen])
}

fn parse_quote(tokens: &[Token]) -> Option<&[Token]> {
    tokens.strip_prefix(&[Token::Quote])
}

fn parse_dot(tokens: &[Token]) -> Option<&[Token]> {
    match tokens {
        [Token::Ident(s), tokens @ ..] if s == "." => Some(tokens),
        _ => None,
    }
}

fn parse_quoted_expression(tokens: &[Token]) -> Option<(Quote, &[Token])> {
    let remaining_tokens = parse_quote(tokens)?;
    let (expr, unconsumed_tokens) = parse_expression(remaining_tokens)?;
    Some((Quote(Rc::new(expr)), unconsumed_tokens))
}

fn parse_cons(tokens: &[Token]) -> Option<(Cons, &[Token])> {
    fn parse_cons_helper(tokens: &[Token]) -> Option<(Cons, &[Token])> {
        if let Some(unconsumed_tokens) = parse_rparen(tokens) {
            Some((Cons(None), unconsumed_tokens))
        } else {
            let (first_expr, remaining_tokens) = parse_expression(tokens)?;
            if let Some(remaining_tokens) = parse_dot(remaining_tokens) {
                let (last_expr, remaining_tokens) =
                    parse_expression(remaining_tokens)?;
                let unconsumed_tokens = parse_rparen(remaining_tokens)?;
                Some((
                    Cons(Some((Rc::new(first_expr), Rc::new(last_expr)))),
                    unconsumed_tokens,
                ))
            } else {
                let (rest, remaining_tokens) =
                    parse_cons_helper(remaining_tokens)?;
                Some((
                    Cons(Some((
                        Rc::new(first_expr),
                        Rc::new(Object::Cons(rest)),
                    ))),
                    remaining_tokens,
                ))
            }
        }
    }

    let remaining_tokens = parse_lparen(tokens)?;
    parse_cons_helper(remaining_tokens)
}

pub fn parse_expression(tokens: &[Token]) -> Option<(Object, &[Token])> {
    if let Some((expr, tokens)) = parse_cons(tokens) {
        Some((Object::Cons(expr), tokens))
    } else if let Some((expr, tokens)) = parse_quoted_expression(tokens) {
        Some((Object::Quote(expr), tokens))
    } else if let Some((expr, tokens)) = parse_integer(tokens) {
        Some((Object::Integer(expr), tokens))
    } else if let Some((expr, tokens)) = parse_symbol(tokens) {
        Some((Object::Symbol(expr), tokens))
    } else {
        None
    }
}

pub fn parse_expressions(
    mut tokens: &[Token],
) -> Option<(Vec<Object>, &[Token])> {
    let mut ret = Vec::new();

    while let Some((expr, remaining_tokens)) = parse_expression(tokens) {
        ret.push(expr);
        tokens = remaining_tokens;
    }

    if tokens.is_empty() {
        Some((ret, tokens))
    } else {
        None
    }
}
