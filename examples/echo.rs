#![no_std]
#![feature(proc_macro)]
#![deny(unsafe_code)]
// #![deny(warnings)]
#![feature(const_fn)]
#![feature(used)]

extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate esp8266_at;
extern crate panic_abort;
extern crate stm32f103xx_hal as blue_pill;

// use blue_pill::dma::{CircBuffer, dma1};
use blue_pill::prelude::*;
use blue_pill::serial::{Event, Rx, Serial, Tx};
use blue_pill::stm32f103xx;
use blue_pill::stm32f103xx::USART3 as USART3_PERIPHERAL;
// use cortex_m::asm;
use rtfm::{app, Threshold};

app! {
    device: stm32f103xx,

    resources: {
        static TX: Tx<USART3_PERIPHERAL>;
        static RX: Rx<USART3_PERIPHERAL>;
    },

    tasks: {
        USART3: {
            path: echo,
            resources: [TX, RX],
        },
    }
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

    serial.listen(Event::Rxne);
    let (tx, rx) = serial.split();

    init::LateResources { TX: tx, RX: rx }
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}

fn echo(_: &mut Threshold, mut r: USART3::Resources) {
    let byte = r.RX.read().unwrap();
    r.TX.write(byte).unwrap();
}
