//! The stats module contains the functions and structs for Statistics across the project
//!
//! # Further improvements
//! We can consider adding a GUI to display a graph.

mod timer;

use std::io::Result;
// use std::sync::mpsc::{Receiver, Sender};
use crossbeam::channel::Receiver;
use std::time::Instant;
use timer::Timer;

pub fn stats_loop(silent: bool, rx_from_stats: Receiver<usize>) -> Result<()> {
    // since this operation persists on the thread re-declare this
    let mut total_bytes = 0;

    // spawn reference timer
    let start = Instant::now();
    // spawn delta timer
    let mut timer = Timer::new();

    loop {
        // recieve buffer from the Read thread
        let bytes = rx_from_stats.recv().unwrap();

        total_bytes += bytes;

        // timer calculations
        timer.update();
        let rate_per_second = bytes as f64 / timer.delta.as_secs_f64();

        if !silent && timer.ready {
            timer.ready = false;
            eprint!(
                "\r{} {} [{:.0}b/s]",
                total_bytes,
                start.elapsed().as_secs().as_time(),
                rate_per_second
            );
        }

        // break cleanly when empty Vec<u8> sent.
        if bytes == 0 {
            break;
        }
    }

    if !silent {
        eprintln!();
    }

    Ok(())
}

/// The TimeOuput trait adds a `.as_time()` method to `u64`
///
/// # Usage
/// Here's an example on using the trait.
/// ```rust
/// use hypeviewer::stats::TimeOutput;
/// assert_eq!(65_u64.as_time(), String::from("0:01:05"));
/// ```
pub trait TimeOutput {
    fn as_time(&self) -> String;
}

impl TimeOutput for u64 {
    // renders the u64 seconds as a time string
    fn as_time(&self) -> String {
        let (hours, left) = (*self / 3600, *self % 3600);
        let (minutes, seconds) = (left / 60, left % 60);
        format!("{}:{:02}:{:02}", hours, minutes, seconds)
    }
}

// declare some tests
#[cfg(test)] // this line ensures the module only compiles when there are tests
mod tests {

    use super::TimeOutput;

    #[test]
    fn as_time_format() {
        let pairs = vec![
            (5_u64, "0:00:05"),
            (30_u64, "0:00:30"),
            (120_u64, "0:02:00"),
            (313_u64, "0:05:13"),
            (3603_u64, "1:00:03"),
        ];
        for (input, output) in pairs {
            assert_eq!(input.as_time().as_str(), output)
        }
    }
}
