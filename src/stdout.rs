//! Stdout
pub use core::fmt::Write;

/// Stdout implements the core::fmt::Write trait for hal::serial::Write
/// implementations.
pub struct Stdout<'p, T>(pub &'p mut T);

impl<'p, T> Write for Stdout<'p, T>
where
    T: embedded_io::Write,
{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.0
            .write(s.as_bytes())
            .map_err(|_| core::fmt::Error)
            .map(|_| ())
    }
}
