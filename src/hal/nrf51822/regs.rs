use volatile_cell::VolatileCell;
use core::ops::Drop;

ioregs! (GPIO @ 0x50000000 = {  //! power management unit
  0x504 => reg32 out { 31..0 => pins }, // write to gpio
  0x508 => reg32 outset { 31..0 => pins }, // set individual out pins to high
  0x50c => reg32 outclr { 31..0 => pins } , // set individual out pins to low
  0x510 => reg32 in { 31..0 => pins } , // read pin values
  0x514 => reg32 dir { 31..0 => pins } , // set direction of all pins (1=output, 0=input)
  0x518 => reg32 dirset { 31..0 => pins } , // set individual pins to output
  0x51c => reg32 dirclr { 31..0 => pins } , // set individual pins to input
  0x700 => reg32 pin_config[32] {
    0 => dir,
    1 => input,
    3..2 => pull,
    10..8 => drive,
    17..16 => sense,
  },
});

ioregs! (CLOCK @ 0x40000000 = { // Clock interface
  // Tasks
  0x000 => reg32 hfclkstart { 0 => trigger }, //  Start HFCLK crystal oscillator
  0x004 => reg32 hfclkstop { 0 => trigger }, //  Stop HFCLK crystal oscillator
  0x008 => reg32 lfclkstart { 0 => trigger }, //  Start LFCLK source
  0x00C => reg32 lfclkstop { 0 => trigger }, //  Stop LFCLK source
  0x010 => reg32 cal { 0 => trigger }, //  Start calibration of LFCLK RC oscillator
  0x014 => reg32 ctstart { 0 => trigger }, //  Start calibration timer
  0x018 => reg32 ctstop { 0 => trigger }, //  Stop calibration timer
  // Events
  0x100 => reg32 hfclkstarted { 0 => set }, //  HFCLK oscillator started
  0x104 => reg32 lfclkstarted { 0 => set }, //  LFCLK started
  0x10C => reg32 done { 0 => set }, //  Calibration of LFCLK RC oscillator complete event
  0x110 => reg32 ctto { 0 => set }, //  Calibration timer timeout
  // Registers
  0x304 => reg32 intenset { //  Enable interrupt
    0 => hfclkstarted,
    1 => lfclkstarted,
    3 => done,
    4 => ctto,
  },
  0x308 => reg32 intenclr { //  Disable interrupt
    0 => hfclkstarted,
    1 => lfclkstarted,
    3 => done,
    4 => ctto,
  },
  0x408 => reg32 hfclkrun { //  Status indicating that HFCLKSTART task has been triggered
    0 => hfclkstarted,
    1 => lfclkstarted,
    3 => done,
    4 => ctto,
  },
  0x40C => reg32 hfclkstat { //  Which HFCLK source is running
    0 => running
  },
  0x414 => reg32 lfclkrun { //  Status indicating that LFCLKSTART task has been triggered
    0 => running
  },
  0x418 => reg32 lfclkstat { //  Which LFCLK source is running
    1..0 => src,
    16 => state,
  },
  0x41C => reg32 lfclksrccopy { //  Copy of LFCLKSRC register, set when LFCLKSTART task was triggered
    1..0 => src,
  },
  0x518 => reg32 lfclksrc { //  Clock source for the LFCLK
    1..0 => src,
  },
  0x538 => reg32 ctiv { //  Calibration timer interval
    6..0 => value,
  },
  0x550 => reg32 xtalfreq { //  Crystal frequency
    7..0 => value
  },
});

ioregs! (RNG @ 0x4000D000 = {

  0x00 => reg32 start { 0 => trigger },
  0x04 => reg32 stop { 0 => trigger },

  0x100 => reg32 val_ready { 0 => set },

  0x200 => reg32 shorts { //  Shortcut register
    0 => val_ready_stop_shortcut,
  },
  0x304 => reg32 intenset { //  Interrupt enable set register
    0 => val_ready,
  },
  0x308 => reg32 intenclr { //  Interrupt enable clear register
    0 => val_ready,
  },

  0x504 => reg32 config {
    0 => error_correction
  },

  0x508 => reg32 value {
    7..0 => value
  },

});

ioregs! (UART @ 0x40002000 = {  //! Universal asynchronous receiver       transmitter
  0x00 => reg32 start_rx { 0 => trigger },
  0x04 => reg32 stop_rx { 0 => trigger },
  0x08 => reg32 start_tx { 0 => trigger },
  0x0C => reg32 stop_tx { 0 => trigger },

  0x108 => reg32 rxd_ready { 0 => set },
  0x11c => reg32 txd_ready { 0 => set },
  0x124 => reg32 error { 0 => set },

  0x304 => reg32 interrupt_enable_set { 0 => set },
  0x308 => reg32 interrupt_enable_clear { 0 => set },

  0x480 => reg32 error_source {
    0 => overrun,
    1 => bad_parity,
    2 => bad_framing,
    3 => break_error,
  },
  0x500 => reg32 enable {
     2 => enable,
  },
  0x508 => reg32 rts_pin { 31..0 => pin },
  0x50c => reg32 txd_pin { 31..0 => pin },
  0x510 => reg32 cts_pin { 31..0 => pin },
  0x514 => reg32 rxd_pin { 31..0 => pin },
  0x518 => reg32 rxd { 7..0 => data },
  0x51C => reg32 txd { 7..0 => data }
  0x524 => reg32 baud_rate { 31..0 => baud },
  0x56C => reg32 config {
    0 => hardware_flow_control,
    1 => parity_bit,
    2 => rx_parity_source,
    3 => tx_parity_source,
  },

});

