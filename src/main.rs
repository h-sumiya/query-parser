mod node;
mod token;

fn main() {
    let query = r#"category:tag (category1 & category2):(tag1 | tag2 & tag3) word | -word2"#;
    /*
    let count = 100_000;
    let start = std::time::Instant::now();
    for _ in 0..count {
        let tokens = token::Tokens::new(query);
        let nodes = tokens.parse().unwrap();
    }
    println!("{}ms", start.elapsed().as_millis() as f64 / count as f64);
    */
    let tokens = token::Tokens::new(query);
    let nodes = tokens.parse().unwrap();
    println!("{}", nodes);
    //std::thread::sleep(std::time::Duration::from_secs(10));
}
