#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_nrf::gpio::{Level, Output, OutputDrive, Input, Pull};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let mut led1 = Output::new(p.P0_13, Level::High, OutputDrive::Standard);
    let mut button1 = Input::new(p.P0_11, Pull::Up);

    defmt::info!("Starting loop");
    loop {
        button1.wait_for_any_edge().await;
        let level = button1.get_level();
        defmt::info!("Setting to level {:?}", level);
        led1.set_level(level);
    }
}