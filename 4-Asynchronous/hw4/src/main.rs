use std::future::Future

struct Demo;

impl Future for Demo {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        println!("Hello, world!");
        std::task::Poll::Ready(())
    }
}

fn main() {
    println!("Hello, world!");
}
