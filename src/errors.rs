use std::result;

/// Custom Result type many `procure` methods return
pub type Result<T> = result::Result<T, Error>;

/// Custom Error type returned with `procure` [`Result`](type.Result.html)'s
#[derive(Debug)]
pub enum Error {
}
