
use core::option::Option::{self, Some};

extern {
  fn isr_power_clock();
  fn isr_radio();
  fn isr_uart0();
  fn isr_spi_twi0();
  fn isr_spi_twi1();
  fn isr_gpiote();
  fn isr_adc();
  fn isr_timer0();
  fn isr_timer1();
  fn isr_timer2();
  fn isr_rtc0();
  fn isr_temp();
  fn isr_rng();
  fn isr_ecb();
  fn isr_ccm_aar();
  fn isr_wdt();
  fn isr_rtc1();
  fn isr_qdec();
}

const ISR_COUNT: usize = 18;


#[allow(non_upper_case_globals)]
#[link_section=".isr_vector_nvic"]
#[no_mangle]
pub static NVICVectors: [Option<unsafe extern fn()>; ISR_COUNT+1] = [
  None,
  Some(isr_power_clock),
  Some(isr_radio),
  Some(isr_uart0),
  Some(isr_spi_twi0),
  Some(isr_spi_twi1),
  Some(isr_gpiote),
  Some(isr_adc),
  Some(isr_timer0),
  Some(isr_timer1),
  Some(isr_timer2),
  Some(isr_rtc0),
  Some(isr_temp),
  Some(isr_rng),
  Some(isr_ecb),
  Some(isr_ccm_aar),
  Some(isr_wdt),
  Some(isr_rtc1),
  Some(isr_qdec),
];