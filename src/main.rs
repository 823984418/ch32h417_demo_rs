#![no_main]

const _: () = {
    #[allow(unused_imports)]
    use cortex_m_std_dummy;
};

use core::sync::atomic::{AtomicBool, Ordering};

use device::Device;
use device::gpio::CFGR_PIN;
use device::rcc::HB2PCENR;
pub use rpb::device::wch::ch32h417 as device;

pub const DEVICE: Device = unsafe { core::mem::zeroed() };

static NORMAL_STARTUP: AtomicBool = AtomicBool::new(false);

#[unsafe(export_name = "_mp_hook")]
extern "C" fn mp_hook(hartid: usize) -> bool {
    hartid == 0
}

#[riscv_rt::entry]
unsafe fn main() -> ! {
    match riscv::register::mhartid::read() {
        0 => {
            NORMAL_STARTUP.store(true, Ordering::SeqCst);
            DEVICE.PFIC.WAKEIP[1].set(0);
            DEVICE.PFIC.SCTLR.bit_or(1 << 5);
            v3f_main();
        }
        _ => {
            if !NORMAL_STARTUP.load(Ordering::SeqCst) {
                DEVICE.PFIC.SystemReset();
            }
            v5f_main();
        }
    }
}

fn v3f_main() -> ! {
    loop {
        riscv::asm::wfi();
    }
}

fn v5f_main() -> ! {
    DEVICE.RCC.HB2PCENR.bit_or(HB2PCENR::IOPEEN);
    DEVICE.GPIOE.CFGLR.bit_or(CFGR_PIN::PP_OUT.pin(4));
    DEVICE.GPIOE.CFGLR.bit_or(CFGR_PIN::PP_OUT.pin(5));
    loop {
        riscv::asm::wfi();
    }
}
