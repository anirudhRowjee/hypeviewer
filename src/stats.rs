use std::io::Result;
// use std::sync::mpsc::{Receiver, Sender};
use crossbeam::channel::Receiver;

pub fn stats_loop(silent: bool, rx_from_stats: Receiver<usize>) -> Result<()> {
    // since this operation persists on the thread re-declare this
    let mut total_bytes = 0;
    loop {
        // recieve buffer from the Read thread
        let bytes = rx_from_stats.recv().unwrap();

        // let num_bytes = bytes.len();

        total_bytes += bytes;

        if !silent {
            eprint!("\r{}", total_bytes);
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
