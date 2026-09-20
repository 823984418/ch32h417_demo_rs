use crate::rpb::register::Reg32;

#[derive(Debug)]
#[repr(C)]
pub struct PWR {
    pub CTLR: Reg32,
    pub CSR: Reg32,
}
