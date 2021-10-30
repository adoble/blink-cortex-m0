// #![no_std]
// #![no_main]

// // pick a panicking behavior
// use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// // use panic_abort as _; // requires nightly
// // use panic_itm as _; // logs messages over ITM; requires ITM support
// // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

// use cortex_m::asm;
// use cortex_m_rt::entry;

// #[entry]
// fn main() -> ! {
//     asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

//     loop {
//         // your code goes here
//     }
// }

// --------------------------------------------------

#![no_std]
#![no_main]

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::hal;
use bsp::pac;
use feather_m0 as bsp;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use pac::{CorePeripherals, Peripherals};



#[entry]
fn main() -> ! {

    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
   
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = bsp::Pins::new(peripherals.PORT);
    let mut red_led: bsp::RedLed = pins.d13.into();
    let mut pin_9 = pins.d9.into_push_pull_output();
    let mut delay = Delay::new(core.SYST, &mut clocks);

    //let mut serial = bsp::ehal::default_serial!(peripherals, pins, 57600);
    //let mut ser = bsp::hal::default_serial();
    loop {
        delay.delay_ms(200u16);
        red_led.set_high().unwrap();
        pin_9.set_high().unwrap();
        delay.delay_ms(300u16);
        red_led.set_low().unwrap();
        pin_9.set_low().unwrap();
 

    }
}