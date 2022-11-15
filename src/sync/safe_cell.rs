use core::cell::RefCell;

pub struct SafeCell<T> {
    inner: RefCell<T>,
}

impl<T> SafeCell<T> {
    pub fn new<T>(item: T) -> Self {
        SafeCell {
            inner: RefCell::new(item),
        }
    }
}

impl Sync<_> for SafeCell<_> {}
