//! Serial interface
//!
//! You can use the `Serial` interface with these UART instances:
//! * [`UARTHS`](crate::pac::UARTHS)
//! * [`UART1`](crate::pac::UART1)
//! * [`UART2`](crate::pac::UART2)
//! * [`UART3`](crate::pac::UART3)

use core::marker::PhantomData;

use embedded_hal::serial;
use nb;
use void::Void;

use crate::pac::UARTHS;
use crate::clock::Clocks;
use crate::time::Bps;

/// Extension trait that constrains UART peripherals
pub trait SerialExt: Sized {
    /// Constrains UART peripheral so it plays nicely with the other abstractions
    fn constrain(self, baud_rate: Bps, clocks: &Clocks) -> Serial<Self>;
}

impl SerialExt for UARTHS {
    fn constrain(self, baud_rate: Bps, clocks: &Clocks) -> Serial<UARTHS> {
        Serial::new(self, baud_rate, clocks)
    }
}

/// Serial abstraction
pub struct Serial<UART> {
    uart: UART,
}

/// Serial receiver
pub struct Rx<UART> {
    uart: UART,
}

/// Serial transmitter
pub struct Tx<UART> {
    _uart: PhantomData<UART>,
}

impl<UART> Rx<UART> {
    /// Forms `Serial` abstraction from a transmitter and a
    /// receiver half
    pub fn join(self, _tx: Tx<UART>) -> Serial<UART> {
        Serial { uart: self.uart }
    }
}

impl<UART> Serial<UART> {
    /// Splits the `Serial` abstraction into a transmitter and a
    /// receiver half
    pub fn split(self) -> (Tx<UART>, Rx<UART>) {
        (
            Tx {
                _uart: PhantomData
            },
            Rx {
                uart: self.uart
            }
        )
    }

    /// Releases the UART peripheral
    pub fn free(self) -> UART {
        self.uart
    }
}

impl Serial<UARTHS> {
    /// Configures a UART peripheral to provide serial communication
    pub fn new(uart: UARTHS, baud_rate: Bps, clocks: &Clocks) -> Self {
        let div = clocks.cpu().0 / baud_rate.0 - 1;
        unsafe {
            uart.div.write(|w| w.bits(div));
        }

        uart.txctrl.write(|w| w.txen().bit(true));
        uart.rxctrl.write(|w| w.rxen().bit(true));

        Serial { uart }
    }

    /// Starts listening for an interrupt event
    pub fn listen(self) -> Self {
        self.uart.ie.write(|w| w.txwm().bit(false).rxwm().bit(true));
        self
    }

    /// Stops listening for an interrupt event
    pub fn unlisten(self) -> Self {
        self.uart
            .ie
            .write(|w| w.txwm().bit(false).rxwm().bit(false));
        self
    }
}

impl serial::Read<u8> for Rx<UARTHS> {
    type Error = Void;

    fn read(&mut self) -> nb::Result<u8, Void> {
        // NOTE(unsafe) atomic read with no side effects
        let rxdata = unsafe { (*UARTHS::ptr()).rxdata.read() };

        if rxdata.empty().bit_is_set() {
            Err(::nb::Error::WouldBlock)
        } else {
            Ok(rxdata.data().bits() as u8)
        }
    }
}

impl serial::Write<u8> for Tx<UARTHS> {
    type Error = Void;

    fn write(&mut self, byte: u8) -> nb::Result<(), Void> {
        // NOTE(unsafe) atomic read with no side effects
        let txdata = unsafe { (*UARTHS::ptr()).txdata.read() };

        if txdata.full().bit_is_set() {
            Err(::nb::Error::WouldBlock)
        } else {
            unsafe {
                (*UARTHS::ptr()).txdata.write(|w| w.data().bits(byte));
            }
            Ok(())
        }
    }

    fn flush(&mut self) -> nb::Result<(), Void> {
        // NOTE(unsafe) atomic read with no side effects
        let txdata = unsafe { (*UARTHS::ptr()).txdata.read() };

        if txdata.full().bit_is_set() {
            Err(nb::Error::WouldBlock)
        } else {
            Ok(())
        }
    }
}
