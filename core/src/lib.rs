pub mod pb;
pub mod rpc;

/// Helpers to deal with block sources.
pub mod block_view;

pub use event::Event;
pub use function::Function;

mod abi;
mod event;
mod externs;
mod function;
pub mod scalar;

/// Represents the null address static array in bytes (20 bytes) which in hex is equivalent
/// to:
///
/// ```text
/// 0000000000000000000000000000000000000000
/// ```
pub const NULL_ADDRESS: [u8; 20] = [
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8,
];
