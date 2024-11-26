use log::{debug, info};
use std::thread;

#[derive(Debug, Clone)]
struct SharedData {
    x: i32,
}

impl SharedData {
    pub fn new(x: i32) -> SharedData {
        Self { x }
    }

    pub fn inc(&mut self) {
        self.x += 1;
    }
}

pub fn run() {
    debug!("start scoped_thread::run");

    let mut data = SharedData::new(33);
    thread::scope(|s| {
        s.spawn(|| {
            data.inc();
            info!("scoped_thread::run spawn: {:?}", data);
        });
    });
    data.inc();
    info!("scoped_thread::run result: {}", data.x);
}
