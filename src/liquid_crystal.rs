use embedded_hal::digital::v2::OutputPin;
use imxrt_hal::gpt::GPT;
use teensy4_bsp::SysTick;

pub struct LCD<'a, A, B, C, D, E, F> {
    pub en: &'a mut A,
    pub rs: &'a mut B,
    pub d4: &'a mut C,
    pub d5: &'a mut D,
    pub d6: &'a mut E,
    pub d7: &'a mut F,
    pub gpt: &'a mut GPT,
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
        self.delay(50000);
        self.command(0x00); //4 bit shuffle
        self.delay(150);
        self.write4(0x03);
        self.delay(150);
        self.write4(0x03);
        self.delay(150);
        self.write4(0x02);

        self.command(0x0C); // Display mode
        self.command(0x01); // Clear
        self.delay(2900); // Delay per homing
        self.command(0x06); // Entrymode
    }

    pub fn command(&mut self, cmd: u8) {
        self.delay(320); // per char delay
        self.rs.set_low();
        // self.write4(cmd & 0x0F); // 4bit writes send end pulses
        // self.write4(cmd & 0xF0);
        self.write4((cmd & 0xF0) >> 4);
        self.write4(cmd & 0x0F); // 4bit writes send end pulses
    }

    pub fn write_char(&mut self, ch: u8) {
        self.delay(320); // per char delay
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
        self.delay(1);
        self.en.set_low();
    }

    pub fn delay(&mut self, interval_us: u64) {
        self.gpt.set_output_compare_duration(
            imxrt_hal::gpt::OutputCompareRegister::Three,
            core::time::Duration::from_micros(interval_us),
        );
        loop {
            let mut res = self
                .gpt
                .output_compare_status(imxrt_hal::gpt::OutputCompareRegister::Three);
            if res.is_set() {
                res.clear();
                return;
            }
        }
    }
    // pub fn delay(&mut self, interval_ms: u32) {
    //     self.st.delay(interval_ms);
    // }
}
