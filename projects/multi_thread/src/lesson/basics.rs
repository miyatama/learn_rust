use log::{debug, info};
use std::sync::mpsc;
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
