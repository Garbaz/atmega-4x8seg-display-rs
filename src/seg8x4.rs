//! Two levels of abstraction for a common cathode 4x8 segment display
//!
//! ## Usage
//!
//! Create a four digit display
//!
//! ```rs
//! let display = Seg8x4 { ... }.into_four_digit_display();
//! ```

use arduino_hal::port::{
    mode::{OpenDrain, Output},
    Pin,
};

/// For every (hexadecimal) digit from '0' to 'F', which segments have to be
/// enabled and which disabled
const NUM_TO_SEGMENTS: [[bool; 8]; 16] = [
    /*0*/ [true, true, true, true, true, true, false, false],
    /*1*/ [false, true, true, false, false, false, false, false],
    /*2*/ [true, true, false, true, true, false, true, false],
    /*3*/ [true, true, true, true, false, false, true, false],
    /*4*/ [false, true, true, false, false, true, true, false],
    /*5*/ [true, false, true, true, false, true, true, false],
    /*6*/ [true, false, true, true, true, true, true, false],
    /*7*/ [true, true, true, false, false, false, false, false],
    /*8*/ [true, true, true, true, true, true, true, false],
    /*9*/ [true, true, true, true, false, true, true, false],
    /*A*/ [true, true, true, false, true, true, true, false],
    /*B*/ [false, false, true, true, true, true, true, false],
    /*C*/ [true, false, false, true, true, true, false, false],
    /*D*/ [false, true, true, true, true, false, true, false],
    /*E*/ [true, false, false, true, true, true, true, false],
    /*F*/ [true, false, false, false, true, true, true, false],
];

#[derive(Clone, Copy)]
pub enum Digit {
    D1,
    D2,
    D3,
    D4,
}

impl Digit {
    pub const fn next(self) -> Self {
        match self {
            Digit::D1 => Digit::D2,
            Digit::D2 => Digit::D3,
            Digit::D3 => Digit::D4,
            Digit::D4 => Digit::D1,
        }
    }
}

