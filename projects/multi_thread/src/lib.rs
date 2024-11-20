mod lesson;

use log::debug;
use self::lesson::basics::run as basics_run;

pub fn run() {
    debug!("start run");
  basics_run();
}