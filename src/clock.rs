//! Clock configuration
//use crate::pac::PRCI;
// use crate::sysctl::ACLK;
use crate::time::Hertz;

/// Frozen clock frequencies
///
/// The existence of this value indicates that the clock configuration can no
/// longer be changed.
#[derive(Clone, Copy)]
pub struct Clocks {
    pub(crate) aclk: Hertz,
    pub(crate) apb0: Hertz,
    pub(crate) apb1: Hertz,
}

impl Clocks {
    #[doc(hidden)]
    pub fn new() -> Self {
        /*
                [MAIXPY]Pll0:freq:806000000
                [MAIXPY]Pll1:freq:398666666
                [MAIXPY]Pll2:freq:45066666
                [MAIXPY]cpu:freq:403000000
                [MAIXPY]kpu:freq:398666666
                in freq: 26000000
                cpu_freq: 390000000
        */
        Self {
            aclk: Hertz(390_000_000),
            apb0: Hertz(195_000_000),
            apb1: Hertz(195_000_000),
        }
    }

    /// Returns CPU frequency
    pub fn cpu(&self) -> Hertz {
        Hertz(self.aclk.0)
    }

    /// Returns APB0 frequency
    pub fn apb0(&self) -> Hertz {
        self.apb0
    }

    /// Returns APB1 frequency
    pub fn apb1(&self) -> Hertz {
        self.apb1
    }
}
