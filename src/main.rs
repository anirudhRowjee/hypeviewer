//! HypeViewer: `pv` clone written in Rust
//!
//! Command Line Arguments:
//! <infile | optional, positional> <outfile> <silent>

use std::io::Result;
use hypeviewer::{args::Args, read, stats, write};

// the main function can return an error!
fn main() -> Result<()> {


    let args = Args::parse();
    let mut total_bytes = 0;


    loop {

        // read all the data
        let buffer = match read::read(&args.infile)
        {
            Ok(x) if x.is_empty() => break,
            Ok(x) => x,
            Err(_) => break,
        };

        // gather statistics
        stats::stats(args.silent, buffer.len(), &mut total_bytes, false);

        // write to whatever output we want to
        if !write::write(&args.outfile, &buffer)? {
            // exit on error
            break;
        }

    }

    // final block of statistics
    stats::stats(args.silent, 0, &mut total_bytes, true);

    Ok(())
}
