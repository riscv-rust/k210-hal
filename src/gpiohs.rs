//! High-speed GPIO peripheral (GPIOHS)

use crate::bit_utils::{u32_bit_is_clear, u32_bit_is_set, u32_set_bit};
use crate::pac::GPIOHS;
use core::marker::PhantomData;
use embedded_hal::digital::{ErrorType, InputPin, OutputPin};

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

impl GpiohsExt for GPIOHS {
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
    #[inline]
    pub fn into_pull_up_input(self) -> Gpiohs0<Input<PullUp>> {
        GPIOHS::set_output_en(0, false);
        GPIOHS::set_input_en(0, true);
        GPIOHS::set_pullup_en(0, true);
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
    #[inline]
    pub fn trigger_on_edge(&mut self, edge: Edge) {
        // clear all pending bits
        GPIOHS::clear_rise_ip(0);
        GPIOHS::clear_fall_ip(0);
        GPIOHS::clear_high_ip(0);
        GPIOHS::clear_low_ip(0);
        // enable interrupts according to flags
        GPIOHS::set_rise_ie(0, edge.contains(Edge::RISING));
        GPIOHS::set_fall_ie(0, edge.contains(Edge::FALLING));
        GPIOHS::set_high_ie(0, edge.contains(Edge::HIGH));
        GPIOHS::set_low_ie(0, edge.contains(Edge::LOW));
    }

    #[inline]
    pub fn check_edges(&self) -> Edge {
        let mut ans = Edge::empty();
        if GPIOHS::has_rise_ip(0) {
            ans |= Edge::RISING;
        }
        if GPIOHS::has_fall_ip(0) {
            ans |= Edge::FALLING;
        }
        if GPIOHS::has_high_ip(0) {
            ans |= Edge::HIGH;
        }
        if GPIOHS::has_low_ip(0) {
            ans |= Edge::LOW;
        }
        ans
    }

    #[inline]
    pub fn clear_interrupt_pending_bits(&mut self) {
        if GPIOHS::has_rise_ie(0) {
            GPIOHS::set_rise_ie(0, false);
            GPIOHS::clear_rise_ip(0);
            GPIOHS::set_rise_ie(0, true);
        }
        if GPIOHS::has_fall_ie(0) {
            GPIOHS::set_fall_ie(0, false);
            GPIOHS::clear_fall_ip(0);
            GPIOHS::set_fall_ie(0, true);
        }
        if GPIOHS::has_high_ie(0) {
            GPIOHS::set_high_ie(0, false);
            GPIOHS::clear_high_ip(0);
            GPIOHS::set_high_ie(0, true);
        }
        if GPIOHS::has_low_ie(0) {
            GPIOHS::set_low_ie(0, false);
            GPIOHS::clear_low_ip(0);
            GPIOHS::set_low_ie(0, true);
        }
    }
}

impl<MODE> ErrorType for Gpiohs0<MODE> {
    // All GPIO operations are infallible.
    type Error = core::convert::Infallible;
}

impl<MODE> InputPin for Gpiohs0<Input<MODE>> {
    #[inline]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(unsafe {
            let p = &(*GPIOHS::ptr()).input_val as *const _ as *const _;
            u32_bit_is_set(p, 0)
        })
    }

    #[inline]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(unsafe {
            let p = &(*GPIOHS::ptr()).input_val as *const _ as *const _;
            u32_bit_is_clear(p, 0)
        })
    }
}

impl<MODE> OutputPin for Gpiohs0<Output<MODE>> {
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        unsafe {
            let p = &(*GPIOHS::ptr()).output_val as *const _ as *mut _;
            u32_set_bit(p, true, 0);
        }
        Ok(())
    }

    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        unsafe {
            let p = &(*GPIOHS::ptr()).output_val as *const _ as *mut _;
            u32_set_bit(p, false, 0);
        }
        Ok(())
    }
}

trait GpiohsAccess {
    fn peripheral() -> &'static mut crate::pac::gpiohs::RegisterBlock;

    #[inline]
    fn set_drive(index: usize, bit: bool) {
        unsafe {
            let p = &mut Self::peripheral().drive as *mut _ as *mut _;
            u32_set_bit(p, bit, index);
        }
    }

    #[inline]
    fn input_value(index: usize) -> bool {
        unsafe {
            let p = &mut Self::peripheral().input_val as *mut _ as *mut _;
            u32_bit_is_set(p, index)
        }
    }

    #[inline]
    fn set_input_en(index: usize, bit: bool) {
        unsafe {
            let p = &mut Self::peripheral().input_en as *mut _ as *mut _;
            u32_set_bit(p, bit, index);
        }
    }

    #[inline]
    fn set_iof_en(index: usize, bit: bool) {
        unsafe {
            let p = &mut Self::peripheral().iof_en as *mut _ as *mut _;
            u32_set_bit(p, bit, index);
        }
    }

