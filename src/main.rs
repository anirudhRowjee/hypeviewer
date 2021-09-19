//! HypeViewer: `pv` clone written in Rust
//!
//! Command Line Arguments:
//! <infile | optional, positional> <outfile> <silent>

use crossbeam::channel::{bounded, unbounded};
use hypeviewer::{args::Args, read, stats, write};
use std::io::Result;
use std::thread;

// the main function can return an error!
fn main() -> Result<()> {
    let args = Args::parse();

    // destructure into struct
    let Args {
        infile,
        outfile,
        silent,
    } = args;

    //? naming convention -> (transmit_to_thread, recieve_in_thread)
    let (tx_to_stats, rx_in_stats) = unbounded();
    let (tx_to_write, rx_in_write) = bounded(1024);

    // spawn all thread handlers
    let read_handle = thread::spawn(move || read::read_loop(&infile, tx_to_stats, tx_to_write));
    let stats_handle = thread::spawn(move || stats::stats_loop(silent, rx_in_stats));
    let write_handle = thread::spawn(move || write::write_loop(&outfile, rx_in_write));

    // set this up to crash main() if any single thread crashes
    // `.join()` returns a result of results
    let read_io_result = read_handle.join().unwrap();
    let stats_io_result = stats_handle.join().unwrap();
    let write_io_result = write_handle.join().unwrap();

    read_io_result?;
    stats_io_result?;
    write_io_result?;

    Ok(())
}
