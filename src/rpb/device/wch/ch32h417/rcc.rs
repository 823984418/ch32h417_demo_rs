use bitflags::bitflags;

use crate::rpb::register::{Reg, Reg32};

#[repr(C)]
pub struct RCC {
    pub CTLR: Reg<CTLR>,
    pub CFGR0: Reg<CFGR0>,
    pub PLLCFGR: Reg32,
    pub INTR: Reg32,
    pub HB2PRSTR: Reg32,
    pub HB1PRSTR: Reg32,
    pub HBPCENR: Reg32,
    pub HB2PCENR: Reg<HB2PCENR>,
    pub HB1PCENR: Reg32,
    pub BDCTLR: Reg32,
    pub RSTSCKR: Reg32,
    pub HBRSTR: Reg32,
    pub CFGR2: Reg32,
    pub PLLCFGR2: Reg32,
}

bitflags! {
    pub struct CTLR: u32 {
        const CSS_HSE_DIS = 1 << 31;
        const SERDES_PLLRDY = 1 << 29;
        const SERDES_PLLON = 1 << 28;
        const ETH_PLLRDY = 1 << 27;
        const ETH_PLLON = 1 << 27;
        const PLLRDY = 1 << 26;
        const PLLON = 1 << 25;
        const USBSS_PLLRDY = 1 << 24;
        const USBSS_PLLON = 1 << 23;
        const USBHS_PLLRDY = 1 << 22;
        const USBHS_PLLON = 1 << 21;
        const CSSON = 1 << 20;
        const HSEBYP = 1 << 19;
        const HSERDY = 1 << 18;
        const HSEON = 1 << 17;
        const HSICAL_MASK = 0xFF << 8;
        const HSITRIM_MASK = 0x1F << 3;
        const HSIRDY = 1 << 1;
        const HSION = 1 << 0;
    }
}

bitflags! {
    pub struct CFGR0: u32 {
        const ADCSRC = 1 << 31;
        const ADC_DUTY_SEL = 1 << 30;
        const MCO_MASK = 0xF << 24;
        const UTMION = 1 << 23;
        const PIPEON = 1 << 22;
        const RGMIION = 1 << 21;
        const FPRE_MASK = 0x3 << 16;
        const ADCPRE_MASK = 0x3 << 14;
        const PPRE2_MASK = 0x7 << 11;
        const HPRE_MASK = 0xF << 4;

        const SWS_MASK = 0x3 << 2;
        const SWS_PLL = 0b10 << 2;
        const SWS_HSE = 0b01 << 2;
        const SWS_HSI = 0b00 << 2;

        const SW_MASK = 0x3 << 0;
        const SW_PLL = 0b10 << 0;
        const SW_HSE = 0b01 << 0;
        const SW_HSI = 0b00 << 0;
    }
}

bitflags! {
    pub struct HB2PCENR: u32 {
        const I3CEN = 1 << 31;
        const LTDCEN = 1 << 30;
        const GPHAEN = 1 << 27;
        const ECDCEN = 1 << 26;
        const DFSDMEN = 1 << 25;
        const OPCMEN = 1 << 23;
        const TIM12EN = 1 << 22;
        const TIM11EN = 1 << 21;
        const TIM10EN = 1 << 20;
        const TIM9EN = 1 << 19;
        const SDIOEN = 1 << 18;
        const SAIEN = 1 << 16;
        const I2C4EN = 1 << 15;
        const USART1EN = 1 << 14;
        const TIM8EN = 1 << 13;
        const SPI1EN = 1 << 12;
        const TIM1EN = 1 << 11;
        const ADC2EN = 1 << 10;
        const ADC1EN = 1 << 9;
        const IOPFEN = 1 << 7;
        const IOPEEN = 1 << 6;
        const IOPDEN = 1 << 5;
        const IOPCEN = 1 << 4;
        const IOPBEN = 1 << 3;
        const IOPAEN = 1 << 2;
        const HSADCEN = 1 << 1;
        const AFIOEN = 1 << 0;
    }
}

impl RCC {
    pub fn DeInit(&self) {
        self.CTLR.bit_or(CTLR::HSION);
        self.CFGR0.bit_clear(
            CFGR0::ADCSRC
                | CFGR0::ADC_DUTY_SEL
                | CFGR0::MCO_MASK
                | CFGR0::UTMION
                | CFGR0::RGMIION
                | CFGR0::FPRE_MASK
                | CFGR0::ADCPRE_MASK
                | CFGR0::PPRE2_MASK
                | CFGR0::from_bits_retain(0x7 << 8)
                | CFGR0::HPRE_MASK
                | CFGR0::SWS_MASK
                | CFGR0::SW_MASK,
        );
        // TODO:
    }
}
