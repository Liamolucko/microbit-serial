use nrf52833_hal::gpio::p0::P0_11;
use nrf52833_hal::gpio::p0::P0_15;
use nrf52833_hal::gpio::p0::P0_19;
use nrf52833_hal::gpio::p0::P0_21;
use nrf52833_hal::gpio::p0::P0_22;
use nrf52833_hal::gpio::p0::P0_24;
use nrf52833_hal::gpio::p0::P0_28;
use nrf52833_hal::gpio::p0::P0_30;
use nrf52833_hal::gpio::p0::P0_31;
use nrf52833_hal::gpio::p1::P1_05;
use nrf52833_hal::gpio::Output;
use nrf52833_hal::gpio::PushPull;
use nrf52833_hal::prelude::*;
use nrf52833_hal::timer::Instance;
use tiny_led_matrix::DisplayControl;
use tiny_led_matrix::DisplayTimer;
use tiny_led_matrix::Frame;
use tiny_led_matrix::Matrix;
use tiny_led_matrix::Render;
use tiny_led_matrix::RowPlan;

pub struct MicrobitMatrix;
impl Matrix for MicrobitMatrix {
    const MATRIX_COLS: usize = 5;
    const MATRIX_ROWS: usize = 5;
    const IMAGE_COLS: usize = 5;
    const IMAGE_ROWS: usize = 5;
    fn image_coordinates(col: usize, row: usize) -> Option<(usize, usize)> {
        Some((col, row))
    }
}

pub struct DisplayPins {
    pub row1: P0_21<Output<PushPull>>,
    pub row2: P0_22<Output<PushPull>>,
    pub row3: P0_15<Output<PushPull>>,
    pub row4: P0_24<Output<PushPull>>,
    pub row5: P0_19<Output<PushPull>>,
    pub col1: P0_28<Output<PushPull>>,
    pub col2: P0_11<Output<PushPull>>,
    pub col3: P0_31<Output<PushPull>>,
    pub col4: P1_05<Output<PushPull>>,
    pub col5: P0_30<Output<PushPull>>,
}

impl DisplayControl for DisplayPins {
    fn initialise_for_display(&mut self) {}

    fn display_row_leds(&mut self, row: usize, cols: u32) {
        self.row1.set_low().expect("failed to set display pins");
        self.row2.set_low().expect("failed to set display pins");
        self.row3.set_low().expect("failed to set display pins");
        self.row4.set_low().expect("failed to set display pins");
        self.row5.set_low().expect("failed to set display pins");

        if cols & 0b00001 > 0 {
            self.col1.set_low().expect("failed to set display pins");
        } else {
            self.col1.set_high().expect("failed to set display pins");
        }
        if cols & 0b00010 > 0 {
            self.col2.set_low().expect("failed to set display pins");
        } else {
            self.col2.set_high().expect("failed to set display pins");
        }
        if cols & 0b00100 > 0 {
            self.col3.set_low().expect("failed to set display pins");
        } else {
            self.col3.set_high().expect("failed to set display pins");
        }
        if cols & 0b01000 > 0 {
            self.col4.set_low().expect("failed to set display pins");
        } else {
            self.col4.set_high().expect("failed to set display pins");
        }
        if cols & 0b10000 > 0 {
            self.col5.set_low().expect("failed to set display pins");
        } else {
            self.col5.set_high().expect("failed to set display pins");
        }

        if row == 0 {
            self.row1.set_high().expect("failed to set display pins");
        }
        if row == 1 {
            self.row2.set_high().expect("failed to set display pins");
        }
        if row == 2 {
            self.row3.set_high().expect("failed to set display pins");
        }
        if row == 3 {
            self.row4.set_high().expect("failed to set display pins");
        }
        if row == 4 {
            self.row5.set_high().expect("failed to set display pins");
        }
    }

    fn light_current_row_leds(&mut self, cols: u32) {
        // The 'current' row should be left high from the last call to `display_row_leds`.
        if cols & 0b00001 > 0 {
            self.col1.set_low().expect("failed to set display pins");
        }
        if cols & 0b00010 > 0 {
            self.col2.set_low().expect("failed to set display pins");
        }
        if cols & 0b00100 > 0 {
            self.col3.set_low().expect("failed to set display pins");
        }
        if cols & 0b01000 > 0 {
            self.col4.set_low().expect("failed to set display pins");
        }
        if cols & 0b10000 > 0 {
            self.col5.set_low().expect("failed to set display pins");
        }
    }
}

