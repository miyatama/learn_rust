use log::{debug, error, info};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn share_data() {
    debug!("start mutex_channel::share_data");
    error!("このコードは動きません。Mutex::clone()がうごきません。");
    /*
    let counter = Mutex::new(0);
    let handles: Vec<_> = (0..1)
        .map(|_| {
            let counter = Mutex::clone(&counter);
            thread::spawn(move || {
                info!("share_data thread run");
                let mut data = counter.lock().unwrap();
                *data += 1;
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    info!("share_data result: {}", *counter.lock().unwrap());
     */
}

pub fn share_data_use_arc() {
    debug!("start mutex_channel::share_data_use_arc");
    let counter = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                let mut data = counter.lock().unwrap();
                *data += 1;
                info!("share_data_use_arc thread run");
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    info!("share_data_use_arc result: {}", *counter.lock().unwrap());
}