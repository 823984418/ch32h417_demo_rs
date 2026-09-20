use crate::rpb::register::Reg32;

#[repr(C)]
pub struct PWR {
    pub CTLR: Reg32,
    pub CSR: Reg32,
}
