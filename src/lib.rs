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
    pub use embedded_hal::digital::v2::OutputPin as _embedded_hal_digital_v2_OutputPin;
    pub use crate::serial::SerialExt as _k210_hal_serial_SerialExt;
    pub use crate::stdout::Write as _k210_hal_stdout_Write;
    pub use crate::time::U32Ext as _k210_hal_time_U32Ext;
    pub use crate::fpioa::FpioaExt as _k210_hal_fpioa_FpioaExt;
    pub use crate::sysctl::SysctlExt as _k210_hal_sysctl_SysctlExt;
    pub use crate::gpio::GpioExt as _k210_hal_gpio_GpioExt;
}
