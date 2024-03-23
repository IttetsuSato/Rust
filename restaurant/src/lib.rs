// モジュールを使うことで、関連する定義を一つにまとめる
// front_of_houseモジュールの親には暗黙的に作られたcrateモジュールがある
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // 絶対パス
    crate::front_of_house::hosting::add_to_waitlist();

    // 相対パス(front_of_houseはeat_at_restaurantと同じ階層)
    front_of_house::hosting::add_to_waitlist();
}

fn serve_order {}

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