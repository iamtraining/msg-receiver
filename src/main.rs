#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use tokio::sync::mpsc;

mod msg;
use msg::Msg;

mod time;
use time::my_sleep;

mod msg_generator;
use msg_generator::new_generator;

mod msg_receiver;
use msg_receiver::new_receiver;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<Msg>(10);

    tokio::spawn(new_generator(tx));
    tokio::spawn(new_receiver(rx));
    my_sleep(2000).await;
}
