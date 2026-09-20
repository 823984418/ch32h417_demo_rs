use crate::rpb::register::{Reg8, Reg32};

/// 硬件信号量
/// 这是 WCH 的专用外设
#[repr(C)]
pub struct HSEM {
    pub RX: [Reg32; 32],

    RESERVED0: [Reg8; 0x80],

    pub RLRX: [Reg32; 32],

    RESERVED1: [Reg8; 0x80],

    pub LSE: Reg32,

    RESERVED2: [Reg8; 4],

    pub CLR: Reg32,

    pub KEY: Reg32,

    RESERVED3: [Reg8; 0xF0],

    pub IER: Reg32,

    RESERVED4: [Reg8; 4],

    pub ISR: Reg32,

    RESERVE5: [Reg8; 4],

    pub ISM: Reg32,

    RESERVED6: [Reg8; 4],

    pub LSM: Reg32,
}

impl HSEM {
    pub const BASE: usize = 0xE000C000;
}
