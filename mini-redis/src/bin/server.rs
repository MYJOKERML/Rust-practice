#![feature(impl_trait_in_assoc_type)]

use std::net::SocketAddr;

use mini_redis::{S};

#[volo::main]
async fn main() {
    let addr: SocketAddr = "127.0.0.1:1314".parse().unwrap();
    let addr = volo::net::Address::from(addr);

    println!("Server listening on {}", addr);

    volo_gen::volo::example::ItemServiceServer::new(S::new())
        .run(addr)
        .await
        .unwrap();
}
