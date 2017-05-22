
extern crate core;

use super::regs;
use drivers::chario::CharIO;

#[path="../../util/wait_for.rs"]
#[macro_use] mod wait_for;


pub struct Uart {}

pub const UART: Uart = Uart {};

pub enum BaudRate {
	Baud1200 = 0x0004F000,
	Baud2400 = 0x0009D000,
	Baud4800 = 0x0013B000,
	Baud9600 = 0x00275000,
	Baud14400 = 0x003B0000,
	Baud19200 = 0x004EA000,
	Baud28800 = 0x0075F000,
	Baud38400 = 0x009D5000,
	Baud57600 = 0x00EBF000,
	Baud76800 = 0x013A9000,
	Baud115200 = 0x01D7E000,
	Baud230400 = 0x03AFB000,
	Baud250000 = 0x04000000,
	Baud460800 = 0x075F7000,
	Baud921600 = 0x0EBEDFA4,
	Baud1M = 0x10000000,
}

pub struct UartConfig {
	pub tx_pin: u8,
	pub rx_pin: u8,
	pub cts_pin: u8,
	pub rts_pin: u8,
	pub baud: BaudRate,
	pub parity: bool,
	pub flow_control: bool,
}

const UNUSED: u8 = 255;

impl Default for UartConfig {
	#[inline]
 	fn default() -> UartConfig {
   		UartConfig {
   			tx_pin: 24,
   			rx_pin: 25,
   			cts_pin: 26,
   			rts_pin: 27,
   			baud: BaudRate::Baud9600,
   			parity: false,
   			flow_control: false,
    	}
  	}
}

impl Uart {

	pub fn configure(&self, config: UartConfig) {
		use ::hal::pin::GpioDirection;
		use super::pin::{Pin,PinConfig,PinPull};
		let uart = regs::UART();

		Pin::new(config.tx_pin).configure(PinConfig {direction: GpioDirection::Out, pull: PinPull::High, ..Default::default()});
		Pin::new(config.rx_pin).configure(PinConfig {direction: GpioDirection::In, pull: PinPull::High, ..Default::default()});
		// if config.rts_pin < 32 {
			Pin::new(config.rts_pin).configure(PinConfig {direction: GpioDirection::Out, ..Default::default()});
		}
		if config.cts_pin < 32 {
			Pin::new(config.cts_pin).configure(PinConfig {direction: GpioDirection::In, ..Default::default()});
		}

		uart.baud_rate.set_baud(config.baud as u32);
		uart.config
			.set_parity_bit(false)
			.set_hardware_flow_control(false);

		uart.enable.set_enable(true);
		uart.start_rx.set_trigger(true);
		uart.start_tx.set_trigger(true);
		uart.rxd_ready.set_set(false);
		uart.txd.set_data(0);

		uart.rts_pin.set_pin(match config.rts_pin { 0...31 => config.rts_pin as u32, _ => 0xFFFFFFFF});
		uart.cts_pin.set_pin(match config.cts_pin { 0...31 => config.cts_pin as u32, _ => 0xFFFFFFFF});
		uart.rxd_pin.set_pin(match config.rx_pin { 0...31 => config.rx_pin as u32, _ => 0xFFFFFFFF});
		uart.txd_pin.set_pin(match config.tx_pin { 0...31 => config.tx_pin as u32, _ => 0xFFFFFFFF});
	}

	#[inline]
	pub fn write_byte(&self, byte: u8) {
		let uart = regs::UART();
		uart.txd_ready.set_set(false);
		uart.txd.set_data(byte as u32);
		wait_for!(uart.txd_ready.set());
	}

	#[inline]
	pub fn write_bytes(&self, bytes: &[u8]) {
		for byte in bytes {
			self.write_byte(*byte);
		}
	}

	#[inline]
	pub fn write(&self, data: &str) {
		for char in data.bytes() {
			self.write_byte(char);
		}
	}

	pub fn writeln(&self, data: &str) {
		self.write(data);
		self.write("\r\n");
	}

}

impl CharIO for Uart {
  fn putc(&self, value: char) {
    self.write_byte(value as u8);
  }
}

impl core::fmt::Write for Uart {

	fn write_str(&mut self, s: &str) -> core::fmt::Result {
		self.write(s);
		return core::result::Result::Ok(());
	}

}