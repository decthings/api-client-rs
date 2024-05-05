pub use ndarray;

#[cfg(feature = "client")]
pub mod client;

pub mod tensor;
mod varint;
