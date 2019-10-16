#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]
#![allow(dead_code)]
#![allow(unused)]

extern crate panic_semihosting;

// use cortex_m_semihosting::hprintln;
use rtfm::app;
use stm32l0xx_hal::{gpio, pac, prelude::*, rcc, rcc::Config, timer};

mod charlieplexing;
mod led_ring;

use charlieplexing::Sequencer;
use led_ring::pattern::{Rpm, ToDegree, ToRpm};

#[app(device = stm32l0xx_hal::pac)]
const APP: () = {
    static mut PIN_SEQ: Option<Sequencer> = None;

    static mut P: gpio::gpioa::PA1<gpio::Output<gpio::PushPull>> = ();
    static mut TIMER: timer::Timer<pac::TIM2> = ();

    #[init]
    fn init() -> init::LateResources {
        let mcu: pac::Peripherals = device;
        // let _core: rtfm::Peripherals = core;

        let mut rcc_cfg = mcu.RCC.freeze(Config::hsi16());

        // Configure GPIO
        let gpioa = mcu.GPIOA.split(&mut rcc_cfg);
        let mut pin_ra = gpioa.pa8.into_push_pull_output();
        let mut pin_rf = gpioa.pa1.into_push_pull_output();

        pin_ra.set_low().unwrap();
        pin_rf.set_high().unwrap();

        // Configure timers
        let mut tim2 = mcu.TIM2.timer(100.ms(), &mut rcc_cfg);

        // Enable interrupts
        tim2.listen();

        init::LateResources {
            P: pin_rf,
            TIMER: tim2,
        }
    }

    #[interrupt(resources = [P, PIN_SEQ, TIMER])]
    fn TIM2() {
        static mut PAT: Option<led_ring::pattern::StrobeSteps> = None;

        // Poor-man's const fn
        match PAT {
            None => {
                *PAT = Some(reference_strobe_pattern((100.0 / 3.0).rpm()));
            }
            Some(_) => {}
        }

        if let Some(p) = PAT {
            let (t, pattern) = p.next();

            let delay: u32 = (1000.0 * t) as u32;

            resources.TIMER.start(delay.ms());

            *resources.PIN_SEQ = Some(Sequencer::from(pattern));
        }

        if resources.P.is_set_low().unwrap() {
            resources.P.set_high().unwrap();
        } else {
            resources.P.set_low().unwrap();
        }

        resources.TIMER.clear_irq();
    }
};

fn reference_strobe_pattern(rpm: led_ring::pattern::Rpm) -> led_ring::pattern::StrobeSteps {
    led_ring::pattern::StrobeSteps::new(rpm, 12.0.deg())
}
