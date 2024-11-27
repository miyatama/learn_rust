mod lesson;

use self::lesson::basics::data_share as basics_data_share;
use self::lesson::basics::message_passing as basics_message_passing;
use self::lesson::basics::run as basics_run;
use self::lesson::basics::thread_async as basics_thread_async;
use self::lesson::basics::thread_async2 as basics_thread_async2;
use self::lesson::basics::thread_sync as basics_thread_sync;
use self::lesson::basics::thread_sync2 as basics_thread_sync2;
use self::lesson::basics::use_thread_builder as basics_use_thread_builder;
use self::lesson::mutex_channel::share_data as mutex_channel_share_data;
use self::lesson::mutex_channel::share_data_use_arc as mutex_channel_share_data_use_arc;
use self::lesson::mutex_channel::thread_communication as mutex_channel_thread_communication;
use self::lesson::scoped_thread::run as scoped_thread_run;

use log::debug;

pub fn run() {
    debug!("start run");
    basics_run();
    basics_message_passing();
    basics_data_share();
    basics_use_thread_builder();
    basics_thread_sync();
    basics_thread_sync2();
}

pub async fn run_async() {
    basics_thread_async().await;
    basics_thread_async2().await;
}

pub fn run_mutex_channel() {
    mutex_channel_share_data();
    mutex_channel_share_data_use_arc();
    mutex_channel_thread_communication();
}

pub fn run_scoped_thread() {
    scoped_thread_run();
}
