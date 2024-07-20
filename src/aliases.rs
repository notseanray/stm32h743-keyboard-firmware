//! A collection of type aliases to cut down on code clutter

use analog_multiplexer::{DummyPin, Multiplexer};
use stm32h7xx_hal::gpio::gpioa::{PA0, PA1, PA2, PA3, PA4, PA5, PA6, PA7, PA8, PA9, PA10, PA15};
use stm32h7xx_hal::gpio::gpiob::{PB0, PB1, PB3, PB4, PB5, PB8, PB9, PB10, PB12};
use stm32h7xx_hal::gpio::gpioc::{PC13};
use stm32h7xx_hal::gpio::{Alternate, Analog, Input, Output, PushPull, AF5, AF6};

// Handy type aliases to avoid a lot of long lines/typing later...
pub type AnalogPins = (
    PA0<Analog>,
    PA1<Analog>,
    PA2<Analog>,
    PA3<Analog>,
    PA4<Analog>,
);
// Define which pins go to where on your analog multiplexer's select lines
pub type S0 = PC13<Output<PushPull>>; // These just make things easier to read/reason about
pub type S1 = PB8<Output<PushPull>>; // aka "very expressive"
pub type S2 = PB12<Output<PushPull>>;
pub type S3 = PA15<Output<PushPull>>;
pub type EN = DummyPin; // NOTE: We assume the enable pin goes to GND at all times

// Power status pin (goes floating when power is connected)
pub type POWER = PB10<Input>;

// Relay pin
pub type RELAY1 = PB1<Output<PushPull>>;
pub type RELAY2 = PA9<Output<PushPull>>;
pub type RELAY3 = PA10<Output<PushPull>>;

// TODO: Add a proper DummyPin struct somewhere we can use with EN
// NOTE: embedded_hal really needs a DummyPin feature for things like unused driver pins!
pub type SelectPins = (S0, S1, S2, S3, EN);
pub type Multiplex = Multiplexer<SelectPins>;
