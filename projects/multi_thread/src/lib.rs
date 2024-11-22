mod lesson;

use self::lesson::basics::run as basics_run;
use self::lesson::basics::message_passing as basics_message_passing;
use log::debug;

pub fn run() {
    debug!("start run");
    basics_run();
    basics_message_passing();
}
