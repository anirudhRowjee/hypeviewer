use crate::BUF_SIZE;
use crossbeam::channel::Sender;
use std::fs::File;
use std::io::{self, BufReader, Read, Result};

// declare the read function to handle all reading logic
pub fn read_loop(
    infile: &str,
    tx_to_stats: Sender<usize>,
    tx_to_write: Sender<Vec<u8>>,
) -> Result<()> {
    let mut databuf = [0; BUF_SIZE];

    let mut reader: Box<dyn Read> = if !infile.is_empty() {
        // Box is a smart pointer
        // Box<dyn Read> is a smart pointer to an Implemented function on a struct
        Box::new(BufReader::new(File::open(infile)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    // since we return a Result type, we no longer need to break - we can
    // just return an empty Vec<u8> for the errors along with e, and hence we
    // use the ? error redirection operator

    loop {
        // read all the data
        let bytes_read = match reader.read(&mut databuf) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };

        // send value to stats thread
        let _ = tx_to_stats.send(bytes_read);

        // send this buffer to the stats and writer thread, and check if it's an error
        if tx_to_write.send(Vec::from(&databuf[..bytes_read])).is_err() {
            break;
        }
    }
    // TODO send an empty buffer to the stats and writer threads

    let _ = tx_to_stats.send(0);
    let _ = tx_to_write.send(Vec::new());
    Ok(())
    // " "let _ = Sende
}
