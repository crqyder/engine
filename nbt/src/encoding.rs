use binary::{Binary, Buffer, CString, VarI32, VarI64, VarU32, I32, I64, LE, U16};

/// There are two versions of NBT encoding that is used in Minecraft: Bedrock Edition. The first
/// one is called the NetworkLittleEndian encoding which is used mostly over the network and the
/// second encoding is called the LittleEndian encoding which is used for encoding NBT over the
/// storage and files.
pub trait Encoding {
    fn read_int(buf: &mut Buffer) -> Option<i32>;
    fn write_int(val: i32, buf: &mut Buffer);

    fn read_long(buf: &mut Buffer) -> Option<i64>;
    fn write_long(val: i64, buf: &mut Buffer);

    fn read_string(buf: &mut Buffer) -> Option<String>;
    fn write_string(val: &str, buf: &mut Buffer);
}

/// NetworkLittleEndian encoding is used for encoding NBT objects over the network and the wire. It encodes
/// the integers in variable length encoding format which optimizes bandwidth.
#[derive(Debug, Clone, Copy)]
pub struct NetworkLittleEndian;

/// LittleEndian encoding is used for encoding NBT objects for saving NBT files locally such as player world saves,
/// player data, etc.
#[derive(Debug, Clone, Copy)]
pub struct LittleEndian;

impl Encoding for NetworkLittleEndian {
    fn read_int(buf: &mut Buffer) -> Option<i32> {
        let val = VarI32::deserialize(buf)?.get();
        Some(val)
    }

    fn write_int(val: i32, buf: &mut Buffer) {
        VarI32::new(val).serialize(buf);
    }

    fn read_long(buf: &mut Buffer) -> Option<i64> {
        let val = VarI64::deserialize(buf)?.get();
        Some(val)
    }

    fn write_long(val: i64, buf: &mut Buffer) {
        VarI64::new(val).serialize(buf);
    }

    fn read_string(buf: &mut Buffer) -> Option<String> {
        let val = CString::<VarU32>::deserialize(buf)?.get();
        Some(val)
    }

    fn write_string(val: &str, buf: &mut Buffer) {
        let len = val.len() as u32;
        VarU32::new(len).serialize(buf);

        buf.write(&val.as_bytes());
    }
}

impl Encoding for LittleEndian {
    fn read_int(buf: &mut Buffer) -> Option<i32> {
        let val = I32::<LE>::deserialize(buf)?.get();
        Some(val)
    }

    fn write_int(val: i32, buf: &mut Buffer) {
        I32::<LE>::new(val).serialize(buf);
    }

    fn read_long(buf: &mut Buffer) -> Option<i64> {
        let val = I64::<LE>::deserialize(buf)?.get();
        Some(val)
    }

    fn write_long(val: i64, buf: &mut Buffer) {
        I64::<LE>::new(val).serialize(buf);
    }

    fn read_string(buf: &mut Buffer) -> Option<String> {
        let val = CString::<U16<LE>>::deserialize(buf)?.get();
        Some(val)
    }

    fn write_string(val: &str, buf: &mut Buffer) {
        let len = val.len() as u16;
        U16::<LE>::new(len).serialize(buf);

        buf.write(&val.as_bytes());
    }
}
