//! (TODO) System Controller (SYSCTL)

use crate::clock::Clocks;
use crate::pac::{sysctl, SYSCTL};
use crate::time::Hertz;
use core::sync::atomic::Ordering;

const CLOCK_FREQ_IN0: u32 = 26_000_000;

pub(crate) fn sysctl<'a>() -> &'a sysctl::RegisterBlock {
    unsafe { &*(SYSCTL::ptr()) }
}

pub(crate) fn clk_en_cent<'a>() -> &'a sysctl::CLK_EN_CENT {
    &sysctl().clk_en_cent
}

pub(crate) fn clk_en_peri<'a>() -> &'a sysctl::CLK_EN_PERI {
    &sysctl().clk_en_peri
}

pub(crate) fn peri_reset<'a>() -> &'a sysctl::PERI_RESET {
    &sysctl().peri_reset
}

/// Accept freq_in as the input frequency,
/// and try to find a set of parameters (nr, od, nf),
/// which results in a frequency as near as possible to freq
/// note for a PLL:
///   freq_out = freq_in / nr * nf / od
/// The reason why we don't port the complex config algorithm from the
/// official C language SDK is that doing floating number arithmetics
/// efficiently in no_std rust now is currently not very convenient
fn calculate_pll_config(freq_in: u32, freq: u32) -> (u8, u8, u8) {
    // finding a set of (nr, od, nf) which minify abs(freq * nr * od - freq_in * nf)
    // nr, od is in [0b1,    0b10_000], nr * od is in [0b1,   0b100_000_000]
    // nf     is in [0b1, 0b1_000_000]

    // for archiving a higher accuracy, we want the nr * od as large as possible
    // use binary search to find the largest nr_od which freq <= freq_in * 0b1_000_000 / nr_od
    let mut left = 1;
    let mut right = 0b10_000 * 0b10_000 + 1;
    while left + 1 < right {
        let mid = (left + right) / 2;
        let max_freq = freq_in * 0b1_000_000 / mid;
        if freq >= max_freq {
            // in [left, mid)
            right = mid;
        } else {
            // in [mid, right)
            left = mid;
        }
    }
    let nr_od = left;
    // so we got nf
    let nf = freq * nr_od / freq_in;
    let nf = nf.min(0b1_000_000) as u8;

    // decompose nr_od
    for nr in 1..=0b10_000 {
        if (nr_od / nr) * nr == nr_od {
            return (nr as u8, (nr_od / nr) as u8, nf);
        }
    }
    unreachable!()
}

pub trait SysctlExt {
    fn constrain(self) -> Parts;
}

impl SysctlExt for SYSCTL {
    fn constrain(self) -> Parts {
        Parts {
            aclk: ACLK { _ownership: () },
            apb0: APB0 { _ownership: () },
            apb2: APB2 { _ownership: () },
            pll0: PLL0 { _ownership: () },
            spi0: SPI0 { _ownership: () },
            spi1: SPI1 { _ownership: () },
        }
    }
}

// ref: sysctl.c
pub struct Parts {
    /// entry for controlling the frequency of aclk
    pub aclk: ACLK,
    /// entry for controlling the enable/disable/frequency of pll0
    pub pll0: PLL0,
    /// entry for controlling the enable/disable/frequency of apb0
    pub apb0: APB0,
    /// entry for controlling the enable/disable/frequency of apb2
    pub apb2: APB2,
    /// entry for controlling the enable/disable/frequency of spi0
    pub spi0: SPI0,
    /// entry for controlling the enable/disable/frequency of spi1
    pub spi1: SPI1,
    // todo: SRAM, APB-bus, ROM, DMA, AI, PLL1, PLL2, APB1, APB2
}

impl Parts {
    pub fn clocks(&self) -> Clocks {
        Clocks {
            aclk: self.aclk.get_frequency(),
            apb0: self.apb0.get_frequency(),
        }
    }
}

pub struct APB0 {
    _ownership: (),
}

impl APB0 {
    pub(crate) fn enable(&mut self) {
        clk_en_cent().modify(|_r, w| w.apb0_clk_en().set_bit());
    }

    pub fn set_frequency(&mut self, expected_freq: impl Into<Hertz>) -> Hertz {
        let aclk = ACLK::steal();
        let aclk_frequency = aclk.get_frequency().0 as i64;
        // apb0_frequency = aclk_frequency / (apb0_clk_sel + 1)
        let apb0_clk_sel = (aclk_frequency / expected_freq.into().0 as i64 - 1)
            .max(0)
            .min(0b111) as u8;
        unsafe {
            sysctl()
                .clk_sel0
                .modify(|_, w| w.apb0_clk_sel().bits(apb0_clk_sel));
        }
        Hertz(aclk_frequency as u32 / (apb0_clk_sel as u32 + 1))
    }

