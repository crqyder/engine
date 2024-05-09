use std::{alloc, ptr};

pub struct Buffer {
    ptr: *mut u8,
    cap: usize,
    len: usize,
    offset: usize,
    owner: bool,
}

impl Buffer {
    pub fn new(cap: usize) -> Self {
        let ptr =
            unsafe { alloc::alloc(alloc::Layout::from_size_align_unchecked(cap, 1)) as *mut u8 };

        Buffer {
            ptr,
            cap,
            len: 0,
            offset: 0,
            owner: true,
        }
    }

    pub fn capacity(&self) -> usize {
        self.cap
    }

    pub fn length(&self) -> usize {
        self.len
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn remaining(&self) -> usize {
        self.len - self.offset
    }

    pub fn has_remaining(&self) -> bool {
        (self.len - self.offset) > 0
    }

    pub fn remaining_mut(&self) -> usize {
        self.cap - self.len
    }

    pub fn has_remaining_mut(&self) -> bool {
        (self.cap - self.len) > 0
    }

    pub fn split_at(&mut self, index: usize) -> Buffer {
        assert!(index <= (self.cap - 1), "index out of bounds");

        let cap = self.cap - index;
        let mut len = 0;

        if (self.len - 1) >= index {
            len = self.len - index;
            self.len = index;
        }

        self.cap = index;
        self.offset = 0;

        let ptr = unsafe { self.ptr.add(index) };

        Buffer {
            ptr,
            cap,
            len,
            offset: 0,
            owner: false,
        }
    }

    pub fn get_data(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }

    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        let remaining = self.remaining();
        let size = buf.len().min(remaining);

        unsafe {
            ptr::copy(self.ptr.add(self.offset), buf.as_mut_ptr(), size);
        }

        self.offset += size;
        size
    }

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
        if self.owner {
            let layout = alloc::Layout::from_size_align(self.cap, 1).unwrap();
            unsafe { alloc::dealloc(self.ptr, layout) };
        }
    }
}
