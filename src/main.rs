#![no_std]
#![no_main]
#![allow(unused_must_use)]

use teensy4_bsp as bsp;
use teensy4_bsp::hal::gpio;
use teensy4_panic as _;

mod liquid_crystal;
mod logging;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut p = bsp::Peripherals::take().unwrap();
    let pins = bsp::t40::into_pins(p.iomuxc);
    let mut led = bsp::configure_led(pins.p13);

    // See the `logging` module docs for more info.
    assert!(logging::init().is_ok());

    // Init pins
    let mut rs = gpio::GPIO::new(pins.p12).output();
    let mut en = gpio::GPIO::new(pins.p11).output();
    let mut d4 = gpio::GPIO::new(pins.p5).output();
    let mut d5 = gpio::GPIO::new(pins.p4).output();
    let mut d6 = gpio::GPIO::new(pins.p3).output();
    let mut d7 = gpio::GPIO::new(pins.p2).output();

    // General Purpose Timer setup
    let (_, ipg_hz) =
        p.ccm
            .pll1
            .set_arm_clock(imxrt_hal::ccm::PLL1::ARM_HZ, &mut p.ccm.handle, &mut p.dcdc);
    let mut cfg = p.ccm.perclk.configure(
        &mut p.ccm.handle,
        imxrt_hal::ccm::perclk::PODF::DIVIDE_3,
        imxrt_hal::ccm::perclk::CLKSEL::IPG(ipg_hz),
    );
    let mut gpt1 = p.gpt1.clock(&mut cfg);
    gpt1.set_output_interrupt_on_compare(imxrt_hal::gpt::OutputCompareRegister::Three, false);
    gpt1.set_mode(imxrt_hal::gpt::Mode::FreeRunning);
    gpt1.set_reset_on_enable(true);
    gpt1.set_enable(true);

    // LCD Init
    log::info!("Initializing LCD");
    let mut lcd = liquid_crystal::LCD {
        en: &mut en,
        rs: &mut rs,
        d4: &mut d4,
        d5: &mut d5,
        d6: &mut d6,
        d7: &mut d7,
        timer: &mut gpt1.count_down(imxrt_hal::gpt::OutputCompareRegister::Three),
    };
    lcd.init();

    for ch in "Hello world!".chars() {
        lcd.write_char(ch as u8);
    }

    loop {
        led.toggle();
        lcd.delay(1_000_000);
    }
}
