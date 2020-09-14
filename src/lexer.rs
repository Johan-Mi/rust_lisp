#[derive(Clone, PartialEq, Debug)]
pub enum TokenType {
    LParen,
    RParen,
    Quote,
    Ident,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub string: String,
}

pub fn lex(s: &str) -> Vec<Token> {
    let mut ret = Vec::new();

    let mut current_token = Token {
        token_type: TokenType::Ident,
        string: String::new(),
    };

    for c in s.chars() {
        match c {
            '(' => {
                if !current_token.string.is_empty() {
                    ret.push(current_token);
                    current_token = Token {
                        token_type: TokenType::Ident,
                        string: String::new(),
                    };
                }
                ret.push(Token {
                    token_type: TokenType::LParen,
                    string: String::new(),
                });
            }
            ')' => {
                if !current_token.string.is_empty() {
                    ret.push(current_token);
                    current_token = Token {
                        token_type: TokenType::Ident,
                        string: String::new(),
                    };
                }
                ret.push(Token {
                    token_type: TokenType::RParen,
                    string: String::new(),
                });
            }
            '\'' => {
                if !current_token.string.is_empty() {
                    ret.push(current_token);
                    current_token = Token {
                        token_type: TokenType::Ident,
                        string: String::new(),
                    };
                }
                ret.push(Token {
                    token_type: TokenType::Quote,
                    string: String::new(),
                });
            }
            ' ' | '\t' | '\n' => {
                if !current_token.string.is_empty() {
                    ret.push(current_token);
                    current_token = Token {
                        token_type: TokenType::Ident,
                        string: String::new(),
                    };
                }
            }
            _ => {
                current_token.string.push(c);
            }
        }
    }

    if !current_token.string.is_empty() {
        ret.push(current_token);
    }

    ret
}
