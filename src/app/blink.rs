#![feature(no_std)]
#![no_std]

//! Toggle the blue LED (PC8) at 1 Hz. The timing is handled by the TIM7 timer. The main thread
//! sleeps most of the time, and only wakes up on TIM7's interrupts to toggle the LED.

extern crate cortex;
extern crate stm32;

#[no_mangle]
pub fn main() {
    let gpioc = stm32::peripheral::gpioc();
    let nvic = cortex::peripheral::nvic();
    let rcc = stm32::peripheral::rcc();
    let tim7 = stm32::peripheral::tim7();

    // power up TIM7
    rcc.apb1enr.update(|apb1enr| {
        use stm32::rcc::apb1enr::prelude::*;

        apb1enr | TIM7EN
    });

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

    // configure TIM7 with a frequency of 1Hz (assuming a 8MHz clock source)
    tim7.psc.set(999);
    tim7.arr.set(8000);

    // enable TIM7 update interrupt
    tim7.dier.set({
        use stm32::tim::dier::prelude::*;

        UIE
    });

    // update autoreload and prescaler register
    tim7.egr.set({
        use stm32::tim::egr::prelude::*;

        UG
    });

    // start TIM7 counter
    tim7.cr1.update(|cr1| {
        use stm32::tim::cr1::prelude::*;

        cr1 | CEN
    });

    // unmask TIM7 interrupt
    nvic.iser1.set({
        use cortex::nvic::iser1::prelude::*;

        _55  // TIM7
    });

    loop {
        // set PC8 high
        gpioc.bsrr.set({
            use stm32::gpio::bsrr::prelude::*;

            BS8
        });

        // wait for interrupt
        cortex::asm::wfi();

        // set PC8 low
        gpioc.bsrr.set({
            use stm32::gpio::bsrr::prelude::*;

            BR8
        });

        // wait for interrupt
        cortex::asm::wfi();
    }
}

#[no_mangle]
pub extern fn tim7() {
    let tim7 = stm32::peripheral::tim7();

    tim7.sr.update(|sr| {
        use stm32::tim::sr::prelude::*;

        sr & !UIF
    });
}
