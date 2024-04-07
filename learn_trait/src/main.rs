#![allow(unused)]
fn main() {
    pub trait Summary {
        fn summarize_author(&self) -> String {
            String::from("default author")
        }
    
        fn summarize(&self) -> String {
            // "（{}さんの文章をもっと読む）"
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    // impl Summary for NewsArticle {};  // トレイトのデフォルト実装を使用

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize_author(&self) -> String{
            format!("@{}", self.username)
        }
    }

    let tweet = Tweet {
        username: String::from("ittetsu"),
        content: String::from("目黒川の桜を見ました"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("東京オリンピック"),
        location: String::from("東京"),
        author: String::from("ittetsu"),
        content: String::from("東京オリンピックが開催されました"),
    };

    println!("New article available! {}", article.summarize());
}
