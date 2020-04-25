use crate::pac::GPIO;
use embedded_hal::digital::v2::{OutputPin};
use crate::fpioa::{IoPin, Pull};
use core::marker::PhantomData;

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

pub struct Parts {
    pub gpio6: GPIO6,
}

// todo: should this design wrap with FPIOA? maybe we should use advantage of
// its typestate design

pub struct GPIO6 {
    _ownership: ()
}

pub struct Input<MODE>(MODE);

pub struct Floating;

pub struct PullDown;

pub struct PullUp;

pub struct Output;

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

impl<PIN: IoPin, MODE> Gpio<GPIO6, PIN, MODE> {
    pub fn into_floating_input(mut self) -> Gpio<GPIO6, PIN, Input<Floating>> {
        self.pin.set_io_pull(Pull::None);
        unsafe { &(*GPIO::ptr()).direction.modify(
            |_r, w| 
                w.pin6().clear_bit()
        ) };
        Gpio { gpio: self.gpio, pin: self.pin, _mode: PhantomData }
    }

    pub fn into_push_pull_output(mut self) -> Gpio<GPIO6, PIN, Output> {
        self.pin.set_io_pull(Pull::Down);
        unsafe { &(*GPIO::ptr()).direction.modify(
            |_r, w| 
                w.pin6().set_bit()
        ) };
        Gpio { gpio: self.gpio, pin: self.pin, _mode: PhantomData }
    }
}

impl<PIN> OutputPin for Gpio<GPIO6, PIN, Output> {
    type Error = core::convert::Infallible;

    fn set_low(&mut self) -> Result<(), Self::Error> { 
        let ans = unsafe { 
            (*GPIO::ptr()).data_output.write(|w| 
                w.pin6().clear_bit() 
            ) 
        };
        Ok(ans)
    }

    fn set_high(&mut self) -> Result<(), Self::Error> { 
        let ans = unsafe { 
            (*GPIO::ptr()).data_output.write(|w| 
                w.pin6().set_bit() 
            ) 
        };
        Ok(ans)
    }
}

// impl<GPIO: GpioIndex, PIN, MODE> InputPin for Gpio<GPIO, PIN, Input<MODE>> {
//     type Error = Infallible;

//     fn is_high(&self) -> Result<bool, Self::Error> { 
        
//     }

//     fn is_low(&self) -> Result<bool, Self::Error> { 

//     }
// }
