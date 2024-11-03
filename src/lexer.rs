#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Token {
    LParen,
    RParen,
    Quote,
    Ident(String),
}

pub fn lex(mut source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    loop {
        source = source.trim_start();
        source = if let Some(s) = source.strip_prefix('(') {
            tokens.push(Token::LParen);
            s
        } else if let Some(s) = source.strip_prefix(')') {
            tokens.push(Token::RParen);
            s
        } else if let Some(s) = source.strip_prefix('\'') {
            tokens.push(Token::Quote);
            s
        } else {
            let ident_end = source
                .find(|c: char| c.is_whitespace() || matches!(c, '(' | ')' | '\''))
                .unwrap_or(source.len());
            if ident_end == 0 {
                break;
            }
            let (ident, s) = source.split_at(ident_end);
            tokens.push(Token::Ident(ident.to_owned()));
            s
        }
    }

    tokens
}
