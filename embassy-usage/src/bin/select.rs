#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_futures::select::{select, Either};
use embassy_nrf::gpio::{Level, Output, OutputDrive, Input, Pull, Pin};
use {defmt_rtt as _, panic_probe as _};

async fn button_led<B: Pin, L: Pin>(button: &mut Input<'_, B>, led: &mut Output<'_, L>) -> Level {
    button.wait_for_any_edge().await;
    let level = button.get_level();
    led.set_level(level);
    level
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let mut led1 = Output::new(p.P0_13, Level::High, OutputDrive::Standard);
    let mut led2 = Output::new(p.P0_14, Level::High, OutputDrive::Standard);
    let mut button1 = Input::new(p.P0_11, Pull::Up);
    let mut button2 = Input::new(p.P0_12, Pull::Up);

    defmt::info!("Starting loop");
    loop {
        // Note that we don't await here
        let fut1 = button_led(&mut button1, &mut led1);
        let fut2 = button_led(&mut button2, &mut led2);

        match select(fut1, fut2).await {
            Either::First(level) => {
                defmt::info!("Setting led1 to level {:?}", level);
            },
            Either::Second(level) => {
                defmt::info!("Setting led2 to level {:?}", level);
            }
        }
    }
}