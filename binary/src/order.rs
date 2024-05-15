use std::io::{Cursor, Read, Write};

use bytes::{Bytes, BytesMut};

/// ByteOrder represents a trait that is implemened by [`LE`] and [`BE`] i.e. LittleEndian
/// and BigEndian respectively. They define how bytes are ordered while transmitting data
/// over the network or storing locally.
pub trait ByteOrder {
    fn read_u16(buf: &mut Cursor<&Bytes>) -> Option<u16>;
    fn write_u16(val: u16, buf: &mut BytesMut);

    fn read_i16(buf: &mut Cursor<&Bytes>) -> Option<i16>;
    fn write_i16(val: i16, buf: &mut BytesMut);

    fn read_u24(buf: &mut Cursor<&Bytes>) -> Option<u32>;
    fn write_u24(val: u32, buf: &mut BytesMut);

    fn read_u32(buf: &mut Cursor<&Bytes>) -> Option<u32>;
    fn write_u32(val: u32, buf: &mut BytesMut);

    fn read_i32(buf: &mut Cursor<&Bytes>) -> Option<i32>;
    fn write_i32(val: i32, buf: &mut BytesMut);

    fn read_u64(buf: &mut Cursor<&Bytes>) -> Option<u64>;
    fn write_u64(val: u64, buf: &mut BytesMut);

    fn read_i64(buf: &mut Cursor<&Bytes>) -> Option<i64>;
    fn write_i64(val: i64, buf: &mut BytesMut);

    fn read_f32(buf: &mut Cursor<&Bytes>) -> Option<f32>;
    fn write_f32(val: f32, buf: &mut BytesMut);

    fn read_f64(buf: &mut Cursor<&Bytes>) -> Option<f64>;
    fn write_f64(val: f64, buf: &mut BytesMut);
}

/// LE is the little endian byte ordering in which the least significant byte is stored at the smallest
/// memory location and the most significant byte is stored in the highest memory location
#[derive(Debug, Clone, Copy)]
pub struct LE;

/// BE is the big endian byte ordering in which the most significant byte is stored at the smallest memory
/// location and the least significant byte is stored at the highest memory location.
#[derive(Debug, Clone, Copy)]
pub struct BE;

impl ByteOrder for LE {
    fn read_u16(buf: &mut Cursor<&Bytes>) -> Option<u16> {
        let mut bytes = [0_u8; 2];
        if let Ok(len) = buf.read(&mut bytes) {
            if len == 2 {
                return Some(u16::from_le_bytes(bytes));
            }
        }
        None
    }

    fn write_u16(val: u16, buf: &mut BytesMut) {
        let bytes = val.to_le_bytes();
        buf.write(&bytes);
    }

    fn read_i16(buf: &mut Cursor<&Bytes>) -> Option<i16> {
        let mut bytes = [0_u8; 2];
        if let Ok(len) = buf.read(&mut bytes) {
            if len == 2 {
                return Some(i16::from_le_bytes(bytes));
            }
        }
        None
    }

    fn write_i16(val: i16, buf: &mut BytesMut) {
        let bytes = val.to_le_bytes();
        buf.write(&bytes);
    }

    fn read_u24(buf: &mut Cursor<&Bytes>) -> Option<u32> {
        let mut bytes = [0_u8; 3];
        if let Ok(len) = buf.read(&mut bytes) {
            if len == 3 {
                return Some((bytes[0] as u32) | (bytes[1] as u32) << 8 | (bytes[2] as u32) << 16);
            }
        }
        None
    }

    fn write_u24(val: u32, buf: &mut BytesMut) {
        let bytes = [val as u8, (val >> 8) as u8, (val >> 16) as u8];
        buf.write(&bytes);
    }

    fn read_u32(buf: &mut Cursor<&Bytes>) -> Option<u32> {
        let mut bytes = [0_u8; 4];
        if let Ok(len) = buf.read(&mut bytes) {
            if len == 4 {
                return Some(u32::from_le_bytes(bytes));
            }
        }
        None
    }

    fn write_u32(val: u32, buf: &mut BytesMut) {
        let bytes = val.to_le_bytes();
        buf.write(&bytes);
    }

    fn read_i32(buf: &mut Cursor<&Bytes>) -> Option<i32> {
        let mut bytes = [0_u8; 4];
        if let Ok(len) = buf.read(&mut bytes) {
            if len == 4 {
                return Some(i32::from_le_bytes(bytes));
            }
        }
        None
    }

    fn write_i32(val: i32, buf: &mut BytesMut) {
        let bytes = val.to_le_bytes();
        buf.write(&bytes);
    }

    fn read_u64(buf: &mut Cursor<&Bytes>) -> Option<u64> {
        let mut bytes = [0_u8; 8];
        if let Ok(len) = buf.read(&mut bytes) {
            if len == 8 {
                return Some(u64::from_le_bytes(bytes));
            }
        }
        None
    }

    fn write_u64(val: u64, buf: &mut BytesMut) {
        let bytes = val.to_le_bytes();
        buf.write(&bytes);
    }

    fn read_i64(buf: &mut Cursor<&Bytes>) -> Option<i64> {
        let mut bytes = [0_u8; 8];
        if let Ok(len) = buf.read(&mut bytes) {
            if len == 8 {
                return Some(i64::from_le_bytes(bytes));
            }
        }
        None
    }

