use std::io::{self, Write};

use futures::stream::select_all;
use futures::StreamExt;

fn main() {
    //test1();
    pollster::block_on(example());
}

async fn example() {
    let mut streams = vec![];

    for i in 1..=3 {
        let stream = numbers_stream(i, std::time::Duration::from_millis(i * 1000));
        streams.push(stream);
    }

    let mut s = select_all(streams);

    while let Some(i) = s.next().await {
        print!("{}", i);
        io::stdout().flush().unwrap();
        if i == 3 {
            println!("");
        }
    }
}

fn numbers_stream(
    index: u64,
    interval: std::time::Duration,
) -> futures_channel::mpsc::Receiver<u64> {
    let (mut sender, receiver) = futures_channel::mpsc::channel::<u64>(100); // Buffer size 100

    std::thread::spawn(move || {
        for c in 0..10 {
            std::thread::sleep(interval);
            if let Err(_) = sender.try_send(index) {
                break;
            }
        }
    });

    receiver
}
