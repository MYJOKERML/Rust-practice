mod multitask;

use multitask::spawn;

async fn demo() {
    let (tx, rx) = async_channel::bounded(1);
    spawn(demo2(tx));
    println!("Test multitask! Hello, world!");
    let _ = rx.recv().await;
}

async fn demo2(tx: async_channel::Sender<()>) {
    println!("Test multitask! Hello, world 2!");
    let _ = tx.send(()).await;
}

async fn demo_waker() {
    println!("Test Waker!");
}

fn main() {
    let future = demo();
    let w = demo_waker();
    multitask::pollster::block_on(w);
    multitask::block_on(future);
}

