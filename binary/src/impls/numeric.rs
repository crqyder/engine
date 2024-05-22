use crate::{generate, Binary, Buffer, ByteOrder};

generate!(U8, <>, u8);
generate!(I8, <>, i8);
generate!(U16, <E: ByteOrder>, u16);
generate!(I16, <E: ByteOrder>, i16);
generate!(U24, <E: ByteOrder>, u32);
generate!(U32, <E: ByteOrder>, u32);
generate!(I32, <E: ByteOrder>, i32);
generate!(U64, <E: ByteOrder>, u64);
generate!(I64, <E: ByteOrder>, i64);
generate!(F32, <E: ByteOrder>, f32);
generate!(F64, <E: ByteOrder>, f64);
generate!(W32, <>, u32);
generate!(V32, <>, i32);
generate!(W64, <>, u64);
generate!(V64, <>, i64);

macro_rules! impl_unordered {
    ($wrapper:ident, $ty:ty, $n:expr) => {
        impl Binary for $wrapper {
            fn serialize(&self, buf: &mut Buffer) {
                let val = self.as_ref();
                buf.write(&val.to_le_bytes());
            }

            fn deserialize(buf: &mut Buffer) -> Option<Self> {
                let mut bytes = [0u8; $n];

                if buf.read(&mut bytes) == $n {
                    let val = <$ty>::from_le_bytes(bytes);
                    return Some(Self::new(val));
                };

                None
            }
        }
    };
}

impl_unordered!(U8, u8, 1);
impl_unordered!(I8, i8, 1);

macro_rules! impl_ordered {
    ($wrapper:ident, $ty:ty, $read_method:ident, $write_method:ident) => {
        impl<E: ByteOrder> Binary for $wrapper<E> {
            fn serialize(&self, buf: &mut Buffer) {
                E::$write_method(*self.as_ref(), buf)
            }

            fn deserialize(buf: &mut Buffer) -> Option<Self> {
                let val = E::$read_method(buf)?;
                Some(Self::new(val))
            }
        }
    };
}

impl_ordered!(U16, u16, read_u16, write_u16);
impl_ordered!(I16, i16, read_i16, write_i16);
impl_ordered!(U24, u32, read_u24, write_u24);
impl_ordered!(U32, u32, read_u32, write_u32);
impl_ordered!(I32, i32, read_i32, write_i32);
impl_ordered!(U64, u64, read_u64, write_u64);
impl_ordered!(I64, i64, read_i64, write_i64);
impl_ordered!(F32, f32, read_f32, write_f32);
impl_ordered!(F64, f64, read_f64, write_f64);

impl Binary for W32 {
    fn serialize(&self, buf: &mut Buffer) {
        let mut u = *self.as_ref();

        while u >= 0x80 {
            U8::new((u as u8) | 0x80).serialize(buf);
            u >>= 7;
        }

        U8::new(u as u8).serialize(buf);
    }

    fn deserialize(buf: &mut Buffer) -> Option<Self> {
        let mut v: u32 = 0;

        for i in (0..35).step_by(7) {
            let b = U8::deserialize(buf)?.get();
            v |= ((b & 0x7f) as u32) << i;

            if b & 0x80 == 0 {
                return Some(Self::new(v));
            }
        }

        None
    }
}

impl Binary for V32 {
    fn serialize(&self, buf: &mut Buffer) {
        let u = *self.as_ref();
        let mut ux = (u as u32) << 1;

        if u < 0 {
            ux = !ux;
        }

        while ux >= 0x80 {
            U8::new((ux as u8) | 0x80).serialize(buf);
            ux >>= 7;
        }

        U8::new(ux as u8).serialize(buf);
    }

    fn deserialize(buf: &mut Buffer) -> Option<Self> {
        let mut ux: u32 = 0;

        for i in (0..35).step_by(7) {
            let b = U8::deserialize(buf)?.get();
            ux |= ((b & 0x7f) as u32) << i;

            if b & 0x80 == 0 {
                let mut x = (ux >> 1) as i32;
                if ux & 1 != 0 {
                    x = !x;
                }

                return Some(Self::new(x));
            }
        }

        None
    }
}

impl Binary for W64 {
    fn serialize(&self, buf: &mut Buffer) {
        let mut u = *self.as_ref();

        while u >= 0x80 {
            U8::new((u as u8) | 0x80).serialize(buf);
            u >>= 7;
        }

        U8::new(u as u8).serialize(buf);
    }

    fn deserialize(buf: &mut Buffer) -> Option<Self> {
        let mut v: u64 = 0;

        for i in (0..70).step_by(7) {
            let b = U8::deserialize(buf)?.get();
            v |= ((b & 0x7f) as u64) << i;

            if b & 0x80 == 0 {
                return Some(Self::new(v));
            }
        }

        None
    }
}

impl Binary for V64 {
    fn serialize(&self, buf: &mut Buffer) {
        let u = *self.as_ref();
        let mut ux = (u as u32) << 1;

        if u < 0 {
            ux = !ux;
        }

        while ux >= 0x80 {
            U8::new((ux as u8) | 0x80).serialize(buf);
            ux >>= 7;
        }

        U8::new(ux as u8).serialize(buf);
    }

    fn deserialize(buf: &mut Buffer) -> Option<Self> {
        let mut ux: u64 = 0;

        for i in (0..70).step_by(7) {
            let b = U8::deserialize(buf)?.get();
            ux |= ((b & 0x7f) as u64) << i;

            if b & 0x80 == 0 {
                let mut x = (ux >> 1) as i64;
                if ux & 1 != 0 {
                    x = !x;
                }

                return Some(Self::new(x));
            }
        }

        None
    }
}
