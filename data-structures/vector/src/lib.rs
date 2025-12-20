use std::alloc::{Layout, alloc, dealloc};
use std::ptr;

pub struct MyVec<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        MyVec {
            ptr: ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            1
        } else {
            self.capacity * 2
        };

        let layout = Layout::array::<T>(new_capacity).unwrap();

        unsafe {
            let new_ptr = alloc(layout) as *mut T;

            if !self.ptr.is_null() {
                ptr::copy_nonoverlapping(self.ptr, new_ptr, self.len);

                let old_layout = Layout::array::<T>(self.capacity).unwrap();
                dealloc(self.ptr as *mut u8, old_layout);
            }

            self.ptr = new_ptr;
            self.capacity = new_capacity;
        }
    }

    pub fn push(&mut self, value: T) {
        if self.len == self.capacity {
            self.grow();
        }
        unsafe {
            self.ptr.add(self.len).write(value);
        }
        self.len += 1;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        unsafe { Some(&*self.ptr.add(index)) }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        unsafe { Some(self.ptr.add(self.len).read()) }
    }

    pub fn drop(&mut self) {
        unsafe {
            for i in 0..self.len {
                ptr::drop_in_place(self.ptr.add(i));
            }

            if self.capacity != 0 {
                let layout = Layout::array::<T>(self.capacity).unwrap();
                dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}

