#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

extern crate stm32f0xx_hal;
use stm32f0xx_hal::delay::Delay;
use stm32f0xx_hal::prelude::*;
use stm32f0xx_hal::stm32;

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        // (Re-)configure PA1 as output
        let gpioa = p.GPIOA.split();
        let mut led = gpioa.pa1.into_push_pull_output();

        // Constrain clocking registers and configure clock to 8 MHz
        // (i.e. the default) and freeze it
        let rcc = p.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(8.mhz()).freeze();

        // Get delay provider
        let mut delay = Delay::new(cp.SYST, clocks);

        // Toggle the LED roughly every second
        loop {
            led.toggle();
            delay.delay_ms(1_000_u16);
        }
    }

    loop {
        continue;
    }
}
