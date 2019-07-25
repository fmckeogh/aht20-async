//! Linux i2c demo

extern crate aht10;
extern crate linux_embedded_hal as hal;

use aht10::*;

use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: {} /dev/i2c-N", args[0]);
        process::exit(1);
    }
    let i2c_dev = hal::I2cdev::new(&args[1]).unwrap();
    let mut dev = AHT10::new(i2c_dev, hal::Delay).unwrap();
    loop {
        let (h, t) = dev.read().unwrap();
        println!(
            "relative humidity={0}%; temperature={1}C",
            h.rh(),
            t.celsius()
        );
    }
}
