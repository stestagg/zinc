/// Taken from rand::rand (but doesn't rely on std)
// Auto-reseeds from the hardware rng periodically

use core::num::Wrapping;
use core::mem;
use zinc::hal::nrf51822;

#[allow(bad_style)]
type w64 = Wrapping<u64>;
#[allow(bad_style)]
type w32 = Wrapping<u32>;

const RESEED_EVERY:u32 = 100;
static mut reseed_counter: u32 = 0;


struct XorShiftRng {
    x: w32,
    y: w32,
    z: w32,
    w: w32,
}

static mut _rng: XorShiftRng = XorShiftRng {
    x: Wrapping(0x193a6754),
    y: Wrapping(0xa8a7d469),
    z: Wrapping(0x97830e05),
    w: Wrapping(0x113ba7bb),
};

fn reseed() {
    unsafe{
        let mut seed: [u32;4] = mem::uninitialized();
        let u8data: &mut [u8;16] = mem::transmute::<&mut [u32;4], &mut [u8;16]>(&mut seed);
        nrf51822::rand::fill_rand(u8data);

        _rng.x = Wrapping(seed[0]);
        _rng.y = Wrapping(seed[1]);
        _rng.z = Wrapping(seed[2]);
        _rng.w = Wrapping(seed[3]);
    }
}

pub fn rand() -> u32 {
    unsafe{
        if reseed_counter == 0 {
            reseed();
            reseed_counter = RESEED_EVERY;
        }
        reseed_counter -= 1;
        let x = _rng.x;
        let t = x ^ (x << 11);
        _rng.x = _rng.y;
        _rng.y = _rng.z;
        _rng.z = _rng.w;
        let w_ = _rng.w;
        _rng.w = w_ ^ (w_ >> 19) ^ (t ^ (t >> 8));
        _rng.w.0
    }
}