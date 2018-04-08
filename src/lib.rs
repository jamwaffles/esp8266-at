#![no_std]

extern crate cortex_m;
extern crate heapless;

// extern crate stm32f103xx_hal as blue_pill;

// use core::str::from_utf8_unchecked;
use cortex_m::asm;
// #[macro_use(block)]
// extern crate nb;
extern crate embedded_hal as hal;

use hal::serial;

use heapless::String;

pub enum State {
    Uninitialized,
    Resetting,
    Reset,
}

pub enum Command {
    Reset,
}

pub struct ESP8266 {
    /// Receive buffer. ESP8266 has a 256 byte internal buffer so let's mirror that
    rxbuf: String<[u8; 256]>,
    state: State,
}

impl ESP8266 {
    pub fn new() -> Self {
        Self {
            rxbuf: String::from(""),
            state: State::Uninitialized,
        }
    }

    pub fn statemachine(&mut self) {
        match (&self.state, &self.rxbuf.as_str()) {
            (State::Uninitialized, &"OK\r\n") => {
                self.rxbuf.clear();
                self.state = State::Resetting;
            }
            (State::Resetting, &"OK\r\n") => {
                self.rxbuf.clear();
                self.state = State::Reset;
            }
            _ => (),
        }
    }

    pub fn next_command(&self) -> Option<&[u8]> {
        match self.state {
            State::Uninitialized => Some(b"AT\r\n"),
            State::Resetting => Some(b"AT+RST\r\n"),
            _ => None,
        }
    }

    pub fn handle_byte(&mut self, byte: u8) {
        self.rxbuf.push(byte.into()).unwrap();

        self.statemachine();
    }

    pub fn handle_response_part(&mut self, buf: &[u8]) {
        for c in buf {
            self.rxbuf.push(*c as char);
        }
    }

    // pub fn get_buf(&self) -> &str {
    //     self.rxbuf.as_str()
    // }
}
