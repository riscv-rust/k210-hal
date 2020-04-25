//! HAL for the K210 SoC
//!
//! This is an implementation of the [`embedded-hal`] traits for the K210 SoC

// #![deny(missing_docs)] // uncomment for every releases
#![no_std]

pub use k210_pac as pac;

pub mod clock;
pub mod fpioa;
pub mod gpio;
pub mod serial;
pub mod stdout;
pub mod sysctl;
pub mod time;

/// Prelude
pub mod prelude {
    pub use embedded_hal::prelude::*;
    pub use embedded_hal::digital::v2::{
        InputPin as _embedded_hal_digital_v2_InputPin,
        OutputPin as _embedded_hal_digital_v2_OutputPin,
        StatefulOutputPin as _embedded_hal_digital_v2_StatefulOutputPin,
        ToggleableOutputPin as _embedded_hal_digital_v2_ToggleableOutputPin,
    };
    pub use crate::serial::SerialExt as _k210_hal_serial_SerialExt;
    pub use crate::stdout::Write as _k210_hal_stdout_Write;
    pub use crate::time::U32Ext as _k210_hal_time_U32Ext;
    pub use crate::fpioa::FpioaExt as _k210_hal_fpioa_FpioaExt;
    pub use crate::sysctl::SysctlExt as _k210_hal_sysctl_SysctlExt;
    pub use crate::gpio::GpioExt as _k210_hal_gpio_GpioExt;
}

mod atomic {
    use core::sync::atomic::{AtomicU32, Ordering};
    // This function uses AtomicU32, compiles into atomic instructions to prevent data race
    // and optimize for speed.
    //
    // If we don't do like this, we would need to go into critical section, where additional
    // interrupt disabling and enabling operations are required, which needs lots of CSR
    // read/write instructions and costs lot of time.
    //
    // For all `is_one: true` params, the core feature of this function compiles into
    // only one atomic instruction `amoor.w` to set the target register.
    // (For `is_one: false` params, it compiles into ont `amoand.w`).
    // Additional instructions to set the mask may differ between actual applications,
    // this part may cost additional one to two instructions (mainly `lui` and `addi`).
    //
    // Note: we uses `fetch_and(!mask, ...)` instead of `fetch_nand(mask, ...)`; that's
    // because RISC-V's RV32A does not provide an atomic nand instruction, thus `rustc`
    // may compile code into very long binary output.
    #[inline(always)]
    pub(crate) fn u32_atomic_set_bit(r: &AtomicU32, is_one: bool, index: usize) {
        let mask = 1 << index;
        if is_one {
            r.fetch_or(mask, Ordering::Relaxed);
        } else {
            r.fetch_and(!mask, Ordering::Relaxed);
        }
    }

    // This function compiles into RV32A's `amoxor.w` instruction to prevent data
    // race as well as optimize for speed.
    #[inline(always)]
    pub(crate) fn u32_atomic_toggle_bit(r: &AtomicU32, index: usize) {
        let mask = 1 << index;
        r.fetch_xor(mask, Ordering::Relaxed);
    }
}
