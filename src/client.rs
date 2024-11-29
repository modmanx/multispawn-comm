use tokio::{select, sync::mpsc::UnboundedSender};
use tokio_util::sync::CancellationToken;

use crate::data::Packet;

pub struct Client {
    token: CancellationToken,
    s: UnboundedSender<Packet>,
}

impl Client {
    pub fn new(token: CancellationToken, s: UnboundedSender<Packet>) -> Self {
        Client { token, s }
    }
    pub async fn run(&self) {
        println!("running client");
        let token1 = self.token.clone();
        let s1 = self.s.clone();
        let t1 = tokio::spawn(async move {
            loop {
                select! {
                    _ = async {
                        let _ = s1.send(Packet{ data_str: String::from("hello from client 1") });
                    } => {

                    }
                    _ = token1.cancelled() => {
                        println!("client shutting down");
                        break;
                    }
                }
            }
        });

        let token2 = self.token.clone();
        let s2 = self.s.clone();
        let t2 = tokio::spawn(async move {
            loop {
                select! {
                    _ = async {
                        let _ = s2.send(Packet{ data_str: String::from("hello from client 2") });
                    } => {

                    }
                    _ = token2.cancelled() => {
                        println!("client shutting down");
                        break;
                    }
                }
            }
        });

        let _ = tokio::join!(t1, t2);
    }
}