pub struct MicrobitDisplayTimer<T: Instance>(pub T);
impl<T: Instance> DisplayTimer for MicrobitDisplayTimer<T> {
    fn initialise_cycle(&mut self, ticks: u16) {
        // Stop the timer.
        self.0
            .as_timer0()
            .tasks_stop
            .write(|w| w.tasks_stop().set_bit());
        // Clear the timer.
        self.0
            .as_timer0()
            .tasks_clear
            .write(|w| w.tasks_clear().set_bit());
        // Set frequency to 1MHz (16MHz / 2^4). This is the frequency used by the official runtime.
        // It's called a prescaler because it's still using the same underlying, 16MHz clock,
        // but only counts every 4th tick (in this case). So, it's just _scaling_ the output.
        self.0
            .as_timer0()
            .prescaler
            .write(|w| unsafe { w.prescaler().bits(4) });
        // Set the timer's first Capture/Compare register to `ticks`, and then enable the corresponding interrupt.
        // Every time the counter is incremented, the timer will check if it equals this value, and if so trigger the interrupt.
        // So this sets up the timer to send an interrupt once its counter reaches `ticks`.
        self.0.as_timer0().cc[0].write(|w| unsafe { w.cc().bits(ticks.into()) });
        self.0.as_timer0().intenset.write(|w| w.compare0().set());
        // Set the timer to clear itself when it reaches `ticks`.
        // If we don't do this, the counter will just continue until it overflows.
        self.0
            .as_timer0()
            .shorts
            .write(|w| w.compare0_clear().enabled());
        // Start the timer.
        self.0
            .as_timer0()
            .tasks_start
            .write(|w| w.tasks_start().set_bit());
    }

    fn enable_secondary(&mut self) {
        self.0
            .as_timer0()
            .intenset
            .write(|w| w.compare1().set());
    }

    fn disable_secondary(&mut self) {
        self.0
            .as_timer0()
            .intenset
            .write(|w| w.compare1().clear_bit());
    }

    fn program_secondary(&mut self, ticks: u16) {
        self.0.as_timer0().cc[1].write(|w| unsafe { w.cc().bits(ticks.into()) });
    }

    fn check_primary(&mut self) -> bool {
        let reg = &self.0.as_timer0().events_compare[0];
        let fired = reg.read().bits() != 0;
        if fired {
            reg.reset();
        }
        fired
    }

    fn check_secondary(&mut self) -> bool {
        let reg = &self.0.as_timer0().events_compare[1];
        let fired = reg.read().bits() != 0;
        if fired {
            reg.reset();
        }
        fired
    }
}

#[derive(Clone, Copy)]
pub struct MicrobitFrame(pub [RowPlan; 5]);

impl Default for MicrobitFrame {
    fn default() -> Self {
        Self([RowPlan::default(); 5])
    }
}

impl Frame for MicrobitFrame {
    type Mtx = MicrobitMatrix;

    const COLS: usize = 5;
    const ROWS: usize = 5;

    fn row_plan(&self, row: usize) -> &RowPlan {
        &self.0[row]
    }

    fn row_plan_mut(&mut self, row: usize) -> &mut RowPlan {
        &mut self.0[row]
    }
}

/// A 5×5 image supporting the full range of brightnesses for each LED.
///
/// Uses 25 bytes of storage.
#[derive(Copy, Clone, Debug)]
pub struct GreyscaleImage([[u8; 5]; 5]);

impl GreyscaleImage {
    /// Constructs a GreyscaleImage from an array of brightnesses.
    ///
    /// The data should be an array of 5 rows (top first), each of which is an
    /// array of 5 brightness values (left first).
    ///
    /// # Example
    ///
    /// ```
    /// const GREY_HEART: GreyscaleImage = GreyscaleImage::new(&[
    ///     [0, 9, 0, 9, 0],
    ///     [9, 5, 9, 5, 9],
    ///     [9, 5, 5, 5, 9],
    ///     [0, 9, 5, 9, 0],
    ///     [0, 0, 9, 0, 0],
    /// ]);
    /// ```
    pub const fn new(data: &[[u8; 5]; 5]) -> GreyscaleImage {
        GreyscaleImage(*data)
    }

    pub const fn blank() -> GreyscaleImage {
        GreyscaleImage([[0; 5]; 5])
    }
}

impl Render for GreyscaleImage {
    fn brightness_at(&self, x: usize, y: usize) -> u8 {
        self.0[y][x]
    }
}

impl Render for &GreyscaleImage {
    fn brightness_at(&self, x: usize, y: usize) -> u8 {
        GreyscaleImage::brightness_at(self, x, y)
    }
}
