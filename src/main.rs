use std::sync::atomic::AtomicU64;
use std::sync::Arc;
use std::net::SocketAddr;

use warp::Filter;

#[tokio::main]
async fn main() {
    let Args {address, counter} = Args::parse();
    let counter = Arc::new(AtomicU64::new(counter));

    warp::serve(warp::path!("counter").map(move ||{
        format!("{}", counter.clone().fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    })).run(address).await;
}

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Address to server
    #[clap(short, long, default_value = "127.0.0.1:3025")]
    address: SocketAddr,

    /// initial value of counter
    #[clap(short, long, default_value_t = 0)]
    counter: u64,
}
