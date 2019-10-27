use charlieplexing::{ApplyPinState, PinState};

use stm32l0xx_hal::{
    prelude::*,
    gpio::{self, Output, PushPull, Input, Floating, TriState, gpioa, gpiob}
};
use core::pin::Pin;

pub struct PinMapping {
    ra: gpioa::PA8<TriState>,
    rb: gpioa::PA10<TriState>,
    rc: gpiob::PB3<TriState>,
    rd: gpioa::PA12<TriState>,
    re: gpiob::PB5<TriState>,
    rf: gpioa::PA1<TriState>,
}

impl PinMapping {
    pub fn new(gpioa: gpioa::Parts, gpiob: gpiob::Parts) -> PinMapping {
        PinMapping {
            ra: gpioa.pa8.into_tristate_output(),
            rb: gpioa.pa10.into_tristate_output(),
            rc: gpiob.pb3.into_tristate_output(),
            rd: gpioa.pa12.into_tristate_output(),
            re: gpiob.pb5.into_tristate_output(),
            rf: gpioa.pa1.into_tristate_output(),
        }
    }
}

impl ApplyPinState for PinMapping {
    fn apply(&mut self, row_nr: usize, state: PinState) {
        match row_nr {
            0 => { self.ra.set(state).unwrap(); }
            1 => { self.rb.set(state).unwrap(); }
            2 => { self.rc.set(state).unwrap(); }
            3 => { self.rd.set(state).unwrap(); }
            4 => { self.re.set(state).unwrap(); }
            5 => { self.rf.set(state).unwrap(); }
            _ => { }
        }
    }
}