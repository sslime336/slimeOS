use core::cell::{RefCell, RefMut, Ref};

/// SafeCell is used to allow multi-thread usage as it's
/// not allowed to use object without achieving the [`Sync`]
/// trait in rust(for RefCell usage). After all, it is just a wrapper.
pub struct SafeCell<T> {
    inner: RefCell<T>,
}

impl<T> SafeCell<T> {
    pub fn new(item: T) -> Self {
        SafeCell {
            inner: RefCell::new(item),
        }
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        self.inner.borrow()
    }

    pub fn borrow_mut(&mut self) -> RefMut<'_, T> {
        self.inner.borrow_mut()
    }
}

unsafe impl<T> Sync for SafeCell<T> {}
