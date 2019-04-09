//! Stdout
pub use core::fmt::Write;
use nb::block;

/// Stdout implements the core::fmt::Write trait for hal::serial::Write
/// implementations.
pub struct Stdout<'p, T>(pub &'p mut T)
    where
    T: 'p;

impl<'p, T> Write for Stdout<'p, T>
    where
    T: embedded_hal::serial::Write<u8>,
{
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        for byte in s.as_bytes() {
            let res = block!(self.0.write(*byte));

            if res.is_err() {
                return Err(::core::fmt::Error);
            }

            if *byte == '\n' as u8 {
                let res = block!(self.0.write('\r' as u8));

                if res.is_err() {
                    return Err(::core::fmt::Error);
                }
            }
        }
        Ok(())
    }
}
