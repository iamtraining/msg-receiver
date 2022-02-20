use crate::{time::my_sleep, Msg};
use tokio::select;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
pub enum Ctrl {
	Quit,
}

pub async fn new_generator(chan: Sender<Msg>, mut controller: Receiver<Ctrl>) {
	loop {
		tokio::select! {
			msg = chan.send(Msg::Den) => {
				match msg {
					Ok(()) => my_sleep(100).await,
					Err(err) => {
						eprintln!("error while sending the message. error {}", err);
						break;
					}
				}
			}
			msg = chan.send(Msg::Nis) => {
				if let Err(err) = msg {
					eprintln!("error while sending the message. error {}", err);
					break;
				}
			}
			ctrl = controller.recv() => {
				match ctrl {
					Some(Ctrl::Quit) => {
						break
						println!("got a quit message");
					},
					None => {
						break
						println!("None");
					}, // all senders have dropped
				}
			}
		}
	}
}