ioregs! (RTC @ 0x4000B000 = {
  0x00 => reg32 start { 0 => trigger },
  0x04 => reg32 stop { 0 => trigger },
  0x08 => reg32 clear { 0 => trigger },
  0x0C => reg32 trigovrflw { 0 => trigger },

  0x100 => reg32 tick { 0 => set },
  0x104 => reg32 ovrflw { 0 => set },
  0x140 => reg32 compare0 { 0 => set },
  0x144 => reg32 compare1 { 0 => set },
  0x148 => reg32 compare2 { 0 => set },
  0x14C => reg32 compare3 { 0 => set },

  0x300 => reg32 inten {  // Configures which events shall generate a RTC interrupt
    0 => tick,
    1 => ovrflw,
    16 => compare0,
    17 => compare1,
    18 => compare2,
    19 => compare3,
  },
  0x304 => reg32 intenset {  // Configures which events shall generate a RTC interrupt
    0 => tick,
    1 => ovrflw,
    16 => compare0,
    17 => compare1,
    18 => compare2,
    19 => compare3,
  },
  0x308 => reg32 intenclr {  // configures which events shall not generate a rtc interrupt
    0 => tick,
    1 => ovrflw,
    16 => compare0,
    17 => compare1,
    18 => compare2,
    19 => compare3,
  },
  0x340 => reg32 evten {  // configures event enable state for each rtc event
    0 => tick,
    1 => ovrflw,
    16 => compare0,
    17 => compare1,
    18 => compare2,
    19 => compare3,
  },
  0x344 => reg32 evtenset {  // enable event(s). read of this register gives the value of evten.
    0 => tick,
    1 => ovrflw,
    16 => compare0,
    17 => compare1,
    18 => compare2,
    19 => compare3,
  },
  0x348 => reg32 evtenclr {  // disable event(s). read of this register gives the value of evten.
    0 => tick,
    1 => ovrflw,
    16 => compare0,
    17 => compare1,
    18 => compare2,
    19 => compare3,
  },
  0x504 => reg32 counter {  // current counter value
    23..0 => value,
  },
  0x508 => reg32 prescaler {  // 12-bit prescaler for counter frequency (32768/(prescaler+1)) must be written when rtc is stopped
    11..0 => value,
  },
  0x540 => reg32 cc0 {  // compare register
    31..0 => value
  },
  0x544 => reg32 cc1 {  // compare register
    23..0 => value
  },
  0x548 => reg32 cc2 {  // compare register
    23..0 => value
  },
  0x54C => reg32 cc3 {  // compare register
    23..0 => value
  },
  0xffc => reg32 power {
    0 => on
  }
});

ioregs! (TWI @ 0x40003000 = {
  0x000 => reg32 startrx { 0 => trigger }, //  Start TWI receive sequence
  0x008 => reg32 starttx { 0 => trigger }, //  Start TWI transmit sequence
  0x014 => reg32 stop { 0 => trigger }, //  Stop TWI transaction
  0x01C => reg32 suspend { 0 => trigger }, //  Suspend TWI transaction
  0x020 => reg32 resume { 0 => trigger }, //  Resume TWI transaction

  0x104 => reg32 stopped { 0 => set }, //  TWI stopped
  0x108 => reg32 rxdrdy { 0 => set }, //  TWI RXD byte received
  0x11C => reg32 txdsent { 0 => set }, //  TWI TXD byte sent
  0x124 => reg32 error { 0 => set }, //  TWI error
  0x138 => reg32 bb { 0 => set }, //  TWI byte boundary, generated before each byte that is sent or received

  0x200 => reg32 shorts { //  Shortcut register
    0 => bb_suspend_shortcut,
    1 => bb_stop_shortcut,
  },
  0x304 => reg32 intenset { //  Interrupt enable set register
    1 => stopped,
    2 => rxrdy,
    7 => txdsent,
    9 => error,
    14 => bb
  },
  0x308 => reg32 intenclr { //  Interrupt enable clear register
    1 => stopped,
    2 => rxrdy,
    7 => txdsent,
    9 => error,
    14 => bb
  },
  0x4C4 => reg32 errorsrc { //  TWI error source
    1 => anack,  // Error after sending address
    2 => dnack,  // Error after sending data
  },
  0x500 => reg32 enable { //  Enable TWI master
    2..0 => value {
      0 => disable,
      0x5 => enable,
    }
  },
  0x508 => reg32 scl_pin { //  Pin select for SCL
    31..0 => pin,
  },
  0x50C => reg32 sda_pin { //  Pin select for SDA
    31..0 => pin,
  },
  0x518 => reg32 rxd { //  RXD register
    7..0 => data,
  },
  0x51C => reg32 txd { //  TXD register
    7..0 => data,
  },
  0x524 => reg32 frequency { //  TWI frequency
    31..0 => value,
  },
  0x588 => reg32 address { //  Address used in the TWI transfer
    6..0 => value
  },
  0xffc => reg32 power {
    0 => on
  }

});