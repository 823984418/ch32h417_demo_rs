use crate::rpb::register::{Reg8, Reg32};

/// 可编程快速中断控制器
/// 这是 WCH 的专用外设
#[repr(C)]
pub struct PFIC {
    /// 中断使能状态寄存器 x
    ///
    /// # 可用架构
    /// * V3
    /// * V5
    pub ISR: [Reg32; 8],

    /// 中断挂起状态寄存器 x
    ///
    /// # 可用架构
    /// * V3
    /// * V5
    pub IPR: [Reg32; 8],

    /// 中断优先级阈值配置寄存器
    ///
    /// # 可用架构
    /// * V3
    /// * V5
    pub ITHRESDR: Reg32,

    /// 免表中断基地址寄存器
    ///
    /// # 可用架构
    /// * V3(A)
    pub VTFBADDRR: Reg32,

    /// 中断配置寄存器
    ///
    /// # 可用架构
    /// * V3(AFV)
    /// * V5
    pub CFGR: Reg32,

    /// 中断全局状态寄存器
    ///
    /// # 可用架构
    /// * V3
    /// * V5
    pub GISR: Reg32,

    /// VTF 中断 ID 配置寄存器
    ///
    /// # 可用架构
    /// * V3(BCFV)
    /// * V5
    pub VTFIDR: Reg32,

    RESERVED0: [Reg32; 3],

    /// VTF 中断 x 偏移地址寄存器
    ///
    /// # 可用架构
    /// * V3
    /// * V5
    pub VTFADDRR: [Reg32; 4],

    RESERVED1: [Reg8; 0x90],

    /// # 可用架构
    /// * V3
    /// * V5
    pub IENR: [Reg32; 8],

    RESERVED2: [Reg8; 0x60],

    /// # 可用架构
    /// * V3
    /// * V5
    pub IRER: [Reg32; 8],

    RESERVED3: [Reg8; 0x60],

    /// # 可用架构
    /// * V3
    /// * V5
    pub IPSR: [Reg32; 8],

    RESERVED4: [Reg8; 0x60],

    /// # 可用架构
    /// * V3
    /// * V5
    pub IPRR: [Reg32; 8],

    RESERVED5: [Reg8; 0x60],

    /// # 可用架构
    /// * V3
    /// * V5
    pub IACTR: [Reg32; 8],

    RESERVED6: [Reg8; 0xE0],

    /// # 可用架构
    /// * V3
    /// * V5
    pub IPRIOR: [Reg8; 256],

    RESERVED7: [Reg8; 0x100],

    /// # 可用架构
    /// * V3
    /// * V5
    pub IALLOCR: [Reg8; 256],

    /// # 可用架构
    /// * V3
    /// * V5
    pub IAUTR: [Reg32; 8],

    /// # 可用架构
    /// * V3
    /// * V5
    pub WAKEIP: [Reg32; 2],

    RESERVED8: [Reg8; 0x58],

    /// 内核状态寄存器
    ///
    /// # 可用架构
    /// * V3
    /// * V5
    pub CSTAR: [Reg32; 2],

    RESERVED9: [Reg8; 0x4F8],

    /// # 可用架构
    /// * V3
    /// * V5
    pub EENR: Reg32,

    /// # 可用架构
    /// * V3
    /// * V5
    pub EPR: Reg32,

    /// # 可用架构
    /// * V3
    /// * V5
    pub EWUPR: Reg32,

    RESERVED10: [Reg8; 0x84],

    /// # 可用架构
    /// * V3
    /// * V5
    pub SCTLR: Reg32,
}

impl PFIC {
    pub const BASE: usize = 0xE000E000;

    pub const CFGR_KEYCODE: u32 = 0xBEEF0000;

    pub fn SystemReset(&self) {
        self.CFGR.set(Self::CFGR_KEYCODE | 1 << 7);
    }
}
