use log::{debug, info};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

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
    let handle = thread::Builder::new().name("great-thread".into()).spawn(||{
        info!("great-thread executed!!");
    }).unwrap();
    handle.join().unwrap();
}