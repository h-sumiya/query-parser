```toml
[dependencies]
query = { git = "https://github.com/h-sumiya/query-parser" }
```

sampel_query:`category:tag (category1 & category2):(tag1 | tag2) word | -word2`  
escape:`category:t-a-g` => `category:t\-a\-g` or `category:"t-a-g"`  
space:` ` => `" "` or `\ `  
