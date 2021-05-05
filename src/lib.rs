#![no_std]
use core::time::Duration;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::timer::CountDown;
use nb::block;

#[derive(Debug)]
pub enum Error<GPIO> {
    TimerError,
    GPIOError(GPIO),
}

impl<E> From<E> for Error<E> {
    fn from(gpio_err: E) -> Self {
        Self::GPIOError(gpio_err)
    }
}

pub struct LCD1602<EN, RS, D4, D5, D6, D7, Timer> {
    en: EN,
    rs: RS,
    d4: D4,
    d5: D5,
    d6: D6,
    d7: D7,
    timer: Timer,
}

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
        self.write4(0x03)?;
        self.delay(150)?;
        self.write4(0x03)?;
        self.delay(150)?;
        self.write4(0x02)?;

        self.command(0x0C)?; // Display mode
        self.command(0x01)?; // Clear
        self.delay(2900)?; // Delay per homing
        self.command(0x06)?; // Entrymode
        Ok(())
    }

    pub fn command(&mut self, cmd: u8) -> Result<(), Error<E>> {
        self.delay(320)?; // per char delay
        self.rs.set_low()?;
        self.write4((cmd & 0xF0) >> 4)?;
        self.write4(cmd & 0x0F)?; // 4bit writes send end pulses
        Ok(())
    }

    pub fn write_char(&mut self, ch: u8) -> Result<(), Error<E>> {
        self.delay(320)?; // per char delay
        self.rs.set_high()?;
        self.write4((ch & 0xF0) >> 4)?;
        self.write4(ch & 0x0F)?; // 4bit writes send end pulses
        Ok(())
    }

    fn write4(&mut self, data: u8) -> Result<(), Error<E>> {
        self.en.set_low()?;
        if (data & 0x1) > 0 {
            self.d4.set_high()?;
        } else {
            self.d4.set_low()?;
        }
        if (data & 0x2) > 0 {
            self.d5.set_high()?;
        } else {
            self.d5.set_low()?;
        }
        if (data & 0x4) > 0 {
            self.d6.set_high()?;
        } else {
            self.d6.set_low()?;
        }
        if (data & 0x8) > 0 {
            self.d7.set_high()?;
        } else {
            self.d7.set_low()?;
        }
        self.en.set_high()?;
        self.delay(1)?;
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
