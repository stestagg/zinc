#[path="../../util/wait_for.rs"]
#[macro_use] mod wait_for;

use super::regs;


pub struct Rtc {}

pub const RTC: Rtc = Rtc {};


impl Rtc {

	pub fn init(&self) {
		let rtc = regs::RTC();
		rtc.power.set_on(true);

		rtc.stop.set_trigger(true);
		rtc.ovrflw.set_set(false);
		rtc.evten
			.set_tick(false)
			.set_ovrflw(false)
			.set_compare0(false)
			.set_compare1(false)
			.set_compare2(false)
			.set_compare3(false);
		rtc.start.set_trigger(true);
	}
	pub fn set_prescaler(&self, prescaler: u32) {
		let rtc = regs::RTC();
		rtc.prescaler.set_value(prescaler);
		wait_for!(rtc.prescaler.value() == prescaler);
	}

	pub fn enable_tick_iterrupt(&self) {
		let rtc = regs::RTC();
		rtc.evtenset.set_tick(true);
		rtc.intenset.set_tick(true);
	}

	#[inline(always)]
	pub fn clear_tick(&self) {
		regs::RTC().tick.set_set(false);
	}

	pub fn get_count(&self) -> u32 {
		return regs::RTC().counter.value();
	}

	pub fn clear(&self) {
		let rtc = regs::RTC();
		//self.stop();
		rtc.clear.set_trigger(true);
		self.start();
		wait_for!(rtc.counter.value() == 0)
	}

	pub fn start(&self) {
		super::clock::start_lfc();
		regs::RTC().start.set_trigger(true);
	}

	pub fn stop(&self) {
		regs::RTC().stop.set_trigger(true);
	}

}