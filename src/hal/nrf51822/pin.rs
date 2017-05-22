// Zinc, the bare metal stack for rust.
// Copyright 2014 Dzmitry "kvark" Malyshau <kvarkus@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Pin configuration for ST STM32F1.
//!
//! Some pins that could be configured here may be missing from actual MCU
//! depending on the package.

use super::regs;
pub use ::hal::pin::{GpioLevel, GpioDirection};

/// Pin configuration.
#[derive(Clone, Copy)]
pub struct Pin {
  /// Pin index.
  pub index: u8,
}

pub enum PinDriveMode {
  Standard,
  HighDrive,
  Disconnect,
}

pub enum PinPull {
  NoPull = 0,
  Low = 1,
  High = 2,
}

pub enum SenseMode {
  Disabled = 0,
  High = 1,
  Low = 2,
}

pub struct PinConfig{
  pub direction: GpioDirection,
  pub pull: PinPull,
  pub input_connect: bool,
  pub sense: SenseMode,
  pub drive_low: PinDriveMode,
  pub drive_high: PinDriveMode,
}

impl Default for PinConfig {
  #[inline]
  fn default() -> PinConfig {
    PinConfig {
      direction: GpioDirection::Out,
      input_connect: false,
      pull: PinPull::NoPull,
      sense: SenseMode::High,
      drive_low: PinDriveMode::Standard,
      drive_high: PinDriveMode::Standard,
    }
  }
}


impl Pin {
  /// Setup the pin.
  #[inline(always)]
  pub fn new(pin_index: u8) -> Pin {
    Pin {
      index: pin_index,
    }
  }

  #[inline]
  pub fn configure(&self, config: PinConfig) {
    use self::PinDriveMode::*;

    let drive_mode = match (config.drive_low, config.drive_high) {
      (Standard, Standard) => 0,
      (HighDrive, Standard) => 1,
      (Standard, HighDrive) => 2,
      (HighDrive, HighDrive) => 3,
      (Disconnect, Standard) => 4,
      (Disconnect, HighDrive) => 5,
      (Standard, Disconnect) => 6,
      (HighDrive, Disconnect) => 7,
      (Disconnect, Disconnect) => 0, // Not valid
    };

    regs::GPIO().pin_config[self.index as usize]
      .set_drive(drive_mode)
      .set_pull(config.pull as u32)
      .set_input(!config.input_connect)
      .set_sense(config.sense as u32)
      .set_dir(match config.direction {GpioDirection::In => false, GpioDirection::Out => true});
  }

}

impl ::hal::pin::Gpio for Pin {
  fn set_high(&self) {
    let bit: u32 = 1 << self.index as usize;
    regs::GPIO().outset.set_pins(bit);
  }

  fn set_low(&self) {
    let bit: u32 = 1 << self.index as usize;
    regs::GPIO().outclr.set_pins(bit);
  }

  fn level(&self) -> ::hal::pin::GpioLevel {
    let bit = 1u32 << (self.index as usize);

    match regs::GPIO().in.pins() & bit {
      0 => ::hal::pin::Low,
      _ => ::hal::pin::High,
    }
  }

  fn set_direction(&self, _new_mode: GpioDirection) {
    self.configure( PinConfig{
      direction: _new_mode,
      pull: match _new_mode {
        GpioDirection::In => PinPull::High,
        GpioDirection::Out => PinPull::NoPull
      },
      input_connect: match _new_mode { GpioDirection::In => true, GpioDirection::Out => false },
      ..Default::default()
    });
  }

}