use std::sync::Arc;

// use async_channel::Receiver;
use tokio::{
    select,
    sync::{mpsc::UnboundedReceiver, Mutex},
    task::JoinHandle,
};
use tokio_util::sync::CancellationToken;

use crate::data::Packet;

pub struct Server {
    token: CancellationToken,
    r: Arc<Mutex<UnboundedReceiver<Packet>>>,
}

impl Server {
    pub fn new(token: CancellationToken, r: Arc<Mutex<UnboundedReceiver<Packet>>>) -> Self {
        Server { token, r }
    }

    pub async fn run(&self) -> JoinHandle<()> {
        println!("running srvr");
        let pcount: Arc<Mutex<u64>> = Arc::default();
        let token = self.token.clone();
        let pcount1 = pcount.clone();
        let r1 = self.r.clone();
        tokio::spawn(async move {
            loop {
                select! {
                    _ = async {
                            // println!("...");
                            let _data = r1.lock().await.recv().await;
                            let mut lll = pcount1.lock().await;
                            *lll +=1;
                            if *lll > 200_000 {
                                println!("done!");
                                token.cancel();
                            }
                    } => {

                    }
                    _ = token.cancelled() => {
                        println!("server shutting down");
                        break;
                    }
                }
            }
            println!("Total recv: {}", pcount1.lock().await);
        })
    }
}
