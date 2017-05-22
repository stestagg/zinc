#![no_builtins]

#![feature(plugin, start, lang_items,core_intrinsics,asm)]
#![no_std]
#![plugin(macro_zinc)]

extern crate zinc;

use zinc::hal::isr;

pub use core::fmt;
use core::cmp::{min,max};

#[macro_use]
macro_rules! println {
    ( $( $x:expr ),* ) => { let _ = $crate::fmt::write(&mut $crate::ubit::UART, format_args!($($x),*)); $crate::ubit::UART.writeln("");}
}

mod images;
mod ubit;
mod display;
use display::ops;
mod sounds;
mod music;

mod rand;
        
use rand::rand;


#[lang = "panic_fmt"]
extern fn panic_fmt(message: ::core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("Panic at {}:{}, {}", file, line, message);
    loop {}
}

use ubit::*;

const SPLASHES: [&'static display::Image;8] = [
    &images::SPLASH,
    &images::SPLASH2,
    &images::SPLASH3,
    &images::SPLASH4,
    &images::SPLASH5,
    &images::SPLASH6,
    &images::SPLASH7,
    &images::SPLASH8,

];

fn splash() {
    const BG_FLIP_TIME: f32 = 0.03;
    const POLL_TIME: f32 = 0.005;
    const ITERS_PER_FLIP: i32 = (BG_FLIP_TIME / POLL_TIME) as i32;

    loop{
        for splash in SPLASHES.iter() {
            for _ in 0..ITERS_PER_FLIP {
                sleep(POLL_TIME);
                unsafe{
                    if display::cur_brightness < 255 {
                        display::set_brightness(display::cur_brightness + 2);
                        if display::cur_brightness == 255 {
                            music::set_sound(music::SOUSA, true);
                        }
                    }
                }
                if BUTTON_A.button_pressed() || BUTTON_B.button_pressed() {
                    return
                }
            }
            display::draw(splash, 0, 0);
            display::update_display();
        }
    }
}

struct Star{
    img: &'static display::Image,
    x: u8,
    y: f32,
    speed: f32,
}

const NUM_STAR_IMAGES:usize = 5;
const star_images: [&'static display::Image;NUM_STAR_IMAGES] = [
    &images::STAR, &images::ASTEROID, &images::ROCKET, &images::ALIEN,
    &images::ALIEN2,
];

impl Star{

    pub fn new() -> Star {
        Star {
            img: star_images[(rand() as usize) % NUM_STAR_IMAGES],
            x: (rand() % 127) as u8,
            y: - ((rand() % 20) as f32),
            speed: ((rand() % 20 + 10) as f32) / 20.
        }
    }

    pub fn reset(&mut self, score: u32) {
        use ubit::rand;
        self.img = star_images[(rand() as usize) % NUM_STAR_IMAGES];
        self.x = (rand() % (127 - self.img.width) as u32) as u8;
        self.y = -7.;
        let baseline = 40 + (score / 4);
        self.speed = (((rand() % 20) + baseline) as f32) / 200.;
    }

    pub fn clear(&self) {
        display::clear_image(self.img, self.x as i16, self.y as i16);
    }

    pub fn next(&mut self, score: u32) {
        self.y += self.speed;
        if self.y > 64. {
            self.reset(score);
        }
        display::blend(self.img, self.x as i16, self.y as i16);
    }

    pub fn collided(&self, ship_pos: i16) -> bool {
        let ship_max = ship_pos + 10;
        if self.y <= 42. || self.y > 55. { return false; }
        if (self.x as i16) > ship_max { return false; }
        if (self.x as i16) + (self.img.width as i16) < ship_pos + 2 { return false;}
        true
    }
}

const SHIP_Y:i16 = 48;
fn run_game() -> u32{

    let mut ship_pos = 64;
    let mut score: u32 = 0;
    let mut objs: [Star;7] = [
        Star::new(), Star::new(), Star::new(), Star::new(), Star::new(),
        Star::new(), Star::new(), //Star::new(), Star::new(), Star::new(),
    ];
    display::draw(&images::SHIP, ship_pos, SHIP_Y);
    loop{
        score += 1;
        if score == 50 {
            music::set_sound(music::BUMBLE_BEE, true);
        }
        let mut move_ship = 0;
        if ubit::BUTTON_A.button_pressed() {
            move_ship = 1;
        }
        if ubit::BUTTON_B.button_pressed() {
            move_ship -= 1;
        }
        if move_ship != 0 {
            display::clear_image(&images::SHIP, ship_pos, SHIP_Y);
            ship_pos = max(0, min(127 - 12, ship_pos + move_ship));
            display::draw(&images::SHIP, ship_pos, SHIP_Y);
        }
        for obj in objs.iter() {
            obj.clear();
        }

        for obj in objs.iter_mut() {
            obj.next(score);
            if obj.collided(ship_pos) {
                return score/5;
            }
        }

        display::draw_num_right(score/5, 126, 7);
        display::update_display();
    }
}

