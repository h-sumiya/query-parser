mod node;
mod token;

fn main() {
    let query = r#"category:t\ \&\\ag (category1 & category2):(tag1 | tag2) word | -word"2""#;
    let tokens = token::Tokens::new(query);
    println!("{}", tokens);
}