    pub fn get_frequency(&self) -> Hertz {
        let aclk = ACLK::steal();
        let aclk_frequency = aclk.get_frequency().0 as i64;
        let apb0_clk_sel = sysctl().clk_sel0.read().apb0_clk_sel().bits();
        Hertz(aclk_frequency as u32 / (apb0_clk_sel as u32 + 1))
    }
}

// pub struct APB1 {
//     _ownership: ()
// }

pub struct APB2 {
    _ownership: (),
}

impl APB2 {
    pub(crate) fn enable(&mut self) {
        clk_en_cent().modify(|_r, w| w.apb2_clk_en().set_bit());
    }

    pub fn set_frequency(&mut self, expected_freq: impl Into<Hertz>) -> Hertz {
        let aclk = ACLK::steal();
        let aclk_frequency = aclk.get_frequency().0 as i64;
        // apb2_frequency = aclk_frequency / (apb2_clk_sel + 1)
        let apb2_clk_sel = (aclk_frequency / expected_freq.into().0 as i64 - 1)
            .max(0)
            .min(0b111) as u8;
        unsafe {
            sysctl()
                .clk_sel0
                .modify(|_, w| w.apb2_clk_sel().bits(apb2_clk_sel));
        }
        Hertz(aclk_frequency as u32 / (apb2_clk_sel as u32 + 1))
    }

    pub fn get_frequency(&self) -> Hertz {
        let aclk = ACLK::steal();
        let aclk_frequency = aclk.get_frequency().0 as i64;
        let apb2_clk_sel = sysctl().clk_sel0.read().apb2_clk_sel().bits();
        Hertz(aclk_frequency as u32 / (apb2_clk_sel as u32 + 1))
    }
}

/// PLL0, which source is CLOCK_FREQ_IN0,
/// and the output can be used on ACLK(CPU), SPIs, etc.
pub struct PLL0 {
    _ownership: (),
}

impl PLL0 {
    pub(crate) fn steal() -> Self {
        PLL0 { _ownership: () }
    }

    #[inline(always)]
    fn is_locked(&self) -> bool {
        sysctl().pll_lock.read().pll_lock0() == 0b11
    }

    fn lock(&mut self) {
        while !self.is_locked() {
            sysctl()
                .pll_lock
                .modify(|_, w| w.pll_slip_clear0().set_bit())
        }
    }

    #[inline(always)]
    fn reset(&mut self) {
        sysctl().pll0.modify(|_, w| w.reset().clear_bit());
        sysctl().pll0.modify(|_, w| w.reset().set_bit());
        core::sync::atomic::compiler_fence(Ordering::SeqCst);
        core::sync::atomic::compiler_fence(Ordering::SeqCst);
        sysctl().pll0.modify(|_, w| w.reset().clear_bit());
    }

    /// enable PLL0
    pub fn enable(&mut self) {
        sysctl()
            .pll0
            .modify(|_, w| w.bypass().clear_bit().pwrd().set_bit());
        self.reset();
        self.lock();
        sysctl().pll0.modify(|_, w| w.out_en().set_bit());
    }

    /// disable PLL0
    /// use with caution: PLL0 can be used as source clock of ACLK (so also CPU),
    /// if you want to disable PLL0, please make the cpu use external clock first
    pub fn disable(&mut self) {
        sysctl()
            .pll0
            .modify(|_, w| w.bypass().set_bit().pwrd().clear_bit().out_en().clear_bit());
    }

    /// Set frequency of PLL0
    /// Will set the frequency of PLL0 as close to frequency as possible
    /// Return the real frequency of the PLL0
    pub fn set_frequency(&mut self, frequency: impl Into<Hertz>) -> Hertz {
        let is_aclk_using = sysctl().clk_sel0.read().aclk_sel().bit();
        if is_aclk_using {
            sysctl().clk_sel0.modify(|_, w| w.aclk_sel().clear_bit());
        }
        self.disable();
        let (nr, od, nf) = calculate_pll_config(CLOCK_FREQ_IN0, frequency.into().0);
        unsafe {
            sysctl().pll0.modify(|_, w| {
                w.clkr()
                    .bits(nr - 1)
                    .clkf()
                    .bits(nf - 1)
                    .clkod()
                    .bits(od - 1)
                    .bwadj()
                    .bits(nf - 1)
            });
        }
        self.enable();
        // recover aclk_sel
        if is_aclk_using {
            sysctl().clk_sel0.modify(|_, w| w.aclk_sel().set_bit());
        }
        Hertz(CLOCK_FREQ_IN0 / nr as u32 * nf as u32 / od as u32)
    }

    /// Return the frequency of PLL0
    pub fn get_frequency(&self) -> Hertz {
        let nr = sysctl().pll0.read().clkr().bits() + 1;
        let nf = sysctl().pll0.read().clkf().bits() + 1;
        let od = sysctl().pll0.read().clkod().bits() + 1;
        Hertz(CLOCK_FREQ_IN0 / nr as u32 * nf as u32 / od as u32)
    }
}

