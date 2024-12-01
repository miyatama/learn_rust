use log::{debug, info};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

pub fn dodge_dead_lock() {
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

pub fn performance_metric() {
    debug!("start parallel_process::performance_metric");
    let start_time = Instant::now();

    let _ = (0..10).map(|x| {
        info!("wait time: {}", x);
    });

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    info!("elapsed time: {:?}", elapsed_time);
}
