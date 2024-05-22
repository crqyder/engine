use std::ops::{Deref, DerefMut};

/// Buffer represents a fast implementation of zero copy and non growable buffer. It can be
/// internally resized however it does not affect the original length of the vector this buffer
/// allocates.
pub struct Buffer {
    slice: Vec<u8>,
    offset: usize,
    size: usize,
    cap: usize,
}

impl Buffer {
    /// Creates and returns a new Buffer of the specified capacity
    pub fn new(cap: usize) -> Self {
        Self {
            slice: vec![0u8; cap],
            offset: 0,
            size: cap,
            cap: cap,
        }
    }

    /// Returns the size of the buffer.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Sets the size of the buffer to the one provided.
    pub fn resize(&mut self, size: usize) {
        self.size = size;
    }

    /// Returns the number of bytes left in the buffer from the offset to reach the
    /// length of the buffer.
    pub fn remaining(&self) -> usize {
        self.size - self.offset
    }

    /// Returns the original length of the buffer
    pub fn capacity(&self) -> usize {
        self.cap
    }

    /// Advances the buffer by the specified capacity. Returns true if the operation was
    /// successful and returns false if the buffer did not have enough space.
    pub fn advance(&mut self, offset: usize) -> bool {
        if self.remaining() < offset {
            return false;
        }

        self.offset += offset;
        true
    }

    /// Gets the offset and returns it.
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Sets the offset to the one provided.
    pub fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }

    /// Reads n bytes from the current offset and returns a reference to it. Optionally
    /// advances n bytes from the cursor if specified.
    pub fn get(&mut self, n: usize, advance: bool) -> &[u8] {
        let start = self.offset;
        let end = self.offset + n;

        if advance {
            self.offset += n;
        }

        &self.slice[start..end]
    }

    /// Reads into the provided mutable slice. If the length of the provided slice exceeds the
    /// amount of bytes available to be read then it reads as many bytes it can and returns the
    /// number.
    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        let remaining = self.remaining();
        let size = buf.len().min(remaining);

        if remaining < size {
            return 0;
        }

        let start = self.offset;
        let end = self.offset + size;

        unsafe {
            std::ptr::copy_nonoverlapping(
                self.slice[start..end].as_ptr(),
                buf[..size].as_mut_ptr(),
                size,
            )
        }

        self.offset += size;
        size
    }

    /// Writes into the buffer from the provided slice. If the length of the bytes to be written exceeds the
    /// amount of space available in the buffer then it writes as many bytes it can and returns the number.
    pub fn write(&mut self, buf: &[u8]) -> usize {
        let remaining = self.remaining();
        let size = buf.len().min(remaining);

        if remaining < size {
            return 0;
        }

        let start = self.offset;
        let end = self.offset + size;

        unsafe {
            std::ptr::copy_nonoverlapping(
                buf[..size].as_ptr(),
                self.slice[start..end].as_mut_ptr(),
                size,
            )
        }

        self.offset += size;
        size
    }

    /// Resets the Buffer with zero allocation and zero overhead. Resets the offset and resizes
    /// the length back to the original capacity of the buffer.
    pub fn reset(&mut self) {
        self.size = self.cap;
        self.offset = 0;
    }
}

impl AsRef<[u8]> for Buffer {
    fn as_ref(&self) -> &[u8] {
        &self.slice[..self.offset]
    }
}

impl Deref for Buffer {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.slice[..self.size]
    }
}

impl DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.slice[..self.size]
    }
}

impl From<Vec<u8>> for Buffer {
    fn from(value: Vec<u8>) -> Self {
        let len = value.len();

        Self {
            slice: value,
            offset: 0,
            size: len,
            cap: len,
        }
    }
}
