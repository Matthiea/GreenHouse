#![allow(unused)]

pub mod dht {

    use std::{thread::sleep, time::Duration};

    use sysfs_gpio::{Direction, Pin};

    pub struct Dht11 {
        pub pin: u8,
    }

    pub trait Define {
        fn new(pin_number: u8) -> Self;
    }

    impl Define for Dht11 {
        fn new(pin_number: u8) -> Self {
            Dht11 { pin: pin_number }
        }
    }

    pub fn get_data(pin_num: u64) -> Option<[f32; 2]> {
        let pin = Pin::new(pin_num);

        let mut data: [f32; 2] = Default::default();
        let mut raw: Vec<bool> = Vec::new();

        // Initialization of the communication
        if let Ok(_) = pin.with_exported(|| {
            pin.set_direction(Direction::Out)?;
            pin.set_value(0);
            sleep(Duration::from_millis(20));
            pin.set_value(1);
            sleep(Duration::from_micros(40));
            pin.set_direction(Direction::In)?;

            if pin.get_value()? == 0 {
                sleep(Duration::from_micros(160));
                raw = get_bit(pin);
                data = ieee_754(raw);
            }

            Ok(())
        }) {
            if !data.is_empty() {
                Some(data)
            } else {
                None
            }
        } else {
            None
        }
    }

    // Function used to get the two f32
    fn ieee_754(mut bits: Vec<bool>) -> [f32; 2] {
        let mut temp: f32 = 0.0;
        let mut umidy: f32 = 0.0;

        loop {
            let mut y: i32 = 8;

            if temp == 0.0 {
                for _i in 0..16 {
                    temp += match bits[0] {
                        true => f32::powi(2.0, y),
                        false => 0.0,
                    };

                    y -= 1;
                    bits.remove(0);
                }
            } else if umidy == 0.0 {
                for _i in 0..16 {
                    umidy += match bits[0] {
                        true => f32::powi(2.0, y),
                        false => 0.0,
                    };

                    y -= 1;
                    bits.remove(0);
                }
            } else {
                break;
            }
        }

        [temp, umidy]
    }

    fn get_bit(pin: Pin) -> Vec<bool> {
        let mut bit: Vec<bool> = Vec::new();

        while bit.len() <= 40 {
            sleep(Duration::from_micros(50));

            if pin.get_value().unwrap_or_default() == 1 {
                sleep(Duration::from_micros(50));

                if pin.get_value().unwrap_or_default() == 0 {
                    bit.push(true);
                } else {
                    bit.push(false);
                }
            }
            sleep(Duration::from_micros(20));
        }

        bit
    }
}
