fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}


// fn largest_i32(list: &[i32]) -> i32 {
//     let mut largest = list[0];

//     // Q: .iter()とは何をする処理ですか
//     // A: listの要素を一つずつ取り出すイテレータを返します

//     // Q: イテレータとはなんですか
//     // A: イテレータは、コレクションの要素を一つずつ取り出すためのオブジェクトです
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> char {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }


fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
   assert_eq!(result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
   assert_eq!(result, 'y');
}
