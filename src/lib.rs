mod into_iter;
mod raw_vec;

use std::{
    mem,
    ops::{Deref, DerefMut},
    ptr,
};

use crate::into_iter::IntoIter;
use crate::raw_vec::RawVec;

pub struct MyVec<T> {
    buf: RawVec<T>,
    len: usize,
}

unsafe impl<T: Send> Send for MyVec<T> {}
unsafe impl<T: Sync> Sync for MyVec<T> {}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        assert_ne!(std::mem::size_of::<T>(), 0, "No zero-sized types");
        Self {
            buf: RawVec::new(),
            len: 0,
        }
    }

    pub fn cap(&self) -> usize {
        self.buf.cap
    }

    fn ptr(&self) -> *mut T {
        self.buf.ptr.as_ptr()
    }

    fn grow(&mut self) {
        self.buf.grow();
    }

    pub fn push(&mut self, item: T) {
        if self.cap() == self.len {
            self.grow();
        }

        unsafe {
            self.ptr().add(self.len).write(item);
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;

        unsafe { Some(self.ptr().add(self.len).read()) }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        let value = unsafe { &*self.ptr().add(index) };
        Some(value)
    }

    pub fn insert(&mut self, index: usize, elem: T) {
        assert!(index <= self.len, "index out of bounds");
        if self.cap() == self.len {
            self.grow();
        }
        unsafe {
            ptr::copy(
                self.ptr().add(index),
                self.ptr().add(index + 1),
                self.len - index,
            );
            ptr::write(self.ptr().add(index), elem);
        }
        self.len += 1;
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len, "index out of bounds");
        self.len -= 1;
        let result = unsafe {
            let elem = ptr::read(self.ptr().add(index));
            ptr::copy(
                self.ptr().add(index + 1),
                self.ptr().add(index),
                self.len - index,
            );
            elem
        };
        result
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(std::slice::from_raw_parts_mut(self.ptr(), self.len));
            // deallocation is handled by RawVec
        }
    }
}

impl<T> Deref for MyVec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr(), self.len) }
    }
}

impl<T> DerefMut for MyVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr(), self.len) }
    }
}

impl<T> IntoIterator for MyVec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        unsafe {
            let buf = ptr::read(&self.buf);
            let len = self.len;
            mem::forget(self);

            IntoIter {
                start: buf.ptr.as_ptr(),
                end: if buf.cap == 0 {
                    // can't offset off this pointer, it's not allocated!
                    buf.ptr.as_ptr()
                } else {
                    buf.ptr.as_ptr().add(len)
                },
                _buf: buf,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop_insert_remove() {
        let mut vec: MyVec<i32> = MyVec::<i32>::new();
        vec.push(1i32);
        vec.push(2i32);
        vec.push(3i32);
        vec.push(4i32);
        vec.push(5i32);
        vec.push(6i32);

        vec.pop();
        vec.pop();
        assert_eq!(vec.len(), 4);
        assert_eq!(*(vec.get(0).unwrap()), 1i32);

        vec.insert(1, 1i32);
        assert_eq!(vec.len(), 5);
        assert_eq!(*(vec.get(1).unwrap()), 1i32);

        let elem = vec.remove(1);
        assert_eq!(vec.len(), 4);
        assert_eq!(elem, 1i32);
        assert_eq!(*(vec.get(1).unwrap()), 2i32);
    }

    #[test]
    fn test_iter() {
        let mut vec: MyVec<usize> = MyVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);

        let mut iterator = vec.into_iter();
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), Some(2));
        assert_eq!(iterator.next_back(), Some(3));
        assert_eq!(iterator.next_back(), None);
    }
}
