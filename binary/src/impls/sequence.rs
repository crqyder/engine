use std::io::{Cursor, Write};

use bytes::{Bytes, BytesMut};

use crate::{generate, Binary, Prefix};

generate!(Array, <P: Prefix, B: Binary>, Vec<B>);
generate!(Slice, <P: Prefix, B: Binary>, &'a [B], 'a);
generate!(RefBytes, <P: Prefix>, &'a [u8], 'a);

impl<'a, P: Prefix<'a>, B: Binary<'a>> Binary<'a> for Array<P, B> {
    fn serialize(&self, buf: &mut BytesMut) {
        P::encode(self.len(), buf);

        for val in self.iter() {
            val.serialize(buf);
        }
    }

    fn deserialize(buf: &mut Cursor<&'a Bytes>) -> Option<Self> {
        let len = P::decode(buf)?;
        let mut vec = Vec::with_capacity(len);

        for _ in 0..len {
            vec.push(B::deserialize(buf)?);
        }

        Some(Self::new(vec))
    }
}

impl<'a, P: Prefix<'a>> Binary<'a> for RefBytes<'a, P> {
    fn serialize(&self, buf: &mut BytesMut) {
        P::encode(self.len(), buf);
        buf.write(&self);
    }

    fn deserialize(buf: &mut Cursor<&'a Bytes>) -> Option<Self> {
        let len = P::decode(buf)?;
        let start = buf.position() as usize;
        let end = start + len;

        let slice = &buf.get_ref()[start..end];

        Some(Self::new(slice))
    }
}
