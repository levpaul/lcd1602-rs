//! The starter code slowly blinks the LED, and sets up
//! USB logging.

#![no_std]
#![no_main]
#![allow(unused_must_use)]

use teensy4_bsp as bsp;
use teensy4_bsp::hal::gpio;
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

    let mut lcd = liquid_crystal::LCD {
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
