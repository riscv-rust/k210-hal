//! HAL for the K210 SoC
//!
//! This is an implementation of the [`embedded-hal`] traits for the K210 SoC

// #![deny(missing_docs)] // uncomment for every releases
#![no_std]

pub use k210_pac as pac;

pub mod aes;
pub mod apu;
pub mod cache;
pub mod clint;
pub mod clock;
pub mod dmac;
pub mod fft;
pub mod fpioa;
pub mod gpio;
pub mod gpiohs;
pub mod plic;
pub mod serial;
pub mod sha256;
pub mod spi;
pub mod stdout;
pub mod sysctl;
pub mod time;

/// Prelude
pub mod prelude {
    pub use crate::{
        fpioa::FpioaExt as _k210_hal_fpioa_FpioaExt, gpio::GpioExt as _k210_hal_gpio_GpioExt,
        gpiohs::GpiohsExt as _k210_hal_gpiohs_GpiohsExt, plic::PlicExt as _k210_hal_plic_PlicExt,
        serial::SerialExt as _k210_hal_serial_SerialExt, stdout::Write as _k210_hal_stdout_Write,
        sysctl::SysctlExt as _k210_hal_sysctl_SysctlExt, time::U32Ext as _k210_hal_time_U32Ext,
    };
    pub use embedded_hal::prelude::*;
}

mod bit_utils {
    #[inline(always)]
    pub(crate) unsafe fn u32_set_bit(p: *mut u32, bit: bool, index: usize) {
        let mask = 1 << index;
        if bit {
            *p |= mask;
        } else {
            *p &= !mask;
        }
    }

    #[inline(always)]
    pub(crate) unsafe fn u32_toggle_bit(p: *mut u32, index: usize) {
        let mask = 1 << index;
        *p ^= mask;
    }

    #[inline(always)]
    pub(crate) unsafe fn u32_bit_is_set(r: *const u32, index: usize) -> bool {
        (*r & 1 << index) != 0
    }

    #[inline(always)]
    pub(crate) unsafe fn u32_bit_is_clear(r: *const u32, index: usize) -> bool {
        (*r & 1 << index) == 0
    }
}
