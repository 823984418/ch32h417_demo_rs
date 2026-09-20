pub mod gpio;
pub mod pwr;
pub mod rcc;

use crate::rpb::device::wch::ch32h417::gpio::GPIO;
use crate::rpb::device::wch::ch32h417::pwr::PWR;
use crate::rpb::device::wch::ch32h417::rcc::RCC;
use crate::rpb::peripheral::Block;
use crate::rpb::peripheral::riscv32::qingke::hsem::HSEM;
use crate::rpb::peripheral::riscv32::qingke::ipc::IPC;
use crate::rpb::peripheral::riscv32::qingke::pfic::PFIC;
use crate::rpb::peripheral::riscv32::qingke::systick::SysTick;
use crate::rpb::register::Reg32;

#[derive(Debug)]
pub struct Device {
    pub PWR: Block<PWR, 0x40007000>,

    pub GPIOA: Block<GPIO, 0x40010800>,
    pub GPIOB: Block<GPIO, 0x40010C00>,
    pub GPIOC: Block<GPIO, 0x40011000>,
    pub GPIOD: Block<GPIO, 0x40011400>,
    pub GPIOE: Block<GPIO, 0x40011800>,
    pub GPIOF: Block<GPIO, 0x40011C00>,

    pub RCC: Block<RCC, 0x40021000>,

    pub HSEM: Block<HSEM, 0xE000C000>,
    pub PFIC: Block<PFIC, 0xE000E000>,
    pub IPC: Block<IPC, 0xE000D000>,
    pub SysTick0: Block<SysTick, 0xE000F000>,
    pub SysTick1: Block<SysTick, 0xE000F080>,

    pub SYS_CFGR0: Block<Reg32, 0x5003C000>,
}
