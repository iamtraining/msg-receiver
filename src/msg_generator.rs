use crate::{time::my_sleep, Msg};
use tokio::select;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot::{Receiver as o_recv, Sender as o_sendr};
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
pub enum Ctrl {
	Quit,
	Health(oneshot::Sender<HealthResponse>),
}

#[derive(Debug)]
pub enum HealthResponse {
	Healthy,
	Unhealthy,
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
				my_sleep(100).await
			}
			ctrl = controller.recv() => {
				match ctrl {
					Some(Ctrl::Health(otx)) => {
						otx.send(HealthResponse::Healthy).unwrap();
					},
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
	println!("msg generator is stopped")
}
