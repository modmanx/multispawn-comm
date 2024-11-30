use std::time::Duration;
use tokio::{join, signal, sync::mpsc::unbounded_channel, time::sleep};
use tokio_util::sync::CancellationToken;

mod client;
mod data;
#[tokio::main]
async fn main() {
    let token = CancellationToken::new();

    let (s, mut r) = unbounded_channel();

    tokio::spawn({
        let cancel_token = token.clone();
        async move {
            if let Ok(()) = signal::ctrl_c().await {
                println!("received Ctrl-C, shutting down");
                cancel_token.cancel();
            }
        }
    });

    let client = client::Client::new(token.clone(), s.clone());

    let ct_sleep = token.clone();
    tokio::spawn(async move {
        println!("stopping in 2 seconds, or press ctrl-c");
        sleep(Duration::from_secs(2)).await;
        ct_sleep.cancel();
    });

    let sdsd = tokio::spawn(async move {
        client.run().await;
    });
    let mut ii = 0;
    loop {
        if token.is_cancelled() {
            break;
        }
        _ = r.recv().await;
        // println!("{:?}", d);
        ii += 1;
    }

    _ = join!(sdsd);

    println!("total: {}", ii);
}
