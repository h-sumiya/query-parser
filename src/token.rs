use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Token {
    AND,        // &
    OR,         // |
    NOT,        // -
    OpenParen,  // (
    CloseParen, // )
    Split,      // :
    Value(usize),
}

#[derive(Debug)]
pub struct Tokens {
    tokens: Vec<Token>,
    values: Vec<String>,
}

impl Tokens {
    pub fn new(query: &str) -> Self {
        let mut chars = query.chars();
        let mut value = String::with_capacity(20);
        let mut tokens = Tokens {
            tokens: Vec::new(),
            values: Vec::new(),
        };
        while let Some(c) = chars.next() {
            let token = match c {
                ' ' => {
                    if !value.is_empty() {
                        tokens.add_value(&mut value);
                    }
                    continue;
                }
                '&' => Token::AND,
                '|' => Token::OR,
                '-' => Token::NOT,
                '(' => Token::OpenParen,
                ')' => Token::CloseParen,
                ':' => Token::Split,
                '\\' => {
                    if let Some(c) = chars.next() {
                        value.push(c);
                    }
                    continue;
                }
                '"' => {
                    Self::read_value(&mut chars, &mut value);
                    continue;
                }
                _ => {
                    value.push(c);
                    continue;
                }
            };
            if !value.is_empty() {
                tokens.add_value(&mut value);
            }
            tokens.add_token(token);
        }
        if !value.is_empty() {
            tokens.add_value(&mut value);
        }
        tokens
    }

    fn read_value(chars: &mut std::str::Chars, value: &mut String) {
        while let Some(c) = chars.next() {
            match c {
                '"' => {
                    if value.ends_with('\\') {
                        value.pop();
                        value.push('"');
                    } else {
                        break;
                    }
                }
                _ => value.push(c),
            }
        }
    }

    fn add_value(&mut self, value: &mut String) {
        self.add_token(Token::Value(self.values.len()));
        self.values.push(value.clone());
        value.clear();
    }

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }
}

impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for token in &self.tokens {
            match token {
                Token::AND => write!(f, "&")?,
                Token::OR => write!(f, "|")?,
                Token::NOT => write!(f, "-")?,
                Token::OpenParen => write!(f, "(")?,
                Token::CloseParen => write!(f, ")")?,
                Token::Split => write!(f, ":")?,
                Token::Value(i) => {
                    write!(f, "{}", self.values[*i])?;
                }
            }
            write!(f, ",")?;
        }
        Ok(())
    }
}
