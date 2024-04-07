// トレイト：他の型と共有出来る機能を抽象化したもの。型だけでもいいし、デフォルトの実装があってもいい。
// トレイト境界：ジェネリック型パラメータが特定のトレイトを実装していることを指定する方法

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

    pub trait Display {
        fn display(&self) -> String;
    }

    pub trait Debug {
        fn debug(&self) -> String;
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
        fn summarize_author(&self) -> String {
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

    // ------ trait boundary ----- //
    pub fn notify<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    // 複数のトレイト境界を指定する方法
    fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
        todo!()
    }

    fn some_function2<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        todo!()
    }

    // トレイトを実装している何らかの型を返す
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
    
}
