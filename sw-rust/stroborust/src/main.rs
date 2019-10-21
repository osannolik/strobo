#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]
//#![allow(dead_code)]
#![allow(unused)]

extern crate panic_semihosting;

//#[macro_use(block)]
//extern crate nb;

// use cortex_m_semihosting::hprintln;
use rtfm::app;
use stm32l0xx_hal::{gpio, pac, prelude::*, rcc::Config, timer};

mod charlieplexing;
mod led_ring;

use charlieplexing::Sequencer;
use led_ring::pattern::ToDegree;

#[app(device = stm32l0xx_hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        #[init(None)]
        pin_seq: Option<charlieplexing::PinSequence>,

        test_pin: gpio::gpioa::PA1<gpio::Output<gpio::PushPull>>,
        pattern_timer: timer::Timer<pac::TIM2>,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        let mcu: pac::Peripherals = cx.device;

        let mut rcc_cfg = mcu.RCC.freeze(Config::hsi16());

        // Configure GPIO
        let gpioa = mcu.GPIOA.split(&mut rcc_cfg);
        let mut pin_ra = gpioa.pa8.into_push_pull_output();
        let mut pin_rf = gpioa.pa1.into_push_pull_output();

        pin_ra.set_low().unwrap();
        pin_rf.set_high().unwrap();

        // Configure timers
        let mut ptim = mcu.TIM2.timer(100.ms(), &mut rcc_cfg);

        // Enable interrupts
        ptim.listen();

        init::LateResources {
            test_pin: pin_rf,
            pattern_timer: ptim,
        }
    }

    #[task(binds = TIM2, resources = [test_pin, pin_seq, pattern_timer])]
    fn tim2(cx: tim2::Context) {
        static mut PAT: led_ring::pattern::StrobeSteps = led_ring::pattern::RPM33_DEFAULT;

        let delay = (PAT.time() * 1000.0) as u32;

        cx.resources.pattern_timer.start_us(delay.ms());

        *cx.resources.pin_seq = Some(Sequencer::from(PAT.pattern()).pins());

        if cx.resources.test_pin.is_set_low().unwrap() {
            cx.resources.test_pin.set_high().unwrap();
        } else {
            cx.resources.test_pin.set_low().unwrap();
        }

        PAT.next();

        cx.resources.pattern_timer.clear_irq();
    }
};

fn reference_strobe_pattern(rpm: led_ring::pattern::Rpm) -> led_ring::pattern::StrobeSteps {
    led_ring::pattern::StrobeSteps::new(rpm, 12.0.deg())
}
