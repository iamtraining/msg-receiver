use crate::{time::my_sleep, Msg};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{mpsc, oneshot};

pub async fn new_generator(chan: Sender<Msg>) {
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
