#![feature(no_std)]
#![no_std]

//! Turn on the blue LED (PC8)

extern crate stm32;

#[no_mangle]
pub fn main() {
    let rcc = stm32::peripheral::rcc();
    let gpioc = stm32::peripheral::gpioc();

    // power up GPIOC
    rcc.apb2enr.update(|apb2enr| {
        use stm32::rcc::apb2enr::prelude::*;

        apb2enr | IOPCEN
    });

    // set PC8 as output
    gpioc.crh.update(|crh| {
        use stm32::gpio::crh::prelude::*;

        crh.configure(Pin::_8, Mode::Output(GeneralPurpose, PushPull, _2MHz))
    });

    // set PC8 high
    gpioc.bsrr.set({
        use stm32::gpio::bsrr::prelude::*;

        BS8
    });
}
