#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Token {
    LParen,
    RParen,
    Quote,
    Ident(String),
}

pub fn lex(s: &str) -> Vec<Token> {
    let mut ret = Vec::new();

    let mut string_buffer = String::new();

    for c in s.chars() {
        match c {
            '(' => {
                if !string_buffer.is_empty() {
                    ret.push(Token::Ident(string_buffer));
                    string_buffer = String::new();
                }
                ret.push(Token::LParen);
            }
            ')' => {
                if !string_buffer.is_empty() {
                    ret.push(Token::Ident(string_buffer));
                    string_buffer = String::new();
                }
                ret.push(Token::RParen);
            }
            '\'' => {
                if !string_buffer.is_empty() {
                    ret.push(Token::Ident(string_buffer));
                    string_buffer = String::new();
                }
                ret.push(Token::Quote);
            }
            ' ' | '\t' | '\n' => {
                if !string_buffer.is_empty() {
                    ret.push(Token::Ident(string_buffer));
                    string_buffer = String::new();
                }
            }
            _ => {
                string_buffer.push(c);
            }
        }
    }

    if !string_buffer.is_empty() {
        ret.push(Token::Ident(string_buffer));
    }

    ret
}
