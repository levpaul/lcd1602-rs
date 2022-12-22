use crate::error::Error;
use crate::LCD1602;

use crate::error::Error::{InvalidCursorPos, UnsupportedBusWidth};
use crate::lcd1602::BusWidth::FourBits;
use crate::lcd1602::Direction::RightToLeft;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::timer::CountDown;
use embedded_time::duration::{Extensions, Microseconds};
use nb::block;

impl<EN, RS, D4, D5, D6, D7, Timer, E> LCD1602<EN, RS, D4, D5, D6, D7, Timer>
where
    EN: OutputPin<Error = E>,
    RS: OutputPin<Error = E>,
    D4: OutputPin<Error = E>,
    D5: OutputPin<Error = E>,
    D6: OutputPin<Error = E>,
    D7: OutputPin<Error = E>,
    Timer: CountDown,
{
    pub fn new(
        en: EN,
        rs: RS,
        d4: D4,
        d5: D5,
        d6: D6,
        d7: D7,
        timer: Timer,
    ) -> Result<LCD1602<EN, RS, D4, D5, D6, D7, Timer>, Error<E>>
    where
        <Timer as CountDown>::Time: From<Microseconds>,
    {
        let mut lcd = LCD1602 {
            en,
            rs,
            d4,
            d5,
            d6,
            d7,
            timer,
        };
        lcd.init()?;
        Ok(lcd)
    }

    fn init(&mut self) -> Result<(), Error<E>>
    where
        <Timer as CountDown>::Time: From<Microseconds>,
    {
        self.delay(50000)?;
        self.set_bus_width(FourBits)?;

        self.command(0x0C)?; // Display mode
        self.clear()?;
        self.set_entry_mode(RightToLeft, false)?;
        Ok(())
    }

    pub fn set_bus_width(&mut self, bus_width: BusWidth) -> Result<(), Error<E>>
    where
        <Timer as CountDown>::Time: From<Microseconds>,
    {
        match bus_width {
            FourBits => {
                self.write_bus(0x02)?;
                self.delay(39)
            }
            _ => Err(UnsupportedBusWidth),
        }
    }
    pub fn set_entry_mode(
        &mut self,
        text_direction: Direction,
        screen_edge_tracking: bool,
    ) -> Result<(), Error<E>>
    where
        <Timer as CountDown>::Time: From<Microseconds>,
    {
        let mut cmd = 0x04;
        if text_direction == Direction::RightToLeft {
            cmd |= 0x02;
        }
        if screen_edge_tracking {
            cmd |= 0x01;
        }
        self.command(cmd)?;
        self.delay(39)
    }

    pub fn set_position(&mut self, x: u8, y: u8) -> Result<(), Error<E>>
    where
        <Timer as CountDown>::Time: From<Microseconds>,
    {
        match (x, y) {
            (0..=15, 0) => {
                self.command(0x80 | x)?;
                self.delay(1530)
            }
            (0..=15, 1) => {
                self.command(0x80 | (x + 0x40))?;
                self.delay(1530)
            }
            _ => Err(InvalidCursorPos),
        }
    }

    pub fn clear(&mut self) -> Result<(), Error<E>>
    where
        <Timer as CountDown>::Time: From<Microseconds>,
    {
        self.command(0x01)?;
        self.delay(1530)
    }

    pub fn home(&mut self) -> Result<(), Error<E>>
    where
        <Timer as CountDown>::Time: From<Microseconds>,
    {
        self.command(0x02)?;
        self.delay(1530)
    }

    fn command(&mut self, cmd: u8) -> Result<(), Error<E>> {
        self.rs.set_low()?;
        self.write_bus((cmd & 0xF0) >> 4)?;
        self.write_bus(cmd & 0x0F)?; // 4bit writes send end pulses
        Ok(())
    }

    fn write_char(&mut self, ch: u8) -> Result<(), Error<E>> {
        self.rs.set_high()?;
        self.write_bus((ch & 0xF0) >> 4)?;
        self.write_bus(ch & 0x0F)?; // 4bit writes send end pulses
        Ok(())
    }

    pub fn print(&mut self, s: &str) -> Result<(), Error<E>>
    where
        <Timer as CountDown>::Time: From<Microseconds>,
    {
        for ch in s.chars() {
            self.delay(320)?; // per char delay
            self.write_char(ch as u8)?;
        }
        self.delay(1530)
    }

    fn write_bus(&mut self, data: u8) -> Result<(), Error<E>> {
        self.en.set_low()?;
        match (data & 0x1) > 0 {
            true => self.d4.set_high()?,
            false => self.d4.set_low()?,
        };
        match (data & 0x2) > 0 {
            true => self.d5.set_high()?,
            false => self.d5.set_low()?,
        };
        match (data & 0x4) > 0 {
            true => self.d6.set_high()?,
            false => self.d6.set_low()?,
        };
        match (data & 0x8) > 0 {
            true => self.d7.set_high()?,
            false => self.d7.set_low()?,
        };
        self.en.set_high()?;
        self.en.set_low()?;
        Ok(())
    }

    pub fn delay(&mut self, interval_us: u32) -> Result<(), Error<E>>
    where
        <Timer as CountDown>::Time: From<Microseconds>,
    {
        self.timer.start(interval_us.microseconds());
        match block!(self.timer.wait()) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::TimerError),
        }
    }
}

#[derive(PartialEq)]
pub enum Direction {
    LeftToRight,
    RightToLeft,
}

#[derive(PartialEq)]
pub enum BusWidth {
    FourBits,
    EightBits,
}
