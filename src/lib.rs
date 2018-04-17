#![no_std]

// extern crate cortex_m;
extern crate heapless;

// extern crate stm32f103xx_hal as blue_pill;

// use core::str::from_utf8_unchecked;
// use cortex_m::asm;
// #[macro_use(block)]
// extern crate nb;
// extern crate embedded_hal as hal;

use heapless::String;

#[derive(Debug, Copy, Clone)]
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
    rxbuf: String<[u8; 4096]>,

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
        let (next_command, next_state) = if let Some(line) = self.rxbuf.lines().last() {
            match line {
                "OK" => match self.state {
                    State::Uninitialized => (Some("AT+RST\r\n"), Some(State::AtTest)),
                    State::AtTest => (Some("AT+RST\r\n"), Some(State::Reset)),
                    State::SetMode => (Some("AT+CWDHCP=1,1\r\n"), Some(State::EnableDhcp)),
                    // Connect to AP without saving to flash
                    State::EnableDhcp => (
                        Some("AT+CWJAP_CUR=\"ClownsUnderTheBed\",\"3dooty5g\"\r\n"),
                        Some(State::Connecting),
                    ),
                    State::Connecting => (None, Some(State::Connected)),
                    _ => (None, None),
                },
                "ready" => (Some("AT+CWMODE=1\r\n"), Some(State::SetMode)),
                "WIFI DISCONNECT" => (None, Some(State::Disconnected)),
                "WIFI GOT IP" => (None, Some(State::WifiGotIp)),
                _ => match self.state {
                    State::Uninitialized => (Some("AT\r\n"), Some(State::AtTest)),
                    _ => (None, None),
                },
            }
        } else {
            (None, None)
        };

        if next_state.is_some() {
            self.state = next_state.unwrap();
        }

        if next_command.is_some() {
            self.rxbuf.clear();
            self.command = next_command.map(|c| c.as_bytes());
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
