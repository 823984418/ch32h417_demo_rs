use core::cell::UnsafeCell;
use core::fmt::{Debug, Formatter};
use core::ops::{BitAnd, BitOr, Not};

pub struct Reg<T: ?Sized>(UnsafeCell<T>);

impl<T: Debug> Debug for Reg<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let value = self.get();
        Debug::fmt(&value, f)
    }
}

impl<T> Reg<T> {
    pub fn set(&self, value: T) {
        unsafe { core::ptr::write(self.0.get(), value) }
    }

    pub fn get(&self) -> T {
        unsafe { core::ptr::read_volatile(self.0.get()) }
    }

    pub fn bit_or(&self, rhs: T)
    where
        T: BitOr<T, Output = T>,
    {
        self.set(self.get() | rhs);
    }

    pub fn bit_and(&self, rhs: T)
    where
        T: BitAnd<T, Output = T>,
    {
        self.set(self.get() & rhs);
    }

    pub fn bit_clear(&self, rhs: T)
    where
        T: Not<Output = T> + BitAnd<T, Output = T>,
    {
        self.set(self.get() & !rhs);
    }
}

pub type Reg8 = Reg<u8>;
pub type Reg16 = Reg<u16>;
pub type Reg32 = Reg<u32>;
pub type Reg64 = Reg<u64>;
pub type Reg128 = Reg<u128>;
