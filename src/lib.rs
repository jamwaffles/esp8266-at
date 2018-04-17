#![no_std]

// extern crate cortex_m;
extern crate heapless;

// extern crate stm32f103xx_hal as blue_pill;

// use core::str::from_utf8_unchecked;
// use cortex_m::asm;
// #[macro_use(block)]
// extern crate nb;
extern crate embedded_hal as hal;

use heapless::String;

#[derive(Debug)]
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
    /// Receive buffer
    rxbuf: String<[u8; 1024]>,

    /// Current state
    state: State,

    /// Next command to send to the ESP
    command: Option<&'a [u8]>,
}

impl<'a> ESP8266<'a> {
    pub fn new() -> Self {
        Self {
            rxbuf: String::from(""),
            state: State::Uninitialized,
            command: Some(b"AT\r\n"),
        }
    }

    pub fn statemachine(&mut self) {
        let line = self.rxbuf.lines().last().unwrap();

        match line {
            "OK" => match self.state {
                State::Uninitialized => {
                    self.command = Some(b"AT+RST\r\n");
                    self.state = State::AtTest;
                }
                State::AtTest => {
                    self.command = Some(b"AT+RST\r\n");
                    self.state = State::Reset;
                }
                State::SetMode => {
                    self.command = Some(b"AT+CWDHCP=1,1\r\n");
                    self.state = State::EnableDhcp;
                }
                State::EnableDhcp => {
                    // Connect to AP without saving to flash
                    self.command = Some(b"AT+CWJAP_CUR=\"ClownsUnderTheBed\",\"3dooty5g\"\r\n");
                    self.state = State::Connecting;
                }
                State::Connecting => {
                    self.command = None;
                    self.state = State::Connected;
                }
                _ => (),
            },
            "ready" => {
                self.command = Some(b"AT+CWMODE=1\r\n");
                self.state = State::SetMode;
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

    pub fn handle_bytes(&mut self, buf: &[u8]) {
        for byte in buf {
            self.rxbuf.push(*byte as char).expect("Could not enqueue");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn recv_str(esp: &mut ESP8266, recv: &str) {
        for chunk in recv.as_bytes().chunks(8) {
            esp.handle_bytes(chunk);
        }
    }

    fn check_next_cmd(esp: &ESP8266, test: &str) {
        assert_eq!(esp.next_command(), Some(test.as_bytes()));
    }

    #[test]
    fn it_connects() {
        let mut esp = ESP8266::new();

        check_next_cmd(&esp, "AT\r\n");

        recv_str(&mut esp, "AT\r\n\r\nOK\r\n");
        esp.statemachine();
        check_next_cmd(&esp, "AT+RST\r\n");

        recv_str(
            &mut esp,
            "AT+RST\r\n\r\nblablablablablablablablablbalablab\r\nready\r\n",
        );
        esp.statemachine();
        check_next_cmd(&esp, "AT+CWMODE=1\r\n");

        recv_str(&mut esp, "AT+CWMODE=1\r\n\r\nOK\r\n");
        esp.statemachine();
        check_next_cmd(&esp, "AT+CWDHCP=1,1\r\n");

        recv_str(&mut esp, "AT+CWDHCP=1,1\r\n\r\nOK\r\n");
        esp.statemachine();
        check_next_cmd(&esp, "AT+CWJAP_CUR=\"ClownsUnderTheBed\",\"3dooty5g\"\r\n");

        recv_str(
            &mut esp,
            "AT+CWJAP_CUR=\"ClownsUnderTheBed\",\"3dooty5g\"\r\n\r\nOK\r\n",
        );
        esp.statemachine();
        assert_eq!(esp.next_command(), None);
    }
}
