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

use heapless::{RingBuffer, String};

pub enum State {
    Uninitialized,
    Resetting,
    Reset,
    SettingMode,
    Connecting,
    Connected,
}

pub enum Command {
    Reset,
}

pub struct ESP8266 {
    /// Receive buffer. ESP8266 has a 256 byte internal buffer so let's mirror that
    rxbuf: String<[u8; 1024]>,
    state: State,
    ringbuf: RingBuffer<u8, [u8; 1024]>,
}

// static mut RB: RingBuffer<i32, [i32; 4]> = RingBuffer::new();

// let rb = unsafe { &mut RB };

// rb.enqueue(0).unwrap();

// let (mut p, mut c) = rb.split();

impl ESP8266 {
    pub fn new() -> Self {
        Self {
            rxbuf: String::from(""),
            state: State::Uninitialized,
            ringbuf: RingBuffer::new(),
        }
    }

    pub fn statemachine(&mut self) {
        let mut buf: String<[u8; 2014]> = String::new();

        let (_, mut c) = self.ringbuf.split();

        while let Some(byte) = c.dequeue() {
            self.rxbuf.push(byte.into());
        }

        if self.rxbuf.as_str().ends_with("ready\r\n") {
            match &self.state {
                State::Uninitialized => {
                    self.rxbuf.clear();
                    self.state = State::Resetting;
                }
                State::Resetting => {
                    self.rxbuf.clear();
                    self.state = State::SettingMode;
                }
                State::SettingMode => {
                    self.rxbuf.clear();
                    self.state = State::Connecting;
                }
                State::Connecting => {
                    self.rxbuf.clear();
                    self.state = State::Connected;
                }
                _ => (),
            }
        } else if self.rxbuf.as_str().ends_with("OK\r\n") {
            match &self.state {
                State::Uninitialized => {
                    self.rxbuf.clear();
                    self.state = State::Resetting;
                }
                State::Resetting => {
                    self.rxbuf.clear();
                    self.state = State::SettingMode;
                }
                State::SettingMode => {
                    self.rxbuf.clear();
                    self.state = State::Connecting;
                }
                State::Connecting => {
                    self.rxbuf.clear();
                    self.state = State::Connected;
                }
                _ => (),
            }
        }

        // match (&self.state, self.rxbuf.as_str().ends_with("OK")) {
        //     (State::Uninitialized, true) => {
        //         self.rxbuf.clear();
        //         self.state = State::Resetting;
        //     }
        //     (State::Resetting, true) => {
        //         self.rxbuf.clear();
        //         self.state = State::SettingMode;
        //     }
        //     (State::SettingMode, true) => {
        //         self.rxbuf.clear();
        //         self.state = State::Connecting;
        //     }
        //     (State::Connecting, true) => {
        //         self.rxbuf.clear();
        //         self.state = State::Connected;
        //     }
        //     _ => (),
        // }
    }

    pub fn next_command(&self) -> Option<&[u8]> {
        match self.state {
            State::Uninitialized => Some(b"AT\r\n"),
            State::Resetting => Some(b"AT+RST\r\n"),
            // State::SettingMode => Some(b"AT+CWMODE=1\r\n"),
            // State::Connecting => Some(b"AT+CWJAP=ClownsUnderTheBed,3dooty5g\r\n"),
            _ => None,
        }
    }

    pub fn handle_byte(&mut self, byte: u8) {
        // self.rxbuf.push(byte.into()).unwrap();

        // self.statemachine();

        let (mut p, _) = self.ringbuf.split();

        p.enqueue(byte.into()).expect("Could not enqueue");
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