fn game_over(score: u32) {
    music::set_sound(music::BRITSIDE, false);
    display::draw(&images::GAME_OVER, 0, 0);
    display::update_display();
    ubit::sleep(0.5);

    for x in 0..(score+1) {
        display::draw_num_left(x, 81, 5);
        display::update_display();
    }

    const ANIM_TIME:f32 = 0.0001;
    for dx in 0..33 {
        display::set_pixel(true, 64-dx, 16);
        display::set_pixel(true, 64+dx, 16);
        display::set_pixel(true, 64-dx, 24);
        display::set_pixel(true, 65-dx, 25);
        display::set_pixel(true, 64+dx, 24);
        display::set_pixel(true, 65+dx, 25);
        display::update_display();
        ubit::sleep(ANIM_TIME);
    }
    for dy in 0..5 {
        display::set_pixel(true, 31, 16 + dy);
        display::set_pixel(true, 31, 24 - dy);
        display::set_pixel(true, 97, 16 + dy);
        display::set_pixel(true, 97, 24 - dy);
        display::set_pixel(true, 98, 17 + dy);
        display::set_pixel(true, 98, 25 - dy);
        display::update_display();
        ubit::sleep(ANIM_TIME);
    }
    for dx in 33..58 {
        display::set_pixel(true, 64-dx, 20);
        display::set_pixel(true, 64+dx, 20);
        display::update_display();
        ubit::sleep(ANIM_TIME);
    }
    for dy in 20..46 {
        display::set_pixel(true, 6, dy);
        display::set_pixel(true, 122, dy);
        display::update_display();
        ubit::sleep(ANIM_TIME);
    }
    for dx in 0..5 {
        display::set_pixel(true, 6+dx, 45);
        display::set_pixel(true, 121-dx, 45);
        display::update_display();
        ubit::sleep(ANIM_TIME);
    }
    for dy in 0..5 {
        display::set_pixel(true, 11, 44 + dy);
        display::set_pixel(true, 11, 44 - dy);
        display::set_pixel(true, 115, 44 + dy);
        display::set_pixel(true, 115, 44 - dy);
        display::set_pixel(true, 116, 45 + dy);
        display::set_pixel(true, 116, 45 - dy);
        display::update_display();
        ubit::sleep(ANIM_TIME);
    }
    for dx in 0..38 {
        display::set_pixel(true, 11+dx, 39);
        display::set_pixel(true, 115-dx, 39);

        display::set_pixel(true, 11+dx, 49);
        display::set_pixel(true, 115-dx, 49);

        display::set_pixel(true, 12+dx, 50);
        display::set_pixel(true, 116-dx, 50);

        display::update_display();
        ubit::sleep(ANIM_TIME);
    }

    for dy in 0..6 {
        display::set_pixel(true, 49, 39 + dy);
        display::set_pixel(true, 50, 40 + dy);
        display::set_pixel(true, 49, 49 - dy);
        display::set_pixel(true, 50, 50 - dy);
        display::set_pixel(true, 78, 39 + dy);
        display::set_pixel(true, 78, 49 - dy);
        display::update_display();
        ubit::sleep(ANIM_TIME);
    }
    ubit::sleep(1.);
    for y in 0..9 {
        display::draw(&images::RESTART, 4, 64-y);
        display::update_display();
        ubit::sleep(0.1);
    }

    let mut x: i16 = 4;
    loop {
        if BUTTON_A.button_pressed() || BUTTON_B.button_pressed() {
            return
        }
        x -= 1;
        if x == -140 { x += 140}
        display::draw(&images::RESTART, x, 56);
        display::draw(&images::RESTART, x + 140, 56);
        display::update_display();
        ubit::sleep(0.01);
    }

}

#[no_mangle]
pub unsafe extern fn isr_rtc0() {
    nrf51822::rtc::RTC.clear_tick();
    music::tick()
}

#[zinc_main]
fn main() {
    zinc::hal::mem_init::init_stack();
    zinc::hal::mem_init::init_data();
    ubit::init_board();

    use zinc::hal::cortex_m0::{systick,irq,nvic};

    display::init();
    nrf51822::rtc::RTC.set_prescaler(0);
    nrf51822::sleep_us(100000);
    nrf51822::rtc::RTC.enable_tick_iterrupt();
    nvic::enable_irq(11);
    nrf51822::rtc::RTC.start();

    splash();

    music::set_sound(music::SILENCE, true);
    display::fade_to(0, 0.5);
    display::clear();
    display::update_display();
    display::set_brightness(255);
    let score = run_game();
    music::set_sound(music::CRASH, false);

    for _ in 0..2 {
        display::fade_to(1, 0.2);
        display::fade_to(255, 0.2);
    }
    display::clear();

    game_over(score);
}
