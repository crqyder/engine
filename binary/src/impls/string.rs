use crate::{generate, Binary, Buffer, Prefix};

generate!(CString, <P: Prefix>, String);

impl<P: Prefix> Binary for CString<P> {
    fn serialize(&self, buf: &mut Buffer) {
        P::encode(self.len(), buf);
        buf.write(&self.as_bytes());
    }

    fn deserialize(buf: &mut Buffer) -> Option<Self> {
        let len = P::decode(buf)?;

        let mut vec = vec![0u8; len];
        buf.read(&mut vec);

        let str = String::from_utf8(vec).unwrap();
        Some(Self::new(str))
    }
}

mod tests {
    ///
    /// Tests the serialization and deserialization of string to the buffer
    ///
    #[test]
    pub fn string() {
        use crate::{Binary, Buffer, CString, W32};

        let mut buffer = Buffer::new(0);
        CString::<W32>::new("Hello World!".to_string()).serialize(&mut buffer);

        let str = CString::<W32>::deserialize(&mut buffer).unwrap();
        assert_eq!(str.get(), "Hello World!");
    }
}
