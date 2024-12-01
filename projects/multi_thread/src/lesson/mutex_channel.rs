use log::{debug, error, info};
use std::cell::RefCell;
use std::sync::{mpsc, Arc, Mutex};
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
        .map(|_i| {
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

pub fn thread_communication() {
    debug!("start mutex_channel::thread_communication");
    let (sender, receiver) = mpsc::channel();
    let handle = thread::spawn(move || {
        let data = "data from the other side of";
        sender.send(data).unwrap();
    });
    let received_data = receiver.recv().unwrap();
    handle.join().unwrap();
    info!(
        "mutex_channel::thread_communication receive data: {}",
        &received_data
    );
}

pub fn share_channel() {
    debug!("start mutex_channel::share_channel");
    let (sender, receiver) = mpsc::channel();
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let sender = sender.clone();
            thread::spawn(move || {
                let data = format!("data from thread: {}", i);
                sender.send(data).unwrap();
            })
        })
        .collect();

    for _ in 0..10 {
        let received_data = receiver.recv().unwrap();
        info!(
            "mutex_channel::share_channel receive data: {}",
            &received_data
        );
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

pub fn error_handling() {
    debug!("start mutex_channel::error_handling");
    let (sender, receiver) = mpsc::channel();

    let handle = thread::spawn(move || {
        let data = "data from thread in error_handling";
        if let Err(err) = sender.send(data) {
            error!("thread sending error: {:?}", err);
        }
    });

    match receiver.recv() {
        Ok(data) => info!("mutex_channel::error_handling receive data: {}", &data,),
        Err(err) => error!("thread receiving error: {:?}", err),
    }

    handle.join().unwrap();
}

pub fn thread_sync() {
    debug!("start mutex_channel::thread_sync");
    let (sender, receiver) = mpsc::channel();
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let sender = sender.clone();
            thread::spawn(move || {
                let data = format!("thread sync data: {}", i);
                sender.send(data).unwrap();
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    // ここで全てのスレッド完了を受け取る
    for _ in 0..10 {
        let data = receiver.recv().unwrap();
        info!("mutex_channel::thread_sync receive data: {}", &data);
    }
}

// ここで定義しても大丈夫
/*
thread_local! {
    static THREAD_LOCAL_DATA: RefCell<i32> = RefCell::new(0);
}
*/

pub fn thread_local_data() {
    // スレッドローカルなデータ
    thread_local! {
        static THREAD_LOCAL_DATA: RefCell<i32> = RefCell::new(0);
    }
    debug!("start mutex_channel::thread_local_data");
    THREAD_LOCAL_DATA.with(|data| {
        *data.borrow_mut() += 1;
    });

    let handle = thread::spawn(|| {
        THREAD_LOCAL_DATA.with(|data| {
            *data.borrow_mut() += 1;
        });
    });

    handle.join().unwrap();

    THREAD_LOCAL_DATA.with(|data| {
        info!(
            "mutex_channel::thread_local_data result data: {}",
            *data.borrow()
        );
    });
}
