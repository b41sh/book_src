// trait 告诉 Rust 编译器某个特定类型拥有可能与其他类型共享的功能。
// 可以通过 trait 以一种抽象的方式定义共享的行为。
// 可以使用 trait bounds 指定泛型是任何拥有特定行为的类型。

trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }
}

#[derive(Debug)]
struct Tweet {
    username: String,
    content: String,
    replay: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} {}", self.content, self.username)
    }
}

pub fn trait_test() {
    println!("{}", "-----------trait_test--------------------");

    let news_article = NewsArticle {
        headline: String::from("Distributed system"),
        location: String::from("Beijing"),
        author: String::from("baishen"),
        content: String::from("A distributed system is a network that consists of autonomous computers that are connected using a distribution middleware."),
    };

    println!("news article summarize = {}", news_article.summarize());

    let tweet = Tweet {
        username: String::from("baishen"),
        content: String::from("A distributed system is a network that consists of autonomous computers that are connected using a distribution middleware."),
        replay: false,
        retweet: false,
    };

    println!("tweet summarize = {}", tweet.summarize());

    notify(news_article);
    notify(tweet);
}

// trait 作为参数
fn notify(item: impl Summary) {
    println!("news = {}", item.summarize());
}

// trait bound
fn notify2<T: Summary>(item: T) {
    println!("news = {}", item.summarize());
}
