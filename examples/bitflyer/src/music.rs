

use ubit;
use sounds;
use zinc::hal::pin::Gpio;

const _SILENCE: [(u16, u16, u16);2] = [(0,0,1000), (0,0,1000)];
const _CRASH: [(u16, u16, u16);8] = [(0,0,5), (65,40,100), (64,41,110), (63,42,120), (62,43,140), (61, 44, 180), (60, 45, 260), (61, 44, 600)];
const TICK_FREQ: u32 = 36030;


pub struct Sound {
	notes: &'static [(u16,u16,u16)],
	tempo: u32,
}

pub const SILENCE: Sound = Sound{ notes: &_SILENCE, tempo: 1};
pub const BUMBLE_BEE: Sound = Sound{ notes: &sounds::BUMBLE_BEE, tempo: 4};
pub const BRITSIDE: Sound = Sound{ notes: &sounds::BRITSIDE, tempo: 2};
pub const SOUSA: Sound = Sound{ notes: &sounds::SOUSA, tempo: 3};
pub const CRASH: Sound = Sound{ notes: &_CRASH, tempo: 3};


const FREQUENCIES: [u32;128] = [
	8, 8, 9, 9, 10, 10, 11, 12, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 23,
	24, 25, 27, 29, 30, 32, 34, 36, 38, 41, 43, 46, 48, 51, 55, 58, 61, 65,
	69, 73, 77, 82, 87, 92, 97, 103, 110, 116, 123, 130, 138, 146, 155, 164,
	174, 184, 195, 207, 220, 233, 246, 261, 277, 293, 311, 329, 349, 369, 391,
    415, 440, 466, 493, 523, 554, 587, 622, 659, 698, 739, 783, 830, 880, 932,
    987, 1046, 1108, 1174, 1244, 1318, 1396, 1479, 1567, 1661, 1760, 1864, 1975,
    2093, 2217, 2349, 2489, 2637, 2793, 2959, 3135, 3322, 3520, 3729, 3951, 4186,
    4434, 4698, 4978, 5274, 5587, 5919, 6271, 6644, 7040, 7458, 7902, 8372, 8869,
    9397, 9956, 10548, 11175, 11839, 12543
];


pub static mut play_speed: u32 = 100;
static mut tick_num: u32 = 0;
static mut cur_sound: Sound = SILENCE;

static mut cur_index: usize = 0;
static mut index_expires: u32 = 0;

static mut period_a: u32 = 0;
static mut period_b: u32 = 0;

static mut sound_repeat: bool = false;
static mut two: bool = false;

#[inline]
fn pitch_to_period(pitch: u16) -> u32 {
	if pitch == 0 { return 0;  }
	return TICK_FREQ / FREQUENCIES[pitch as usize];
}

pub fn set_sound(sound: Sound, repeat: bool) {
	unsafe{
		sound_repeat = repeat;
		tick_num = 0;
		cur_sound = sound;
		cur_index = 0;
		index_expires = 0;
		period_a = 0;
		period_b = 0;
	}
}

fn next_note() {
	unsafe{
		tick_num = 0;
		cur_index += 1;
		if cur_index >= cur_sound.notes.len() {
			if sound_repeat {
				cur_index = 0;
			} else {
				set_sound(SILENCE, true);
			}
		}
		let (next_pitch_a, next_pitch_b, next_duration) = cur_sound.notes[cur_index];
		index_expires = (next_duration as u32 / cur_sound.tempo) * play_speed;
		period_a = pitch_to_period(next_pitch_a);
		period_b = pitch_to_period(next_pitch_b);
		two = period_b != 0;
	}
}

pub fn tick() {
	unsafe{
		tick_num += 1;
		if tick_num >= index_expires {
			next_note();
		}

		let period = if two && (tick_num / 32) % 3 == 0 { period_b } else { period_a };

		if period == 0 { ubit::PIN_0.set_low(); return; }
		if tick_num % period >  period / 2 {
			ubit::PIN_0.set_high();
		} else {
			ubit::PIN_0.set_low();
		}
	}
}