use crate::BUF_SIZE;
use std::fs::File;
use std::io::{self, BufReader, Read, Result};

// declare the read function to handle all reading logic
pub fn read(infile: &str) -> Result<Vec<u8>>
{

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

    let bytes_read = reader.read(&mut databuf)?; 

    Ok(Vec::from(&databuf[..bytes_read]))
}

