//! Delay devices and providers
use embedded_hal::delay::DelayNs;
use riscv::register::mcycle;

/// Machine mode cycle counter (`mcycle`) as a delay provider
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct McycleDelay {
    /// The clock speed of the core, in Hertz
    ticks_second: u32,
}

impl McycleDelay {
    /// Constructs the delay provider.
    /// `ticks_second` should be the clock speed of the core, in Hertz
    #[inline]
    pub const fn new(ticks_second: u32) -> Self {
        Self { ticks_second }
    }
}

impl DelayNs for McycleDelay {
    #[inline]
    fn delay_ns(&mut self, ns: u32) {
        let t0 = mcycle::read();
        let ns_64: u64 = ns.into();
        let clock = ((ns_64 * (self.ticks_second as u64)) / 1_000_000_000u64) as usize;
        while mcycle::read().wrapping_sub(t0) <= clock {}
    }
}
