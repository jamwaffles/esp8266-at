#![no_std]

extern crate cortex_m;

extern crate stm32f103xx_hal as blue_pill;

use core::str::from_utf8_unchecked;
use cortex_m::asm;
#[macro_use(block)]
extern crate nb;
extern crate embedded_hal as hal;

use hal::serial;

pub struct ESP8266<R, W, RC, WC> {
	rx: R,
	tx: W,
    rxc: RC,
    txc: WC,
}

impl<R, W, RC, WC> ESP8266<R, W, RC, WC> where
    R: hal::serial::Read<u8>,
    W: hal::serial::Write<u8>
    {
    	pub fn new(tx: W, rx: R, txc: WC, rxc: RC) -> Self {
    		let mut esp = Self {
    			tx, rx, txc, rxc
    		};

            let mut rst_resp: [u8; 2] = [0; 2];
            esp.transfer(b"AT+RST\r\n", &mut rst_resp);

            // asm::bkpt();

    		let mut ate_resp: [u8; 2] = [0; 2];
    		esp.transfer(b"ATE0\r\n", &mut ate_resp);
            // str::from_utf8_unchecked(rev);

            // asm::bkpt();

            let mut sock_en_resp: [u8; 2] = [0; 2];
            esp.transfer(b"AT+CIPMUX=1\r\n", &mut sock_en_resp);

    		// asm::bkpt();

            let mut some_resp: [u8; 2] = [0; 2];
            esp.transfer(b"AT+CIPDINFO=1\r\n", &mut some_resp);

            // asm::bkpt();

    		esp
    	}

    	fn transfer(&mut self, send: &[u8], recv: &mut [u8]) {
			for c in send {
				block!(self.tx.write(*c));
			}

			for i in 0..recv.len() {
				let mut res = 0;

				while true {
					match self.rx.read() {
						Ok(c) => {
							res = c;

							break;
						},
						_ => ()
					}
				}
                // res = block!(self.rx.read());
                asm::bkpt();

				recv[i] = res;
			}
    	}
}