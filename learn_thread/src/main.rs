use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // クロージャに使用している値の所有権を奪わせる
    let handle = thread::spawn(move || {
        // こちらがベクタ: {:?}
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