    fn write_i64(val: i64, buf: &mut BytesMut) {
        let bytes = val.to_le_bytes();
        buf.write(&bytes);
    }

    fn read_f32(buf: &mut Cursor<&Bytes>) -> Option<f32> {
        let mut bytes = [0_u8; 4];
        if let Ok(len) = buf.read(&mut bytes) {
            if len == 4 {
                return Some(f32::from_le_bytes(bytes));
            }
        }
        None
    }

    fn write_f32(val: f32, buf: &mut BytesMut) {
        let bytes = val.to_le_bytes();
        buf.write(&bytes);
    }

    fn read_f64(buf: &mut Cursor<&Bytes>) -> Option<f64> {
        let mut bytes = [0_u8; 8];
        if let Ok(len) = buf.read(&mut bytes) {
            if len == 8 {
                return Some(f64::from_le_bytes(bytes));
            }
        }
        None
    }

    fn write_f64(val: f64, buf: &mut BytesMut) {
        let bytes = val.to_le_bytes();
        buf.write(&bytes);
    }
}

impl ByteOrder for BE {
    fn read_u16(buf: &mut Cursor<&Bytes>) -> Option<u16> {
        let mut bytes = [0_u8; 2];
        if let Ok(len) = buf.read(&mut bytes) {
            if len == 2 {
                return Some(u16::from_be_bytes(bytes));
            }
        }
        None
    }

    fn write_u16(val: u16, buf: &mut BytesMut) {
        let bytes = val.to_be_bytes();
        buf.write(&bytes);
    }

    fn read_i16(buf: &mut Cursor<&Bytes>) -> Option<i16> {
        let mut bytes = [0_u8; 2];
        if let Ok(len) = buf.read(&mut bytes) {
            if len == 2 {
                return Some(i16::from_be_bytes(bytes));
            }
        }
        None
    }

    fn write_i16(val: i16, buf: &mut BytesMut) {
        let bytes = val.to_be_bytes();
        buf.write(&bytes);
    }

    fn read_u24(buf: &mut Cursor<&Bytes>) -> Option<u32> {
        let mut bytes = [0_u8; 3];
        if let Ok(len) = buf.read(&mut bytes) {
            if len == 3 {
                return Some((bytes[2] as u32) << 16 | (bytes[1] as u32) << 8 | bytes[0] as u32);
            }
        }
        None
    }

    fn write_u24(val: u32, buf: &mut BytesMut) {
        let bytes = [(val >> 16) as u8, (val >> 8) as u8, val as u8];
        buf.write(&bytes);
    }

    fn read_u32(buf: &mut Cursor<&Bytes>) -> Option<u32> {
        let mut bytes = [0_u8; 4];
        if let Ok(len) = buf.read(&mut bytes) {
            if len == 4 {
                return Some(u32::from_be_bytes(bytes));
            }
        }
        None
    }

    fn write_u32(val: u32, buf: &mut BytesMut) {
        let bytes = val.to_be_bytes();
        buf.write(&bytes);
    }

    fn read_i32(buf: &mut Cursor<&Bytes>) -> Option<i32> {
        let mut bytes = [0_u8; 4];
        if let Ok(len) = buf.read(&mut bytes) {
            if len == 4 {
                return Some(i32::from_be_bytes(bytes));
            }
        }
        None
    }

    fn write_i32(val: i32, buf: &mut BytesMut) {
        let bytes = val.to_be_bytes();
        buf.write(&bytes);
    }

    fn read_u64(buf: &mut Cursor<&Bytes>) -> Option<u64> {
        let mut bytes = [0_u8; 8];
        if let Ok(len) = buf.read(&mut bytes) {
            if len == 8 {
                return Some(u64::from_be_bytes(bytes));
            }
        }
        None
    }

    fn write_u64(val: u64, buf: &mut BytesMut) {
        let bytes = val.to_be_bytes();
        buf.write(&bytes);
    }

    fn read_i64(buf: &mut Cursor<&Bytes>) -> Option<i64> {
        let mut bytes = [0_u8; 8];
        if let Ok(len) = buf.read(&mut bytes) {
            if len == 8 {
                return Some(i64::from_be_bytes(bytes));
            }
        }
        None
    }

    fn write_i64(val: i64, buf: &mut BytesMut) {
        let bytes = val.to_be_bytes();
        buf.write(&bytes);
    }

    fn read_f32(buf: &mut Cursor<&Bytes>) -> Option<f32> {
        let mut bytes = [0_u8; 4];
        if let Ok(len) = buf.read(&mut bytes) {
            if len == 4 {
                return Some(f32::from_be_bytes(bytes));
            }
        }
        None
    }

    fn write_f32(val: f32, buf: &mut BytesMut) {
        let bytes = val.to_be_bytes();
        buf.write(&bytes);
    }

    fn read_f64(buf: &mut Cursor<&Bytes>) -> Option<f64> {
        let mut bytes = [0_u8; 8];
        if let Ok(len) = buf.read(&mut bytes) {
            if len == 8 {
                return Some(f64::from_be_bytes(bytes));
            }
        }
        None
    }

    fn write_f64(val: f64, buf: &mut BytesMut) {
        let bytes = val.to_be_bytes();
        buf.write(&bytes);
    }
}
