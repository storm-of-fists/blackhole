use core::{
    cell::{Cell, UnsafeCell},
    ops::{Deref, DerefMut},
};

pub enum MutCellError {
    AlreadyBorrowed,
}

/// This is similar to RefCell, but you should only ever have a single
/// reference, immutable or mutable. The idea behind this API is that
/// with a large enough program, there just isn't a good way to know
/// what the code will do.
///
/// Instead, we can make users responsible for this. We can teach them
/// that before passing an MutCell to another function call, we need
/// to explicitly drop the MutCellRef.
///
/// let foo = MutCell::new(10);
/// let foo_ref = foo.get()?;
///
/// foo_ref += 10;
///
/// Before passing foo into some function, the user needs to drop
/// foo_ref explicitly.
///
/// drop(foo_ref);
///
/// foo_func(foo);
///
/// Then inside the foo_func, they can acquire the reference and do with
/// the data what they need.
pub struct MutCell<T> {
    value: UnsafeCell<T>,
    borrowed: Cell<bool>,
}

impl<T> MutCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            borrowed: Cell::new(false),
        }
    }

    pub fn get(&self) -> Result<MutCellRef<'_, T>, MutCellError> {
        if self.borrowed.get() {
            return Err(MutCellError::AlreadyBorrowed);
        }

        self.borrowed.replace(true);

        Ok(MutCellRef {
            // SAFETY: We guard from multiple mutable references by checking
            // the borrow_cell above.
            value: unsafe { &mut (*self.value.get()) },
            borrowed: &self.borrowed,
        })
    }
}

pub struct MutCellRef<'a, T> {
    value: &'a mut T,
    borrowed: &'a Cell<bool>,
}

impl<T> Drop for MutCellRef<'_, T> {
    fn drop(&mut self) {
        self.borrowed.replace(false);
    }
}

impl<T> Deref for MutCellRef<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<T> DerefMut for MutCellRef<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value
    }
}
