// // block_on with Waker
// use std::sync::Mutex;
// use std::sync::Condvar;
// use std::sync::Arc;
// use std::task::Wake;
// use std::task::Waker;

// struct Signal {
//     state: Mutex<State>,
//     cond: Condvar,
// }

// enum State {
//     Empty,
//     Waiting,
//     Notified,
// }

// impl Signal {
//     fn new() -> Self {
//         Signal {
//             state: Mutex::new(State::Empty),
//             cond: Condvar::new(),
//         }
//     }

//     fn wait(&self) {
//         let mut state = self.state.lock().unwrap();
//         match *state {
//             State::Empty => {
//                 *state = State::Waiting;
//                 while let State::Waiting = *state {
//                     state = self.cond.wait(state).unwrap();
//                 }
//             }

//             State::Waiting => {
//                 panic!("multiple wait");
//             }

//             State::Notified => {
//                 *state = State::Empty;
//             }
//         }
//     }

//     fn notify(&self) {
//         let mut state = self.state.lock().unwrap();
//         match *state {
//             State::Empty => {
//                 *state = State::Notified;
//             }

//             State::Waiting => {
//                 *state = State::Notified;
//                 self.cond.notify_one();
//             }

//             State::Notified => {}
//         }
//     }
// }
    
// impl Wake for Signal {
//     fn wake(self: Arc<Self>) {
//         self.notify();
//     }
// }

// // #[stable(feature = "wake_trait", since = "1.51.0")]
// impl<W: Wake + Send + Sync + 'static> From<Arc<W>> for Waker {
//     fn from(waker: Arc<W>) -> Waker {
//         unsafe { Waker::from_raw(raw_waker(waker)) }
//     }
    
// }
