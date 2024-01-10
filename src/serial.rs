//! Serial interface
//!
//! You can use the `Serial` interface with these UART instances:
//! * [`UARTHS`](crate::pac::UARTHS)
//! * [`UART1`](crate::pac::UART1)
//! * [`UART2`](crate::pac::UART2)
//! * [`UART3`](crate::pac::UART3)

use core::convert::Infallible;
use core::mem;

use crate::clock::Clocks;
use crate::pac::{uart1, UART1, UART2, UART3, UARTHS};
use crate::time::Bps;

/// Extension trait that constrains UART peripherals
pub trait SerialExt: Sized {
    /// Configures a UART peripheral to provide serial communication
    fn configure(self, baud_rate: Bps, clocks: &Clocks) -> Serial<Self>;
}

/// Serial abstraction
pub struct Serial<UART> {
    uart: UART,
}

impl<UART> Serial<UART> {
    /// Splits the `Serial` abstraction into a transmitter and a
    /// receiver half
    #[inline]
    pub fn split(self) -> (Tx<UART>, Rx<UART>) {
        (
            Tx { uart: self.uart },
            Rx {
                // clippy allow: inner marker variable only indicates ownership, does not include actual data
                uart: unsafe {
                    #[allow(clippy::uninit_assumed_init)]
                    mem::MaybeUninit::uninit().assume_init()
                },
            },
        )
    }

    /// Forms `Serial` abstraction from a transmitter and a
    /// receiver half
    #[inline]
    pub fn join(tx: Tx<UART>, rx: Rx<UART>) -> Self {
        let _ = rx; // note(discard): Zero-sized typestate struct
        Serial { uart: tx.uart }
    }

    /// Releases the UART peripheral
    #[inline]
    pub fn free(self) -> UART {
        // todo: power down this UART
        self.uart
    }
}

/// Serial transmitter
pub struct Tx<UART> {
    uart: UART,
}

/// Serial receiver
pub struct Rx<UART> {
    uart: UART,
}

impl SerialExt for UARTHS {
    #[inline]
    fn configure(self, baud_rate: Bps, clocks: &Clocks) -> Serial<UARTHS> {
        let uart = self;

        let div = clocks.cpu().0 / baud_rate.0 - 1;
        unsafe {
            uart.div.write(|w| w.bits(div));
        }

        uart.txctrl.write(|w| w.txen().bit(true));
        uart.rxctrl.write(|w| w.rxen().bit(true));

        Serial { uart }
    }
}

impl Serial<UARTHS> {
    /// Starts listening for an interrupt event
    #[inline]
    pub fn listen(self) -> Self {
        self.uart.ie.write(|w| w.txwm().bit(false).rxwm().bit(true));
        self
    }

    /// Stops listening for an interrupt event
    #[inline]
    pub fn unlisten(self) -> Self {
        self.uart
            .ie
            .write(|w| w.txwm().bit(false).rxwm().bit(false));
        self
    }
}

impl embedded_io::ErrorType for Rx<UARTHS> {
    type Error = core::convert::Infallible;
}

impl embedded_io::Read for Rx<UARTHS> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Infallible> {
        while self.uart.rxdata.read().empty().bit_is_set() {
            // Block until rxdata available.
            core::hint::spin_loop()
        }
        let len = buf.len();
        for slot in buf {
            *slot = self.uart.rxdata.read().data().bits();
        }
        Ok(len)
    }
}

impl embedded_io::ErrorType for Tx<UARTHS> {
    type Error = core::convert::Infallible;
}

