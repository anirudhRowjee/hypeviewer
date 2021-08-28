pub mod args;
pub mod read;
pub mod write;
pub mod stats;

// constant value : persistent buffer size (16 Kb)
const BUF_SIZE: usize = 16 * 1024;
