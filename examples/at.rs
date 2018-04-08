#![no_std]
#![feature(proc_macro)]
#![deny(unsafe_code)]
// #![deny(warnings)]
#![feature(const_fn)]
#![feature(used)]

#[macro_use(singleton)]
extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate esp8266_serial;
extern crate stm32f103xx_hal as blue_pill;
#[macro_use(block)]
extern crate nb;

// use blue_pill::dma::{CircBuffer, dma1};
use blue_pill::delay::Delay;
use blue_pill::dma::{CircBuffer, Event, dma1};
use blue_pill::prelude::*;
use blue_pill::serial::{Event as SerialEvent, Rx, Serial, Tx};
use blue_pill::stm32f103xx;
use blue_pill::stm32f103xx::USART3 as USART3_PERIPHERAL;
use blue_pill::timer::{self, Timer};
use cortex_m::asm;
use esp8266_serial::ESP8266;
use rtfm::{app, Threshold};

app! {
    device: stm32f103xx,

    resources: {
        static TX: Tx<USART3_PERIPHERAL>;
        static RX: Rx<USART3_PERIPHERAL>;
        // static BUFFER: [[u8; 8]; 2] = [[0; 8]; 2];
        // static CB: CircBuffer<[u8; 8], dma1::C3>;
        static WIFI: ESP8266;
    },

    // init: {
    //     resources: [BUFFER],
    // },

    tasks: {
        // DMA1_CHANNEL3: {
        //     path: rx,
        //     resources: [TX, CB, WIFI],
        // },

        USART3: {
            path: usart,
            resources: [RX, TX, WIFI],
        },

        SYS_TICK: {
            path: tick,
            resources: [TX, WIFI],
        },
    },
}

fn init(p: init::Peripherals) -> init::LateResources {
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

    let mut serial = Serial::usart3(
        p.device.USART3,
        (tx, rx),
        &mut afio.mapr,
        9_600.bps(),
        clocks,
        &mut rcc.apb1,
    );

    // let mut channels = p.device.DMA1.split(&mut rcc.ahb);

    serial.listen(SerialEvent::Rxne);
    let (mut tx, rx) = serial.split();

    // let sent = b"AT+RST";

    // for c in sent {
    //     block!(tx.write(*c as u8)).ok();
    // }

    let wifi = ESP8266::new();

    let mut delay = Delay::new(p.core.SYST, clocks);
    Timer::syst(delay.free(), 1.hz(), clocks).listen(timer::Event::Update);

    // channels.3.listen(Event::HalfTransfer);
    // channels.3.listen(Event::TransferComplete);
    init::LateResources {
        // CB: rx.circ_read(channels.3, r.BUFFER),
        WIFI: wifi,
        RX: rx,
        TX: tx,
    }
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}

fn tick(_t: &mut Threshold, mut r: SYS_TICK::Resources) {
    if let Some(send) = r.WIFI.next_command() {
        for c in send {
            block!(r.TX.write(*c as u8)).ok();
        }
    }
}

// fn rx(_t: &mut Threshold, mut r: DMA1_CHANNEL3::Resources) {
//     let out = r.CB
//         .peek_both(|_wip, _buf, _half| {
//             let b = _buf;
//             let half = _half;
//             let wip = _wip;
//             asm::bkpt();
//             _buf.clone()
//         })
//         .unwrap();

//     asm::bkpt();
// }

fn usart(_t: &mut Threshold, mut r: USART3::Resources) {
    let byte = r.RX.read();

    r.WIFI.handle_byte(byte.unwrap());

    // let out = r.CB
    //     .peek_both(|_wip, _buf, _half| {
    //         // let b = _buf;
    //         // let half = _half;
    //         // let wip = _wip;
    //         // asm::bkpt();
    //         // _buf.clone()
    //         _wip.clone()
    //     })
    //     .unwrap();
}
