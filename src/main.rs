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
use libc::{STDERR_FILENO, STDOUT_FILENO, c_int, c_void, size_t, ssize_t};
pub use rpb::device::wch::ch32h417 as device;
use rpb::device::wch::ch32h417::gpio::{GPIO_Af, GPIO_Pin, GPIO_Pins, GPIO_Port};
use rpb::peripheral::riscv32::qingke::csr::cache_strtg_ctlr::CACHE_STRTG_CTLR;
use rpb::peripheral::riscv32::qingke::csr::corecfgr::CORECFGR;
use rpb::peripheral::riscv32::qingke::csr::inestcr::INESTCR;
use rpb::peripheral::riscv32::qingke::csr::intsyscr::INTSYSCR;
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
            DEVICE.CSR.corecfgr.set(
                CORECFGR::from_bits_retain(0x1237 << 16)
                    | CORECFGR::CSTA_FAULT_IE
                    | CORECFGR::IE_REMAP_EN
                    | CORECFGR::FETCH_MODE_1,
            );
            DEVICE
                .CSR
                .intsyscr
                .set(INTSYSCR::PMTCFG_1 | INTSYSCR::INESTEN | INTSYSCR::HWSTKEN);
            DEVICE.CSR.inestcr.set(INESTCR::NEST_LVL_2);
            unsafe { allocator_init() };
            NORMAL_STARTUP.store(true, Ordering::SeqCst);
            DEVICE.PFIC.WAKEIP[1].set(0);
            DEVICE.PFIC.SCTLR.bit_or(SCTLR::SETEVENT);
            v3f_main();
        }
        1 => {
            DEVICE.CSR.corecfgr.set(
                CORECFGR::from_bits_retain(0x1237 << 16)
                    | CORECFGR::NLP_EN
                    | CORECFGR::GHR_EN
                    | CORECFGR::LSU_DUAL
                    | CORECFGR::CSTA_FAULT_IE
                    | CORECFGR::IE_REMAP_EN
                    | CORECFGR::FETCH_MODE_0,
            );
            DEVICE.CSR.cache_strtg_ctlr.set(
                CACHE_STRTG_CTLR::IC_MEM1_STRTG
                    | CACHE_STRTG_CTLR::IC_MEM0_STRTG
                    | CACHE_STRTG_CTLR::IC_SRAM_STRTG
                    | CACHE_STRTG_CTLR::IC_CODE_STRTG,
            );
            DEVICE.CSR.inestcr.set(INESTCR::NEST_LVL_8);
            if !NORMAL_STARTUP.load(Ordering::SeqCst) {
                DEVICE.PFIC.SystemReset();
            }
            v5f_main();
        }
        _ => unreachable!(),
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

    let clocks = DEVICE.RCC.GetClocksFreq();

    DEVICE.RCC.HB2PeriphClockCmd(
        HB2PCENR::AFIOEN | HB2PCENR::USART1EN | HB2PCENR::IOPAEN | HB2PCENR::IOPEEN,
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
    DEVICE.USART1.Init(
        &USART_Init {
            BaudRate: 9600,
            WordLength: USART_WordLength::WordLength_8b,
            StopBits: USART_StopBits::StopBits_1,
            Parity: USART_Parity::Parity_No,
            Mode: USART_Mode::Mode_Tx,
            HardwareFlowControl: USART_HardwareFlowControl::empty(),
        },
        clocks.HCLK,
    );
    DEVICE.USART1.Cmd(true);

    DEVICE.GPIOE.Init(&GPIO_Init {
        Pin: GPIO_Pins::PIN4 | GPIO_Pins::PIN5,
        Speed: SPEED_PIN::SPEED_180M,
        Mode: CFGR_PIN::PP_OUT,
    });

    println!("{:#?}", DEVICE.CSR);

    let mut delay = delay::McycleDelay::new(400_000_000);
    loop {
        DEVICE.GPIOE.SetBits(GPIO_Pins::PIN4 | GPIO_Pins::PIN5);
        // delay.delay_ms(500);
        riscv::asm::delay(200_000_000);

        DEVICE.GPIOE.ResetBits(GPIO_Pins::PIN4 | GPIO_Pins::PIN5);
        // delay.delay_ms(500);
        riscv::asm::delay(200_000_000);
    }
}

pub const fn wch_interrupt_fast<F: Fn() + Copy + 'static>(f: F) -> extern "C" fn() {
    let _ = f;

    /// Monomorphized trampoline: reconstitutes the zero-sized closure and runs it.
    extern "C" fn wch_interrupt_fast_trap_inner<F: Fn() + Copy + 'static>() {
        const { assert!(size_of::<F>() == 0) };
        // SAFETY: `F` is asserted to be a ZST (its only inhabitant has zero bytes),
        // so an all-zero bit pattern is a valid `F`.
        let f: F = unsafe { core::mem::zeroed() };
        f();
    }

    #[unsafe(naked)]
    extern "C" fn wch_interrupt_fast_trap<F: Fn() + Copy + 'static>() {
        core::arch::naked_asm!(
            // Reserve a 16-byte aligned frame and save `ra` so that `call`
            // (which clobbers `ra`) does not destroy the interrupted context.
            "addi sp, sp, -16",
            "sw   ra, 0(sp)",

            // Invoke the monomorphized Rust wrapper, which constructs the ZST
            // closure `F` and calls it.
            "call {inner}",

            // Restore `ra` and the stack, then return from the trap.
            "lw   ra, 0(sp)",
            "addi sp, sp, 16",
            "mret",
            inner = sym wch_interrupt_fast_trap_inner::<F>,
        );
    }

    wch_interrupt_fast_trap::<F>
}
