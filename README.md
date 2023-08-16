```toml
[dependencies]
query = { git = "https://github.com/h-sumiya/query-parser" }
```

```rust
use query::Tokens;

fn main() {
    let query = r#"category:tag (category1 & category2):(tag1 | tag2 & tag3) word | -word2"#;
    let tokens = Tokens::new(query);
    let nodes = tokens.parse().unwrap();
    println!("{}", nodes);
}
```
