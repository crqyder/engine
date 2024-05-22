use crate::{generate, Binary, Buffer, Prefix};

generate!(Array, <P: Prefix, B: Binary>, Vec<B>);
generate!(RemBuf, <>, Vec<u8>);

impl<P: Prefix, B: Binary> Binary for Array<P, B> {
    fn serialize(&self, buf: &mut Buffer) {
        P::encode(self.len(), buf);

        for val in self.iter() {
            val.serialize(buf);
        }
    }

    fn deserialize(buf: &mut Buffer) -> Option<Self> {
        let len = P::decode(buf)?;
        let mut vec = Vec::with_capacity(len);

        for _ in 0..len {
            vec.push(B::deserialize(buf)?);
        }

        Some(Self::new(vec))
    }
}

impl<P: Prefix> Binary for RemBuf<P> {
    fn serialize(&self, buf: &mut Buffer) {
        P::encode(self.len(), buf);
        buf.write(&self);
    }

    fn deserialize(buf: &mut Buffer) -> Option<Self> {
        let len = P::decode(buf)?;
        let start = buf.position() as usize;
        let end = start + len;

        let slice = &buf.get_ref()[start..end];

        Some(Self::new(slice))
    }
}
