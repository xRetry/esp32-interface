use esp_idf_hal::{prelude::*, gpio::{Pins, Pin, PinDriver, InputPin, OutputPin, Gpio18, Gpio10, Gpio17, Disabled, AnyIOPin, Input, Output}, peripheral::Peripheral};

enum PinMode<'a, T> where T: Pin {
    Disabled(PinDriver<'a, T, Disabled>),
    DigitalInput(PinDriver<'a, T, Input>),
    DigitalOutput(PinDriver<'a, T, Output>),
}

impl<'a, T: InputPin> PinMode<'a, T> {
    fn into_digital_input(self) -> Self {
        PinMode::DigitalInput((match self {
            PinMode::Disabled(d) => d.into_input(),
            PinMode::DigitalInput(d) => d.into_input(),
            PinMode::DigitalOutput(d) => d.into_input(),
        }).unwrap())
    }

    fn digital_read(&self) -> u8 {
        return match self {
            PinMode::Disabled(d) => 0,
            PinMode::DigitalInput(d) => d.is_high() as u8,
            PinMode::DigitalOutput(d) => 0,
        }
    }
}

impl<'a, T: OutputPin> PinMode<'a, T> {
    fn into_digital_output(self) -> Self {
        PinMode::DigitalOutput((match self {
            PinMode::Disabled(d) => d.into_output(),
            PinMode::DigitalInput(d) => d.into_output(),
            PinMode::DigitalOutput(d) => d.into_output(),
        }).unwrap())
    }

    fn digital_write(&self) -> u8 {
        return match self {
            PinMode::Disabled(d) => 0,
            PinMode::DigitalInput(d) => d.is_high() as u8,
            PinMode::DigitalOutput(d) => 0,
        }
    }
}

pub struct Driver<'a> {
    gpio10: PinMode<'a, Gpio10>,
    gpio18: PinMode<'a, Gpio18>,
}

impl<'a> Driver<'_> {
    pub fn new(pins: Pins) -> Driver<'a> {
        Driver{
            gpio10: PinMode::Disabled(PinDriver::disabled(pins.gpio10).unwrap()),
            gpio18: PinMode::Disabled(PinDriver::disabled(pins.gpio18).unwrap()),
        }
    }

    pub fn digital_read(&self, pin_nr: i32) -> u8 {
        match pin_nr {
            18 => self.gpio18.digital_read(),
            _ => 0,
        }
    }

    pub fn set_digital_input(mut self, pin_nr: i32) -> Self {
        match pin_nr {
            18 => self.gpio18 = self.gpio18.into_digital_input(),
            _ => (),
        }
        return self
    }
}

