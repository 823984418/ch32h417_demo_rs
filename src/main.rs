#![no_main]

const _: () = {
    #[allow(unused_imports)]
    use cortex_m_std_dummy;
};

pub mod pfic;

use rtt_target::rprintln;

#[unsafe(export_name = "_mp_hook")]
extern "C" fn mp_hook(hartid: usize) -> bool {
    hartid == 0
}

#[riscv_rt::entry]
fn main() -> ! {
    if riscv::register::mhartid::read() == 0 {
        rtt_target::rtt_init_print!();
        rprintln!("helloworld 0");
    } else {
        rprintln!("helloworld 1");
    }
    loop {}
}