    #[inline]
    fn set_iof_sel(index: usize, bit: bool) {
        unsafe {
            let p = &mut Self::peripheral().iof_sel as *mut _ as *mut _;
            u32_set_bit(p, bit, index);
        }
    }

    #[inline]
    fn set_output_en(index: usize, bit: bool) {
        unsafe {
            let p = &mut Self::peripheral().output_en as *mut _ as *mut _;
            u32_set_bit(p, bit, index);
        }
    }

    #[inline]
    fn set_output_value(index: usize, bit: bool) {
        unsafe {
            let p = &mut Self::peripheral().output_val as *mut _ as *mut _;
            u32_set_bit(p, bit, index);
        }
    }

    #[inline]
    fn set_output_xor(index: usize, bit: bool) {
        unsafe {
            let p = &mut Self::peripheral().output_xor as *mut _ as *mut _;
            u32_set_bit(p, bit, index);
        }
    }

    #[inline]
    fn set_pullup_en(index: usize, bit: bool) {
        unsafe {
            let p = &mut Self::peripheral().pullup_en as *mut _ as *mut _;
            u32_set_bit(p, bit, index);
        }
    }

    #[inline]
    fn set_rise_ie(index: usize, bit: bool) {
        unsafe {
            let p = &mut Self::peripheral().rise_ie as *mut _ as *mut _;
            u32_set_bit(p, bit, index);
        }
    }

    #[inline]
    fn clear_rise_ip(index: usize) {
        unsafe {
            let p = &mut Self::peripheral().rise_ip as *mut _ as *mut _;
            u32_set_bit(p, true, index);
        }
    }

    #[inline]
    fn set_fall_ie(index: usize, bit: bool) {
        unsafe {
            let p = &mut Self::peripheral().fall_ie as *mut _ as *mut _;
            u32_set_bit(p, bit, index);
        }
    }

    #[inline]
    fn clear_fall_ip(index: usize) {
        unsafe {
            let p = &mut Self::peripheral().fall_ip as *mut _ as *mut _;
            u32_set_bit(p, true, index);
        }
    }

    #[inline]
    fn set_high_ie(index: usize, bit: bool) {
        unsafe {
            let p = &mut Self::peripheral().high_ie as *mut _ as *mut _;
            u32_set_bit(p, bit, index);
        }
    }

    #[inline]
    fn clear_high_ip(index: usize) {
        unsafe {
            let p = &mut Self::peripheral().high_ip as *mut _ as *mut _;
            u32_set_bit(p, true, index);
        }
    }

    #[inline]
    fn set_low_ie(index: usize, bit: bool) {
        unsafe {
            let p = &mut Self::peripheral().low_ie as *mut _ as *mut _;
            u32_set_bit(p, bit, index);
        }
    }

    #[inline]
    fn clear_low_ip(index: usize) {
        unsafe {
            let p = &mut Self::peripheral().low_ip as *mut _ as *mut _;
            u32_set_bit(p, true, index);
        }
    }

    #[inline]
    fn has_rise_ie(index: usize) -> bool {
        unsafe {
            let p = &mut Self::peripheral().rise_ie as *mut _ as *mut _;
            u32_bit_is_set(p, index)
        }
    }

    #[inline]
    fn has_fall_ie(index: usize) -> bool {
        unsafe {
            let p = &mut Self::peripheral().fall_ie as *mut _ as *mut _;
            u32_bit_is_set(p, index)
        }
    }

    #[inline]
    fn has_high_ie(index: usize) -> bool {
        unsafe {
            let p = &mut Self::peripheral().high_ie as *mut _ as *mut _;
            u32_bit_is_set(p, index)
        }
    }

    #[inline]
    fn has_low_ie(index: usize) -> bool {
        unsafe {
            let p = &mut Self::peripheral().low_ie as *mut _ as *mut _;
            u32_bit_is_set(p, index)
        }
    }

    #[inline]
    fn has_rise_ip(index: usize) -> bool {
        unsafe {
            let p = &mut Self::peripheral().rise_ip as *mut _ as *mut _;
            u32_bit_is_set(p, index)
        }
    }

    #[inline]
    fn has_fall_ip(index: usize) -> bool {
        unsafe {
            let p = &mut Self::peripheral().fall_ip as *mut _ as *mut _;
            u32_bit_is_set(p, index)
        }
    }

    #[inline]
    fn has_high_ip(index: usize) -> bool {
        unsafe {
            let p = &mut Self::peripheral().high_ip as *mut _ as *mut _;
            u32_bit_is_set(p, index)
        }
    }

    #[inline]
    fn has_low_ip(index: usize) -> bool {
        unsafe {
            let p = &mut Self::peripheral().low_ip as *mut _ as *mut _;
            u32_bit_is_set(p, index)
        }
    }
}

impl GpiohsAccess for GPIOHS {
    #[inline]
    fn peripheral() -> &'static mut crate::pac::gpiohs::RegisterBlock {
        unsafe { &mut *(GPIOHS::ptr() as *mut _) }
    }
}
