//! Trait for reading entire lines over serial

use arduino_hal::{hal::Atmega, usart::UsartOps, Usart};
use embedded_hal::serial::Read as _;
use nb::block;

pub trait ReadLine {
    /// Read bytes until the byte `b'\n'`, aka a newline, is encountered. Writes
    /// the read bytes into the provided buffer until the buffer is full. Does _not_
    /// include the `b'\n'` itself, or a C-style terminating `b'\0'`.
    ///
    /// Returns a slice into the buffer of the read bytes for convenience.
    fn read_line<'a, const N: usize>(&mut self, buffer: &'a mut [u8; N]) -> &'a [u8];
}

impl<USART, RX, TX> ReadLine for Usart<USART, RX, TX>
where
    USART: UsartOps<Atmega, RX, TX>,
{
    fn read_line<'a, const N: usize>(&mut self, buffer: &'a mut [u8; N]) -> &'a [u8] {
        let mut i = 0;
        while i < N {
            match block!(self.read()) {
                Ok(b) if b != b'\n' => {
                    buffer[i] = b;
                    i += 1;
                }
                _ => break,
            }
        }
        &buffer[0..i]
    }
}
