use crate::error::Error;
use crate::LCD1602;

use crate::lcd1602::Direction::RightToLeft;
use core::time::Duration;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::timer::CountDown;
use nb::block;

const MAX_ROWS: u8 = 2;
const MAX_COLS: u8 = 16;

impl<EN, RS, D4, D5, D6, D7, Timer, E> LCD1602<EN, RS, D4, D5, D6, D7, Timer>
where
    EN: OutputPin<Error = E>,
    RS: OutputPin<Error = E>,
    D4: OutputPin<Error = E>,
    D5: OutputPin<Error = E>,
    D6: OutputPin<Error = E>,
    D7: OutputPin<Error = E>,
    Timer: CountDown<Time = Duration>,
{
    pub fn new(
        en: EN,
        rs: RS,
        d4: D4,
        d5: D5,
        d6: D6,
        d7: D7,
        timer: Timer,
    ) -> Result<LCD1602<EN, RS, D4, D5, D6, D7, Timer>, Error<E>> {
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

    fn init(&mut self) -> Result<(), Error<E>> {
        self.delay(50000)?;
        self.command(0x00)?; //4 bit shuffle
        self.delay(150)?;
        self.send(0x03)?;
        self.delay(150)?;
        self.send(0x02)?;

        self.command(0x0C)?; // Display mode
        self.clear()?;
        self.set_entry_mode(RightToLeft, false)?;
        Ok(())
    }

    pub fn set_entry_mode(
        &mut self,
        text_direction: Direction,
        screen_edge_tracking: bool,
    ) -> Result<(), Error<E>> {
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

    pub fn clear(&mut self) -> Result<(), Error<E>> {
        self.command(0x01)?;
        self.delay(1530)
    }

    pub fn home(&mut self) -> Result<(), Error<E>> {
        self.command(0x02)?;
        self.delay(1530)
    }

    pub fn set_cgram_addr(&mut self, addr: u8) -> Result<(), Error<E>> {
        match addr >> 5 > 0 {
            true => Err(Error::InvalidAddr),
            false => self.command(0x04 + addr),
        }
    }

    fn set_ddram_addr(&mut self, addr: u8) -> Result<(), Error<E>> {
        match addr & 0x80 > 0 {
            true => Err(Error::InvalidAddr),
            false => self.command(0x80 + addr),
        }
    }

    pub fn set_cursor(&mut self, col: u8, row: u8) -> Result<(), Error<E>> {
        if col > MAX_COLS || row > MAX_ROWS {
            return Err(Error::InvalidCursorPos);
        }
        self.set_ddram_addr(col * 0x40 + row)?;
        self.delay(39)
    }

    fn command(&mut self, cmd: u8) -> Result<(), Error<E>> {
        self.delay(320)?; // per char delay
        self.rs.set_low()?;
        self.send((cmd & 0xF0) >> 4)?;
        self.send(cmd & 0x0F)?; // 4bit writes send end pulses
        Ok(())
    }

    fn write(&mut self, ch: u8) -> Result<(), Error<E>> {
        self.rs.set_high()?;
        self.send((ch & 0xF0) >> 4)?;
        self.send(ch & 0x0F)?; // 4bit writes send end pulses
        Ok(())
    }

    pub fn print(&mut self, s: &str) -> Result<(), Error<E>> {
        for ch in s.chars() {
            self.delay(320)?; // per char delay
            self.write(ch as u8)?;
        }
        Ok(())
    }

    fn send(&mut self, data: u8) -> Result<(), Error<E>> {
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

    pub fn delay(&mut self, interval_us: u64) -> Result<(), Error<E>> {
        self.timer.start(Duration::from_micros(interval_us));
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