
use super::regs;
use drivers::chario::CharIO;
use ::hal::nrf51822::pin::{SenseMode,GpioDirection,PinDriveMode};

#[path="../../util/wait_for.rs"]
#[macro_use] mod wait_for;


pub struct Twi {}

pub const TWI: Twi = Twi {};

pub enum Frequency {
	K100 = 0x01980000,
	K250 = 0x4000000,
	K400 = 0x06680000,
}

pub struct TwiConfig {
	pub sda_pin: u8,
	pub scl_pin: u8,
	pub freq: Frequency,
}

impl Default for TwiConfig {
 	fn default() -> TwiConfig {
   		TwiConfig {
   			sda_pin: 30,
   			scl_pin: 0,
   			freq: Frequency::K100,
    	}
  	}
}

impl Twi {

	pub fn configure(&self, config: TwiConfig) {
		use ::hal::pin::GpioDirection;
		use super::pin::{Pin, PinConfig, PinPull, PinDriveMode, SenseMode};
		let twi = regs::TWI();

		twi.power.set_on(false);
		wait_for!(twi.power.on() == false);
		twi.power.set_on(true);
		twi.enable.set_value(regs::TWI_enable_value::disable);

		Pin::new(config.scl_pin).configure(PinConfig {
			direction: GpioDirection::In,
			pull: PinPull::NoPull,
			input_connect: true,
			drive_low: PinDriveMode::Standard,
			drive_high: PinDriveMode::Disconnect,
			sense: SenseMode::Disabled,
			..Default::default()
		});
		Pin::new(config.sda_pin).configure(PinConfig {
			direction: GpioDirection::In,
			pull: PinPull::NoPull,
			input_connect: true,
			drive_low: PinDriveMode::Standard,
			drive_high: PinDriveMode::Disconnect,
			sense: SenseMode::Disabled,
			..Default::default()
		});

		twi.sda_pin.set_pin(match config.sda_pin { 0...31 => config.sda_pin as u32, _ => 0xFFFFFFFF});
		twi.scl_pin.set_pin(match config.scl_pin { 0...31 => config.scl_pin as u32, _ => 0xFFFFFFFF});
		twi.frequency.set_value(config.freq as u32);

		twi.enable.set_value(regs::TWI_enable_value::enable);
		twi.shorts.set_bb_suspend_shortcut(false).set_bb_stop_shortcut(false);
	}

	pub fn set_address(&self, to: u8) {
		regs::TWI().address.set_value(to as u32);
	}

	pub fn write_byte(&self, byte: u8) {
		let twi = regs::TWI();
		twi.txd.set_data(byte as u32);
		wait_for!(twi.txdsent.set());
		twi.txdsent.set_set(false);
	}

	pub fn write_bytes(&self, bytes: &[u8]) {
		let twi = regs::TWI();
		twi.starttx.set_trigger(true);
		for byte in bytes {
			self.write_byte(*byte);
		}
		twi.stop.set_trigger(true);
	}

	pub fn write_bytes_prefix(&self, prefix: u8, bytes: &[u8]) {
		let twi = regs::TWI();
		twi.starttx.set_trigger(true);
		self.write_byte(prefix);
		for byte in bytes {
			self.write_byte(*byte);
		}
		twi.stop.set_trigger(true);
	}

	pub fn send_bytes(&self, to: u8, data: &[u8]) {
		self.set_address(to);
		self.write_bytes(data);
	}

}