//! General Purpose Input/Output (GPIO)

use core::marker::PhantomData;
use core::sync::atomic::AtomicU32;
use crate::pac::GPIO;
use crate::fpioa::{IoPin, Pull};
use crate::atomic::{u32_atomic_set_bit, u32_atomic_toggle_bit};
use embedded_hal::digital::v2::{OutputPin, StatefulOutputPin, InputPin, ToggleableOutputPin};

/// Extension trait to split a GPIO peripheral in independent pins
pub trait GpioExt {
    // todo: require ownership of sysctl part
    fn split(self) -> Parts;
}

impl GpioExt for GPIO {
    fn split(self) -> Parts {
        // todo: use sysctl part to enable clock
        Parts { 
            gpio6: GPIO6 { _ownership: () }
        }
    }
}

/// GPIO Parts
pub struct Parts {
    pub gpio6: GPIO6,
}

// todo: should this design wrap with FPIOA? maybe we should use advantage of
// its typestate design

/// GPIO Pin
pub struct GPIO6 {
    _ownership: ()
}

/// Input mode (type state)
pub struct Input<MODE>(MODE);

/// Floating input (type state)
pub struct Floating;

/// Pull down input (type state)
pub struct PullDown;

/// Pull up input (type state)
pub struct PullUp;

/// Output mode (type state)
pub struct Output;

/// Marker trait for active states
pub trait Active {}

impl Active for Input<Floating> {}

impl Active for Input<PullUp> {}

impl Active for Input<PullDown> {}

impl Active for Output {}

pub struct Gpio<GPIO, PIN, MODE> {
    gpio: GPIO,
    pin: PIN,
    _mode: PhantomData<MODE>,
}

// todo: a more proper gpio implementation

impl<GPIO, PIN> Gpio<GPIO, PIN, Input<Floating>> {
    // todo: verify default GPIO mode
    pub fn new(gpio: GPIO, pin: PIN) -> Gpio<GPIO, PIN, Input<Floating>> {
        Gpio { gpio, pin, _mode: PhantomData }
    }
}

impl<PIN: IoPin, MODE: Active> Gpio<GPIO6, PIN, MODE> {
    pub fn into_floating_input(mut self) -> Gpio<GPIO6, PIN, Input<Floating>> {
        self.pin.set_io_pull(Pull::None);
        let r: &AtomicU32 = unsafe { &*(&(*GPIO::ptr()).direction as *const _ as *const _) };
        u32_atomic_set_bit(r, false, 6);
        Gpio { gpio: self.gpio, pin: self.pin, _mode: PhantomData }
    }

    pub fn into_pull_up_input(mut self) -> Gpio<GPIO6, PIN, Input<PullUp>> {
        self.pin.set_io_pull(Pull::Up);
        let r: &AtomicU32 = unsafe { &*(&(*GPIO::ptr()).direction as *const _ as *const _) };
        u32_atomic_set_bit(r, false, 6);
        Gpio { gpio: self.gpio, pin: self.pin, _mode: PhantomData }
    }

    pub fn into_pull_down_input(mut self) -> Gpio<GPIO6, PIN, Input<PullDown>> {
        self.pin.set_io_pull(Pull::Down);
        let r: &AtomicU32 = unsafe { &*(&(*GPIO::ptr()).direction as *const _ as *const _) };
        u32_atomic_set_bit(r, false, 6);
        Gpio { gpio: self.gpio, pin: self.pin, _mode: PhantomData }
    }

    pub fn into_push_pull_output(mut self) -> Gpio<GPIO6, PIN, Output> {
        self.pin.set_io_pull(Pull::Down);
        let r: &AtomicU32 = unsafe { &*(&(*GPIO::ptr()).direction as *const _ as *const _) };
        u32_atomic_set_bit(r, true, 6);
        Gpio { gpio: self.gpio, pin: self.pin, _mode: PhantomData }
    }
}

impl<PIN> OutputPin for Gpio<GPIO6, PIN, Output> {
    type Error = core::convert::Infallible;

    fn set_high(&mut self) -> Result<(), Self::Error> {
        let r: &AtomicU32 = unsafe { &*(&(*GPIO::ptr()).data_output as *const _ as *const _) };
        u32_atomic_set_bit(r, true, 6);
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        let r: &AtomicU32 = unsafe { &*(&(*GPIO::ptr()).data_output as *const _ as *const _) };
        u32_atomic_set_bit(r, false, 6);
        Ok(())
    }
}

impl<PIN> StatefulOutputPin for Gpio<GPIO6, PIN, Output> {
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(unsafe { 
            (*GPIO::ptr()).data_output.read().pin6().bit_is_set()
        })
    }

    fn is_set_low(&self) -> Result<bool, Self::Error> { 
        Ok(unsafe { 
            (*GPIO::ptr()).data_output.read().pin6().bit_is_clear()
        })
    }
}

impl<PIN> ToggleableOutputPin for Gpio<GPIO6, PIN, Output> {
    type Error = core::convert::Infallible;

    fn toggle(&mut self) -> Result<(), Self::Error> { 
        let r: &AtomicU32 = unsafe { &*(&(*GPIO::ptr()).data_output as *const _ as *const _) };
        u32_atomic_toggle_bit(r, 6);
        Ok(())
    }
}

impl<PIN, MODE> InputPin for Gpio<GPIO6, PIN, Input<MODE>> {
    type Error = core::convert::Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> { 
        Ok(unsafe { 
            (*GPIO::ptr()).data_input.read().pin6().bit_is_set()
        })
    }

    fn is_low(&self) -> Result<bool, Self::Error> { 
        Ok(unsafe { 
            (*GPIO::ptr()).data_input.read().pin6().bit_is_clear()
        })
    }
}
