use std::{sync::mpsc, thread, time::Duration}; // multi-producer, single-consumer like a liver

// メッセージ受け渡しによるスレッド間データ転送
fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("I"),
            String::from("love"),
            String::from("your"),
            String::from("smile"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // rxはイテレータ
    for received in rx {
        println!("Got: {}", received);
    }
}