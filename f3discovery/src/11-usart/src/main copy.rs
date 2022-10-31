#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (usart1, mono_timer, mut itm) = aux11::init();

    let instant = mono_timer.now();

    for character in "The quick brown fox jumps over the lazy dog".chars() {
        while usart1.isr.read().txe().bit_is_clear() {}
        usart1.tdr.write(|w| w.tdr().bits(character as u16));
    }
    let elapsed = instant.elapsed(); // in ticks

    iprintln!(&mut itm.stim[0],
    "`for` loop took {} ticks ({} us)",
    elapsed,
    elapsed as f32 / mono_timer.frequency().0 as f32 * 1e6
    );

    loop {}
}
