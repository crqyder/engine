use std::fmt::Debug;

pub mod order;
pub use order::*;

pub mod prefix;
pub use prefix::*;

pub mod impls;
pub use impls::*;

pub mod buffer;
pub use buffer::*;

/// Binary represents a trait that is implemented for all the objects that can be serialized
/// and deserialized over the network.
pub trait Binary: Sized + Debug {
    fn serialize(&self, buf: &mut Buffer);
    fn deserialize(buf: &mut Buffer) -> Option<Self>;
}
