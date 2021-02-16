//! High-speed GPIO peripheral (GPIOHS)

// use crate::bit_utils::{u32_bit_is_clear, u32_bit_is_set, u32_set_bit, u32_toggle_bit};
use crate::fpioa::{IoPin, Mode, Pull};
use crate::pac;
use core::marker::PhantomData;
use embedded_hal::digital::v2::{InputPin, OutputPin, StatefulOutputPin, ToggleableOutputPin};

// reused type state
pub use crate::gpio::{Active, Floating, Input, Output, PullDown, PullUp, Unknown};

/// GPIOHS Index
pub trait GpiohsIndex {
    type FUNC;
    const INDEX: u8;
}

pub trait GpiohsExt {
    fn split(self) -> Parts;
}

pub use gpiohs_pins::*;

macro_rules! def_gpiohs_pins {
    ($($GPIOHSX: ident: ($num: expr, $gpiohsx: ident, $func: ident);)+) => {

        pub struct Parts {
            $( pub $gpiohsx: $GPIOHSX, )+
        }


        impl GpiohsExt for pac::GPIOHS {
            fn split(self) -> Parts {
                Parts {
                    $( $gpiohsx: $GPIOHSX { _ownership: () }, )+
                }
            }
        }

        /// All GPIOHS pins
        pub mod gpiohs_pins {
            use super::GpiohsIndex;
            $(
            /// GPIOHS pin
            pub struct $GPIOHSX {
                pub(crate) _ownership: ()
            }

            impl GpiohsIndex for $GPIOHSX {
                type FUNC = crate::fpioa::functions::$func;
                const INDEX: u8 = $num;
            }
            )+
        }
    }
}

def_gpiohs_pins! {
    GPIOHS0: (0, gpiohs0, GPIOHS0);
    GPIOHS1: (1, gpiohs1, GPIOHS1);
    GPIOHS2: (2, gpiohs2, GPIOHS2);
    GPIOHS3: (3, gpiohs3, GPIOHS3);
    GPIOHS4: (4, gpiohs4, GPIOHS4);
    GPIOHS5: (5, gpiohs5, GPIOHS5);
    GPIOHS6: (6, gpiohs6, GPIOHS6);
    GPIOHS7: (7, gpiohs7, GPIOHS7);
    GPIOHS8: (8, gpiohs8, GPIOHS8);
    GPIOHS9: (9, gpiohs9, GPIOHS9);
    GPIOHS10: (10, gpiohs10, GPIOHS10);
    GPIOHS11: (11, gpiohs11, GPIOHS11);
    GPIOHS12: (12, gpiohs12, GPIOHS12);
    GPIOHS13: (13, gpiohs13, GPIOHS13);
    GPIOHS14: (14, gpiohs14, GPIOHS14);
    GPIOHS15: (15, gpiohs15, GPIOHS15);
    GPIOHS16: (16, gpiohs16, GPIOHS16);
    GPIOHS17: (17, gpiohs17, GPIOHS17);
    GPIOHS18: (18, gpiohs18, GPIOHS18);
    GPIOHS19: (19, gpiohs19, GPIOHS19);
    GPIOHS20: (20, gpiohs20, GPIOHS20);
    GPIOHS21: (21, gpiohs21, GPIOHS21);
    GPIOHS22: (22, gpiohs22, GPIOHS22);
    GPIOHS23: (23, gpiohs23, GPIOHS23);
    GPIOHS24: (24, gpiohs24, GPIOHS24);
    GPIOHS25: (25, gpiohs25, GPIOHS25);
    GPIOHS26: (26, gpiohs26, GPIOHS26);
    GPIOHS27: (27, gpiohs27, GPIOHS27);
    GPIOHS28: (28, gpiohs28, GPIOHS28);
    GPIOHS29: (29, gpiohs29, GPIOHS29);
    GPIOHS30: (30, gpiohs30, GPIOHS30);
    GPIOHS31: (31, gpiohs31, GPIOHS31);
}

/// GPIOHS wrapper struct
pub struct Gpiohs<GPIOHS, PIN, MODE> {
    gpiohs: GPIOHS,
    pin: PIN,
    _mode: PhantomData<MODE>,
}

impl<GPIOHS: GpiohsIndex, PIN: Mode<GPIOHS::FUNC>> Gpiohs<GPIOHS, PIN, Unknown> {
    pub fn new(gpiohs: GPIOHS, pin: PIN) -> Gpiohs<GPIOHS, PIN, Unknown> {
        Gpiohs {
            gpiohs,
            pin,
            _mode: PhantomData,
        }
    }
}

impl<GPIOHS, PIN, MODE> Gpiohs<GPIOHS, PIN, MODE> {
    pub fn free(self) -> (GPIOHS, PIN) {
        (self.gpiohs, self.pin)
    }
}

