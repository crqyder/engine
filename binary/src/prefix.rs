use std::io::Cursor;

use bytes::{Bytes, BytesMut};

use crate::{Binary, ByteOrder, I16, I32, U16, U32, V32, W32};

/// Prefix trait is implemented for those integral and numerical types that can serialize the
/// length of a prefixed datatype such as strings, arrays, etc.
pub trait Prefix<'a>: Binary<'a> {
    fn encode(len: usize, buf: &mut BytesMut);
    fn decode(buf: &mut Cursor<&'a Bytes>) -> Option<usize>;
}

macro_rules! impl_prefix {
    ($wrapper:ident, <$($gen:ident: $gen_constraint:ident),*>, $ty:ty) => {
        impl<'a, $($gen: $gen_constraint),*> Prefix<'a> for $wrapper<$($gen),*> {
            fn encode(prefix: usize, buf: &mut BytesMut) {
                let val = prefix as $ty;
                Self::new(val).serialize(buf);
            }

            fn decode(buf: &mut Cursor<&'a Bytes>) -> Option<usize> {
                let val = Self::deserialize(buf)?.get();
                Some(val as usize)
            }
        }
    };
}

impl_prefix!(U16, <E: ByteOrder>, u16);
impl_prefix!(I16, <E: ByteOrder>, i16);
impl_prefix!(U32, <E: ByteOrder>, u32);
impl_prefix!(I32, <E: ByteOrder>, i32);
impl_prefix!(W32, <>, u32);
impl_prefix!(V32, <>, i32);
