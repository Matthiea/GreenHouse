pub mod dht11 {

    use std::{
        thread::sleep,
        time::{Duration, Instant},
    };
    use sysfs_gpio::{Direction, Pin};

    pub fn get_data(pin_num: u64) -> Option<[f32; 2]> {
        let pin = Pin::new(pin_num);
        let data: [f32; 2];
        let raw: Vec<bool>;

        //initialization of the comunication
        pin.with_exported(|| {
            pin.set_direction(Direction::Out).unwrap();

            pin.set_value(0);
            sleep(Duration::from_micros(20));
            pin.set_value(1);
            sleep(Duration::from_micros(40));

            pin.set_direction(Direction::In).unwrap();
            pin.set_edge(sysfs_gpio::Edge::FallingEdge);

            if pin.get_value().unwrap() == 0 {
                sleep(Duration::from_micros(160));
                raw = get_bit(pin);
                data = IEEE_754(raw);
                return Some(data);
            } else {
                return None;
            }
        });
    }

    // function used for get the two f32
    fn IEEE_754(bits: Vec<bool>) -> [f32; 2] {
        let mut temp: f32 = 0.0;
        let mut umidy: f32 = 0.0;

        return [temp, umidy];
    }

    fn get_bit(pin: Pin) -> Vec<bool> {
        let mut bit: Vec<bool>;

        while bit.len() <= 40 {
            let start = Instant::now();

            if start.elapsed() >= Duration::from_micros(50) && pin.get_value().unwrap() == 1 {
                let start_bit = Instant::now();
                if pin.get_value().unwrap() == 1 && start_bit.elapsed() >= Duration::from_micros(70)
                {
                    bit.push(true);
                } else if pin.get_value().unwrap() == 1
                    && start_bit.elapsed() >= Duration::from_micros(26)
                {
                    bit.push(false);
                }
            }
        }
        return bit;
    }
}
