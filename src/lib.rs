//!Simple HTTP client with built-in HTTPS support.
//!Currently it's in heavy development and may frequently change.
pub mod error;
pub mod request;
pub mod response;
// pub mod tls;
pub mod uri;

mod chunked;
