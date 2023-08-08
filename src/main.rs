
#[derive(Debug, PartialEq)]
enum Token {
    AND,        // &
    OR,         // |
    NOT,        // -
    OpenParen,  // (
    CloseParen, // )
    Split,      // :
    Value(String),
}

fn read_value(chars: &mut std::str::Chars) -> String {
    let mut value = String::new();
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
    value
}

fn tokenize(query: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = query.chars();
    let mut value = String::new();
    while let Some(c) = chars.next() {
        let token = match c {
            ' ' => {
                if !value.is_empty() {
                    tokens.push(Token::Value(value));
                    value = String::new();
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
            '"' => Token::Value(read_value(&mut chars)),
            _ => {
                value.push(c);
                continue;
            }
        };
        if !value.is_empty() {
            tokens.push(Token::Value(value));
            value = String::new();
        }
        tokens.push(token);
    }
    if !value.is_empty() {
        tokens.push(Token::Value(value));
    }
    tokens
}

fn main() {
    let query = r#"category:t\ \&\\ag (category1 & category2):(tag1 | tag2) word | -word2"#;
    let tokens = tokenize(query);
    println!("{:?}", tokens);

}