impl embedded_io::Write for Tx<UARTHS> {
    #[inline]
    fn write(&mut self, bytes: &[u8]) -> Result<usize, Infallible> {
        while self.uart.txdata.read().full().bit_is_set() {
            // Block until txdata available.
            core::hint::spin_loop()
        }
        for byte in bytes {
            unsafe {
                self.uart.txdata.write(|w| w.data().bits(*byte));
            }
        }
        Ok(bytes.len())
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Infallible> {
        while self.uart.txdata.read().full().bit_is_set() {
            // Block until flush complete. If you don't want a block, use embedded_io_async traits instead.
            core::hint::spin_loop()
        }
        Ok(())
    }
}

mod closed_trait {
    use core::ops::Deref;
    /// Trait to be able to generalize over UART1/UART2/UART3
    pub trait UartX: Deref<Target = super::uart1::RegisterBlock> {
        const INDEX: u8;
    }
}
use closed_trait::UartX;

impl UartX for UART1 {
    const INDEX: u8 = 1;
}
impl UartX for UART2 {
    const INDEX: u8 = 2;
}
impl UartX for UART3 {
    const INDEX: u8 = 3;
}

const UART_RECEIVE_FIFO_1: u32 = 0;
const UART_SEND_FIFO_8: u32 = 3;

impl<UART: UartX> SerialExt for UART {
    #[inline]
    fn configure(self, baud_rate: Bps, clocks: &Clocks) -> Serial<UART> {
        let uart = self;

        // Hardcode these for now:
        let data_width = 8; // 8 data bits
        let stopbit_val = 0; // 1 stop bit
        let parity_val = 0; // No parity
                            // Note: need to make sure that UARTx clock is enabled through sysctl before here
        let divisor = clocks.apb0().0 / baud_rate.0;
        let dlh = ((divisor >> 12) & 0xff) as u8;
        let dll = ((divisor >> 4) & 0xff) as u8;
        let dlf = (divisor & 0xf) as u8;
        unsafe {
            // Set Divisor Latch Access Bit (enables DLL DLH) to set baudrate
            uart.lcr.write(|w| w.bits(1 << 7));
            uart.dlh_ier.write(|w| w.bits(dlh.into()));
            uart.rbr_dll_thr.write(|w| w.bits(dll.into()));
            uart.dlf.write(|w| w.bits(dlf.into()));
            // Clear Divisor Latch Access Bit after setting baudrate
            uart.lcr
                .write(|w| w.bits((data_width - 5) | (stopbit_val << 2) | (parity_val << 3)));
            // Write IER
            uart.dlh_ier.write(|w| w.bits(0x80)); /* THRE */
            // Write FCT
            uart.fcr_iir.write(|w| {
                w.bits(UART_RECEIVE_FIFO_1 << 6 | UART_SEND_FIFO_8 << 4 | 0x1 << 3 | 0x1)
            });
        }

        Serial { uart }
    }
}

impl<UART: UartX> Serial<UART> {
    /// Starts listening for an interrupt event
    #[inline]
    pub fn listen(self) -> Self {
        // TODO
        self
    }

    /// Stops listening for an interrupt event
    #[inline]
    pub fn unlisten(self) -> Self {
        // TODO
        self
    }
}

impl<UART: UartX> embedded_io::ErrorType for Rx<UART> {
    type Error = core::convert::Infallible;
}

impl<UART: UartX> embedded_io::Read for Rx<UART> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Infallible> {
        while (self.uart.lsr.read().bits() & (1 << 0)) == 0 {
            // Data Ready bit
            core::hint::spin_loop()
        }
        let len = buf.len();
        for slot in buf {
            *slot = (self.uart.rbr_dll_thr.read().bits() & 0xff) as u8;
        }
        Ok(len)
    }
}

impl<UART: UartX> embedded_io::ErrorType for Tx<UART> {
    type Error = core::convert::Infallible;
}

impl<UART: UartX> embedded_io::Write for Tx<UART> {
    #[inline]
    fn write(&mut self, bytes: &[u8]) -> Result<usize, Infallible> {
        while (self.uart.lsr.read().bits() & (1 << 5)) != 0 {
            // Transmit Holding Register Empty bit
            core::hint::spin_loop();
        }
        for byte in bytes {
            unsafe {
                self.uart.rbr_dll_thr.write(|w| w.bits(*byte as u32));
            }
        }
        Ok(bytes.len())
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Infallible> {
        // TODO
        Ok(())
    }
}
