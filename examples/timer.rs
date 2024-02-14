#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::port::mode::Output;
use arduino_hal::port::Pin;
use atmega_4x8seg_display_rs::timer::timer_setup;
use avr_device::interrupt::Mutex;
use avr_device::{asm, interrupt};
use panic_halt as _;

struct InterruptState {
    blinker: Pin<Output>,
}

static mut INTERRUPT_STATE: Mutex<Option<InterruptState>> = Mutex::new(None);

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let led = pins.d13.into_output();

    interrupt::free(|_cs| unsafe {
        *INTERRUPT_STATE.get_mut() = Some(InterruptState {
            blinker: led.downgrade(),
        });
    });

    timer_setup(&dp.TC1, 2);

    // Enable interrupts globally, not a replacement for the specific interrupt enable
    unsafe {
        interrupt::enable();
    }

    loop {
        asm::sleep()
    }
}

#[interrupt(atmega328p)]
fn TIMER1_COMPA() {
    interrupt::free(|_cs| {
        if let Some(state) = unsafe { INTERRUPT_STATE.get_mut() } {
            state.blinker.toggle();
        }
    });
}
