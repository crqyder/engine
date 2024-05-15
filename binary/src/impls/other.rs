use std::io::Cursor;

use bytes::{Bytes, BytesMut};

use crate::{generate, Binary, U8};

generate!(Bool, <>, bool);

impl Binary<'_> for Bool {
    fn serialize(&self, buf: &mut BytesMut) {
        let val = if *self.as_ref() { 0x01 } else { 0x00 };
        U8::new(val).serialize(buf);
    }

    fn deserialize(buf: &mut Cursor<&Bytes>) -> Option<Self> {
        let val = U8::deserialize(buf)?;
        let b = match val.get() {
            0x01 => true,
            0x00 => false,
            v => panic!("Unable to deBinary the value of bool from value {}", v),
        };

        Some(Self::new(b))
    }
}
