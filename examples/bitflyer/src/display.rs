#![no_builtins]

use zinc::hal::nrf51822::twi;
use super::ubit;
use core::{fmt, mem,num};
use core::cmp::{min,max};
use core::slice::{Iter};
use core::iter::{Map};
use zinc::util::strconv;
use core::option::Option::Some;

pub struct Image {
    pub data: &'static[u8],
    pub width: u8,
    pub height: u8,
}

use images;

struct DisplayBuffer {
    buffer: [u8;1024],
    first_dirty: u8,
    last_dirty: u8,
}

const DISPLAY_ID: u8 = 60;

static mut BUFFER: DisplayBuffer = DisplayBuffer{
    first_dirty: 0,
    last_dirty: 7,
    buffer: images::_SPLASH
};

enum Command {
    Off = 174,
    On = 175,
    MemoryAddressMode = 0x20,
    ColumnAddress = 0x21,
    PageAddress = 0x22,
    BottomToTop = 192,
    TopToBottom = 200,
    RightToLeft = 160,
    LeftToRight = 161,
    ChargePumpOn = 20,
    ChargePumpSet = 141,
    Contrast = 0x81,
}

enum AddressMode {
    Horizontal = 0,
    Vertical = 1,
    Page = 2,
}


macro_rules! send_command {
    ( $($x:expr),+ ) => {
        $(
            twi::TWI.write_bytes(&[128, $x as u8]);
        )*
    }
}

pub mod ops {

    pub trait Op {
        #[inline(always)]
        fn op(  &mut u8, u8);
    }

    pub struct Set {}

    impl Op for Set {
        #[inline(always)]
        fn op(dest: &mut u8, src: u8) {
            *dest = src;
        }
    }

    pub struct Clear {}
    impl Op for Clear {
        #[inline(always)]
        fn op(dest: &mut u8, src: u8) {
            *dest &= !src;
        }
    }

    pub struct Over {}
    impl Op for Over {
        #[inline(always)]
        fn op(dest: &mut u8, src: u8) {
            *dest |= src;
        }
    }


}

pub static mut cur_brightness: u8 = 0;

pub fn set_brightness(brightness: u8) {
    send_command!(Command::Contrast, brightness);
    unsafe{ cur_brightness = brightness; }
}

pub fn fade_to(val: u8, over: f32) {
    let start_val: i16 = unsafe { cur_brightness as i16 };
    let steps = (val as i16 - start_val).abs();
    let per_step_sleep = over / steps as f32;
    let per_step_delta: i16 = ((val as f32 - start_val as f32) / steps as f32) as i16;
    for i in 0..steps {
        set_brightness((start_val + (per_step_delta * i)) as u8);
        ubit::sleep(per_step_sleep)
    }
}

pub fn update_display() {
    let i2c = twi::TWI;
    unsafe{
        if BUFFER.first_dirty > 7 {
            return;
        }
        send_command!(Command::PageAddress, BUFFER.first_dirty, 7);
        let start_index: usize = (BUFFER.first_dirty as usize) * 128;
        let end_index: usize = ((BUFFER.last_dirty as usize) + 1) * 128;
        i2c.write_bytes_prefix(64, &BUFFER.buffer[start_index..end_index]);
        BUFFER.first_dirty = 8;
        BUFFER.last_dirty = 8;
    }
}

pub fn invalidate() {
  unsafe {
    BUFFER.first_dirty = 0;
    BUFFER.last_dirty = 7;
  }
}

fn mark_dirty(row: u8) {
    unsafe {
        BUFFER.first_dirty = min(row, BUFFER.first_dirty);
        BUFFER.last_dirty = if BUFFER.last_dirty > 7 { row } else { max(row, BUFFER.last_dirty) };
    }
}


fn paint_full<O: ops::Op>(img: &Image) {
    unsafe{
        for (dest, src) in BUFFER.buffer.iter_mut().zip(img.data.iter()) {
            O::op(dest, *src);
        }
        invalidate();
    }
}

fn slice_paint<O: ops::Op>(dest: &mut [u8], src: &[u8], shift: i16, dest_index: usize, src_index: usize, len: usize)
{
    let dest_iter = dest[dest_index..dest_index + len].iter_mut();
    let src_iter = src[src_index..src_index + len].iter();
    if shift == 0 {
        for (dest, src) in dest_iter.zip(src_iter) { O::op(dest, *src); }
    } else {
        for (dest, src) in dest_iter.zip(src_iter) {
            let shifted: u8 = if shift < 0 { (*src) >> -shift } else { (*src) << shift };
            O::op(dest, shifted);
        }
    }
}

fn paint_onerow<O: ops::Op>(img: &Image, x: i16, row: u8) {
    let xmin = if x < 0 { 0 } else { x };
    let xmax = if x + (img.width as i16) > 127 {127} else {x + (img.width as i16)};
    let img_start_x: usize = if x < 0 { -x as usize } else { 0 };
    let img_end_x: usize = if x + (img.width as i16) > 127 {127 - x} else {img.width as i16} as usize;

    let row_offset: usize = (row as usize) * 128;
    let start_index: usize = row_offset + (xmin as usize);
    unsafe{
        slice_paint::<O>(&mut BUFFER.buffer, img.data, 0, start_index, img_start_x, (xmax - xmin) as usize);
    }
    mark_dirty(row);
}


