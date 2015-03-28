#![feature(core)]
#![feature(no_std)]
#![no_std]

//! Transmit "Hello, world!" via USART1 (PA8) using the Channel 4 of DMA1.

extern crate core;
extern crate cortex;
extern crate stm32;

use core::prelude::*;

const CLOCK: u32 = 8_000_000;
const BAUD_RATE: u32 = 115_200;

#[no_mangle]
pub fn main() {
    let dma1 = stm32::peripheral::dma1();
    let gpioa = stm32::peripheral::gpioa();
    let nvic = cortex::peripheral::nvic();
    let rcc = stm32::peripheral::rcc();
    let usart1 = stm32::peripheral::usart1();

    // Enable GPIOA and USART1
    rcc.apb2enr.update(|apb2enr| {
        use stm32::rcc::apb2enr::prelude::*;

        apb2enr | IOPAEN | USART1EN
    });

    // Enable DMA1
    rcc.ahbenr.update(|ahbenr| {
        use stm32::rcc::ahbenr::prelude::*;

        ahbenr | DMA1EN
    });

    // Configure PA8 as USART1 TX
    gpioa.crh.update(|crh| {
        use stm32::gpio::crh::prelude::*;

        crh.configure(Pin::_9, Mode::Output(Alternate, PushPull, _2MHz))
    });

    // Enable USART and transmitter
    usart1.cr1.set({
        use stm32::usart::cr1::prelude::*;

        TE | UE
    });

    // Connect the USART1 transmission event to DMA
    usart1.cr3.set({
        use stm32::usart::cr3::prelude::*;

        DMAT
    });

    // Set baud rate
    usart1.brr.set({
        (CLOCK / BAUD_RATE) as u16
    });

    // Configure Channel 4 of DMA1
    dma1.ccr4.set({
        use stm32::dma::ccr::prelude::*;

        TCIE | DIR | MINC | PSIZE::_8 | MSIZE::_8
    });

    // Enqueue message in the DMA channel
    let message = "Hello, world!\n\r";

    dma1.cmar4.set(message.as_ptr() as u32);
    dma1.cndtr4.set(message.len() as u16);
    dma1.cpar4.set(&usart1.dr as *const _ as u32);

    // Enable DMA interrupt
    nvic.iser0.set({
        use cortex::nvic::iser0::prelude::*;

        _14  // DMA1_Channel4
    });

    // Start DMA transmission
    dma1.ccr4.update(|ccr| {
        use stm32::dma::ccr::prelude::*;

        ccr | EN
    });

    // Wait for interrupt
    cortex::asm::wfi();
}

#[no_mangle]
pub fn dma1_channel4() {
    let dma1 = stm32::peripheral::dma1();

    // Clear transmission complete flag
    dma1.ifcr.set({
        use stm32::dma::ifcr::prelude::*;

        CTCIF4
    });
}
