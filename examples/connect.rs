#![no_std]
#![feature(proc_macro)]
#![deny(unsafe_code)]
// #![deny(warnings)]
#![feature(const_fn)]
#![feature(used)]

extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate esp8266_driver;
extern crate stm32f103xx_hal as blue_pill;

use blue_pill::dma::{CircBuffer, Event, dma1};
use blue_pill::prelude::*;
use blue_pill::serial::Serial;
use blue_pill::stm32f103xx;
use cortex_m::asm;
use rtfm::{app, Threshold};

app! {
    device: stm32f103xx,

    resources: {
        static BUFFER: [[u8; 8]; 2] = [[0; 8]; 2];
        static CB: CircBuffer<[u8; 8], dma1::C3>;
    },

    init: {
        resources: [BUFFER],
    },

    tasks: {
        DMA1_CHANNEL3: {
            path: rx,
            resources: [CB],
        },
    }
}

fn init(p: init::Peripherals, r: init::Resources) -> init::LateResources {
    let mut flash = p.device.FLASH.constrain();
    let mut rcc = p.device.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = p.device.AFIO.constrain(&mut rcc.apb2);

    // let mut gpioa = p.device.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = p.device.GPIOB.split(&mut rcc.apb2);

    // USART1
    // let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    // let rx = gpioa.pa10;

    // USART1
    // let tx = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
    // let rx = gpiob.pb7;

    // USART2
    // let tx = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    // let rx = gpioa.pa3;

    // USART3
    let tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
    let rx = gpiob.pb11;

    let serial = Serial::usart3(
        p.device.USART3,
        (tx, rx),
        &mut afio.mapr,
        9_600.bps(),
        clocks,
        &mut rcc.apb1,
    );

    let (tx, rx) = serial.split();

    let mut channels = p.device.DMA1.split(&mut rcc.ahb);
    channels.3.listen(Event::HalfTransfer);
    channels.3.listen(Event::TransferComplete);

    let (_, c, tx) = tx.write_all(channels.2, b"AT\r\n").wait();

    init::LateResources {
        CB: rx.circ_read(channels.3, r.BUFFER),
    }
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}

fn rx(_t: &mut Threshold, mut r: DMA1_CHANNEL3::Resources) {
    r.CB
        .peek(|_buf, _half| {
            let foo = _buf;
            let bar = _half;
            asm::bkpt();
        })
        .unwrap();
}
