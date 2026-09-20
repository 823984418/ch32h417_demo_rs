use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

pub mod cortex_m;
pub mod riscv32;

pub struct Block<T, const ADDR: usize>(PhantomData<*mut T>);

impl<T: Debug, const ADDR: usize> Debug for Block<T, ADDR> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}

impl<T, const ADDR: usize> Deref for Block<T, ADDR> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(ADDR as *const T) }
    }
}

impl<T, const ADDR: usize> DerefMut for Block<T, ADDR> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(ADDR as *mut T) }
    }
}
