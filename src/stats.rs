pub fn stats(
    silent: bool, // to show or not to show
    num_read: usize,
    total_bytes: &mut usize, // reference to global counter
    last: bool // indicate whether this is the last output or not
)
{
    // dereference and mutate
    *total_bytes += num_read;

    if !silent {
        eprint!("\r{}", total_bytes);
        if last {
            eprintln!();
        }
    }

}
