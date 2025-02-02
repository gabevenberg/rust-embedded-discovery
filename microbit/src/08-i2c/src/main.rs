#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use microbit::hal::prelude::*;

#[cfg(feature = "v1")]
use microbit::{hal::twi, pac::twi0::frequency::FREQUENCY_A};

#[cfg(feature = "v2")]
use microbit::{hal::twim, pac::twim0::frequency::FREQUENCY_A};

use lsm303agr::{AccelOutputDataRate, Lsm303agr, MagOutputDataRate};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    #[cfg(feature = "v1")]
    let i2c = { twi::Twi::new(board.TWI0, board.i2c.into(), FREQUENCY_A::K100) };

    #[cfg(feature = "v2")]
    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz10).unwrap();
    sensor.set_mag_odr(MagOutputDataRate::Hz10).unwrap();
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();
    loop {
        if sensor.accel_status().unwrap().xyz_new_data && sensor.mag_status().unwrap().xyz_new_data
        {
            let acceldata = sensor.accel_data().unwrap();
            rprintln!(
                "Acceleration: x {:>6} y {:>6} z {:>6}",
                acceldata.x,
                acceldata.y,
                acceldata.z
            );
            let magdata = sensor.mag_data().unwrap();
            rprintln!(
                "Magnometer  : x {:>6} y {:>6} z {:>6}",
                magdata.x,
                magdata.y,
                magdata.z
            );
        }
    }
}
