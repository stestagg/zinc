
use super::regs;

#[path="../../util/wait_for.rs"]
#[macro_use] mod wait_for;


pub fn start() {
	regs::RNG().start.set_trigger(true);
}

pub fn set_error_correction(enable: bool) {
	regs::RNG().config.set_error_correction(enable);
}

pub fn rand_byte() -> u8 {
	wait_for!(regs::RNG().val_ready.set());
	let val = regs::RNG().value.value() as u8;
	regs::RNG().val_ready.set_set(false);
	return val
}

pub fn fill_rand(buf: &mut [u8]) {
	for byte in buf {
		*byte = rand_byte();
	}
}