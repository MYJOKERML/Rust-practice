use std::{
    future::Future,
    sync::{Arc, Condvar, Mutex},
    task::{Context, Poll, Wake, Waker},
};

#[cfg(feature = "macro")]
pub use pollster_macro::{main, test};

pub trait FutureExt: Future {
    fn block_on(self) -> Self::Output where Self: Sized { block_on(self) }
}

impl<F: Future> FutureExt for F {}

enum SignalState {
    Empty,
    Waiting,
    Notified,
}

pub struct Signal {
    state: Mutex<SignalState>,
    cond: Condvar,
}

impl Signal {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(SignalState::Empty),
            cond: Condvar::new(),
        }
    }

    pub fn wait(&self) {
        let mut state = self.state.lock().unwrap();
        match *state {
            SignalState::Notified => {
                *state = SignalState::Empty;
            }
            SignalState::Waiting => {
                panic!("Multiple threads waiting on the same signal: Open a bug report!");
            }
            SignalState::Empty => {
                *state = SignalState::Waiting;
                while let SignalState::Waiting = *state {
                    state = self.cond.wait(state).unwrap();
                }
            }
        }
    }

    pub fn notify(&self) {
        let mut state = self.state.lock().unwrap();
        match *state {
            SignalState::Notified => {}
            SignalState::Empty => {
                *state = SignalState::Notified;
            }
            SignalState::Waiting => {
                *state = SignalState::Empty;
                self.cond.notify_one();
            }
        }
    }
}

impl Wake for Signal {
    fn wake(self: Arc<Self>) {
        self.notify();
    }
}

pub fn block_on<F: Future>(mut fut: F) -> F::Output {
    let mut fut = unsafe{ std::pin::Pin::new_unchecked(&mut fut) };
    let signal = Arc::new(Signal::new());

    let waker = Waker::from(Arc::clone(&signal));
    let mut context = Context::from_waker(&waker);

    loop {
        match fut.as_mut().poll(&mut context) {
            Poll::Pending => signal.wait(),
            Poll::Ready(item) => break item,
        }
    }
}