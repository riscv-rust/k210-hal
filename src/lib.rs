//! HAL for the K210 SoC
//!
//! This is an implementation of the [`embedded-hal`] traits for the K210 SoC

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

pub use k210_pac as pac;

pub mod clock;
pub mod prelude;
pub mod serial;
pub mod stdout;
pub mod time;
