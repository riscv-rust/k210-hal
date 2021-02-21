//! (TODO) Serial Peripheral Interface (SPI)

use crate::clock::Clocks;
use crate::pac::spi0::ctrlr0::TMOD_A as transfer_mode;
use crate::pac::SPI0;
use crate::sysctl::{self, APB2};
use core::convert::Infallible;
pub use embedded_hal::spi::{Mode, Phase, Polarity};

pub struct Spi<SPI> {
    spi: SPI,
    // Different with other MCUs like STM32 and Atmel
    // k210 use fpioa to map a pin directly to SPI SS 0-3 instead of using an ordinary GPIO
    // when transferring data, we'll set `pac::SPIX::ser` to (1 << cs_id) to select this device
    cs_id: u8,
}

impl Spi<SPI0> {
    pub fn spi0(
        spi: SPI0,
        cs_id: u8, // todo: currently we presume SPI0_SS<cs_id> is already configured correctly, maybe we can do this for the user?
        mode: Mode,
        frame_format: FrameFormat,
        endian: Endian,
        clock: &Clocks,
        apb2: &mut APB2,
    ) -> Self {
        let work_mode = hal_mode_to_pac(mode);
        let frame_format = frame_format_to_pac(frame_format);
        let endian = endian as u32;
        let data_bit_length = 8; // todo more length
        let _ = clock; // todo
        unsafe {
            // no interrupts for now
            spi.imr.write(|w| w.bits(0x0));
            // no dma for now
            spi.dmacr.write(|w| w.bits(0x0));
            spi.dmatdlr.write(|w| w.bits(0x10));
            spi.dmardlr.write(|w| w.bits(0x0));
            // no slave access for now
            spi.ser.write(|w| w.bits(0x0));
            spi.ssienr.write(|w| w.bits(0x0));
            // set control registers
            spi.ctrlr0.write(|w| {
                // no need to set tmod here, which will (and needs to) be set on each send/recv
                w.work_mode()
                    .variant(work_mode)
                    .frame_format()
                    .variant(frame_format)
                    .data_length()
                    .bits(data_bit_length - 1)
            });
            spi.spi_ctrlr0.reset(); // standard
            spi.endian.write(|w| w.bits(endian));
        }
        // enable APB0 bus
        apb2.enable();
        // enable peripheral via sysctl
        sysctl::clk_en_peri().modify(|_r, w| w.spi0_clk_en().set_bit());
        Spi { spi, cs_id }
    }

    pub fn release(self) -> SPI0 {
        // power off
        sysctl::clk_en_peri().modify(|_r, w| w.spi0_clk_en().clear_bit());
        self.spi
    }

    /// for making our life easier to use the same SPI interface but with different chip selected
    pub fn take_for_cs(self, cs_id: u8) -> Self {
        Self {
            spi: self.spi,
            cs_id,
        }
    }
}

// todo: Shall we make FrameFormat a type parameter instead?
// so FullDuplex<u8> can be implemented for Spi<SPI0, FrameFormat::Standard> only
impl embedded_hal::spi::FullDuplex<u8> for Spi<SPI0> {
    /// An enumeration of SPI errors
    type Error = Infallible;

    /// Reads the word stored in the shift register
    ///
    /// **NOTE** A word must be sent to the slave before attempting to call this
    /// method.
    fn try_read(&mut self) -> nb::Result<u8, Self::Error> {
        self.spi
            .ctrlr0
            .modify(|_, w| w.tmod().variant(transfer_mode::RECV));
        unsafe {
            // C sdk said ctrlr1 = rx_len(1) / frame_width(1) - 1;
            self.spi.ctrlr1.write(|w| w.bits(0x0));
            // enable spi
            self.spi.ssienr.write(|w| w.bits(0x1));
            // select that chip
            self.spi.ser.write(|w| w.bits(0x1 << self.cs_id));
            // clear dr
            self.spi.dr[0].write(|w| w.bits(0xffffffff));
        }
        let bytes_in_buffer = self.spi.rxflr.read().bits();
        let result = if bytes_in_buffer == 0 {
            Err(nb::Error::WouldBlock)
        } else {
            Ok(self.spi.dr[0].read().bits() as u8)
        };
        self.spi.ser.reset();
        self.spi.ssienr.reset();
        result
    }

    /// Sends a word to the slave
    fn try_send(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        self.spi
            .ctrlr0
            .modify(|_, w| w.tmod().variant(transfer_mode::TRANS));
        unsafe {
            self.spi.ssienr.write(|w| w.bits(0x0));
            self.spi.ser.write(|w| w.bits(0x1 << self.cs_id));
        }
        const MAX_FIFO_SIZE: u32 = 32;
        let empty_in_buffer = MAX_FIFO_SIZE - self.spi.txflr.read().bits();
        let result = if empty_in_buffer == 0 {
            Err(nb::Error::WouldBlock)
        } else {
            unsafe {
                self.spi.dr[0].write(|w| w.bits(word as u32));
            }
            Ok(())
        };
        self.spi.ser.reset();
        self.spi.ssienr.reset();
        result
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FrameFormat {
    Standard,
    Dual,
    Quad,
    Octal,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Endian {
    Little = 0,
    Big = 1,
}

#[inline]
fn hal_mode_to_pac(mode: Mode) -> crate::pac::spi0::ctrlr0::WORK_MODE_A {
    use crate::pac::spi0::ctrlr0::WORK_MODE_A;
    use {Phase::*, Polarity::*};
    match (mode.polarity, mode.phase) {
        (IdleLow, CaptureOnFirstTransition) => WORK_MODE_A::MODE0,
        (IdleLow, CaptureOnSecondTransition) => WORK_MODE_A::MODE1,
        (IdleHigh, CaptureOnFirstTransition) => WORK_MODE_A::MODE2,
        (IdleHigh, CaptureOnSecondTransition) => WORK_MODE_A::MODE3,
    }
}

#[inline]
fn frame_format_to_pac(frame_format: FrameFormat) -> crate::pac::spi0::ctrlr0::FRAME_FORMAT_A {
    use crate::pac::spi0::ctrlr0::FRAME_FORMAT_A;
    match frame_format {
        FrameFormat::Standard => FRAME_FORMAT_A::STANDARD,
        FrameFormat::Dual => FRAME_FORMAT_A::DUAL,
        FrameFormat::Quad => FRAME_FORMAT_A::QUAD,
        FrameFormat::Octal => FRAME_FORMAT_A::OCTAL,
    }
}
