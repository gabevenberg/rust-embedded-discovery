#![no_main]
#![no_std]

use core::fmt::Write;
use cortex_m_rt::entry;
use heapless::Vec;
use nb::block;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

#[cfg(feature = "v1")]
use microbit::{
    hal::prelude::*,
    hal::uart,
    hal::uart::{Baudrate, Parity},
};

#[cfg(feature = "v2")]
use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

#[cfg(feature = "v2")]
mod serial_setup;
#[cfg(feature = "v2")]
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    #[cfg(feature = "v1")]
    let mut serial = {
        uart::Uart::new(
            board.UART0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        )
    };

    #[cfg(feature = "v2")]
    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    // A buffer with 32 bytes of capacity
    let mut buffer: Vec<u8, 32> = Vec::new();

    loop {
        buffer.clear();

        loop {
            let char = block!(serial.read()).unwrap();
            if char == 13 {
                break;
            }
            block!(serial.write(char)).unwrap();
            if buffer.push(char).is_err() {
                write!(serial, "input buffer is full").unwrap();
                break;
            }
            block!(serial.flush()).unwrap();
        }

        for byte in [b'\n', b'\r']
            .iter()
            .chain(buffer.iter().rev().chain(&[b'\n', b'\r']))
        {
            block!(serial.write(*byte)).unwrap();
        }
        block!(serial.flush()).unwrap();
    }
}

