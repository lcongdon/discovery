#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{Delay, DelayMs, LedArray, OutputSwitch, entry};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let interval = 50_u16;
    
    // set initial status for LEDs in loop   
    leds[0].on().ok();
    leds[1].on().ok();

    let mut index = 0;

    loop {

        leds[(index + 1) % 8].on().ok();
        delay.delay_ms(interval);

        leds[index].off().ok();
        delay.delay_ms(interval);

        index = (index + 1) % 8;
    }
}
