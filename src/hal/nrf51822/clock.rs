

#[path="../../util/wait_for.rs"]
#[macro_use] mod wait_for;

use super::regs;

pub fn start_lfc() {
	let clock = regs::CLOCK();
	clock.lfclkstart.set_trigger(true);
	wait_for!(clock.lfclkrun.running());
}

pub fn calibrate() {
	let clock = regs::CLOCK();
	clock.ctstart.set_trigger(true);
}