//! The starter code slowly blinks the LED, and sets up
//! USB logging.

#![no_std]
#![no_main]
#![allow(unused_must_use)]

use embedded_hal::digital::v2::OutputPin;
use teensy4_bsp as bsp;
use teensy4_bsp::hal::gpio;
use teensy4_bsp::SysTick;
use teensy4_panic as _;

mod liquid_crystal;
mod logging;

const LED_PERIOD_MS: u32 = 1_000;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = bsp::Peripherals::take().unwrap();
    let mut systick = bsp::SysTick::new(cortex_m::Peripherals::take().unwrap().SYST);
    let pins = bsp::t40::into_pins(p.iomuxc);
    let mut led = bsp::configure_led(pins.p13);

    // See the `logging` module docs for more info.
    assert!(logging::init().is_ok());
    // Init
    let mut rs = gpio::GPIO::new(pins.p12);
    rs.set_fast(true);
    let mut rs = rs.output();
    let mut en = gpio::GPIO::new(pins.p11);
    en.set_fast(true);
    let mut en = en.output();
    let mut d4 = gpio::GPIO::new(pins.p5);
    d4.set_fast(true);
    let mut d4 = d4.output();
    let mut d5 = gpio::GPIO::new(pins.p4);
    d5.set_fast(true);
    let mut d5 = d5.output();
    let mut d6 = gpio::GPIO::new(pins.p3);
    d6.set_fast(true);
    let mut d6 = d6.output();
    let mut d7 = gpio::GPIO::new(pins.p2);
    d7.set_fast(true);
    let mut d7 = d7.output();

    let mut lcd = LCD {
        en: &mut en,
        rs: &mut rs,
        d4: &mut d4,
        d5: &mut d5,
        d6: &mut d6,
        d7: &mut d7,
        st: &mut systick,
    };

    lcd.init();
    let mut i = 72;
    loop {
        i += 1;
        i %= 255;
        led.toggle();
        lcd.delay(LED_PERIOD_MS);
        lcd.write_char(i);
        log::info!("Hello world");
    }
}
struct LCD<'a, A, B, C, D, E, F> {
    en: &'a mut A,
    rs: &'a mut B,
    d4: &'a mut C,
    d5: &'a mut D,
    d6: &'a mut E,
    d7: &'a mut F,
    st: &'a mut SysTick,
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
    fn init(&mut self) {
        self.st.delay(50);
        self.command(0x00);
        self.st.delay(5);
        self.write4(0x03);
        self.write4(0x02);

        self.command(0x0C); // Display mode
        self.command(0x01); // Clear
        self.command(0x06); // Entrymode
    }

    fn command(&mut self, cmd: u8) {
        self.st.delay(1); // per char delay
        self.rs.set_low();
        self.en.set_low();
        self.write4(cmd & 0x0F); // 4bit writes send end pulses
        self.write4(cmd & 0xF0);
    }

    fn write4(&mut self, data: u8) {
        if data & 0x1 == 0x1 {
            self.d4.set_high();
        } else {
            self.d4.set_low();
        };
        if data & 0x2 == 0x2 {
            self.d5.set_high();
        } else {
            self.d5.set_low();
        };
        if data & 0x4 == 0x4 {
            self.d6.set_high();
        } else {
            self.d6.set_low();
        };
        if data & 0x8 == 0x8 {
            self.d7.set_high();
        } else {
            self.d7.set_low();
        };
        self.en.set_high();
        self.en.set_low();
    }

    fn write_char(&mut self, ch: u8) {
        self.st.delay(1); // per char delay
        self.rs.set_low();
        self.en.set_low();
        self.write4(ch & 0x0F); // 4bit writes send end pulses
        self.write4(ch & 0xF0);
    }

    fn delay(&mut self, interval_ms: u32) {
        self.st.delay(interval_ms);
    }
}
