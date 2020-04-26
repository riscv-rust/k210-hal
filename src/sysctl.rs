//! (TODO) System Controller (SYSCTL)

pub trait SysctlExt {
    fn split(self) -> Parts;
}

// ref: sysctl.c
pub struct Parts {
    // todo: PLL0, PLL1, PLL2
    // todo: CPU, SRAM, APB-bus, ROM, DMA, AI
    // pub apb0: APB0,
    // pub apb1: APB1,
    // pub apb2: APB2,
}

// pub struct APB0 {
//     _ownership: ()
// }

// pub struct APB1 {
//     _ownership: ()
// }

// pub struct APB2 {
//     _ownership: ()
// }
