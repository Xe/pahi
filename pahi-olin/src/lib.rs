use std::io::prelude::*;

pub mod error;

/// Process is an individual CommonWA process. It is the collection of resources
/// and other macguffins that the child module ends up requiring.
pub struct Process<'a, 'b, 'c> {
    name: String,

    stdin: &'a dyn Read,
    stdout: &'b dyn Write,
    stderr: &'c dyn Write,
}
