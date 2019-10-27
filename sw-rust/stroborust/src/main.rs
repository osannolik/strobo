#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]
//#![allow(dead_code)]
#![allow(unused)]

extern crate panic_semihosting;
extern crate charlieplexing;

//#[macro_use(block)]
//extern crate nb;

mod board;
mod led_ring;

// use cortex_m_semihosting::hprintln;
use rtfm::app;
use stm32l0xx_hal::{gpio, pac, prelude::*, rcc::Config, timer};

use led_ring::ToDegree;
use charlieplexing::{ApplyPinState, Sequencer, Charlieplexer};
use board::PinMapping;

#[app(device = stm32l0xx_hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        #[init(None)]
        pin_seq: Option<charlieplexing::Sequencer>,

        pattern_timer: timer::Timer<pac::TIM2>,
        charlie_timer: timer::Timer<pac::TIM21>,

        pin_map: PinMapping,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        let mcu: pac::Peripherals = cx.device;

        let mut rcc_cfg = mcu.RCC.freeze(Config::hsi16());

        // Configure GPIO
        let gpioa = mcu.GPIOA.split(&mut rcc_cfg);
        let gpiob = mcu.GPIOB.split(&mut rcc_cfg);

        // Configure timers
        let mut ptim = mcu.TIM2.timer(100.ms(), &mut rcc_cfg);
        let mut ctim = mcu.TIM21.timer(200.us(), &mut rcc_cfg);

        // Enable interrupts
        ptim.listen();
        ctim.listen();

        init::LateResources {
            pattern_timer: ptim,
            charlie_timer: ctim,
            pin_map: board::PinMapping::new(gpioa, gpiob),
        }
    }

    #[task(binds = TIM2, resources = [pin_seq, pattern_timer])]
    fn pattern_update(mut cx: pattern_update::Context) {
        static mut PAT: led_ring::StrobeSteps = led_ring::RPM33_DEFAULT;

        let delay = (PAT.time() * 1_000_000.0) as u32;

        cx.resources.pattern_timer.start(delay.us());

        *cx.resources.pin_seq = Some(Sequencer::new(PAT.pattern()));

        PAT.next();

        cx.resources.pattern_timer.clear_irq();
    }

    #[task(binds = TIM21, resources = [pin_seq, pin_map, charlie_timer])]
    fn charlie_update(cx: charlie_update::Context) {
        cx.resources.charlie_timer.clear_irq();

        match cx.resources.pin_seq {
            Some(mut seq) => {
                seq.apply(cx.resources.pin_map);
                *cx.resources.pin_seq = Some(seq.next());
            }
            None => {}
        }
    }
};

fn reference_strobe_pattern(rpm: led_ring::Rpm) -> led_ring::StrobeSteps {
    led_ring::StrobeSteps::new(rpm, 12.0.deg())
}
