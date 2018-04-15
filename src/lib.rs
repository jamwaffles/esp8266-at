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
    AtTest,
    Connected,
    Connecting,
    Disconnected,
    EnableDhcp,
    Reset,
    Resetting,
    SetMode,
    Uninitialized,
    WifiGotIp,
    WifiReady,
}

pub enum Command {
    Reset,
}

pub struct ESP8266<'a> {
    /// Receive buffer. ESP8266 has a 256 byte internal buffer so let's mirror that
    rxbuf: String<[u8; 1024]>,
    state: State,
    ringbuf: RingBuffer<u8, [u8; 1024]>,
    command: Option<&'a [u8]>,
}

// static mut RB: RingBuffer<i32, [i32; 4]> = RingBuffer::new();

// let rb = unsafe { &mut RB };

// rb.enqueue(0).unwrap();

// let (mut p, mut c) = rb.split();

impl<'a> ESP8266<'a> {
    pub fn new() -> Self {
        Self {
            rxbuf: String::from(""),
            state: State::Uninitialized,
            ringbuf: RingBuffer::new(),
            command: Some(b"AT\r\n"),
        }
    }

    pub fn statemachine(&mut self) {
        let mut buf: String<[u8; 1024]> = String::new();

        let (_, mut c) = self.ringbuf.split();

        while let Some(byte) = c.dequeue() {
            self.rxbuf.push(byte.into());
        }

        for line in self.rxbuf.lines() {
            match line {
                "OK" => match self.state {
                    State::Uninitialized => {
                        self.command = Some(b"AT\r\n");
                        self.state = State::AtTest;
                    }
                    State::AtTest => {
                        self.command = Some(b"AT+RST\r\n");
                        self.state = State::Reset;
                    }
                    State::SetMode => {
                        self.state = State::EnableDhcp;
                        self.command = Some(b"AT+CWDHCP=1,1\r\n");
                    }
                    State::EnableDhcp => {
                        self.state = State::Connecting;
                        // Connect to AP without saving to flash
                        self.command = Some(b"AT+CWJAP_CUR=\"ClownsUnderTheBed\",\"3dooty5g\"\r\n");
                    }
                    State::Connecting => {
                        self.state = State::Connected;
                        self.command = None;
                    }
                    _ => (),
                },
                "ready" => {
                    self.state = State::SetMode;
                    self.command = Some(b"AT+CWMODE=1\r\n");
                }
                "WIFI DISCONNECT" => self.state = State::Disconnected,
                // "WIFI CONNECTED" => self.state = State::WifiConnected,
                "WIFI GOT IP" => self.state = State::WifiGotIp,

                _ => match self.state {
                    State::Uninitialized => {
                        self.command = Some(b"AT\r\n");
                        self.state = State::AtTest;
                    }
                    _ => (),
                },
            }
        }
    }

    pub fn next_command(&self) -> Option<&[u8]> {
        // match self.state {
        //     State::Uninitialized => Some(b"AT\r\n"),
        //     State::Resetting => Some(b"AT+RST\r\n"),
        //     State::SettingMode => Some(b"AT+CWMODE=1\r\n"),
        //     State::EnablingDhcp => Some(b"AT+CWDHCP=1,1\r\n"),
        //     State::Connecting => Some(b"AT+CWJAP=\"ClownsUnderTheBed\",\"3dooty5g\"\r\n"),
        //     _ => None,
        // }
        self.command
    }

    pub fn handle_byte(&mut self, byte: u8) {
        // self.rxbuf.push(byte.into()).unwrap();

        // self.statemachine();

        let (mut p, _) = self.ringbuf.split();

        p.enqueue(byte.into()).expect("Could not enqueue");
    }

    pub fn handle_bytes(&mut self, buf: &[u8]) {
        let len = self.ringbuf.len();

        let (mut p, _) = self.ringbuf.split();

        for byte in buf {
            p.enqueue(*byte).expect("Could not enqueue");
        }
    }

    // pub fn get_buf(&self) -> &str {
    //     self.rxbuf.as_str()
    // }
}