impl From<u8> for Digit {
    fn from(value: u8) -> Self {
        match value % 4 {
            0 => Self::D1,
            1 => Self::D2,
            2 => Self::D3,
            _ => Self::D4,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Segment {
    SA,
    SB,
    SC,
    SD,
    SE,
    SF,
    SG,
    SP,
}

impl From<u8> for Segment {
    fn from(value: u8) -> Self {
        match value % 8 {
            0 => Self::SA,
            1 => Self::SB,
            2 => Self::SC,
            3 => Self::SD,
            4 => Self::SE,
            5 => Self::SF,
            6 => Self::SG,
            _ => Self::SP,
        }
    }
}

/// Represents the raw 4x8 segment display. Usually you'd create this struct and
/// then call `into_four_digit_display` to turn it into a convenient stateful
/// four digit display (See [`FourDigitDisplay`]).
///
/// `sa` to `sf` are the pins to which the cathodes of the seven segments are
/// connected to.
///
/// `sp` is the pin to which cathode of the decimal point is connected to.
///
/// `d1` to `d4` are the pins to which the anodes of the four digits are
/// connected to.
/// 
/// **Don't forget the resistors on the anode pins, or things will go _pop_!**
///
/// The cathode pins are in the "open drain" mode, such that we can toggle them
/// between being pulled low or disconnected. This is very important, since if
/// we were to pull a cathode pin high and an anode pin low, we would be
/// applying a reverse voltage to the LED in the display, ergo it go _pop_. We
/// don't want that!
///
/// The anode pins are set as normal outputs, such that we can toggle them
/// between being pulled high and pulled low. We don't actually have to pull
/// them low, but there is no "open source" mode.
pub struct Seg8x4 {
    pub sa: Pin<OpenDrain>,
    pub sb: Pin<OpenDrain>,
    pub sc: Pin<OpenDrain>,
    pub sd: Pin<OpenDrain>,
    pub se: Pin<OpenDrain>,
    pub sf: Pin<OpenDrain>,
    pub sg: Pin<OpenDrain>,
    pub sp: Pin<OpenDrain>,
    pub d1: Pin<Output>,
    pub d2: Pin<Output>,
    pub d3: Pin<Output>,
    pub d4: Pin<Output>,
}

impl Seg8x4 {
    pub fn enable_digit(&mut self, digit: Digit) {
        match digit {
            Digit::D1 => self.d1.set_high(),
            Digit::D2 => self.d2.set_high(),
            Digit::D3 => self.d3.set_high(),
            Digit::D4 => self.d4.set_high(),
        }
    }

    pub fn disable_digit(&mut self, digit: Digit) {
        match digit {
            Digit::D1 => self.d1.set_low(),
            Digit::D2 => self.d2.set_low(),
            Digit::D3 => self.d3.set_low(),
            Digit::D4 => self.d4.set_low(),
        }
    }

    pub fn disable_all_digits(&mut self) {
        self.d1.set_low();
        self.d2.set_low();
        self.d3.set_low();
        self.d4.set_low();
    }

    pub fn enable_segment(&mut self, segment: Segment) {
        match segment {
            Segment::SA => self.sa.set_low(),
            Segment::SB => self.sb.set_low(),
            Segment::SC => self.sc.set_low(),
            Segment::SD => self.sd.set_low(),
            Segment::SE => self.se.set_low(),
            Segment::SF => self.sf.set_low(),
            Segment::SG => self.sg.set_low(),
            Segment::SP => self.sp.set_low(),
        }
    }

    pub fn disable_segment(&mut self, segment: Segment) {
        match segment {
            Segment::SA => self.sa.set_high(),
            Segment::SB => self.sb.set_high(),
            Segment::SC => self.sc.set_high(),
            Segment::SD => self.sd.set_high(),
            Segment::SE => self.se.set_high(),
            Segment::SF => self.sf.set_high(),
            Segment::SG => self.sg.set_high(),
            Segment::SP => self.sp.set_high(),
        }
    }

    pub fn disable_all_segments(&mut self) {
        self.sa.set_high();
        self.sb.set_high();
        self.sc.set_high();
        self.sd.set_high();
        self.se.set_high();
        self.sf.set_high();
        self.sg.set_high();
        self.sp.set_high();
    }

    pub fn enable_segments(&mut self, segments: [bool; 8]) {
        for (i, s) in segments.into_iter().enumerate() {
            if s {
                self.enable_segment((i as u8).into());
            } else {
                self.disable_segment((i as u8).into());
            }
        }
    }

    pub fn set_segments_to_number(&mut self, number: u8) {
        self.enable_segments(NUM_TO_SEGMENTS[(number % 16) as usize]);
    }

    pub fn enable_decimal_point(&mut self) {
        self.enable_segment(Segment::SP);
    }

    pub fn disable_decimal_point(&mut self) {
        self.disable_segment(Segment::SP);
    }

    pub fn into_four_digit_display(self) -> FourDigitDisplay {
        self.into()
    }
}

/// A convenient stateful four digit display abstracting over [`Seg8x4`].
///
/// The intended use is to set the number to be displayed with
/// [`Self::set_number`] (say in the main loop of the program), and to rapidly
/// rotate through calling for each digit `Self::show` (say in a timer ISR).
pub struct FourDigitDisplay {
    state_d1: [bool; 8],
    state_d2: [bool; 8],
    state_d3: [bool; 8],
    state_d4: [bool; 8],
    pub seg8x4: Seg8x4,
}

impl FourDigitDisplay {
    /// Set the display to show the given number in the given base. The number
    /// can be any `u16`, but only the lowest four digits will be displayed
    /// (obviously). But the base **has to be** less than or equal to 16, or the
    /// function will **panic**.
    ///
    /// Usually you'd call this function from the main loop of your program.
    pub fn set_number(&mut self, number: u16, base: u8) {
        assert!(base <= 16);

        let base1 = base as u16;
        let base2 = base1 * base1;
        let base3 = base1 * base2;

        self.state_d1 = NUM_TO_SEGMENTS[((number / base3) % base1) as usize];
        self.state_d2 = NUM_TO_SEGMENTS[((number / base2) % base1) as usize];
        self.state_d3 = NUM_TO_SEGMENTS[((number / base1) % base1) as usize];
        self.state_d4 = NUM_TO_SEGMENTS[(number % base1) as usize];
    }

    /// Enable the decimal point for a digit.
    pub fn set_decimal_point(&mut self, digit: Digit) {
        self.state_mut(digit)[7] = true;
    }

    fn state_mut(&mut self, digit: Digit) -> &mut [bool; 8] {
        match digit {
            Digit::D1 => &mut self.state_d1,
            Digit::D2 => &mut self.state_d2,
            Digit::D3 => &mut self.state_d3,
            Digit::D4 => &mut self.state_d4,
        }
    }

    fn state(&self, digit: Digit) -> [bool; 8] {
        match digit {
            Digit::D1 => self.state_d1,
            Digit::D2 => self.state_d2,
            Digit::D3 => self.state_d3,
            Digit::D4 => self.state_d4,
        }
    }

    /// Show one digit of the four digit display.
    ///
    /// Since we have a common cathode display, only one digit at a time can be
    /// shown. Therefore for all four digits to be visible, this function has to
    /// be called rapidly for each digit one after the other.
    ///
    /// Usually you'd call this function from a timer ISR.
    pub fn show(&mut self, digit: Digit) {
        self.seg8x4.disable_all_digits();
        self.seg8x4.enable_segments(self.state(digit));
        self.seg8x4.enable_digit(digit);
    }
}

impl From<Seg8x4> for FourDigitDisplay {
    fn from(value: Seg8x4) -> Self {
        Self {
            state_d1: [false; 8],
            state_d2: [false; 8],
            state_d3: [false; 8],
            state_d4: [false; 8],
            seg8x4: value,
        }
    }
}
