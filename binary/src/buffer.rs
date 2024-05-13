use std::{
    alloc::{self, Layout},
    ops::{Deref, DerefMut},
    ptr,
};

/// Buffer is an implementation of fast and zero overhead Buffer which
/// can be used for serializing and deserializing binary datatypes over
/// the wire with support for simultaneous reading and writing.
pub struct Buffer {
    layout: Layout,
    ptr: *mut u8,

    cap: usize,
    len: usize,
    offset: usize,
}

impl Buffer {
    /// Creates and returns a new Buffer of the specified capacity
    pub fn new(cap: usize) -> Self {
        let layout = unsafe { alloc::Layout::from_size_align_unchecked(cap, 1) };
        let ptr = unsafe { alloc::alloc(layout) as *mut u8 };

        Buffer {
            layout,
            ptr,
            cap,
            len: 0,
            offset: 0,
        }
    }

    /// Creates and returns a new Buffer of the specified capacity and also
    /// fills this buffer with zeroed values.
    pub fn zeroed(cap: usize) -> Self {
        let layout = unsafe { alloc::Layout::from_size_align_unchecked(cap, 1) };
        let ptr = unsafe { alloc::alloc_zeroed(layout) as *mut u8 };

        Buffer {
            layout,
            ptr,
            cap,
            len: cap,
            offset: 0,
        }
    }

    /// Returns the capacity of the buffer
    pub fn capacity(&self) -> usize {
        self.cap
    }

    /// Returns the number of bytes written to the buffer
    pub fn length(&self) -> usize {
        self.len
    }

    /// Returns the current position of the cursor within the buffer
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Returns the number of bytes left to be read from this buffer
    pub fn remaining(&self) -> usize {
        self.len - self.offset
    }

    /// Returns whether the buffer has any bytes remaining to be read
    pub fn has_remaining(&self) -> bool {
        (self.len - self.offset) > 0
    }

    /// Returns the number of bytes that can be written to the buffer
    pub fn remaining_mut(&self) -> usize {
        self.cap - self.len
    }

    /// Returns whether more bytes can be written to this buffer
    pub fn has_remaining_mut(&self) -> bool {
        (self.cap - self.len) > 0
    }

    /// Advances the cursor's position by the provided offset to skip
    /// reading the provided number of bytes.
    pub fn advance(&mut self, n: usize) -> Option<()> {
        if self.remaining() < n {
            return None;
        }

        self.offset += n;
        Some(())
    }

    /// Reads n bytes from the current offset and returns a reference to it. Optionally
    /// advances n bytes from the cursor if specified.
    pub fn get(&mut self, n: usize, advance: bool) -> &[u8] {
        let start = self.offset;
        let end = self.offset + n;

        if advance {
            self.offset += n;
        }

        &self[start..end]
    }

    /// Reads the data from the buffer into the provided slice and returns
    /// the number of bytes read.
    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        let remaining = self.remaining();
        let size = buf.len().min(remaining);

        unsafe {
            ptr::copy(self.ptr.add(self.offset), buf.as_mut_ptr(), size);
        }

        self.offset += size;
        size
    }

    /// Writes the provided slice into the buffer and returns the number of bytes
    /// written.
    pub fn write(&mut self, buf: &[u8]) -> usize {
        let remaining = self.remaining_mut();
        let size = buf.len().min(remaining);

        unsafe {
            ptr::copy(buf.as_ptr(), self.ptr.add(self.len), size);
        }

        self.len += size;
        size
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { alloc::dealloc(self.ptr, self.layout) };
    }
}

impl Deref for Buffer {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

impl From<Vec<u8>> for Buffer {
    fn from(mut value: Vec<u8>) -> Self {
        let cap = value.capacity();
        let len = value.len();
        let ptr = value.as_mut_ptr();
        let layout = Layout::from_size_align(cap, 1).unwrap();

        std::mem::forget(value);

        Buffer {
            layout,
            ptr,
            cap,
            len,
            offset: 0,
        }
    }
}

mod tests {
    ///
    /// Reads a datagram from the UDPsocket into the zeroed Buffer
    ///
    #[test]
    pub fn udp_socket_read() {
        use crate::*;
        use std::net::*;

        let Ok(socket) = UdpSocket::bind("127.0.0.1:19132") else {
            return;
        };

        let mut buffer = Buffer::zeroed(1500);

        let (len, addr) = socket.recv_from(&mut buffer).unwrap();

        println!(
            "Read {:?} bytes from {:?} => {:?}",
            len,
            addr,
            &buffer[..len]
        );
    }

    ///
    /// Gets a reference from the buffer without advancing it
    ///
    #[test]
    pub fn buffer_get_no_adv() {
        use crate::*;
        use std::net::*;

        let socket = UdpSocket::bind("127.0.0.1:19132").unwrap();
        let mut buffer = Buffer::zeroed(1500);

        let (len, addr) = socket.recv_from(&mut buffer).unwrap();
        let buf = buffer.get(len, false);

        println!("Read {:?} bytes from {:?} => {:?}", len, addr, &buf);
        assert_eq!(buffer.offset(), 0);
    }
}
