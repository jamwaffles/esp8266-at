#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_itm;
extern crate stm32f1xx_hal as hal;

use cortex_m::iprintln;
use cortex_m::singleton;
use cortex_m_rt::ExceptionFrame;
use cortex_m_rt::{entry, exception};
use esp8266_at;
use hal::prelude::*;
use hal::serial::Serial;
use hal::stm32;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut itm = cp.ITM;

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let channels = dp.DMA1.split(&mut rcc.ahb);

    // USART1
    let tx = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
    let rx = gpiob.pb7;

    let serial = Serial::usart1(
        dp.USART1,
        (tx, rx),
        &mut afio.mapr,
        9_600.bps(),
        clocks,
        &mut rcc.apb2,
    );

    let (tx, rx) = serial.split();

    let buf = singleton!(: [u8; 8] = [0; 8]).unwrap();

    iprintln!(&mut itm.stim[0], "Hello, world!");

    let (_buf, _c, _rx) = rx.read_exact(channels.5, buf).wait();

    loop {}
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
