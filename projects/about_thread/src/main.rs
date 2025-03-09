use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn main() {
    simple_spawn();
    handle_join();
    with_variable();
    shared_memory_thread();
    shared_memory_thread2();
    message_passing();
    message_passing_bidirectional();
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

fn shared_memory_thread2() {
    println!("shared_memory_thread2");
    let mut handles = Vec::new();
    let initial_data = vec![
        (1u8, 1u8),
        (1u8, 1u8),
        (1u8, 1u8),
        (1u8, 1u8),
        (1u8, 1u8),
        (1u8, 1u8),
        (1u8, 1u8),
        (1u8, 1u8),
        (1u8, 1u8),
        (1u8, 1u8),
    ];
    let data: Arc<Mutex<Vec<(u8, u8)>>> = Arc::new(Mutex::new(initial_data.clone()));

    for x in 0..10 {
        let data_ref = data.clone();
        handles.push(thread::spawn(move || {
            println!("shared memory thread: {}", x);
            let mut data = data_ref.lock().unwrap();
            data[x] = (data[x].0 + 1u8, data[x].1 + 2u8);
        }));
    }
    for handle in handles {
        let _ = handle.join();
    }
    println!("after: {:?}", data);
}

fn message_passing() {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let data = rx.recv().unwrap();
        println!("recive data: {}", data);
    });
    let _ = tx.send("hello world");
    let _ = handle.join();
}

fn message_passing_bidirectional() {
    let mut handles = Vec::new();
    let mut data = vec![1; 10];
    let mut send_channel = Vec::new();
    let mut recv_channel = Vec::new();

    for _ in 0..10 {
        let (send_tx, send_rx) = mpsc::channel();
        let (recv_tx, recv_rx) = mpsc::channel();
        send_channel.push(send_tx);
        recv_channel.push(recv_rx);

        handles.push(thread::spawn(move || {
            let mut data = send_rx.recv().unwrap();
            data += 1;
            let _ = recv_tx.send(data);
        }));
    }

    for x in 0..10 {
        let _ = send_channel[x].send(data[x]);
    }

    for x in 0..10 {
        data[x] = recv_channel[x].recv().unwrap();
    }

    for handle in handles {
        let _ = handle.join();
    }
    eprintln!("message passing bidirectional: {:?}", data);
}
