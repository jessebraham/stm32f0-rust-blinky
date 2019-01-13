#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

extern crate stm32f0xx_hal;
use stm32f0xx_hal::{delay::Delay, prelude::*, stm32};

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    if let (Some(mut p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        cortex_m::interrupt::free(move |cs| {
            // Configure clock to 8 MHz (i.e. the default) and freeze it
            let mut rcc = p.RCC.configure().sysclk(8.mhz()).freeze(&mut p.FLASH);

            // (Re-)configure PA1 as output
            let gpioa = p.GPIOA.split(&mut rcc);
            let mut led = gpioa.pa1.into_push_pull_output(cs);

            // Get delay provider
            let mut delay = Delay::new(cp.SYST, &rcc);

            // Toggle the LED roughly every second
            loop {
                led.toggle();
                delay.delay_ms(1_000_u16);
            }
        });
    }

    loop {
        continue;
    }
}
