use embedded_hal::digital::v2::OutputPin;
use teensy4_bsp::SysTick;

pub struct LCD<'a, A, B, C, D, E, F> {
    pub en: &'a mut A,
    pub rs: &'a mut B,
    pub d4: &'a mut C,
    pub d5: &'a mut D,
    pub d6: &'a mut E,
    pub d7: &'a mut F,
    pub st: &'a mut SysTick,
}

impl<'a, A, B, C, D, E, F> LCD<'a, A, B, C, D, E, F>
where
    A: OutputPin,
    B: OutputPin,
    C: OutputPin,
    D: OutputPin,
    E: OutputPin,
    F: OutputPin,
{
    pub fn init(&mut self) {
        self.st.delay(50);
        self.command(0x00);
        self.st.delay(5);
        self.write4(0x03);
        self.write4(0x02);

        self.command(0x0C); // Display mode
        self.command(0x01); // Clear
        self.command(0x06); // Entrymode
    }

    pub fn command(&mut self, cmd: u8) {
        self.st.delay(3); // per char delay
        self.rs.set_low();
        // self.write4(cmd & 0x0F); // 4bit writes send end pulses
        // self.write4(cmd & 0xF0);
        self.write4((cmd & 0xF0) >> 4);
        self.write4(cmd & 0x0F); // 4bit writes send end pulses
    }

    pub fn write_char(&mut self, ch: u8) {
        log::info!("Writing char");
        self.st.delay(3); // per char delay
        self.rs.set_high();
        self.write4(ch & 0x0F); // 4bit writes send end pulses
        self.write4((ch & 0xF0) >> 4);
    }

    fn write4(&mut self, data: u8) {
        self.en.set_low();
        if (data & 0x1) > 0 {
            self.d4.set_high();
        } else {
            self.d4.set_low();
        }
        if (data & 0x2) > 0 {
            self.d5.set_high();
        } else {
            self.d5.set_low();
        }
        if (data & 0x4) > 0 {
            self.d6.set_high();
        } else {
            self.d6.set_low();
        }
        if (data & 0x8) > 0 {
            self.d7.set_high();
        } else {
            self.d7.set_low();
        }
        self.en.set_high();
        self.en.set_low();
    }

    pub fn delay(&mut self, interval_ms: u32) {
        self.st.delay(interval_ms);
    }
}
