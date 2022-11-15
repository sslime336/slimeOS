use core::cell::RefCell;

pub struct SafeCell<T> {
    inner: RefCell<T>,
}

impl<T> SafeCell<T> {
    pub fn new(item: T) -> Self {
        SafeCell {
            inner: RefCell::new(item),
        }
    }
}

unsafe impl<T> Sync for SafeCell<T> {}
