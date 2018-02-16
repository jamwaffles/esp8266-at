#![no_std]

extern crate embedded_hal as hal;

use hal::serial;

pub struct ESP8266<Serial> {
	serial: Serial
}

impl<Serial> ESP8266<Serial> where
    Serial: hal::serial::Read<u8> + hal::serial::Write<u8>
    {
    	fn new(serial: Serial) -> Self {
    		Self {
    			serial: serial,
    		}
    	}
}