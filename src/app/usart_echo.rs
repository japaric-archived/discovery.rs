#![feature(core)]
#![feature(no_std)]
#![no_std]

//! An echo "server" over USART1. All the received bytes will be retransmitted back.

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

    // Enable USART, transmitter, receiver and receiver interrupt
    usart1.cr1.set({
        use stm32::usart::cr1::prelude::*;

        TCIE | RXNEIE | TE | RE | UE
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

    loop {
        // wait for incoming data
        cortex::asm::wfi();

        match usart1.dr.get().u8() as char {
            // Map carriage return to a newline (like termios' ICRNL)
            '\r' => {
                usart1.dr.set('\n' as u8);
                cortex::asm::wfi();
                // NB minicom requires `\n\r` to go to the start of the next line
                usart1.dr.set('\r' as u8);
            },
            byte => {
                usart1.dr.set(byte as u8);
            },
        }

        // wait until transmission is over
        cortex::asm::wfi();
    }
}

#[no_mangle]
pub fn usart1() {
    let usart1 = stm32::peripheral::usart1();

    // Clear "read data register not empty" and "transmission complete" flag
    usart1.sr.update(|sr| {
        use stm32::usart::sr::prelude::*;

        sr & !TC & !RXNE
    });
}