impl<GPIOHS: GpiohsIndex, PIN: IoPin, MODE: Active> Gpiohs<GPIOHS, PIN, MODE> {
    pub fn into_floating_input(mut self) -> Gpiohs<GPIOHS, PIN, Input<Floating>> {
        self.pin.set_io_pull(Pull::None);
        self.direction_in();
        Gpiohs {
            gpiohs: self.gpiohs,
            pin: self.pin,
            _mode: PhantomData,
        }
    }

    pub fn into_pull_up_input(mut self) -> Gpiohs<GPIOHS, PIN, Input<PullUp>> {
        self.pin.set_io_pull(Pull::Up);
        self.direction_in();
        self.enable_pullup();
        Gpiohs {
            gpiohs: self.gpiohs,
            pin: self.pin,
            _mode: PhantomData,
        }
    }

    pub fn into_pull_down_input(mut self) -> Gpiohs<GPIOHS, PIN, Input<PullDown>> {
        self.pin.set_io_pull(Pull::Down);
        self.direction_in();
        self.disable_pullup();
        Gpiohs {
            gpiohs: self.gpiohs,
            pin: self.pin,
            _mode: PhantomData,
        }
    }

    pub fn into_push_pull_output(mut self) -> Gpiohs<GPIOHS, PIN, Output> {
        self.pin.set_io_pull(Pull::Down);
        self.direction_out();
        Gpiohs {
            gpiohs: self.gpiohs,
            pin: self.pin,
            _mode: PhantomData,
        }
    }

    #[inline]
    fn direction_in(&mut self) {
        unsafe {
            (*pac::GPIOHS::ptr())
                .output_en
                .modify(|r, w| w.bits(r.bits() & (!(1 << GPIOHS::INDEX))));
            (*pac::GPIOHS::ptr())
                .input_en
                .modify(|r, w| w.bits(r.bits() | (1 << GPIOHS::INDEX)));
        }
    }

    #[inline]
    fn direction_out(&mut self) {
        unsafe {
            (*pac::GPIOHS::ptr())
                .output_en
                .modify(|r, w| w.bits(r.bits() | (1 << GPIOHS::INDEX)));
            (*pac::GPIOHS::ptr())
                .input_en
                .modify(|r, w| w.bits(r.bits() & (!(1 << GPIOHS::INDEX))));
        }
    }

    #[inline]
    fn enable_pullup(&mut self) {
        unsafe {
            (*pac::GPIOHS::ptr())
                .pullup_en
                .modify(|r, w| w.bits(r.bits() | (1 << GPIOHS::INDEX)));
        }
    }

    #[inline]
    fn disable_pullup(&mut self) {
        unsafe {
            (*pac::GPIOHS::ptr())
                .pullup_en
                .modify(|r, w| w.bits(r.bits() & (!(1 << GPIOHS::INDEX))));
        }
    }
}

impl<GPIOHS: GpiohsIndex, PIN, MODE> InputPin for Gpiohs<GPIOHS, PIN, Input<MODE>> {
    type Error = core::convert::Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(
            unsafe {
                ((*pac::GPIOHS::ptr()).input_val.read().bits() >> GPIOHS::INDEX) & 0b1 == 0b1
            },
        )
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(
            unsafe {
                ((*pac::GPIOHS::ptr()).input_val.read().bits() >> GPIOHS::INDEX) & 0b1 == 0b0
            },
        )
    }
}

impl<GPIOHS: GpiohsIndex, PIN> OutputPin for Gpiohs<GPIOHS, PIN, Output> {
    type Error = core::convert::Infallible;

    fn set_high(&mut self) -> Result<(), Self::Error> {
        unsafe {
            (*pac::GPIOHS::ptr())
                .output_val
                .modify(|r, w| w.bits(r.bits() | (1 << GPIOHS::INDEX)));
        }
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        unsafe {
            (*pac::GPIOHS::ptr())
                .output_val
                .modify(|r, w| w.bits(r.bits() & (!(1 << GPIOHS::INDEX as u8))));
        }
        Ok(())
    }
}

impl<GPIOHS: GpiohsIndex, PIN> StatefulOutputPin for Gpiohs<GPIOHS, PIN, Output> {
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(unsafe {
            ((*pac::GPIOHS::ptr()).output_val.read().bits() >> GPIOHS::INDEX) & 0b1 == 0b1
        })
    }

    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(unsafe {
            ((*pac::GPIOHS::ptr()).output_val.read().bits() >> GPIOHS::INDEX) & 0b1 == 0b0
        })
    }
}

impl<GPIOHS: GpiohsIndex, PIN> ToggleableOutputPin for Gpiohs<GPIOHS, PIN, Output> {
    type Error = core::convert::Infallible;

    fn toggle(&mut self) -> Result<(), Self::Error> {
        unsafe {
            (*pac::GPIOHS::ptr())
                .output_val
                .modify(|r, w| w.bits(r.bits() ^ (1 << GPIOHS::INDEX)))
        }
        Ok(())
    }
}

/// Gpiohs pin trigger edge type
#[derive(Clone, Copy, Debug)]
pub enum Edge {
    None,
    Falling,
    Rising,
    Both,
    Low,
    High = 8,
}

// TODO: interrupt
// TODO: Drive Strength