use log::{debug, error, info};
use std::thread;

pub fn create_thread() {
    debug!("start fork_join::create_thread ");
    // sub tasks
    let handle1 = thread::spawn(|| {
        info!("sub task1 work");
    });
    let handle2 = thread::spawn(|| {
        info!("sub task2 work");
    });

    // run thread
    handle1.join().unwrap();
    handle2.join().unwrap();

    // any summary
}
