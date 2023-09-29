use std::{mem, ptr};

use crate::raw_vec::RawVec;

pub struct IntoIter<T> {
    pub _buf: RawVec<T>,
    pub start: *const T,
    pub end: *const T,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.start == self.end {
            return None;
        }
        unsafe {
            let result = ptr::read(self.start);
            self.start = self.start.offset(1);
            Some(result)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.end as usize - self.start as usize) / mem::size_of::<T>();
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        if self.start == self.end {
            return None;
        }
        unsafe {
            self.end = self.end.offset(-1);
            Some(ptr::read(self.end))
        }
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}
