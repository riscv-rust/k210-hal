//! High-speed GPIO peripheral (GPIOHS)

use crate::pac;
use crate::fpioa::Mode;
use core::marker::PhantomData;
use crate::bit_utils::{u32_set_bit, u32_toggle_bit, u32_bit_is_set, u32_bit_is_clear};
use embedded_hal::digital::v2::{InputPin, OutputPin};

pub trait GpiohsIndex {
    type FUNC;
    const INDEX: u8;
}

trait GpiohsAccess {
    fn peripheral() -> &'static mut pac::gpiohs::RegisterBlock;

    fn set_drive(index: usize, bit: bool) {
        unsafe { 
            let p = &mut Self::peripheral().drive as *mut _ as *mut _; 
            u32_set_bit(p, bit, index);
        }
    }

    fn input_value(index: usize) -> bool {
        unsafe { 
            let p = &mut Self::peripheral().input_val as *mut _ as *mut _; 
            u32_bit_is_set(p, index)
        }
    }

    fn set_input_en(index: usize, bit: bool) {
        unsafe { 
            let p = &mut Self::peripheral().input_en as *mut _ as *mut _; 
            u32_set_bit(p, bit, index);
        }
    }

    fn set_iof_en(index: usize, bit: bool) {
        unsafe { 
            let p = &mut Self::peripheral().iof_en as *mut _ as *mut _; 
            u32_set_bit(p, bit, index);
        }
    }

    fn set_iof_sel(index: usize, bit: bool) {
        unsafe { 
            let p = &mut Self::peripheral().iof_sel as *mut _ as *mut _; 
            u32_set_bit(p, bit, index);
        }
    }

    fn set_output_en(index: usize, bit: bool) {
        unsafe { 
            let p = &mut Self::peripheral().output_en as *mut _ as *mut _; 
            u32_set_bit(p, bit, index);
        }
    }

    fn set_output_value(index: usize, bit: bool) {
        unsafe { 
            let p = &mut Self::peripheral().output_val as *mut _ as *mut _; 
            u32_set_bit(p, bit, index);
        }
    }

    fn set_output_xor(index: usize, bit: bool) {
        unsafe { 
            let p = &mut Self::peripheral().output_xor as *mut _ as *mut _; 
            u32_set_bit(p, bit, index);
        }
    }

    fn toggle_pin(index: usize) {
        unsafe { 
            let p = &mut Self::peripheral().output_val as *mut _ as *mut _; 
            u32_toggle_bit(p, index);
        }
    }

    fn set_pullup_en(index: usize, bit: bool) {
        unsafe { 
            let p = &mut Self::peripheral().pullup_en as *mut _ as *mut _; 
            u32_set_bit(p, bit, index);
        }
    }

    // todo: {high, low, fall, rise}_{ie, ip}
}

/// Unknown mode (type state)
pub struct Unknown;

/// Input mode (type state)
pub struct Input<MODE>(MODE);

pub struct PullUp;

/// Output mode (type state)
pub struct Output<MODE>(MODE);

pub trait GpiohsExt {
    fn split(self) -> Parts;
}

impl GpiohsExt for pac::GPIOHS {
    fn split(self) -> Parts {
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
        // let r: &mut u32 = unsafe { &*(&(*pac::GPIOHS::ptr()).input_en as *const _ as *const _) };
        // u32_set_bit(r, true, GPIOHS::INDEX as usize);
        // let r: &mut u32 = unsafe { &*(&(*pac::GPIOHS::ptr()).output_en as *const _ as *const _) };
        // u32_set_bit(r, false, GPIOHS::INDEX as usize);
        unsafe {
            let ptr = pac::GPIOHS::ptr();
            (*ptr)
                .output_en
                .modify(|r, w| 
                    w.bits(r.bits() & !(1 << (GPIOHS::INDEX as usize))));
            (*ptr)
                .input_en
                .modify(|r, w| 
                    w.bits(r.bits() | (1 << (GPIOHS::INDEX as usize))));
        }
        Gpiohs { gpiohs: self.gpiohs, pin: self.pin, _mode: PhantomData }
    }

    // todo: all modes
}

impl<GPIOHS: GpiohsIndex, PIN, MODE> InputPin for Gpiohs<GPIOHS, PIN, Input<MODE>> {
    type Error = core::convert::Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> { 
        Ok(unsafe { 
            let p = &(*pac::GPIOHS::ptr()).input_val as *const _ as *const _;
            u32_bit_is_set(p, GPIOHS::INDEX as usize)
        })
    }

    fn is_low(&self) -> Result<bool, Self::Error> { 
        Ok(unsafe { 
            let p = &(*pac::GPIOHS::ptr()).input_val as *const _ as *const _;
            u32_bit_is_clear(p, GPIOHS::INDEX as usize)
        })
    }
}

impl<GPIOHS: GpiohsIndex, PIN, MODE> OutputPin for Gpiohs<GPIOHS, PIN, Output<MODE>> {
    type Error = core::convert::Infallible;

    fn set_high(&mut self) -> Result<(), Self::Error> {
        unsafe { 
            let p = &(*pac::GPIOHS::ptr()).output_val as *const _ as *mut _;
            u32_set_bit(p, true, GPIOHS::INDEX as usize);
        }
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        unsafe { 
            let p = &(*pac::GPIOHS::ptr()).output_val as *const _ as *mut _;
            u32_set_bit(p, false, GPIOHS::INDEX as usize);
        }
        Ok(())
    }
}
