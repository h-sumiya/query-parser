mod node;
mod token;

fn main() {
    let query = r#"category:tag (category1 & category2):(tag1 | tag2 & tag3) word | -word2"#;
    let tokens = token::Tokens::new(query);
    let nodes = tokens.parse().unwrap();
    println!("{}", nodes);
}
