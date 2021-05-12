//! # lcd1602_rs
//!
//! `lcd1602_rs` is a simple embedded-hal driver for a 1602 LCD screen
#![no_std]
mod error;
mod lcd1602;

pub struct LCD1602<EN, RS, D4, D5, D6, D7, Timer> {
    en: EN,
    rs: RS,
    d4: D4,
    d5: D5,
    d6: D6,
    d7: D7,
    timer: Timer,
}
