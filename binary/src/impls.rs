use crate::{Binary, Buffer, ByteOrder, Prefix};

/*
    This macro is used to generate type definitions for the specified Wrapper and also generates
    a Debug trait implementation for the Wrapper printing the inner types.
*/
#[macro_export]
macro_rules! generate {
    ($name:ident, <$($gen:ident: $gen_constraint:ident),*>, $type:ty) => {
        #[allow(non_snake_case)]
        #[derive(PartialEq)]
        pub struct $name<$($gen: $gen_constraint),*> {
            val: $type,
            $( $gen_constraint: std::marker::PhantomData<$gen>, )*
        }

        impl<$($gen: $gen_constraint),*> $name<$($gen),*> {
            pub fn new(val: $type) -> Self {
                Self {
                    val,
                    $( $gen_constraint: std::marker::PhantomData, )*
                }
            }

            pub fn get(self) -> $type {
                self.val
            }
        }

        impl<$($gen: $gen_constraint),*> AsRef<$type> for $name<$($gen),*> {
            fn as_ref(&self) -> &$type{
                &self.val
            }
        }

        impl<$($gen: $gen_constraint),*> std::fmt::Debug for $name<$($gen),*> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self.val)
            }
        }

        impl<$($gen: $gen_constraint),*> core::ops::Deref for $name<$($gen),*> {
            type Target = $type;

            fn deref(&self) -> &Self::Target{
                &self.val
            }
        }

        impl<$($gen: $gen_constraint),*> core::ops::DerefMut for $name<$($gen),*> {
            fn deref_mut(&mut self) -> &mut Self::Target{
                &mut self.val
            }
        }

        impl<$($gen: $gen_constraint),*> From<$type> for $name<$($gen),*> {
            fn from(value: $type) -> $name<$($gen),*> {
                Self::new(value)
            }
        }

        impl<$($gen: $gen_constraint),*> From<$name<$($gen),*>> for $type {
            fn from(value: $name<$($gen),*>) -> $type {
                value.get()
            }
        }
    };
}

generate!(U8, <>, u8);
generate!(I8, <>, i8);
generate!(Bool, <>, bool);
generate!(U16, <E: ByteOrder>, u16);
generate!(I16, <E: ByteOrder>, i16);
generate!(U24, <E: ByteOrder>, u32);
generate!(U32, <E: ByteOrder>, u32);
generate!(I32, <E: ByteOrder>, i32);
generate!(U64, <E: ByteOrder>, u64);
generate!(I64, <E: ByteOrder>, i64);
generate!(F32, <E: ByteOrder>, f32);
generate!(F64, <E: ByteOrder>, f64);
generate!(VarU32, <>, u32);
generate!(VarI32, <>, i32);
generate!(VarU64, <>, u64);
generate!(VarI64, <>, i64);
generate!(CString, <P: Prefix>, String);
generate!(Array, <P: Prefix, B: Binary>, Vec<B>);

impl<P: Prefix> From<&str> for CString<P> {
    fn from(value: &str) -> Self {
        CString::new(value.to_string())
    }
}

/*
    This macro is used to implement the Binary trait for unordered types that do not require the
    ByteOrder to specify the order in which they are encoded or decoded.
*/
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
                    Some(Self::new(val))
                } else {
                    None
                }
            }
        }
    };
}

impl_unordered!(U8, u8, 1);
impl_unordered!(I8, i8, 1);

/*
    This macro is used to generate the Binary trait for those types that are ordered in
    either LittleEndian or BigEndian.
*/
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

/*
    The following implementations are custom implementations of the Binary trait due to them being a
    little too complex to derive a common macro for each one of them.
*/

impl Binary for Bool {
    fn serialize(&self, buf: &mut Buffer) {
        let val = if *self.as_ref() { 0x01 } else { 0x00 };
        U8::new(val).serialize(buf);
    }

    fn deserialize(buf: &mut Buffer) -> Option<Self> {
        let val = U8::deserialize(buf)?;
        let b = match val.get() {
            0x01 => true,
            0x00 => false,
            v => panic!("Unable to deBinary the value of bool from value {}", v),
        };

        Some(Self::new(b))
    }
}

impl Binary for VarU32 {
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

        panic!("VarU32 overflow")
    }
}

