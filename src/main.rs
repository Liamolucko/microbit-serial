#![no_std]
#![no_main]

extern crate panic_semihosting;
use cortex_m_rt::entry;
use nrf52833_hal::gpio::p0;
use nrf52833_hal::gpio::p1;
use nrf52833_hal::gpio::Level;
use nrf52833_hal::pac::Peripherals;
use nrf52833_hal::uarte::Baudrate;
use nrf52833_hal::uarte::Parity;
use nrf52833_hal::uarte::Pins;
use nrf52833_hal::uarte::Uarte;

#[entry]
fn main() -> ! {
    if let Some(p) = Peripherals::take() {
        let p0 = p0::Parts::new(p.P0);
        let p1 = p1::Parts::new(p.P1);
        let pins = Pins {
            rxd: p1.p1_08.degrade().into_floating_input(),
            txd: p0.p0_06.degrade().into_push_pull_output(Level::Low),
            cts: None,
            rts: None,
        };
        let mut serial = Uarte::new(p.UARTE0, pins, Parity::EXCLUDED, Baudrate::BAUD115200);
        // this needs to be in RAM for some reason so store it in a local variable
        let string = *b"hello world\n\r";
        serial.write(&string).unwrap();
    }

    loop {}
}
