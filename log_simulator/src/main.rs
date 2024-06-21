// log_simulator/src/main.rs

use std::thread;
use std::time::Duration;

fn main() {
    let logs = [
        "INFO: Starting application",
        "DEBUG: Initializing components",
        "INFO: Application running",
        "ERROR: Something went wrong",
        "BREAKPOINT: Simulating a breakpoint",
        "DEBUG: Pausing execution",
        "INFO: Application shutting down",
    ];

    for log in &logs {
        println!("{}", log);
        thread::sleep(Duration::from_secs(1));
    }
}
