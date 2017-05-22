#![feature(asm)]

//! HAL for NXP LPC11xx.

mod regs;
pub mod uart;
pub mod pin;
pub mod rtc;
pub mod clock;
pub mod twi;
pub mod rand;
pub mod isr;

pub fn wait_for_event() {
    unsafe{ asm!("sev; wfe" :::: "volatile")}
}


pub fn sleep_us(mut us: u64) {
	unsafe{
		loop{
			asm!(
		       "NOP
		       NOP
		       NOP
		       NOP
		       NOP
		       NOP
		       NOP
		       NOP" :::: "volatile"
		    );
		   	us -= 1;
		   	if us == 0 { return; }
	    }
	}
}



pub fn init() {
	clock::start_lfc();
	clock::calibrate();
	rand::set_error_correction(false);
	rand::start();

}