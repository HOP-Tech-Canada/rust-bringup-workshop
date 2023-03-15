// We don't have an actual main function
#![no_main]
// We don't have the standard library
#![no_std]

use defmt_rtt as _; // global logger
use nrf52840_hal as _; // memory layout
use panic_probe as _; // panic behavior
 
 /// ! means we never exit
#[cortex_m_rt::entry]
fn main() -> ! {
    loop {

        defmt::info!("Hello, World!");
        // Pause the execution
        cortex_m::asm::bkpt();
    }
}