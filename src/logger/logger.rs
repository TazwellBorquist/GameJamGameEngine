use std::fs::{ File, OpenOptions };
use std::io::Write as _;
use std::thread;

use std::sync::Arc;
use std::sync::atomic::
    { AtomicBool, Ordering::Relaxed };
use std::sync::mpsc::
    { Sender, channel };


use std::time::
    {Instant, Duration};

/// Formats a duration into "[hh:mm:ss]".
/// The hour section expands as needed
fn _log_time_stamp(d: Duration) -> String {
    let hours = d.as_secs() / (60 * 60);
    let minutes = (d.as_secs() % (60 * 60)) / 60;
    let seconds = d.as_secs() % 60;
    format!("[{:02}:{:02}:{:02}]", hours, minutes, seconds)
}

/// Async logger uses an mpsc channel to hand strings to a logging thread
/// The strings are then written to a log file with the format:
/// `[hh:mm:ss] log message`
pub struct AsyncLogger {
    is_log_running: Arc<AtomicBool>,
    log_thread: Option<thread::JoinHandle<()>>,
    send: Sender<String>,
}

impl AsyncLogger {
    pub fn new() -> Self {
        let log_file: File = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("./latest.log")
            .expect("failed to open logger");

        let is_running = Arc::new(AtomicBool::new(true));
        let is_running_thread = is_running.clone();

        let (send, recv) = channel::<String>();

        let joinhandle = thread::spawn( move || {
            let start_time = Instant::now();
            'logging: loop {
                // match is_running and maybe a message
                let _ = match (is_running_thread.load(Relaxed), recv.try_recv()) {
                    // message recieved
                    (_,     Ok(raw_msg ) ) =>  {
                        let msg = format!(
                            "{} {}",
                            _log_time_stamp(Instant::now().duration_since(start_time)),
                            raw_msg);
                            match writeln!(&log_file, "{}", msg.as_str()) {
                                Ok(_) => (),
                                Err(x) => println!("{}", x),
                            };
                    },

                    // log_thread no longer running
                    (false,          Err(_) ) => break 'logging,

                    // nothing to do
                    (true,                _ ) => continue,
                };
            }
        } );
        
        Self {
            is_log_running: is_running,
            log_thread: Some(joinhandle),
            send: send,
        }
    }

    pub fn log(self: &Self, s: String) -> () {
        match self.send.clone().send(s) {
            Err(x) => println!("Failed to log: {x}"),
            _ => ()
        };
    }

    pub fn init(self: &Self) -> () {
        self.log("Started Logger!".to_string())
    }

    pub fn close(self: &mut Self) -> () {
        self.log("Stopping Logger!".to_string());
        let _ = self.is_log_running.swap(false, Relaxed);
        let _ = self.log_thread.take().unwrap().join();
    }
}