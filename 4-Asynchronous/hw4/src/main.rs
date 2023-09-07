use tokio::sync::mpsc;
use std::sync::{Arc, Mutex};

struct Runtime {
    tx: mpsc::Sender<Box<dyn FnOnce() + Send>>,
}

impl Runtime {
    fn new() -> Self {
        let (tx, rx) = mpsc::channel::<Box<dyn FnOnce() + Send>>(100);
        let rt = Arc::new(Mutex::new(Runtime {
            tx,
        }));

        tokio::spawn(async move {
            while let Some(task) = rx.recv().await {
                task();
            }
        });

        Runtime {
            tx: tx.clone(),
        }
    }

    fn spawn<F>(&self, future: F)
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        let tx = self.tx.clone();
        let task = Box::new(move || {
            tokio::spawn(async move {
                future.await;
            });
        });

        tx.blocking_send(task).unwrap();
    }

    fn block_on<F>(&self, future: F)
    where
        F: std::future::Future<Output = ()>,
    {
        let mut rt = self.tx.lock().unwrap();
        let mut local = tokio::runtime::Builder::new()
            .basic_scheduler()
            .enable_all()
            .build()
            .unwrap();
        
        local.block_on(future);
    }
}

#[tokio::main]
async fn main() {
    let runtime = Arc::new(Runtime::new());

    for i in 0..10 {
        let runtime_clone = runtime.clone();
        runtime.spawn(async move {
            println!("Task {} is running.", i);
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            println!("Task {} is done.", i);
        });
    }

    runtime.block_on(async {
        println!("All tasks are completed.");
    });
}
