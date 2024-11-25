use log::{debug, error, info};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::time::{sleep, Duration};

pub fn run() {
    debug!("basics::run");
    let handle = thread::spawn(|| {
        info!("hello from thread in basics::run()");
    });
    info!("hello from basics::run()");
    handle.join().unwrap();
}

pub fn message_passing() {
    debug!("basics::message_passing");
    let (tx, rx) = mpsc::channel();
    // moveはtxの所有権を引き渡してる
    let handle = thread::spawn(move || {
        let message = String::from("new message");
        tx.send(message).unwrap();
    });
    let received = rx.recv().unwrap();
    info!("{}", received);
    handle.join().unwrap();

    let data = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        info!("moved data: {:?}", data);
    });
    // メイン側ではdataが使えない(move済み)
    // debug!("moving data? : {:?}", data);
    handle.join().unwrap();
}

pub fn data_share() {
    debug!("basics::data_share");
    let counter = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
                // panic!("help me!");
            })
        })
        .collect();

    for handle in handles {
        match handle.join() {
            Ok(_) => debug!("thread execute successfly"),
            Err(e) => debug!("thread execute panicd: {:?}", e),
        }
    }
    info!("result: {}", *counter.lock().unwrap());
}

pub fn use_thread_builder() {
    debug!("basics::use_thread_builder");
    let handle = thread::Builder::new()
        .name("great-thread".into())
        .spawn(|| {
            info!("great-thread executed!!");
        })
        .unwrap();
    handle.join().unwrap();
}

pub fn thread_sync() {
    debug!("basics::thread_sync");
    // 下記箇所でエラーとなるので本関数は利用不可
    // let counter = Mutex::clone(&counter);
    /*
    let mut counter = Mutex::new(0);
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let counter = Mutex::clone(&counter);
            thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    info!("result: {}", *counter.lock().unwrap());
    */
    error!("この関数は実行できません");
}

struct Semaphore {
    inner: Mutex<usize>,
}

impl Semaphore {
    fn new(initial: usize) -> Semaphore {
        Semaphore {
            inner: Mutex::new(initial),
        }
    }

    fn acquire(&self) {
        let mut count = self.inner.lock().unwrap();
        while *count == 0 {
            // リソース確保待ち
            drop(count);
            thread::yield_now();
            count = self.inner.lock().unwrap();
        }
        *count -= 1;
    }

    fn release(&self) {
        let mut count = self.inner.lock().unwrap();
        *count += 1;
    }
}

pub fn thread_sync2() {
    debug!("basics::thread_sync2");
    let semaphore = Arc::new(Semaphore::new(2));
    let handles: Vec<_> = (0..5)
        .map(|i| {
            let semaphore = Arc::clone(&semaphore);
            thread::spawn(move || {
                info!("thread {} waiting for resource", i);
                semaphore.acquire();
                info!("thread {} acquired resource", i);
                thread::sleep(std::time::Duration::from_secs(2));
                info!("thread {} release resource", i);
                semaphore.release();
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
}

pub async fn thread_async() {
    async_example().await;
}

async fn async_example() {
    info!("start async example");
    sleep(Duration::from_secs(2)).await;
    info!("end async example");
}
