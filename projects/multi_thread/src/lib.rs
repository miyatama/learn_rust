mod lesson;

use self::lesson::basics::data_share as basics_data_share;
use self::lesson::basics::message_passing as basics_message_passing;
use self::lesson::basics::run as basics_run;
use log::debug;

pub fn run() {
    debug!("start run");
    basics_run();
    basics_message_passing();
    basics_data_share();
}
