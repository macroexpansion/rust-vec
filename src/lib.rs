use std::alloc;
use std::ptr::NonNull;

pub struct MyVec<T> {
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn push(&mut self, item: T) {
        assert_ne!(std::mem::size_of::<T>(), 0, "No zero-sized types");
        if self.capacity == 0 {
            let layout = alloc::Layout::array::<T>(4).expect("Could not allocate layout");
            // SAFETY: the layout is hardcoded to be 4 * size_of::<T>() and size_of::<T>()  is greater
            // than 0
            let ptr: *mut T = unsafe { alloc::alloc(layout) } as *mut T;
            let ptr: NonNull<T> = NonNull::new(ptr).expect("Could not allocate memory");
            // why is this ok? ptr is non-null pointer
            unsafe {
                ptr.as_ptr().write(item); // dont use *ptr.as_ptr() = val because it will read the
                                          // pointer first
            };

            self.ptr = ptr;
            self.len = 1;
            self.capacity = 4;
        } else if self.len < self.capacity {
            unsafe {
                self.ptr.as_ptr().add(self.len).write(item);
            }
            self.len += 1;
        } else {
            debug_assert!(self.len == self.capacity);

            let new_capacity = self.capacity.checked_mul(2).expect("Arithmetic overflow");
            let size = std::mem::size_of::<T>() * self.capacity;
            let align = std::mem::align_of::<T>();
            // size.checked_add(size % align).expect()
            let ptr = unsafe {
                let layout = alloc::Layout::from_size_align_unchecked(size, align);
                let new_size = std::mem::size_of::<T>() * new_capacity;
                let ptr = alloc::realloc(self.ptr.as_ptr() as *mut u8, layout, new_size);
                let ptr = NonNull::new(ptr as *mut T).expect("Could not reallocate memory");
                ptr.as_ptr().add(self.len).write(item);
                ptr
            };
            self.ptr = ptr;
            self.len += 1;
            self.capacity = new_capacity;
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        let value = unsafe { &*self.ptr.as_ptr().add(index) };
        Some(value)
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len));
            let layout = alloc::Layout::from_size_align_unchecked(
                std::mem::size_of::<T>() * self.capacity,
                std::mem::align_of::<T>(),
            );
            alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec: MyVec<i32> = MyVec::<i32>::new();
        vec.push(1i32);
        vec.push(2i32);
        vec.push(3i32);
        vec.push(4i32);

        vec.push(1i32);
        vec.push(1i32);

        assert_eq!(*(vec.get(0).unwrap()), 1i32);
        assert_eq!(*(vec.get(1).unwrap()), 2i32);
        assert_eq!(vec.len(), 6);
        assert_eq!(vec.capacity(), 8);
    }
}
