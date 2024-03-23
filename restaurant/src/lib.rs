// front_of_houseというファイルから読み込む
mod front_of_house;

// // useを使ってmoduleをスコープに持ち込む
// // あえて使用したい関数の親モジュールを読み込むことで、関数がローカルで定義されていないことを明らかにする
// use crate::front_of_house::hosting;

// // 再公開 (re-export) することで、他のモジュールからも使えるようになる
// pub use crate::front_of_house::hosting;

use front_of_house::hosting;

fn serve_order() {
    println!("serve_order");
}

mod back_of_house {
    // enumはpubをつけると全て公開される
    pub enum Appetizer {
        Soup,
        Salad,
    }

    // 構造体はpubをつけた項目のみ公開される
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // superは親モジュールと同じ階層からの相対パス
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
        // // 絶対パス
    // crate::front_of_house::hosting::add_to_waitlist();

    // // 相対パス(front_of_houseはeat_at_restaurantと同じ階層)
    // front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();

    // 夏 (summer) にライ麦 (Rye) パン付き朝食を注文
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // やっぱり別のパンにする
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);


    // 下の行のコメントを外すとコンパイルできない。食事についてくる
    // 季節のフルーツを知ることも修正することも許されていないので
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// use std::fmt::Result;
// use std::io::Result as IoResult; // asで名前を変更することができる

// fn function1() -> Result {
//     // --snip--
// }

// fn function2() -> IoResult<()> {
//     // --snip--
// }