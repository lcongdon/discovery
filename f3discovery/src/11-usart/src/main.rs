#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};
use heapless::Vec;

#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, mut _itm) = aux11::init();

    // A buffer with 32 bytes of capacity
    let mut buffer: Vec<u8, 32> = Vec::new();

    loop {
        buffer.clear();

        loop {
            // Wait until there's data available
            while usart1.isr.read().rxne().bit_is_clear() {}

            // Retrieve the data
            let _byte = usart1.rdr.read().rdr().bits() as u8;

            // Add it to the buffer
            let push_result = buffer.push(_byte);
            match push_result {
                Ok(()) => {
                    iprintln!(&mut _itm.stim[0], "received: {:x}", _byte)
                }
                Err(error) => {
                    iprintln!(&mut _itm.stim[0], "overflow: {:x}", error)
                }
            };

            // Check for new line
            if _byte == b'\r' {
                break;
            }
        }

        // Write the data
        for character in buffer.iter().rev() {
            while usart1.isr.read().txe().bit_is_clear() {}
            usart1.tdr.write(|w| w.tdr().bits(*character as u16));
        }
        while usart1.isr.read().txe().bit_is_clear() {}
        usart1.tdr.write(|w| w.tdr().bits(b'\r' as u16));
        while usart1.isr.read().txe().bit_is_clear() {}
        usart1.tdr.write(|w| w.tdr().bits(b'\n' as u16));
    }
}
