use std::ops::{Deref, DerefMut};
use std::pin::Pin;

pub struct RawPointer<T: Sized> {
    pointer: *mut T,
}

impl<T: Sized> RawPointer<T> {
    pub unsafe fn new_from_pin(pin: Pin<&mut T>) -> Self {
        unsafe {
            Self {
                pointer: pin.get_unchecked_mut()
            }
        }
    }

    fn get(&self) -> &T {
        unsafe { self.pointer.as_ref().unwrap() }
    }

    fn get_mut(&mut self) -> &mut T {
        unsafe { self.pointer.as_mut().unwrap() }
    }
}

impl<T: Sized> Deref for RawPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T: Sized> DerefMut for RawPointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

impl<T: Sized> Copy for RawPointer<T> {}

impl<T: Sized> Clone for RawPointer<T> {
    fn clone(&self) -> Self {
        Self {
            pointer: self.pointer
        }
    }
}

#[macro_export]
macro_rules! create_raw_ptr {
    ($object:expr) => {
        {
            use raw_pointer::RawPointer;
            use std::pin::Pin;

            let object_pin = Pin::new(&mut $object);

            RawPointer::new_from_pin(object_pin)
        }
    }
}
