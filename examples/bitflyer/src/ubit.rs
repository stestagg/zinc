
use zinc::hal::cortex_m0::systick;
use zinc::hal::nrf51822::pin::*;
use zinc::hal::pin;

pub use zinc::hal::pin::Gpio;
pub use zinc::hal::nrf51822;

pub use zinc::hal::nrf51822::uart::{UART, BaudRate};
pub use zinc::hal::nrf51822::rand;
pub use zinc::drivers::chario::CharIO;

pub const MICROBIT_PIN: [u8; 21] = [3, 2, 1, 4, 5, 17, 12, 11, 18, 10, 6, 26, 20, 23, 22, 21, 16, 99, 99, 0, 30];

pub const PIN_0: Pin = Pin { index: MICROBIT_PIN[0]};
pub const PIN_1: Pin = Pin { index: MICROBIT_PIN[1]};
pub const PIN_2: Pin = Pin { index: MICROBIT_PIN[2]};
pub const PIN_3: Pin = Pin { index: MICROBIT_PIN[3]};
pub const BUTTON_A: Pin = Pin { index: MICROBIT_PIN[5]};
pub const BUTTON_B: Pin = Pin { index: MICROBIT_PIN[11]};

pub trait UbitButton {
    fn button_pressed(&self) -> bool;
}

impl UbitButton for Pin {
    fn button_pressed(&self) -> bool {
        use zinc::hal::pin::Gpio;
        return self.level() == GpioLevel::Low;
    }
}

pub fn sleep_ms(ms: u32) {
    nrf51822::sleep_us((ms as u64) * 1000);
}

pub fn sleep(s: f32) {
    let us = (s * 1e6) as u64;
    nrf51822::sleep_us(us);
}

pub fn init_board() {
    PIN_0.configure(PinConfig {
        direction: GpioDirection::Out,
        ..Default::default()
    });
    PIN_1.configure(PinConfig {
        direction: GpioDirection::Out,
        ..Default::default()
    });
    PIN_2.configure(PinConfig {
        direction: GpioDirection::Out,
        ..Default::default()
    });
    PIN_3.configure(PinConfig {
        direction: GpioDirection::Out,
        ..Default::default()
    });
    BUTTON_A.configure(PinConfig {
        direction: GpioDirection::In,
        input_connect: true,
        sense: SenseMode::Low,
        ..Default::default()
    });
    BUTTON_B.configure(PinConfig {
        direction: GpioDirection::In,
        input_connect: true,
        sense: SenseMode::Low,
        ..Default::default()
    });
    UART.configure(nrf51822::uart::UartConfig {
        baud: BaudRate::Baud115200,
        ..Default::default()
    });

    nrf51822::init();
    sleep(0.1)
}