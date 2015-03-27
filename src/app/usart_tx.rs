#![feature(core)]
#![feature(no_std)]
#![no_std]

//! Transmit "Hello, world!" via USART1 (PA8)

extern crate core;
extern crate cortex;
extern crate stm32;

use core::prelude::*;

const CLOCK: u32 = 8_000_000;
const BAUD_RATE: u32 = 115_200;

#[no_mangle]
pub fn main() {
    let gpioa = stm32::peripheral::gpioa();
    let nvic = cortex::peripheral::nvic();
    let rcc = stm32::peripheral::rcc();
    let usart1 = stm32::peripheral::usart1();

    // Enable GPIOA and USART1
    rcc.apb2enr.update(|apb2enr| {
        use stm32::rcc::apb2enr::prelude::*;

        apb2enr | IOPAEN | USART1EN
    });

    // Configure PA8 as USART1 TX
    gpioa.crh.update(|crh| {
        use stm32::gpio::crh::prelude::*;

        crh.configure(Pin::_9, Mode::Output(Alternate, PushPull, _2MHz))
    });

    // Enable USART, transmitter and transmission complete interrupt
    usart1.cr1.set({
        use stm32::usart::cr1::prelude::*;

        TE | TCIE | UE
    });

    // Set baud rate
    usart1.brr.set({
        (CLOCK / BAUD_RATE) as u16
    });

    // Enable USART1 interrupt
    nvic.iser1.set({
        use cortex::nvic::iser1::prelude::*;

        _37  // USART1
    });

    let message = "Hello, world!\n\r";

    for byte in message.bytes() {
        // enqueue byte in the transmit register
        usart1.dr.set(byte);

        // wait for interrupt
        cortex::asm::wfi();
    }
}

#[no_mangle]
pub fn usart1() {
    let usart1 = stm32::peripheral::usart1();

    // Clear transmission complete flag
    usart1.sr.update(|sr| {
        use stm32::usart::sr::prelude::*;

        sr & !TC
    });
}
