//! Hypeviewer - A simple clone of `pipeviewer`

pub mod args;
pub mod read;
pub mod stats;
pub mod write;

// constant value : persistent buffer size (16 Kb)
const BUF_SIZE: usize = 16 * 1024;
