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
use nrf52833_hal::pac::TIMER4;
use nrf52833_hal::prelude::*;
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
        if row == 0 {
            self.row1.set_high().expect("failed to set display pins");
        } else {
            self.row1.set_low().expect("failed to set display pins");
        }
        if row == 1 {
            self.row2.set_high().expect("failed to set display pins");
        } else {
            self.row2.set_low().expect("failed to set display pins");
        }
        if row == 2 {
            self.row3.set_high().expect("failed to set display pins");
        } else {
            self.row3.set_low().expect("failed to set display pins");
        }
        if row == 3 {
            self.row4.set_high().expect("failed to set display pins");
        } else {
            self.row4.set_low().expect("failed to set display pins");
        }
        if row == 4 {
            self.row5.set_high().expect("failed to set display pins");
        } else {
            self.row5.set_low().expect("failed to set display pins");
        }

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

pub struct MicrobitDisplayTimer(pub TIMER4);
impl DisplayTimer for MicrobitDisplayTimer {
    fn initialise_cycle(&mut self, ticks: u16) {
        self.0.prescaler.write(|w| unsafe { w.bits(4) });
        self.0.cc[0].write(|w| unsafe { w.cc().bits(ticks.into()) });
        self.0.tasks_clear.write(|w| unsafe { w.bits(1) });
        self.0.shorts.modify(|_, w| w.compare0_clear().enabled());
        self.0.tasks_start.write(|w| unsafe { w.bits(1) });
        self.0.intenset.modify(|_, w| w.compare0().set());
    }

    fn enable_secondary(&mut self) {
        self.0.intenset.modify(|_, w| w.compare1().set());
    }

    fn disable_secondary(&mut self) {
        self.0.intenset.modify(|_, w| w.compare1().clear_bit());
    }

    fn program_secondary(&mut self, ticks: u16) {
        self.0.cc[1].write(|w| unsafe { w.cc().bits(ticks.into()) });
    }

    fn check_primary(&mut self) -> bool {
        let reg = &self.0.events_compare[0];
        let fired = reg.read().bits() != 0;
        if fired {
            reg.reset();
        }
        fired
    }

    fn check_secondary(&mut self) -> bool {
        let reg = &self.0.events_compare[1];
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
