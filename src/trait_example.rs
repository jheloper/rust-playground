pub fn example_trait() {
    let news_article = NewsArticle {
        headline: String::from("headline!"),
        location: String::from("New York"),
        author: String::from("Tom"),
        content: String::from("this is news article!")
    };

    println!("news article summary is {}", news_article.summary());
}

trait Summarizable {
    fn summary(&self) -> String;
}

struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}