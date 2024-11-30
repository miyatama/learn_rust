use log::{debug, error, info};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn dead_lock() {
    debug!("start parallel_process::dead_lock");
    let resource1 = Arc::new(Mutex::new(()));
    let resource2 = Arc::new(Mutex::new(()));

    let resource1_clone = Arc::clone(&resource1);
    let resource2_clone = Arc::clone(&resource2);

    let handle1 = thread::spawn(move || {
        let _guard1 = resource1_clone.lock().unwrap();
        let _guard2 = resource2_clone.lock().unwrap();
        info!("parallel_process::dead_lock thread1");
    });

    let resource1_clone = Arc::clone(&resource1);
    let resource2_clone = Arc::clone(&resource2);

    let handle2 = thread::spawn(move || {
        let _guard1 = resource1_clone.lock().unwrap();
        let _guard2 = resource2_clone.lock().unwrap();
        info!("parallel_process::dead_lock thread2");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
