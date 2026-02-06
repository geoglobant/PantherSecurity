//! Adapters live here (FFI, HTTP, storage, crypto, etc.).
//! Implementations will depend on platform targets and will be added incrementally.

pub mod ffi;
pub mod serialization;
pub mod http;
