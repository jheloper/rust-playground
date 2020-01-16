pub fn example_trait() {
    let news_article = NewsArticle {
        headline: String::from("headline!"),
        location: String::from("New York"),
        author: String::from("Tom"),
        content: String::from("this is news article!")
    };

    println!("news article summary is {}", news_article.summary());

    let tweet = Tweet {
        username: String::from("Peter"),
        content: String::from("this is tweet content!"),
        reply: false,
        retweet: false
    };

    println!("tweet summary is {}", tweet.summary());

    let feed = Feed {
        username: String::from("John"),
        content: String::from("this is feed content!")
    };

    println!("feed summary is {}", feed.summary());
}

trait Summarizable {
    fn author_summary(&self) -> String;

    fn summary(&self) -> String {
        format!("(Read more from {}...)", self.author_summary())
    }
}

struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn author_summary(&self) -> String {
        format!("by {}", self.author)
    }
}

struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn author_summary(&self) -> String {
        format!("@{}", self.username)
    }
}

struct Feed {
    pub username: String,
    pub content: String
}

impl Summarizable for Feed {
    fn author_summary(&self) -> String {
        format!("by {}", self.username)
    }
}