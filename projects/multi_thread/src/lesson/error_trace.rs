use log::{debug, error, info};
use std::panic;

pub fn custom_panic_handler() {
    debug!("start error_trace::custom_panic_handler");
    panic::set_hook(Box::new(|panic_info| {
        if let Some(location) = panic_info.location() {
            error!(
                "panic occured in '{}' at {}",
                location.file(),
                location.line()
            );
        } else {
            error!("panic occured but location is unknown");
        }
        if let Some(message) = panic_info.payload().downcast_ref::<&str>() {
            error!("message: {}", message);
        } else {
            error!("panic occured but message is unknown");
        }
    }));
    panic!("this is panic");
}
