#![no_main]

pub mod delay;

const _: () = {
    #[allow(unused_imports)]
    use cortex_m_std_dummy;
};

use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicBool, Ordering};

use device::Device;
use device::gpio::{CFGR_PIN, GPIO_Init, SPEED_PIN};
use device::rcc::HB2PCENR;
use device::usart::{
    STATR, USART_HardwareFlowControl, USART_Init, USART_Mode, USART_Parity, USART_StopBits,
    USART_WordLength,
};
use embedded_hal::delay::DelayNs;
use libc::{STDERR_FILENO, STDOUT_FILENO, c_int, c_void, size_t, ssize_t};
pub use rpb::device::wch::ch32h417 as device;
use rpb::device::wch::ch32h417::gpio::{GPIO_Af, GPIO_Pin, GPIO_Pins, GPIO_Port};
use rpb::peripheral::riscv32::qingke::pfic::SCTLR;

pub const DEVICE: Device = unsafe { core::mem::zeroed() };

static NORMAL_STARTUP: AtomicBool = AtomicBool::new(false);

#[unsafe(export_name = "_mp_hook")]
extern "C" fn mp_hook(hartid: usize) -> bool {
    hartid == 0
}

unsafe fn allocator_init() {
    const HEAP_SIZE: usize = 64 * 1024;

    #[global_allocator]
    static ALLOCATOR: embedded_alloc::LlffHeap = embedded_alloc::LlffHeap::empty();

    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        ALLOCATOR.init(&raw mut HEAP as usize, HEAP_SIZE);
    }
}

#[riscv_rt::entry]
unsafe fn main() -> ! {
    match riscv::register::mhartid::read() {
        0 => {
            allocator_init();
            NORMAL_STARTUP.store(true, Ordering::SeqCst);
            DEVICE.PFIC.WAKEIP[1].set(0);
            DEVICE.PFIC.SCTLR.bit_or(SCTLR::SETEVENT);
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

#[unsafe(no_mangle)]
unsafe extern "C" fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t {
    match fd {
        STDOUT_FILENO | STDERR_FILENO => unsafe {
            let buff = unsafe { core::slice::from_raw_parts(buf as *const u8, count) };
            for &c in buff {
                while !DEVICE.USART1.GetFlagStatus().intersects(STATR::TXE) {}
                DEVICE.USART1.SendData(c as u32);
            }
            count as ssize_t
        },
        _ => 0,
    }
}

fn v5f_main() -> ! {
    DEVICE.RCC.DeInit();
    DEVICE.SetSYSCLK_400M_CoreCLK_V5F_400M_V3F_100M_HSE();

    DEVICE.RCC.HB2PeriphClockCmd(
        HB2PCENR::AFIOEN | HB2PCENR::USART1EN | HB2PCENR::IOPAEN,
        true,
    );
    DEVICE
        .AFIO
        .PinAFConfig(GPIO_Port::PA, GPIO_Pin::PIN9, GPIO_Af::AF7);
    DEVICE.GPIOA.Init(&GPIO_Init {
        Pin: GPIO_Pins::PIN9,
        Speed: SPEED_PIN::SPEED_180M,
        Mode: CFGR_PIN::AF_PP_OUT,
    });
    DEVICE.USART1.Init(&USART_Init {
        BaudRate: 9600,
        WordLength: USART_WordLength::WordLength_8b,
        StopBits: USART_StopBits::StopBits_1,
        Parity: USART_Parity::Parity_No,
        Mode: USART_Mode::Mode_Tx,
        HardwareFlowControl: USART_HardwareFlowControl::empty(),
    });
    DEVICE.USART1.Cmd(true);

    DEVICE.RCC.HB2PeriphClockCmd(HB2PCENR::IOPEEN, true);

    DEVICE.GPIOE.Init(&GPIO_Init {
        Pin: GPIO_Pins::PIN4 | GPIO_Pins::PIN5,
        Speed: SPEED_PIN::SPEED_180M,
        Mode: CFGR_PIN::PP_OUT,
    });

    let mut delay = delay::McycleDelay::new(400_000_000);
    loop {
        DEVICE.GPIOE.CFGLR.bit_or(CFGR_PIN::PP_OUT.pin(4));
        DEVICE.GPIOE.CFGLR.bit_or(CFGR_PIN::PP_OUT.pin(5));
        delay.delay_ms(500);

        DEVICE.GPIOE.CFGLR.bit_clear(CFGR_PIN::PP_OUT.pin(4));
        DEVICE.GPIOE.CFGLR.bit_clear(CFGR_PIN::PP_OUT.pin(5));
        delay.delay_ms(500);
    }
}
