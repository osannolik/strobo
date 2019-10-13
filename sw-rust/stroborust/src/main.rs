#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

// use cortex_m_semihosting::hprintln;
use rtfm::app;
use stm32l0xx_hal::{timer, gpio, pac, prelude::*, rcc::Config};

#[app(device = stm32l0xx_hal::pac)]
const APP: () = {
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
        pin_rf.set_low().unwrap();

        // Configure timers
        let mut tim2 = mcu.TIM2.timer(100.ms(), &mut rcc_cfg);

        // Enable interrupts
        tim2.listen();

        init::LateResources { P: pin_rf, TIMER: tim2 }
    }

    #[interrupt(resources = [P, TIMER])]
    fn TIM2() {
        if resources.P.is_set_low().unwrap() {
            resources.P.set_high().unwrap();
        } else {
            resources.P.set_low().unwrap();
        }

        resources.TIMER.clear_irq();
    }
};
