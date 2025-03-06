use std::sync::LazyLock;
use std::sync::Mutex;

mod logger;

pub static LOGGER: LazyLock<Mutex<logger::AsyncLogger>> = LazyLock::new(
    || Mutex::new(logger::AsyncLogger::new())
);

#[macro_export]
macro_rules! logger_init {
    () => {
        crate::logger::LOGGER.lock().unwrap().init()
    };
}

#[macro_export]
macro_rules! logger_close {
    () => {
        crate::logger::LOGGER.lock().unwrap().close()
    };
}

#[macro_export]
macro_rules! log {
    ( $l:literal ) => {
        crate::logger::LOGGER.lock().unwrap().log(stringify!($l).to_string())
    };
    ( $p:expr, $( $arg:expr ),+ ) => {
        crate::logger::LOGGER.lock().unwrap().log(format!($p, $($arg),+))
    }
}