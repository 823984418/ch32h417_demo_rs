use crate::rpb::register::Reg32;

/// 核间通信
/// 这是 WCH 的专用外设
#[repr(C)]
pub struct IPC {
    /// 控制寄存器
    pub CTLR: Reg32,

    /// 中断状态寄存器
    pub ISR: Reg32,

    /// 中断屏蔽寄存器
    pub ISM: Reg32,

    RESERVED0: Reg32,

    /// 使能寄存器
    pub ENA: Reg32,

    /// 状态寄存器
    pub STS: Reg32,

    /// 置位寄存器
    pub SET: Reg32,

    /// 清零寄存器
    pub CLR: Reg32,

    /// 消息寄存器
    pub MSG: [Reg32; 4],
}

impl IPC {
    pub const BASE: usize = 0xE000D000;
}
