use log::{debug, info};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
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

pub fn data_split() {
    debug!("start fork_join::data_split");
    let data = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5]));
    let handle1 = thread::spawn({
        let data = Arc::clone(&data);
        move || {
            let mut data = data.lock().unwrap();
            for elem in data.iter_mut() {
                *elem *= 2;
            }
        }
    });
    let handle2 = thread::spawn({
        let data = Arc::clone(&data);
        move || {
            let mut data = data.lock().unwrap();
            for elem in data.iter_mut() {
                *elem += 1;
            }
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let final_result = data.lock().unwrap().clone();
    info!("result: {:?}", final_result);
}

pub fn use_rayon() {
    debug!("start fork_join::use_rayon");
    let mut data = vec![1, 2, 3, 4, 5];
    data.par_iter_mut().for_each(|elem| {
        *elem *= 2;
    });
    info!("result: {:?}", data);
}

pub fn spilt_and_join() {
    debug!("start fork_join::spilt_and_join");
    let data = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    let handle1 = thread::spawn({
        let data = Arc::clone(&data);
        move || {
            let data = data.lock().unwrap();
            let sum: i32 = data[..5].iter().sum();
            sum
        }
    });
    let handle2 = thread::spawn({
        let data = Arc::clone(&data);
        move || {
            let data = data.lock().unwrap();
            let sum: i32 = data[5..].iter().sum();
            sum
        }
    });
    let result1 = handle1.join().unwrap();
    let result2 = handle2.join().unwrap();
    let final_result = result1 + result2;
    info!("result: {:?}", final_result);
}

/**
 * FnOnce: 一回のみ呼び出し可能
 * FnMut: 可変借用
 * Fn: 不変
 */
pub fn closure_and_capture() {
    debug!("start fork_join::closure_and_capture");
    let x = 10;

    // FnOnce
    let consume_x = || {
        debug!("Consume x: {}", x);
    };

    // FnMut
    let mut_x = || {
        debug!("mutable borrow of x: {}", x);
    };

    // Fn
    let immut_x = || {
        debug!("immutable borrow of x: {}", x);
    };

    consume_x();
    mut_x();
    immut_x();
}
