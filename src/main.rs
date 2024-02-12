#![allow(dead_code)]

mod lib {
    pub mod dht;

    pub mod photoresistor;
}

use std::{thread::sleep, time::Duration};

use lib::dht::dht11::get_data;

fn main() {
    loop {
        let x = get_data(60).unwrap();

        sleep(Duration::from_secs(1));

        let y = x[0];
        let z = x[1];

        println!("temp:{}, humidity: {}", y, z);
    }
}
