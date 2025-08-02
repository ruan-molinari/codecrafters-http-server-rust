pub const HTTP_VERSION: &str = "HTTP/1.1";

// #[macro_use]
mod header;
mod method;
mod status;

pub use crate::header::*;
pub use crate::method::*;
pub use crate::status::*;
