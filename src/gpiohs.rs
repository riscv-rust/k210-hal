//! High-speed GPIO peripheral (GPIOHS)

use crate::pac;
use core::marker::PhantomData;
use crate::bit_utils::{u32_set_bit, u32_toggle_bit, u32_bit_is_set, u32_bit_is_clear};
use embedded_hal::digital::v2::{InputPin, OutputPin};

// todo: verify

/// Floating mode (type state)
pub struct Floating;

/// PullUp mode (type state)
pub struct PullUp;

/// Input mode (type state)
pub struct Input<MODE>(MODE);

/// Output mode (type state)
pub struct Output<MODE>(MODE);

pub trait GpiohsExt {
    fn split(self) -> Parts;
}

impl GpiohsExt for pac::GPIOHS {
    fn split(self) -> Parts {
        Parts { 
            gpiohs0: Gpiohs0 { _mode: PhantomData },
        }
    }
}

pub struct Parts {
    pub gpiohs0: Gpiohs0<Input<Floating>>,
}

pub struct Gpiohs0<MODE> {
    _mode: PhantomData<MODE>,
}

impl<MODE> Gpiohs0<MODE> {
    pub fn into_pull_up_input(self) -> Gpiohs0<Input<PullUp>> {
        pac::GPIOHS::set_output_en(0, false);
        pac::GPIOHS::set_input_en(0, true);
        pac::GPIOHS::set_pullup_en(0, true);
        Gpiohs0 { _mode: PhantomData }
    }

    // todo: all modes
}

bitflags::bitflags! {
    pub struct Edge: u8 {
        const RISING =  0b00000001;
        const FALLING = 0b00000010;
        const HIGH =    0b00000100;
        const LOW =     0b00001000;
    }
}

impl<MODE> Gpiohs0<MODE> {
    pub fn trigger_on_edge(&self, edge: Edge) {
        pac::GPIOHS::set_rise_ie(0, false);
        pac::GPIOHS::set_rise_ip(0, true);
        pac::GPIOHS::set_fall_ie(0, false);
        pac::GPIOHS::set_fall_ip(0, true);
        pac::GPIOHS::set_high_ie(0, false);
        pac::GPIOHS::set_high_ip(0, true);
        pac::GPIOHS::set_low_ie(0, false);
        pac::GPIOHS::set_low_ip(0, true);
        if edge.contains(Edge::RISING) {
            pac::GPIOHS::set_rise_ie(0, true);
        } 
        if edge.contains(Edge::FALLING) {
            pac::GPIOHS::set_fall_ie(0, true);
        } 
        if edge.contains(Edge::HIGH) {
            pac::GPIOHS::set_high_ie(0, true);
        } 
        if edge.contains(Edge::LOW) {
            pac::GPIOHS::set_low_ie(0, true);
        } 
    }
}

impl<MODE> InputPin for Gpiohs0<Input<MODE>> {
    type Error = core::convert::Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> { 
        Ok(unsafe { 
            let p = &(*pac::GPIOHS::ptr()).input_val as *const _ as *const _;
            u32_bit_is_set(p, 0)
        })
    }

    fn is_low(&self) -> Result<bool, Self::Error> { 
        Ok(unsafe { 
            let p = &(*pac::GPIOHS::ptr()).input_val as *const _ as *const _;
            u32_bit_is_clear(p, 0)
        })
    }
}

impl<MODE> OutputPin for Gpiohs0<Output<MODE>> {
    type Error = core::convert::Infallible;

    fn set_high(&mut self) -> Result<(), Self::Error> {
        unsafe { 
            let p = &(*pac::GPIOHS::ptr()).output_val as *const _ as *mut _;
            u32_set_bit(p, true, 0);
        }
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        unsafe { 
            let p = &(*pac::GPIOHS::ptr()).output_val as *const _ as *mut _;
            u32_set_bit(p, false, 0);
        }
        Ok(())
    }
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

    fn set_rise_ie(index: usize, bit: bool) {
        unsafe {
            let p = &mut Self::peripheral().rise_ie as *mut _ as *mut _; 
            u32_set_bit(p, bit, index);
        }
    }
    
    fn set_rise_ip(index: usize, bit: bool) {
        unsafe {
            let p = &mut Self::peripheral().rise_ip as *mut _ as *mut _; 
            u32_set_bit(p, bit, index);
        }
    }
    
    fn set_fall_ie(index: usize, bit: bool) {
        unsafe {
            let p = &mut Self::peripheral().fall_ie as *mut _ as *mut _; 
            u32_set_bit(p, bit, index);
        }
    }
    
    fn set_fall_ip(index: usize, bit: bool) {
        unsafe {
            let p = &mut Self::peripheral().fall_ip as *mut _ as *mut _; 
            u32_set_bit(p, bit, index);
        }
    }
    
    fn set_high_ie(index: usize, bit: bool) {
        unsafe {
            let p = &mut Self::peripheral().high_ie as *mut _ as *mut _; 
            u32_set_bit(p, bit, index);
        }
    }
    
    fn set_high_ip(index: usize, bit: bool) {
        unsafe {
            let p = &mut Self::peripheral().high_ip as *mut _ as *mut _; 
            u32_set_bit(p, bit, index);
        }
    }
    
    fn set_low_ie(index: usize, bit: bool) {
        unsafe {
            let p = &mut Self::peripheral().low_ie as *mut _ as *mut _; 
            u32_set_bit(p, bit, index);
        }
    }
    
    fn set_low_ip(index: usize, bit: bool) {
        unsafe {
            let p = &mut Self::peripheral().low_ip as *mut _ as *mut _; 
            u32_set_bit(p, bit, index);
        }
    }
}

impl GpiohsAccess for pac::GPIOHS {
    fn peripheral() -> &'static mut pac::gpiohs::RegisterBlock {
        unsafe { &mut *(pac::GPIOHS::ptr() as *mut _) }
    }
}
