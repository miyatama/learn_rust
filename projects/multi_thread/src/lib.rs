mod lesson;

use self::lesson::basics::data_share as basics_data_share;
use self::lesson::basics::message_passing as basics_message_passing;
use self::lesson::basics::run as basics_run;
use self::lesson::basics::thread_async as basics_thread_async;
use self::lesson::basics::thread_async2 as basics_thread_async2;
use self::lesson::basics::thread_sync as basics_thread_sync;
use self::lesson::basics::thread_sync2 as basics_thread_sync2;
use self::lesson::basics::use_thread_builder as basics_use_thread_builder;
use self::lesson::error_trace::custom_panic_handler as error_trace_custom_panic_handler;
use self::lesson::error_trace::debug_output as error_trace_debug_output;
use self::lesson::fork_join::create_thread as fork_join_create_thread;
use self::lesson::fork_join::data_split as fork_join_data_split;
use self::lesson::fork_join::use_rayon as fork_join_use_rayon;
use self::lesson::mutex_channel::error_handling as mutex_channel_error_handling;
use self::lesson::mutex_channel::share_channel as mutex_channel_share_channel;
use self::lesson::mutex_channel::share_data as mutex_channel_share_data;
use self::lesson::mutex_channel::share_data_use_arc as mutex_channel_share_data_use_arc;
use self::lesson::mutex_channel::thread_communication as mutex_channel_thread_communication;
use self::lesson::mutex_channel::thread_local_data as mutex_channel_thread_local_data;
use self::lesson::mutex_channel::thread_sync as mutex_channel_thread_sync;
use self::lesson::parallel_process::dodge_dead_lock as parallel_process_dodge_dead_lock;
use self::lesson::parallel_process::performance_metric as parallel_process_performance_metric;
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
    mutex_channel_share_channel();
    mutex_channel_error_handling();
    mutex_channel_thread_sync();
    mutex_channel_thread_local_data();
}

pub fn run_scoped_thread() {
    scoped_thread_run();
}

pub fn run_error_trace() {
    error_trace_custom_panic_handler();
    error_trace_debug_output();
}

pub fn run_parallel_process() {
    parallel_process_dodge_dead_lock();
    parallel_process_performance_metric();
}

pub fn run_fork_join() {
    fork_join_create_thread();
    fork_join_data_split();
    fork_join_use_rayon();
}
