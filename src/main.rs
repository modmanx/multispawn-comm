use std::sync::Arc;

// use async_channel::{Receiver, Sender};
use tokio::{
    join, signal,
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver},
        Mutex,
    },
};
use tokio_util::sync::CancellationToken;

mod client;
mod data;
mod srvr;

#[tokio::main]
async fn main() {
    let token = CancellationToken::new();

    // let (s: Sender<data::Packet>, r: Receiver<data::Packet>) = async_channel::unbounded();
    let (s, r) = unbounded_channel();

    tokio::spawn({
        let cancel_token = token.clone();
        async move {
            if let Ok(()) = signal::ctrl_c().await {
                println!("received Ctrl-C, shutting down");
                cancel_token.cancel();
            }
        }
    });
    let xr = Arc::new(Mutex::new(r));
    let srvr = srvr::Server::new(token.clone(), xr.clone());
    let client = client::Client::new(token.clone(), s.clone());

    let join_result = join!(
        tokio::spawn(async move {
            srvr.run().await;
        }),
        tokio::spawn(async move {
            client.run().await;
        }),
    );

    println!("join result: {:?}", join_result);
}