fn paint_onerow_misaligned<O: ops::Op>(img: &Image, x: i16, y: i16) {
    let xmin = if x < 0 { 0 } else { x };
    let xmax = if x + (img.width as i16) > 127 {127} else {x + (img.width as i16)};
    let img_start_x: usize = if x < 0 { -x as usize } else { 0 };
    let img_end_x: usize = if x + (img.width as i16) > 127 {127 - x} else {img.width as i16} as usize;

    let first_row: i16 = if y < 0 { -1 } else {y / 8};
    let rem = (y+8) % 8;

    let start_index: usize = (first_row as usize * 128) + (xmin as usize);

    if first_row >= 0 {
        unsafe{
            slice_paint::<O>(&mut BUFFER.buffer, img.data, rem, start_index, img_start_x, (xmax - xmin) as usize);
        }
        mark_dirty(first_row as u8);
    }
    if first_row < 7 {
        unsafe{
            slice_paint::<O>(&mut BUFFER.buffer, img.data, -8 + rem, start_index + 128, img_start_x, (xmax - xmin) as usize);
        }
        mark_dirty((first_row + 1) as u8);
    }
}

pub fn set_pixel(val: bool, x: u8, y: u8) {
    let byte_index = (((y / 8) as usize) * 128) + x as usize;
    let byte_mask = 1 << y % 8;
    unsafe {
        let dest: &mut u8 = &mut BUFFER.buffer[byte_index as usize];
        match (val) {
            true  => *dest |= byte_mask,
            false => *dest &= !byte_mask
        };
    }
    invalidate();
    //mark_dirty(y/8);
}

pub enum Align{
    Left,
    Right,
}

const DIGIT_WIDTH: u16 = 6;
const DIGITS: [&'static Image;10] = [
        &images::N0,
        &images::N1,
        &images::N2,
        &images::N3,
        &images::N4,
        &images::N5,
        &images::N6,
        &images::N7,
        &images::N8,
        &images::N9];

pub fn draw_num_right(mut num: u32,mut x: u16, row: u8) {
    let mut base:u32  = 10;
    while num > 0 {
        let rem = num % base;

        num -= rem;
        let digit = rem / (base / 10);
        x -= DIGIT_WIDTH;
        draw(DIGITS[digit as usize], x as i16, (row * 8) as i16);
        base *= 10;
    }
    mark_dirty(row);
}

pub fn draw_num_left(mut num: u32,mut x: u16, row: u8) {
    let mut digits: [u8;10] = [0;10];
    let mut base:u32 = 10;
    let mut i: usize = 9;
    while num > 0 {
        let rem = num % base;
        num -= rem;
        let digit = rem / (base / 10);
        digits[i] = digit as u8;
        base *= 10;
        i -= 1;
    }
    if (i < 9) { i += 1; }
    for j in i..10 {
        //println!("{}, {}, {}", j, digits[j], x);
        draw(DIGITS[digits[j] as usize], x as i16, (row * 8) as i16);
        x += DIGIT_WIDTH;
    }
    mark_dirty(row);
}

pub fn paint<O: ops::Op>(img: &Image, x: i16, y: i16) {
    if y > 63 || x > 127 || x + (img.width as i16) <= 0 || y + (img.height as i16) <= 0{
        return;
    }
    if x==0 && y==0 && img.width == 128 && img.height == 64 {
        return paint_full::<O>(img)
    }
    if img.height == 8 && y % 8 == 0 && y >= 0 && y < 64 {
        return paint_onerow::<O>(img, x, (y/8) as u8);
    }
    if img.height == 8 {
        return paint_onerow_misaligned::<O>(img, x, y);
    }
    println!("Not implemented");
}

pub fn draw(img: &Image, x: i16, y: i16) {
    paint::<ops::Set>(img, x, y);
}

pub fn blend(img: &Image, x: i16, y: i16) {
    paint::<ops::Over>(img, x, y);
}

pub fn clear_image(img: &Image, x: i16, y: i16) {
    paint::<ops::Clear>(img, x, y);
}


pub fn clear() {
    unsafe{
        for byte in BUFFER.buffer.iter_mut() {
            *byte = 0;
        }
        invalidate();
    }
}

pub fn init() {
    let i2c = twi::TWI;

    i2c.configure(twi::TwiConfig {
        sda_pin: 30,
        scl_pin: 0,
        freq: twi::Frequency::K400,
    });
    ubit::sleep(0.1);
    i2c.set_address(DISPLAY_ID);
    send_command!(Command::Off);
    ubit::sleep(0.01);
    send_command!(Command::On);
    send_command!(Command::TopToBottom, Command::LeftToRight);
    ubit::sleep(0.01);
    send_command!(Command::MemoryAddressMode, AddressMode::Horizontal);
    send_command!(Command::ColumnAddress, 0, 127);
    set_brightness(0);
    update_display();
    send_command!(Command::ChargePumpSet, Command::ChargePumpOn);
}