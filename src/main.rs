//! HypeViewer: `pv` clone written in Rust
//!
//! Command Line Arguments:
//! <infile | optional, positional> <outfile> <silent>

extern crate clap;
use clap::{App, Arg};
use std::env;
use std::io::{self, ErrorKind, Read, Result, Write};

// constant value : persistent buffer size (16 Kb)
const BUF_SIZE: usize = 16 * 1024;

// the main function can return an error!
fn main() -> Result<()> {
    // parse command line arguments with clap
    let matches = App::new("HypeViewer")
        .arg(Arg::with_name("infile").help("Read from a file instead of stdin"))
        .arg(
            Arg::with_name("outfile")
                .short("o")
                .long("outfile")
                .takes_value(true)
                .help("Write output to a file instead of stdout"),
        )
        .arg(Arg::with_name("silent").short("s").long("silent"))
        .get_matches();

    // match the options!
    let infile = matches.value_of("infile").unwrap_or_default();
    let outfile = matches.value_of("outfile").unwrap_or_default();
    // if the option isn't present, use the command line argument
    let be_silent = if matches.is_present("silent") {
        true
    } else {
        // check for the PV_SILENT environment variable
        // to ensure we're allowed to pipe the output out
        !env::var("PV_SILENT")
            // unwrap_or takes a Result type and does something on error
            .unwrap_or_default()
            .is_empty()
    };

    // type inferred with `let`
    let mut total_bytes = 0;

    // the dbg! macro replaces print debugging with a smarter, compiler-implemented version
    // of the same thing
    // dbg!(be_silent);

    // data buffer
    let mut databuf = [0; BUF_SIZE];

    loop {
        let bytes_read = match io::stdin().read(&mut databuf) {
            // break if no bytes read at all
            Ok(0) => break,
            // if it's a non-zero value, return it
            Ok(x) => x,
            Err(_) => break,
        };

        // the dbg! macro isn't a replacement for logging, and mustn't be used
        // in place of it

        // the eprintln! macro lets us perform formatted error handling
        if !be_silent {
            eprint!("\r{}", total_bytes);
        }
        total_bytes += bytes_read;

        // writing all bytes to buffer
        // pass a slice to the write_all function

        // we use if let to handle one specific type of error in Rust
        if let Err(e) = io::stdout().write_all(&databuf[..bytes_read]) {
            // pipe-busting is normal, exit silently
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }
            return Err(e);
        }

        // the above could also be accomplished by using just a question mark
        // io::stdout().write_all(&databuf[..bytes_read])?;
    }

    Ok(())
}
