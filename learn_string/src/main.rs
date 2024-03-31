fn main() {
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // format!はprintln!と同じように文字列を作るが、println!は値を返す
    let s = format!("{}-{}-{}", s1, s2, s3);

    let s1 = String::from("hello");
    //`String` cannot be indexed by `{integer}`
    // Stringはu8のベクタなので、例えば日本語は2バイト文字なので0番目のインデックスだけでは文字にならない
    // let h = s1[0]; 

    let hello = "Здравствуйте";

    // 文字列スライスは適切なバイトを指定しないとパニックする
    let s = &hello[0..3];
    println!("{}", s); // byte index 3 is not a char boundary; it is inside 'д' (bytes 2..4) of `Здравствуйте`

    // let s = &hello[0..1]; // thread 'main' panicked at 'byte index 0 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`', learn_string/src/main.rs:45:13
    // println!("{}", s);

    // charsを使えば人間でいう一文字づつ取り出せる
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
