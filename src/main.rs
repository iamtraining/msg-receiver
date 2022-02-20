#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::time::Duration;
use tokio::sync::{mpsc, oneshot};

use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::{sleep, Sleep};

#[derive(Debug)]
enum Msg {
    Den,
    Nis,
}

fn my_sleep(ms: u64) -> Sleep {
    sleep(Duration::from_millis(ms))
}

async fn msg_generator(chan: Sender<Msg>) {
    loop {
        if let Err(err) = chan.send(Msg::Den).await {
            eprintln!("error while sending message{}", err);
            break;
        };
        match chan.send(Msg::Nis).await {
            Ok(()) => {
                println!("ok")
            }
            Err(err) => {
                eprintln!("error while sending message{}", err);
                break;
            }
        };
        my_sleep(200).await;
    }
}

async fn msg_receiver(mut chan: Receiver<Msg>) {
    while let Some(msg) = chan.recv().await {
        println!("receiver got a message: {:?}", msg);
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<Msg>(10);

    tokio::spawn(msg_generator(tx));
    tokio::spawn(msg_receiver(rx));
    my_sleep(2000).await;
}
