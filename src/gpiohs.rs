//! High-speed GPIO peripheral (GPIOHS)

use crate::pac;
use crate::fpioa::Mode;
use core::sync::atomic::AtomicU32;
use core::mem::transmute;
use core::marker::PhantomData;
use crate::bit_utils::{u32_atomic_set_bit, u32_atomic_toggle_bit, u32_bit_is_set, u32_bit_is_clear};
use embedded_hal::digital::v2::InputPin;

pub trait GpiohsIndex {
    type FUNC;
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

/// Unknown mode (type state)
pub struct Unknown;

/// Input mode (type state)
pub struct Input<MODE>(MODE);

pub struct PullUp;

pub trait GpiohsExt {
    // todo: use &mut APB0
    fn split(self) -> Parts;
}

impl GpiohsExt for pac::GPIOHS {
    fn split(self) -> Parts {
        // todo: enable clock
        Parts { 
            gpiohs0: GPIOHS0 { _ownership: () },
        }
    }
}

pub struct Parts {
    pub gpiohs0: GPIOHS0,
}

pub struct GPIOHS0 {
    _ownership: (),
}

impl GpiohsIndex for GPIOHS0 {
    type FUNC = crate::fpioa::functions::GPIOHS0;
    const INDEX: u8 = 0;
}

pub struct Gpiohs<GPIOHS, PIN, MODE> {
    gpiohs: GPIOHS,
    pin: PIN,
    _mode: PhantomData<MODE>,
}

impl<GPIOHS: GpiohsIndex, PIN: Mode<GPIOHS::FUNC>> Gpiohs<GPIOHS, PIN, Unknown> {
    pub fn new(gpiohs: GPIOHS, pin: PIN) -> Gpiohs<GPIOHS, PIN, Unknown> {
        Gpiohs { gpiohs, pin, _mode: PhantomData }
    }
}

impl<GPIOHS, PIN, MODE> Gpiohs<GPIOHS, PIN, MODE> {
    pub fn free(self) -> (GPIOHS, PIN) {
        (self.gpiohs, self.pin)
    }
}

use crate::fpioa::{Pull, IoPin};

impl<GPIOHS: GpiohsIndex, PIN: IoPin, MODE> Gpiohs<GPIOHS, PIN, MODE> {
    pub fn into_pull_up_input(mut self) -> Gpiohs<GPIOHS, PIN, Input<PullUp>> {
        self.pin.set_io_pull(Pull::Up);
        let r: &AtomicU32 = unsafe { &*(&(*pac::GPIO::ptr()).direction as *const _ as *const _) };
        u32_atomic_set_bit(r, false, GPIOHS::INDEX as usize);
        Gpiohs { gpiohs: self.gpiohs, pin: self.pin, _mode: PhantomData }
    }
    // todo: all modes
}

impl<GPIOHS: GpiohsIndex, PIN, MODE> InputPin for Gpiohs<GPIOHS, PIN, Input<MODE>> {
    type Error = core::convert::Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> { 
        let r: &u32 = unsafe { &*(&(*pac::GPIOHS::ptr()).input_val as *const _ as *const _) };
        Ok(u32_bit_is_set(r, GPIOHS::INDEX as usize))
    }

    fn is_low(&self) -> Result<bool, Self::Error> { 
        let r: &u32 = unsafe { &*(&(*pac::GPIOHS::ptr()).input_val as *const _ as *const _) };
        Ok(u32_bit_is_clear(r, GPIOHS::INDEX as usize))
    }
}
