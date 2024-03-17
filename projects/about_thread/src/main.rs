use std::sync::{Arc, Mutex, mpsc};
use std::thread;

fn main() {
    simple_spawn();
    handle_join();
    with_variable();
    shared_memory_thread();
    message_passing();
}

fn simple_spawn() {
    thread::spawn(|| {
        println!("simple spawn");
    });
}

fn handle_join() {
    let handle = thread::spawn(|| {
        println!("handle join");
    });
    let _ = handle.join();
}

fn with_variable() {
    let mut handles = Vec::new();
    for x in 0..10 {
        handles.push(thread::spawn(move || {
            println!("with variable: {}", x);
        }));
    }
    for handle in handles {
        let _ = handle.join();
    }
}

fn shared_memory_thread() {
    let mut handles = Vec::new();
    let data = Arc::new(Mutex::new(vec![1; 10]));

    for x in 0..10 {
        let data_ref = data.clone();
        handles.push(thread::spawn(move || {
            println!("shared memory thread: {}", x);
            let mut data = data_ref.lock().unwrap();
            data[x] += 1;
        }));
    }
    for handle in handles {
        let _ = handle.join();
    }
    println!("after: {:?}", data);
}

fn message_passing() {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move ||{
        let data = rx.recv().unwrap();
        println!("recive data: {}", data);
    });
    let _ = tx.send("hello world");
    let _ = handle.join();
}
