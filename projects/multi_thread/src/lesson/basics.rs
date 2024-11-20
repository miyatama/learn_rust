use std::thread;
use log::debug;

pub fn run() {
    let handle = thread::spawn(|| {
        debug!("hello from thread in basics::run()");
    });
    debug!("hello from basics::run()");
    handle.join().unwrap();
}