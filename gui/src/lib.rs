pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String
}

impl Draw for Button {
    fn draw(&self) {
        // 描画ロジック
    }
}

// トレイトオブジェクト = 共通の振る舞いを抽象化する目的
pub trait Draw {
    fn draw(&self);
}

// Screenは、実際に描画するコンポーネントがどんな型かを知る必要はない
// drawメソッドを持っていたいらそれはコンポーネントとみなす（ダックタイピング）
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// use gui::{Screen, Button};

// fn main() {
//     let screen = Screen {
//         components: vec![
//             Box::new(SelectBox {
//                 width: 75,
//                 height: 10,
//                 options: vec![
//                     // はい
//                     String::from("Yes"),
//                     // 多分
//                     String::from("Maybe"),
//                     // いいえ
//                     String::from("No")
//                 ],
//             }),
//             Box::new(Button {
//                 width: 50,
//                 height: 10,
//                 // 了解
//                 label: String::from("OK"),
//             }),
//         ],
//     };

//     screen.run();
// }