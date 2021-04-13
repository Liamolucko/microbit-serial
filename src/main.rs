#![no_std]
#![no_main]

extern crate panic_semihosting;

mod display;
mod timer;

use display::DisplayPins;
use display::GreyscaleImage;
use display::MicrobitDisplayTimer;
use display::MicrobitFrame;
use nrf52833_hal::gpio::p0;
use nrf52833_hal::gpio::p0::P0_14;
use nrf52833_hal::gpio::p0::P0_23;
use nrf52833_hal::gpio::p1;
use nrf52833_hal::gpio::Floating;
use nrf52833_hal::gpio::Input;
use nrf52833_hal::gpio::Level;
use nrf52833_hal::pac::TIMER0;
use nrf52833_hal::pac::TIMER4;
use nrf52833_hal::prelude::*;
use nrf52833_hal::Timer;
use rtic::app;
use tiny_led_matrix::Display;
use tiny_led_matrix::Frame;

#[app(device = nrf52833_hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        display_pins: DisplayPins,
        display_timer: MicrobitDisplayTimer<TIMER4>,
        display: Display<MicrobitFrame>,
        btn_a: P0_14<Input<Floating>>,
        btn_b: P0_23<Input<Floating>>,
        timer0: Timer<TIMER0>,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        let p0 = p0::Parts::new(cx.device.P0);
        let p1 = p1::Parts::new(cx.device.P1);
        let mut display_pins = DisplayPins {
            col1: p0.p0_28.into_push_pull_output(Level::Low),
            col2: p0.p0_11.into_push_pull_output(Level::Low),
            col3: p0.p0_31.into_push_pull_output(Level::Low),
            col4: p1.p1_05.into_push_pull_output(Level::Low),
            col5: p0.p0_30.into_push_pull_output(Level::Low),
            row1: p0.p0_21.into_push_pull_output(Level::Low),
            row2: p0.p0_22.into_push_pull_output(Level::Low),
            row3: p0.p0_15.into_push_pull_output(Level::Low),
            row4: p0.p0_24.into_push_pull_output(Level::Low),
            row5: p0.p0_19.into_push_pull_output(Level::Low),
        };
        tiny_led_matrix::initialise_control(&mut display_pins);
        let mut display_timer = MicrobitDisplayTimer(cx.device.TIMER4);
        tiny_led_matrix::initialise_timer(&mut display_timer);
        let display = Display::new();

        init::LateResources {
            display_pins,
            display_timer,
            display,
            btn_a: p0.p0_14.into_floating_input(),
            btn_b: p0.p0_23.into_floating_input(),
            timer0: Timer::new(cx.device.TIMER0),
        }
    }

    #[idle(resources = [btn_a, btn_b, display, timer0])]
    fn idle(mut cx: idle::Context) -> ! {
        loop {
            let mut img = [[0; 5]; 5];
            if cx.resources.btn_a.is_low().unwrap() {
                img[1][1] = 9;
            }
            if cx.resources.btn_b.is_low().unwrap() {
                img[1][3] = 9;
            }
            if cx.resources.btn_a.is_low().unwrap() && cx.resources.btn_b.is_low().unwrap() {
                img[3][0] = 9;
                img[3][4] = 9;
                img[4] = [0, 9, 9, 9, 0];
            }
            cx.resources.timer0.delay_ms(6u8);
            let mut frame = MicrobitFrame::default();
            frame.set(&GreyscaleImage::new(&img));
            cx.resources
                .display
                .lock(|display| display.set_frame(&frame));
        }
    }

    #[task(binds = TIMER4, priority = 2, resources = [display, display_timer, display_pins])]
    fn timer4(cx: timer4::Context) {
        cx.resources
            .display
            .handle_event(cx.resources.display_timer, cx.resources.display_pins);
    }
};
