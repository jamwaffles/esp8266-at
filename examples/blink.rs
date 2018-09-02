//! Sample program to make sure RTFM systick works correctly

#![no_std]
#![no_main]
#![deny(unsafe_code)]

extern crate cortex_m;
#[macro_use(entry, exception)]
extern crate cortex_m_rt;
extern crate cortex_m_rtfm as rtfm;
extern crate cortex_m_semihosting as semihosting;
extern crate either;
extern crate esp8266_at;
extern crate heapless;
extern crate panic_itm;
extern crate stm32f103xx_hal as blue_pill;

use blue_pill::gpio::gpioc::PC13;
use blue_pill::gpio::{Output, PushPull};
use blue_pill::prelude::*;
use blue_pill::stm32f103xx;
use blue_pill::timer::{Event as TimerEvent, Timer};
use core::fmt::Write;
use cortex_m::asm;
use cortex_m_rt::ExceptionFrame;
use rtfm::{app, Threshold};
use semihosting::hio;

app! {
    device: stm32f103xx,

    resources: {
        static LED: PC13<Output<PushPull>>;
        static HSTDOUT: hio::HStdout;
    },

    tasks: {
        SYS_TICK: {
            path: sys_tick,
            resources: [LED, HSTDOUT],
        },
    },
}

// Wrap `app!()`-generated `main()` to fix return type
fn ent() -> ! {
    main();

    loop {}
}

entry!(ent);

fn init(p: init::Peripherals) -> init::LateResources {
    let mut flash = p.device.FLASH.constrain();
    let mut rcc = p.device.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioc = p.device.GPIOC.split(&mut rcc.apb2);

    Timer::syst(p.core.SYST, 10.hz(), clocks).listen(TimerEvent::Update);

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    led.set_low();

    let mut hstdout = hio::hstdout().unwrap();

    writeln!(hstdout, "INIT").unwrap();

    init::LateResources {
        LED: led,
        HSTDOUT: hstdout,
    }
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}

fn sys_tick(_t: &mut Threshold, mut r: SYS_TICK::Resources) {
    writeln!(r.HSTDOUT, "TICK, LED low: {}", r.LED.is_set_low()).unwrap();

    if r.LED.is_set_low() {
        r.LED.set_high()
    } else {
        r.LED.set_low()
    }
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
