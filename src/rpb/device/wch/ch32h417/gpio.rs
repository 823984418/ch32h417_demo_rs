use bitflags::bitflags;

use crate::rpb::register::{Reg, Reg32};

#[derive(Debug)]
#[repr(C)]
pub struct GPIO {
    pub CFGLR: Reg<CFGR>,
    pub CFGHR: Reg<CFGR>,
    pub INDR: Reg32,
    pub OUTDR: Reg32,
    pub BSHR: Reg32,
    pub BCR: Reg32,
    pub LCKR: Reg32,
    pub SPEED: Reg<SPEED>,
}

bitflags! {
    #[derive(Debug, Copy, Clone)]
    pub struct CFGR: u32 {}
}

bitflags! {
    #[derive(Debug, Copy, Clone)]
    pub struct CFGR_PIN: u32 {
        const AF_OD_OUT = 0b11_01;
        const AF_PP_OUT = 0b10_01;
        const OD_OUT = 0b01_01;
        const PP_OUT = 0b00_01;

        const RES_IN = 0b10_00;
        const FLOAT_IN = 0b01_00;
        const ANALOG_IN = 0b00_00;
    }
}

impl CFGR_PIN {
    pub fn pin(self, n: u32) -> CFGR {
        CFGR::from_bits_retain(self.bits() << (n * 4))
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone)]
    pub struct SPEED: u32 {}
}

bitflags! {
    #[derive(Debug, Copy, Clone)]
    pub struct SPEED_PIN: u32 {
        const SPEED_180M = 0b11;
        const SPEED_100M = 0b10;
        const SPEED_50M = 0b01;
        const SPEED_10M = 0b00;
    }
}

impl SPEED_PIN {
    pub fn pin(self, n: u32) -> SPEED {
        SPEED::from_bits_retain(self.bits() << (n * 2))
    }
}

impl GPIO {}