pub struct ACLK {
    _ownership: (),
}

/// ACLK clock frequency control
impl ACLK {
    pub(crate) fn steal() -> Self {
        ACLK { _ownership: () }
    }

    /// make ACLK use external clock, ie. CLOCK_FREQ_IN0
    pub(crate) fn use_external(&mut self) {
        sysctl().clk_sel0.modify(|_, w| w.aclk_sel().clear_bit());
    }

    /// Return whether the ACLK is using external clock
    pub fn is_using_external(&self) -> bool {
        !sysctl().clk_sel0.read().aclk_sel().bit()
    }

    /// make ACLK use pll0 clock, with aclk_divider_sel
    pub fn use_pll0(&mut self, aclk_divider_sel: u8) {
        unsafe {
            sysctl().clk_sel0.modify(|_, w| {
                w.aclk_divider_sel()
                    .bits(aclk_divider_sel)
                    .aclk_sel()
                    .set_bit()
            });
        }
    }

    /// Set the frequency of ACLK
    /// if frequency == CLOCK_FREQ_IN0, use external clock directly
    /// else frequency settings here are based on existing settings on PLL0
    /// We won't adjust PLL0 here because there are so many devices based on it.
    pub fn set_frequency(&mut self, expected_freq: impl Into<Hertz>) -> Hertz {
        let expected_freq = expected_freq.into().0;
        if expected_freq == CLOCK_FREQ_IN0 {
            self.use_external();
            Hertz(CLOCK_FREQ_IN0)
        } else {
            // aclk = pll0 / (2 << aclk_divider_sel)
            let pll0 = PLL0::steal().get_frequency().0;
            let mut aclk_divider_sel = 0u8;
            // aclk_divider_sel is 2 bits
            if expected_freq < pll0 / (2 << 0b11) {
                aclk_divider_sel = 0b11;
            } else {
                for i in 0b00u8..0b11 {
                    if pll0 / (2 << i) <= expected_freq {
                        aclk_divider_sel = i;
                        break;
                    }
                }
            }
            self.use_pll0(aclk_divider_sel);
            Hertz(pll0 / (2 << aclk_divider_sel))
        }
    }

    /// Get the frequency of ACLK
    pub fn get_frequency(&self) -> Hertz {
        if self.is_using_external() {
            Hertz(CLOCK_FREQ_IN0)
        } else {
            let pll0 = PLL0::steal().get_frequency().0;
            let aclk_divider_sel = sysctl().clk_sel0.read().aclk_divider_sel().bits();
            Hertz(pll0 / (2 << aclk_divider_sel))
        }
    }
}

pub struct SPI0 {
    _ownership: (),
}

impl SPI0 {
    pub fn set_frequency(&mut self, expected_freq: impl Into<Hertz>) -> Hertz {
        let expected_freq = expected_freq.into().0;
        // spi0 = source(pll0) / ((spi0_clk_threshold + 1) * 2)
        let source = PLL0::steal().get_frequency().0;
        let spi0_clk_threshold = (source / expected_freq / 2 - 1).min(u8::max_value() as _) as u8;
        unsafe {
            sysctl()
                .clk_th1
                .modify(|_, w| w.spi0_clk().bits(spi0_clk_threshold));
        }
        Hertz(source / ((spi0_clk_threshold as u32 + 1) * 2))
    }
    pub fn get_frequency(&self) -> Hertz {
        let source = PLL0::steal().get_frequency().0;
        let spi0_clk_threshold = sysctl().clk_th1.read().spi0_clk().bits() as u32;
        Hertz(source / ((spi0_clk_threshold as u32 + 1) * 2))
    }
}

pub struct SPI1 {
    _ownership: (),
}

impl SPI1 {
    pub fn set_frequency(&mut self, expected_freq: impl Into<Hertz>) -> Hertz {
        let expected_freq = expected_freq.into().0;
        // spi1 = source(pll0) / ((spi1_clk_threshold + 1) * 2)
        let source = PLL0::steal().get_frequency().0;
        let spi1_clk_threshold = (source / expected_freq / 2 - 1).min(u8::max_value() as _) as u8;
        unsafe {
            sysctl()
                .clk_th1
                .modify(|_, w| w.spi1_clk().bits(spi1_clk_threshold));
        }
        Hertz(source / ((spi1_clk_threshold as u32 + 1) * 2))
    }
    pub fn get_frequency(&self) -> Hertz {
        let source = PLL0::steal().get_frequency().0;
        let spi1_clk_threshold = sysctl().clk_th1.read().spi1_clk().bits() as u32;
        Hertz(source / ((spi1_clk_threshold as u32 + 1) * 2))
    }
}
