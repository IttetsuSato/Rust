use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    // 関数の結果ではなくクロージャ自体を変数に保存する
    // クロージャは最初に呼び出された時点の引数と返り値の型を持つ
    // let expensive_closure = |num| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    
    if intensity < 25 {
        println!(
            // 今日は{}回腕立て伏せをしてください！
            "Today, do {} pushups!",
            // expensive_closure(intensity)
            expensive_result.value(intensity)

        );

        println!(
            // 次に、{}回腹筋をしてください！
            "Next, do {} situps!",
            // expensive_closure(intensity)
            expensive_result.value(intensity)

        );
    } else {
        if random_number == 3 {
            // 今日は休憩してください！水分補給を忘れずに！
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                // 今日は、{}分間走ってください！
                "Today, run for {} minutes!",
                // expensive_closure(intensity)
                expensive_result.value(intensity)

            );
        }
    }
}

// 全てのクロージャは、Fn, FnMut, FnOnceのいずれかのトレイトを実装している
// メモ化用の構造体
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

// クロージャは同じスコープで定義されている変数を使用できる（関数は使用できない）
// fn main() {
//     let x = 4;

//     let equal_to_x = |z| z == x;

//     let y = 4;

//     assert!(equal_to_x(y));
// }
