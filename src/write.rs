use crossbeam::channel::Receiver;
use std::fs::File;
use std::io::{self, BufWriter, ErrorKind, Result, Write};

pub fn write_loop(outfile: &str, rx_from_stats: Receiver<Vec<u8>>) -> Result<()> {
    // create Generic `reader` and `writer` handles that both are the same type
    // of buffered I/O reader/writers so that we can use either based on whether or not
    // we've recieved file arguments for reader and writers

    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    loop {
        // recieve the bytes necessary from the stats thread
        let buffer: Vec<u8> = rx_from_stats.recv().unwrap();

        // let num_bytes = buffer.len();
        if buffer.is_empty() {
            break;
        }

        /*
        // IMPORTANT: performance optimization
        // we move quit to its own scope to check if we need to quit or not,
        // but we do so in a non-blocking way as holding the lock for a long
        // write would bog down other threads as well
        // the lock is automatically released once it falls out of scope.
        {
            let quit = quit.lock().unwrap();
            if *quit {
                break;
            }
        }
        */

        // we use if let to handle one specific type of error in Rust
        if let Err(e) = writer.write_all(&buffer) {
            // pipe-busting is normal, exit silently
            if e.kind() == ErrorKind::BrokenPipe {
                // cleanly stop
                return Ok(());
            }
            return Err(e);
        }
    }

    // the above could also be accomplished by using just a question mark
    // but only if we didn't want to handle specific error cases separately
    // io::stdout().write_all(&databuf[..bytes_read])?;

    Ok(())
}
