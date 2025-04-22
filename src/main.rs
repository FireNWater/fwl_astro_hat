
use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

struct  DCPorts {
    port_1: u8,
    port_2: u8,
    port_3: u8,
    port_extra: u8,
}

struct DewHeaters {
    heater_1: u8,
    heater_2: u8,
}

fn main() -> Result<(), Box<dyn Error>> {

    // Gpio uses BCM pin numbering. Set pin numbers according to the BCM pinout.
    let dc = DCPorts {
        port_1: 22,
        port_2: 23,
        port_3: 24,
        port_extra: 25,
    };

    let dew_heaters = DewHeaters {
        heater_1: 12,
        heater_2: 13,
    };

    let check_pins =  [dc.port_1, 
                                dc.port_2, 
                                dc.port_3, 
                                dc.port_extra, 
                                dew_heaters.heater_1, 
                                dew_heaters.heater_2];

    for pin in check_pins {

    println!("Blinking LED {} on a {}.", pin, DeviceInfo::new()?.model());
    let mut pin = Gpio::new()?.get(dc.port_3)?.into_output();
    // Blink the LED by setting the pin's logic level high for 500 ms.
    pin.set_high();
    thread::sleep(Duration::from_millis(500));
    pin.set_low();

    }
        
    Ok(())
}
//////////////////////////////////////////////////////////////////////////////////
