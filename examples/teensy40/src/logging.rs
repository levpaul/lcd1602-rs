//! USB logging support
//!
//! If you don't want USB logging, remove
//!
//! - this module
//! - the `log` dependency in Cargo.toml

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

use bsp::hal::ral::usb::USB1;
use bsp::interrupt;
use teensy4_bsp as bsp;

/// Specify any logging filters here
///
/// See the BSP docs for more information
/// on logging filters.
const FILTERS: &[bsp::usb::Filter] = &[
    // Try enabling this filter to only see
    // log messages from main.rs.
    //
    // ("teensy_1602", None),
];

/// Initialize the USB logging system, and prepares the
/// USB ISR with the poller
///
/// When `init` returns, the USB interrupt will be enabled,
/// and the host may begin to interface the device.
/// You should only call this once.
///
/// # Panics
///
/// Panics if the imxrt-ral USB1 instance is already taken.
pub fn init() -> Result<bsp::usb::Reader, bsp::usb::Error> {
    let inst = USB1::take().unwrap();
    bsp::usb::init(
        inst,
        bsp::usb::LoggingConfig {
            filters: FILTERS,
            ..Default::default()
        },
    )
    .map(|(poller, reader)| {
        setup(poller);
        reader
    })
}

/// Setup the USB ISR with the USB poller
fn setup(poller: bsp::usb::Poller) {
    static POLLER: Mutex<RefCell<Option<bsp::usb::Poller>>> = Mutex::new(RefCell::new(None));

    #[cortex_m_rt::interrupt]
    fn USB_OTG1() {
        cortex_m::interrupt::free(|cs| {
            POLLER
                .borrow(cs)
                .borrow_mut()
                .as_mut()
                .map(|poller| poller.poll());
        });
    }

    cortex_m::interrupt::free(|cs| {
        *POLLER.borrow(cs).borrow_mut() = Some(poller);
        // Safety: invoked in a critical section that also prepares the ISR
        // shared memory. ISR memory is ready by the time the ISR runs.
        unsafe { cortex_m::peripheral::NVIC::unmask(bsp::interrupt::USB_OTG1) };
    });
}
