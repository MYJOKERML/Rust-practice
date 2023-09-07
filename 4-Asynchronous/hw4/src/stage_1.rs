// use std::future::Future;
// use std::task::{Waker, RawWaker, RawWakerVTable, Context};
// use std::pin::Pin;
// use std::task::Poll;
// use std::time::Duration;

// mod stage_2;

// // use futures::future;

// // struct Demo;

// // impl Future for Demo {
// //     type Output = ();

// //     fn poll(self: std::pin::Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
// //         println!("Hello, world!");
// //         std::task::Poll::Ready(())
// //     }
// // }

// // simple block_on

// fn dummy_waker() -> Waker {
//     static  DATA: () = ();
//     unsafe { Waker::from_raw(RawWaker::new(&DATA, &VTABLE)) }
// }

// const VTABLE: RawWakerVTable = 
//     RawWakerVTable::new(
//         vtable_clone, vtable_wake, vtable_wake_by_ref, vtable_drop
//     );

// unsafe fn  vtable_clone(_p: *const ()) -> RawWaker {
//     RawWaker::new(_p, &VTABLE)
// }

// unsafe fn vtable_wake(_p: *const ()) {}

// unsafe fn vtable_wake_by_ref(_p: *const ()) {}

// unsafe fn vtable_drop(_p: *const ()) {}

// fn block_on<F: Future>(future: F) -> F::Output {
//     let mut fut: Pin<&mut F> = std::pin::pin!(future);
//     let waker: Waker = dummy_waker();

//     let mut cx: Context<'_> = Context::from_waker(&waker);
//     loop {
//         if let Poll::Ready(output ) = fut.as_mut().poll(&mut cx) {
//             return output;
//         }
//     }
// }

// async fn demo() {
//     let (tx, rx) = async_channel::bounded::<()>(1);
//     std::thread::spawn(move|| {
//         std::thread::sleep(Duration::from_secs(3));
//         let _ = tx.send_blocking(());
//     });
//     let _ = rx.recv().await;
//     println!("hello world!");
// }

// fn main() {
//     // let future = do_something();
//     block_on(demo());
// }
