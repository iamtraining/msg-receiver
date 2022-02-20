use crate::Msg;
use tokio::sync::mpsc::{Receiver, Sender};

pub async fn new_receiver(mut chan: Receiver<Msg>) {
	while let Some(msg) = chan.recv().await {
		println!("receiver got a message: {:?}", msg);
	}
}
