use k210_pac::{CLINT, PLIC, UARTHS, GPIOHS, KPU, FFT, DMAC, GPIO, UART1, UART2, UART3, SPI0, SPI1, SPI2, SPI3, I2S0, APU, I2S1, I2S2, I2C0, I2C1, I2C2, SHA256, TIMER0, TIMER1, TIMER2, WDT0, WDT1, OTP, DVP, SYSCTL, AES, RTC};
use crate::external_pins::ExternalPins;

/// All the peripherals
#[allow(non_snake_case)]
pub struct Peripherals {
    // All the peripherals except FPIOA
    #[doc = "CLINT"]
    pub CLINT: CLINT,
    #[doc = "PLIC"]
    pub PLIC: PLIC,
    #[doc = "UARTHS"]
    pub UARTHS: UARTHS,
    #[doc = "GPIOHS"]
    pub GPIOHS: GPIOHS,
    #[doc = "KPU"]
    pub KPU: KPU,
    #[doc = "FFT"]
    pub FFT: FFT,
    #[doc = "DMAC"]
    pub DMAC: DMAC,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "UART2"]
    pub UART2: UART2,
    #[doc = "UART3"]
    pub UART3: UART3,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "SPI2"]
    pub SPI2: SPI2,
    #[doc = "SPI3"]
    pub SPI3: SPI3,
    #[doc = "I2S0"]
    pub I2S0: I2S0,
    #[doc = "APU"]
    pub APU: APU,
    #[doc = "I2S1"]
    pub I2S1: I2S1,
    #[doc = "I2S2"]
    pub I2S2: I2S2,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "SHA256"]
    pub SHA256: SHA256,
    #[doc = "TIMER0"]
    pub TIMER0: TIMER0,
    #[doc = "TIMER1"]
    pub TIMER1: TIMER1,
    #[doc = "TIMER2"]
    pub TIMER2: TIMER2,
    #[doc = "WDT0"]
    pub WDT0: WDT0,
    #[doc = "WDT1"]
    pub WDT1: WDT1,
    #[doc = "OTP"]
    pub OTP: OTP,
    #[doc = "DVP"]
    pub DVP: DVP,
    #[doc = "SYSCTL"]
    pub SYSCTL: SYSCTL,
    #[doc = "AES"]
    pub AES: AES,
    #[doc = "RTC"]
    pub RTC: RTC,

    /// External pins
    pub pins: ExternalPins,
}

impl Peripherals {
    fn construct(p: k210_pac::Peripherals) -> Self {
        Peripherals {
            CLINT: p.CLINT,
            PLIC: p.PLIC,
            UARTHS: p.UARTHS,
            GPIOHS: p.GPIOHS,
            KPU: p.KPU,
            FFT: p.FFT,
            DMAC: p.DMAC,
            GPIO: p.GPIO,
            UART1: p.UART1,
            UART2: p.UART2,
            UART3: p.UART3,
            SPI0: p.SPI0,
            SPI1: p.SPI1,
            SPI2: p.SPI2,
            SPI3: p.SPI3,
            I2S0: p.I2S0,
            APU: p.APU,
            I2S1: p.I2S1,
            I2S2: p.I2S2,
            I2C0: p.I2C0,
            I2C1: p.I2C1,
            I2C2: p.I2C2,
            SHA256: p.SHA256,
            TIMER0: p.TIMER0,
            TIMER1: p.TIMER1,
            TIMER2: p.TIMER2,
            WDT0: p.WDT0,
            WDT1: p.WDT1,
            OTP: p.OTP,
            DVP: p.DVP,
            SYSCTL: p.SYSCTL,
            AES: p.AES,
            RTC: p.RTC,
            pins: ExternalPins::new(),
        }
    }

    /// Returns all the peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        k210_pac::Peripherals::take().map(Peripherals::construct)
    }

    /// Unchecked version of `Peripherals::take`
    pub unsafe fn steal() -> Self {
        Peripherals::construct(k210_pac::Peripherals::steal())
    }
}
