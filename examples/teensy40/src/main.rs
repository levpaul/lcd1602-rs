#![no_std]
#![no_main]
use lcd1602_rs::LCD1602;
use teensy4_bsp as bsp;
use teensy4_bsp::hal::gpio;
use teensy4_panic as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut p = bsp::Peripherals::take().unwrap();
    let pins = bsp::t40::into_pins(p.iomuxc);
    let mut led = bsp::configure_led(pins.p13);

    // Init pins
    let rs = gpio::GPIO::new(pins.p12).output();
    let en = gpio::GPIO::new(pins.p11).output();
    let d4 = gpio::GPIO::new(pins.p5).output();
    let d5 = gpio::GPIO::new(pins.p4).output();
    let d6 = gpio::GPIO::new(pins.p3).output();
    let d7 = gpio::GPIO::new(pins.p2).output();

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
    let t = gpt1.count_down(imxrt_hal::gpt::OutputCompareRegister::Three);

    // LCD Init
    let mut lcd = LCD1602::new(en, rs, d4, d5, d6, d7, t).unwrap();

    loop {
        lcd.print("hello world!").ok();
        led.toggle();
        lcd.delay(1_000_000 as u64).ok();
        led.toggle();
        lcd.clear().ok();
        lcd.delay(1_000_000 as u64).ok();
    }
}
