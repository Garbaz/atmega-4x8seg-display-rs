#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use atmega_4x8seg_display_rs::{
    readline::ReadLine as _,
    seg8x4::{Digit, FourDigitDisplay, Seg8x4},
    timer::timer_setup,
};
use avr_device::interrupt;
use avr_device::interrupt::Mutex;
use panic_halt as _;
use ufmt::uwriteln;

struct InterruptState {
    digit: Digit,
    display: FourDigitDisplay,
}

/// The common state between the main loop and the timer interrupt. Use
/// `mut_interrupt_state` to interact with this!
static mut INTERRUPT_STATE: Mutex<Option<InterruptState>> = Mutex::new(None);

/// Safely access & mutate the interrupt state.
///
/// Fair warning: I don't know if this actually is fully safe. But it works.
fn mut_interrupt_state<F: FnOnce(&mut Option<InterruptState>)>(update: F) {
    interrupt::free(|_cs| unsafe {
        update(INTERRUPT_STATE.get_mut());
    });
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // Here we set up our 4x8 segment display. If we want to connect the display
    // to different pins, this is where we'd change things.
    let display = Seg8x4 {
        sa: pins.d11.into_opendrain_high().downgrade(),
        sb: pins.d13.into_opendrain_high().downgrade(),
        sc: pins.d3.into_opendrain_high().downgrade(),
        sd: pins.d5.into_opendrain_high().downgrade(),
        se: pins.d6.into_opendrain_high().downgrade(),
        sf: pins.d12.into_opendrain_high().downgrade(),
        sg: pins.d2.into_opendrain_high().downgrade(),
        sp: pins.d4.into_opendrain_high().downgrade(),
        d1: pins.d8.into_output().downgrade(),
        d2: pins.d9.into_output().downgrade(),
        d3: pins.d10.into_output().downgrade(),
        d4: pins.d7.into_output().downgrade(),
    }
    .into_four_digit_display();

    mut_interrupt_state(|is| {
        *is = Some(InterruptState {
            display,
            digit: Digit::D1,
        });
    });

    // We set up the timer such that each digit is shown 200 times per second.
    // If the frequency is too high, the main loop doesn't have time to run. If
    // its too low, the display will flicker.
    timer_setup(&dp.TC1, 4 * 200);

    unsafe {
        interrupt::enable();
    }

    uwriteln!(serial, "Give me some 4 digit numbers:").unwrap();

    loop {
        // Read a line from serial and convert it into a number, ignoring any
        // non-digits in the input.
        let number = {
            let mut buffer = [0; 32];
            let bytes = serial.read_line(&mut buffer);

            let mut factor = 1;
            bytes
                .iter()
                .rev()
                .filter(|b| b.is_ascii_digit())
                .map(|b| {
                    let r = factor * ((b - b'0') as u16);
                    factor *= 10;
                    r
                })
                .sum()
        };

        // Update the display to show the newly read number
        mut_interrupt_state(|is| {
            if let Some(is) = is {
                is.display.set_number(number, 10);
            }
        });
    }
}

#[interrupt(atmega328p)]
fn TIMER1_COMPA() {
    // Upon each timer interrupt, we rotate through to the next digit to show.
    mut_interrupt_state(|is| {
        if let Some(is) = is {
            is.digit = is.digit.next();
            is.display.show(is.digit);
        }
    });
}
