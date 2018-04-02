#![no_std]

extern crate cortex_m;

extern crate stm32f103xx_hal as blue_pill;

use core::str::from_utf8_unchecked;
use cortex_m::asm;
#[macro_use(block)]
extern crate nb;
extern crate embedded_hal as hal;

use hal::serial;

pub struct ESP8266 {}

impl ESP8266 {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle_byte(&mut self, byte: u8) {}
}
