pub mod photoresistor {

    use std::{fs::File, io::Read};

    pub fn analog_reader(pin: u8) -> u16 {
        let analog_pin = format!("/sys/devices/ocp.3/helper.12/AIN{}", pin);

        let mut buf = String::new();

        let mut val = match File::open(&analog_pin) {
            Ok(mut file) => match file.read_to_string(&mut buf) {
                Ok(_) => match buf.trim().parse::<u16>() {
                    Ok(numero) => numero,
                    Err(e) => panic!("C'è stato un errore nella conversione del numero {}", e),
                },
                Err(e) => panic!("C'è stato un errore nella letture del valore {}", e),
            },

            Err(e) => panic!("C'è stato un errore nella lettura del file {}", e),
        };

        val
    }
}
