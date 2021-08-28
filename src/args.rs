use clap::{App, Arg};
use std::env;

// declare a struct for the Arugments
pub struct Args {
    pub infile: String,
    pub outfile: String,
    pub silent: bool
}

// IMPLement a TRAIT on the struct Args, similar to how we implement
// a method on a class
impl Args {
    pub fn parse() -> Self {
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

        // match the command line arguments
        let infile = matches
            .value_of("infile")
            .unwrap_or_default()
            .to_string();
        let outfile = matches
            .value_of("outfile")
            .unwrap_or_default()
            .to_string();

        // if the option isn't present, use the command line argument
        let silent = if matches.is_present("silent") {
            true
        } else {
            // check for the PV_SILENT environment variable
            // to ensure we're allowed to pipe the output out
            !env::var("PV_SILENT")
                // unwrap_or takes a Result type and does something on error
                .unwrap_or_default()
                .is_empty()
        };

        // return the parsed Data
        Self {
            infile, outfile, silent
        }
    }
}
