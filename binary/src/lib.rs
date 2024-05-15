#[allow(hidden_glob_reexports)]
use bytes::{Bytes, BytesMut};
use std::{fmt::Debug, io::Cursor};

pub mod order;
pub use order::*;

pub mod prefix;
pub use prefix::*;

pub mod impls;
pub use impls::*;

/// Binary represents a trait that is implemented for all the objects that can be serialized
/// and deserialized over the network.
pub trait Binary<'a>: Sized + Debug {
    fn serialize(&self, buf: &mut BytesMut);
    fn deserialize(buf: &mut Cursor<&'a Bytes>) -> Option<Self>;
}
