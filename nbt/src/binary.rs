use super::Encoding;
use binary::{generate, Binary, Buffer, U8};
use engine_api::nbt::{Compound, List, Tag, NBT};

// This macro generates the RootNBT object which contains a wrapper around
// a NBT object with the specified encoding.
generate!(RootNBT, <E: Encoding>, NBT);

impl<E: Encoding> Binary for RootNBT<E> {
    fn serialize(&self, buf: &mut Buffer) {
        serialize_tag(self.val.tag(), buf);
        E::write_string("", buf);
        encode::<E>(&self.val, buf);
    }

    fn deserialize(buf: &'a mut Buffer) -> Option<Self> {
        let tag = deserialize_tag(buf)?;
        E::read_string(buf)?;
        let val = decode::<E>(tag, buf)?;

        Some(Self::new(val))
    }
}

#[inline]
fn serialize_tag(tag: Tag, buf: &mut Buffer) {
    let val = tag as u8;
    U8::new(val).serialize(buf);
}

#[inline]
fn deserialize_tag(buf: &mut Buffer) -> Option<Tag> {
    let byte = U8::deserialize(buf)?.get();
    let tag = Tag::from_byte(byte)?;

    Some(tag)
}

/// This function encodes the provided NBT object into the specified buffer.
fn encode<E: Encoding>(nbt: &NBT, buf: &mut Buffer) {
    match nbt {
        NBT::Byte(v) => {
            buf.write(&v.to_le_bytes());
        }
        NBT::Short(v) => {
            buf.write(&v.to_le_bytes());
        }
        NBT::Int(v) => E::write_int(*v, buf),
        NBT::Long(v) => E::write_long(*v, buf),
        NBT::Float(v) => {
            buf.write(&v.to_le_bytes());
        }
        NBT::Double(v) => {
            buf.write(&v.to_le_bytes());
        }
        NBT::ByteArray(v) => {
            E::write_int(v.len() as i32, buf);

            unsafe {
                let vec: &Vec<u8> = std::mem::transmute(v);
                buf.write(&vec);
            }
        }
        NBT::String(v) => E::write_string(v, buf),
        NBT::List(v) => {
            serialize_tag(v.tag(), buf);

            E::write_int(v.len() as i32, buf);

            for item in v.iter() {
                encode::<E>(item, buf);
            }
        }
        NBT::Compound(v) => {
            for (name, item) in v.iter() {
                serialize_tag(item.tag(), buf); // TypeID of the NBT object
                E::write_string(name, buf); // Name of the NBT object
                encode::<E>(item, buf); // The NBT object encoded
            }

            serialize_tag(Tag::End, buf) // Tag End to signify end of Compound
        }
        NBT::IntArray(v) => {
            E::write_int(v.len() as i32, buf);

            for item in v.iter() {
                E::write_int(*item, buf);
            }
        }
        NBT::LongArray(v) => {
            E::write_int(v.len() as i32, buf);

            for item in v.iter() {
                E::write_long(*item, buf);
            }
        }
    }
}

/// This function decodes the NBT object with the specified Tag from the buffer and returns it
/// if successful.
fn decode<E: Encoding>(id: Tag, buf: &mut Buffer) -> Option<NBT> {
    match id {
        Tag::End => None,
        Tag::Byte => {
            let mut data = [0u8; 1];
            buf.read(&mut data);

            Some(NBT::Byte(i8::from_le_bytes(data)))
        }
        Tag::Short => {
            let mut data = [0u8; 2];
            buf.read(&mut data);

            Some(NBT::Short(i16::from_le_bytes(data)))
        }
        Tag::Int => {
            let val = E::read_int(buf)?;
            Some(NBT::Int(val))
        }
        Tag::Long => {
            let val = E::read_long(buf)?;
            Some(NBT::Long(val))
        }
        Tag::Float => {
            let mut data = [0u8; 4];
            buf.read(&mut data);

            Some(NBT::Float(f32::from_le_bytes(data)))
        }
        Tag::Double => {
            let mut data = [0u8; 8];
            buf.read(&mut data);

            Some(NBT::Double(f64::from_le_bytes(data)))
        }
        Tag::ByteArray => {
            let len = E::read_int(buf)? as usize;
            let mut array = vec![0u8; len];

            buf.read(&mut array);

            unsafe {
                let val: Vec<i8> = std::mem::transmute(array);
                Some(NBT::ByteArray(val))
            }
        }
        Tag::String => {
            let string = E::read_string(buf)?;
            Some(NBT::String(string))
        }
        Tag::List => {
            let list_type = deserialize_tag(buf)?;
            let mut len = E::read_int(buf)?;

            if list_type == Tag::End {
                len = 0;
            }

            let mut list = List::with_capacity(list_type, len as usize);

            for _ in 0..len {
                if let Some(element) = decode::<E>(list_type, buf) {
                    list.push(element);
                } else {
                    return None;
                }
            }

            Some(NBT::List(list))
        }
        Tag::Compound => {
            let mut compound = Compound::new();

            loop {
                let tag = deserialize_tag(buf)?;

                // We encountered the end of a compound tag. Break the loop.
                if tag == Tag::End {
                    break;
                }

                let name = E::read_string(buf)?;

                if let Some(value) = decode::<E>(tag, buf) {
                    compound.put(&name, value);
                } else {
                    return None;
                }
            }

            Some(NBT::Compound(compound))
        }
        Tag::IntArray => {
            let len = E::read_int(buf)?;
            let mut array = Vec::with_capacity(len as usize);

            for _ in 0..len {
                let data = E::read_int(buf)?;
                array.push(data);
            }

            Some(NBT::IntArray(array))
        }
        Tag::LongArray => {
            let len = E::read_int(buf)?;
            let mut array = Vec::with_capacity(len as usize);

            for _ in 0..len {
                let data = E::read_long(buf)?;
                array.push(data);
            }

            Some(NBT::LongArray(array))
        }
    }
}
