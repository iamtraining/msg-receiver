#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use failure::Fallible;
use tokio::sync::{mpsc, oneshot};

mod msg;
use msg::Msg;

mod time;
use time::my_sleep;

mod msg_generator;
use msg_generator as msggen;

mod msg_receiver;
use msg_receiver as msgrecv;

#[tokio::main]
async fn main() -> Fallible<()> {
    let (tx, rx) = mpsc::channel::<Msg>(10);
    let (ctx, crx) = mpsc::channel::<msggen::Ctrl>(10);
    let (otx, orx) = oneshot::channel::<msggen::HealthResponse>();

    tokio::spawn(msggen::new_generator(tx, crx));
    tokio::spawn(msgrecv::new_receiver(rx));

    my_sleep(2000).await; // печатать сообщения

    println!("sending health mes sage");
    ctx.send(msggen::Ctrl::Health(otx)).await?; // здоровье
    println!("health message have been sent");

    let response = orx.await?;
    println!("received HealthResponse {:?}", response);

    my_sleep(2000).await; // еще печатать сообщение

    println!("sending quit message");
    ctx.send(msggen::Ctrl::Quit).await?; // выйти
    println!("quit message have been sent");

    my_sleep(2000).await; // не печатать сообщения

    Ok(())
}
