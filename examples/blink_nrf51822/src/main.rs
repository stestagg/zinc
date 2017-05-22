#![no_builtins]

#![feature(plugin, start, lang_items,core_intrinsics,asm)]
#![no_std]
#![plugin(macro_zinc)]

extern crate zinc;

use zinc::hal::isr;

pub use core::fmt;
use core::cmp::{min,max};

// #[macro_use]
// macro_rules! println {
//     ( $( $x:expr ),* ) => { let _ = $crate::fmt::write(&mut $crate::ubit::UART, format_args!($($x),*)); $crate::ubit::UART.writeln("");}
// }



mod ubit;
use ubit::*;


#[zinc_main]
fn main() {
    zinc::hal::mem_init::init_stack();
    zinc::hal::mem_init::init_data();

    ubit::UART.configure(nrf51822::uart::UartConfig {
        baud: BaudRate::Baud115200,
        ..Default::default()
    });
    ubit::UART.write("Hello world");
    loop {};
}
