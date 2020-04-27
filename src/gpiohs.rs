//! High-speed GPIO peripheral (GPIOHS)

use crate::pac;
use core::sync::atomic::AtomicU32;
use core::mem::transmute;
use crate::bit_utils::{u32_atomic_set_bit, u32_atomic_toggle_bit};

pub trait GpiohsIndex {
    const INDEX: u8;
}

trait GpiohsAccess {
    fn peripheral() -> &'static pac::gpiohs::RegisterBlock;

    fn set_drive(index: usize, bit: bool) {
        let p = Self::peripheral();
        let r: &AtomicU32 = unsafe { transmute(&p.drive) };
        u32_atomic_set_bit(r, bit, index);
    }

    fn input_value(index: usize) -> bool {
        let p = Self::peripheral();
        (p.input_val.read().bits() >> (index & 31) & 1) != 0
    }

    fn set_input_en(index: usize, bit: bool) {
        let p = Self::peripheral();
        let r: &AtomicU32 = unsafe { transmute(&p.input_en) };
        u32_atomic_set_bit(r, bit, index);
    }

    fn set_iof_en(index: usize, bit: bool) {
        let p = Self::peripheral();
        let r: &AtomicU32 = unsafe { transmute(&p.iof_en) };
        u32_atomic_set_bit(r, bit, index);
    }

    fn set_iof_sel(index: usize, bit: bool) {
        let p = Self::peripheral();
        let r: &AtomicU32 = unsafe { transmute(&p.iof_sel) };
        u32_atomic_set_bit(r, bit, index);
    }

    fn set_output_en(index: usize, bit: bool) {
        let p = Self::peripheral();
        let r: &AtomicU32 = unsafe { transmute(&p.output_en) };
        u32_atomic_set_bit(r, bit, index);
    }

    fn set_output_value(index: usize, bit: bool) {
        let p = Self::peripheral();
        let r: &AtomicU32 = unsafe { transmute(&p.output_val) };
        u32_atomic_set_bit(r, bit, index);
    }

    fn set_output_xor(index: usize, bit: bool) {
        let p = Self::peripheral();
        let r: &AtomicU32 = unsafe { transmute(&p.output_xor) };
        u32_atomic_set_bit(r, bit, index);
    }

    fn toggle_pin(index: usize) {
        let p = Self::peripheral();
        let r: &AtomicU32 = unsafe { transmute(&p.output_val) };
        u32_atomic_toggle_bit(r, index);
    }

    fn set_pullup_en(index: usize, bit: bool) {
        let p = Self::peripheral();
        let r: &AtomicU32 = unsafe { transmute(&p.pullup_en) };
        u32_atomic_set_bit(r, bit, index);
    }

    // todo: {high, low, fall, rise}_{ie, ip}
}
