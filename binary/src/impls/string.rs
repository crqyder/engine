use std::io::{Cursor, Read, Write};

use bytes::{Bytes, BytesMut};

use crate::{generate, Binary, Prefix};

generate!(OString, <P: Prefix>, String);
generate!(CString, <P: Prefix>, &'a str, 'a);

impl<'a, P: Prefix<'a>> Binary<'a> for OString<P> {
    fn serialize(&self, buf: &mut BytesMut) {
        P::encode(self.len(), buf);
        buf.write(&self.as_bytes());
    }

    fn deserialize(buf: &mut Cursor<&'a Bytes>) -> Option<Self> {
        let len = P::decode(buf)?;

        let mut vec = vec![0u8; len];
        buf.read(&mut vec);

        let str = String::from_utf8(vec).unwrap();
        Some(Self::new(str))
    }
}

impl<'a, P: Prefix<'a>> Binary<'a> for CString<'a, P> {
    fn serialize(&self, buf: &mut BytesMut) {
        P::encode(self.len(), buf);
        buf.write(&self.as_bytes());
    }

    fn deserialize(buf: &mut Cursor<&'a Bytes>) -> Option<Self> {
        let len = P::decode(buf)?;
        let start = buf.position() as usize;
        let end = start + len;

        let slice = &buf.get_ref()[start..end];

        let str = std::str::from_utf8(slice).unwrap();
        Some(Self::new(str))
    }
}

mod tests {
    ///
    /// Tests the serialization and deserialization of string to the buffer
    ///
    #[test]
    pub fn string() {
        use crate::{Binary, CString, W32};
        use bytes::BytesMut;
        use std::io::Cursor;

        let mut buffer = BytesMut::with_capacity(13);
        CString::<W32>::new("Hello World!").serialize(&mut buffer);

        let buffer = buffer.freeze();

        let mut reader = Cursor::new(&buffer);

        let str = CString::<W32>::deserialize(&mut reader).unwrap();
        assert_eq!(str.get(), "Hello World!");
    }
}