impl Binary for VarI32 {
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

        panic!("VarI32 overflow")
    }
}

impl Binary for VarU64 {
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

        panic!("VarU64 overflow")
    }
}

impl Binary for VarI64 {
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

        panic!("VarI64 overflow")
    }
}

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

/*
    Only use Vec<u8> directly for Serializer when the length is not prefixed or when you are
    not sure of the length.
*/
impl Binary for Vec<u8> {
    fn serialize(&self, buf: &mut Buffer) {
        buf.write(&self);
    }

    fn deserialize(buf: &mut Buffer) -> Option<Self> {
        let mut vec = vec![0u8; buf.remaining()];
        buf.read(&mut vec);

        Some(vec)
    }
}

/*
    Only use Vec<T> directly for Serializer when the length is not prefixed or when you are
    not sure of the length.
*/
impl<T: Binary> Binary for Vec<T> {
    fn serialize(&self, buf: &mut Buffer) {
        for item in self.iter() {
            item.serialize(buf);
        }
    }

    fn deserialize(buf: &mut Buffer) -> Option<Self> {
        let mut items = Vec::new();

        while buf.remaining() != 0 {
            let item = T::deserialize(buf)?;
            items.push(item);
        }

        Some(items)
    }
}

/// TOption is a generic Option implementation in Rust where the value T gets serialised
/// when the value indicating it is encoded as true.
pub type TOption<T> = Optional<true, T>;

/// FOption is a generic Option implementation in Rust where the value T gets serialised
/// when the value indicating it is encoded as false.
pub type FOption<T> = Optional<false, T>;

/// Optional is a wrapper around the generic Option provided by Rust with support for conditional
/// serialisation of the type T: Binary depending upon whether the first value encoded is matched
/// with the one configured.
#[derive(Debug)]
pub struct Optional<const S: bool, T: Binary> {
    value: Option<T>,
}

impl<const S: bool, T: Binary> Optional<S, T> {
    pub fn get(self) -> Option<T> {
        self.value
    }
}

impl<const S: bool, T: Binary> From<Option<T>> for Optional<S, T> {
    fn from(value: Option<T>) -> Self {
        Self { value }
    }
}

impl<const S: bool, T: Binary> From<Optional<S, T>> for Option<T> {
    fn from(value: Optional<S, T>) -> Option<T> {
        value.value
    }
}

impl<const S: bool, T: Binary> Binary for Optional<S, T> {
    fn serialize(&self, buf: &mut Buffer) {
        match &self.value {
            Some(value) => {
                Bool::new(S).serialize(buf);
                value.serialize(buf);
            }
            None => Bool::new(!S).serialize(buf),
        }
    }

    fn deserialize(buf: &mut Buffer) -> Option<Self> {
        let s = Bool::deserialize(buf)?.get();

        if s == S {
            let value = T::deserialize(buf)?;
            Some(Optional { value: Some(value) })
        } else {
            Some(Optional { value: None })
        }
    }
}

mod tests {
    ///
    /// Tests the serialization and deserialization of string to the buffer
    ///
    #[test]
    pub fn string() {
        use crate::{Binary, Buffer, CString, VarU32};

        let mut buffer = Buffer::new(13);
        CString::<VarU32>::new("Hello World!".to_string()).serialize(&mut buffer);

        assert_eq!(buffer.len(), 13);
        assert_eq!(buffer.offset(), 0);

        let str = CString::<VarU32>::deserialize(&mut buffer).unwrap();

        assert_eq!(buffer.len(), 13);
        assert_eq!(buffer.offset(), 13);
        assert_eq!(str.get(), "Hello World!");
    }

    ///
    /// Tests the serialization and deserialization of Option Types
    ///
    #[test]
    pub fn option() {
        use crate::{Binary, Buffer, FOption, VarU32};

        let mut buffer = Buffer::new(6);
        let option: FOption<VarU32> = Some(100.into()).into();
        option.serialize(&mut buffer);

        assert_eq!(buffer.offset(), 0);

        let val = FOption::<VarU32>::deserialize(&mut buffer).unwrap();
        let val = val.get().unwrap();

        assert_eq!(buffer.len(), 2);
        assert_eq!(buffer.offset(), 2);
        assert_eq!(val.get(), 100);
    }
}
