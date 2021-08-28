use std::fs::File;
use std::io::{self, BufWriter, ErrorKind, Write, Result};

pub fn write(outfile: &str, buffer: &[u8]) -> Result<bool>
{

    // create Generic `reader` and `writer` handles that both are the same type
    // of buffered I/O reader/writers so that we can use either based on whether or not
    // we've recieved file arguments for reader and writers

    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
        // Box::new(File::create(outfile)?)
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    // we use if let to handle one specific type of error in Rust
    if let Err(e) = writer.write_all(&buffer) {
        // pipe-busting is normal, exit silently
        if e.kind() == ErrorKind::BrokenPipe {
            return Ok(false);
        }
        return Err(e);
    }

    // the above could also be accomplished by using just a question mark
    // but only if we didn't want to handle specific error cases separately
    // io::stdout().write_all(&databuf[..bytes_read])?;

    Ok(true)
}
