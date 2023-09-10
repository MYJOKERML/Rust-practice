pub mod pollster;

use pollster::Signal;
use std::{
    cell::RefCell,
    collections::VecDeque,
    sync::{Arc, Mutex},
    task::{Context, Poll, Wake, Waker},
};
use std::future::Future;
use futures::future::{BoxFuture, FutureExt};
use scoped_tls::scoped_thread_local;


scoped_thread_local!(static SIGNAL: Arc<Signal>);
scoped_thread_local!(static RUNNABLE: Mutex<VecDeque<Arc<Task>>>);

struct Task {
    future: RefCell<BoxFuture<'static, ()>>,
    signal: Arc<Signal>,
}

unsafe impl Send for Task {}
unsafe impl Sync for Task {}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        RUNNABLE.with(|runnable| runnable.lock().unwrap().push_back(self.clone()));
        self.signal.notify();
    }
}

pub fn block_on<F: Future>(future: F) -> F::Output {
    let mut fut = std::pin::pin!(future);
    let signal = Arc::new(Signal::new());
    let waker = Waker::from(signal.clone());

    let mut cx = Context::from_waker(&waker);

    let runnable = Mutex::new(VecDeque::with_capacity(1024));
    SIGNAL.set(&signal, || {
        RUNNABLE.set(&runnable, || {
            loop {
                if let Poll::Ready(output) = fut.as_mut().poll(&mut cx) {
                    return output;
                }
                while let Some(task) = runnable.lock().unwrap().pop_front() {
                    let waker = Waker::from(task.clone());
                    let mut cx = Context::from_waker(&waker);
                    let _ = task.future.borrow_mut().as_mut().poll(&mut cx);
                }
                signal.wait();
            }
        })
    })
}

pub fn spawn<F: Future<Output = ()> + 'static + std::marker::Send>(fut: F) {
    let t = Arc::new(Task {
        future: RefCell::new(fut.boxed()),
        signal: Arc::new(Signal::new()),
    });

    RUNNABLE.with(|runnable| runnable.lock().unwrap().push_back(t));
}