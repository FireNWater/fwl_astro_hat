
use std::error::Error;
use std::{thread, u8};
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

struct  DCPorts {
    port_1: u8,
    port_2: u8,
    port_3: u8,
    port_extra: u8,
}

impl DCPorts {
    fn turn_on (&self, port: u8) -> Result<(), Box<dyn Error>> {
        // need to figure out how to pass the individual port number into self
        let mut pin = Gpio::new()?.get(port)?.into_output();
        pin.set_high();
        Ok(())
    }

    fn turn_off (&self, port: u8) -> Result<(), Box<dyn Error>> {
        // need to figure out how to pass the individual port number into self
        let mut pin = Gpio::new()?.get(port)?.into_output();
        pin.set_low();
        Ok(())
    }
}

struct DewHeaters {
    heater_1: u8,
    heater_2: u8,
}

impl DewHeaters {
    fn set_power (&self, power: u8) {
        // code to set hardware PWM from 0-100% cycle at 10Hz
    }
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

dc.turn_on(dc.port_1)?;
delay(500);
dc.turn_off(dc.port_1)?;

dc.turn_on(dc.port_2)?;
delay(500);
dc.turn_off(dc.port_2)?;

dc.turn_on(dc.port_3)?;
delay(500);
dc.turn_off(dc.port_3)?;

dc.turn_on(dc.port_extra)?;
delay(500);
dc.turn_off(dc.port_extra)?;


//     let check_pins =  [dc.port_1, 
//                                 dc.port_2, 
//                                 dc.port_3, 
//                                 dc.port_extra, 
//                                 dew_heaters.heater_1, 
//                                 dew_heaters.heater_2];

// // Cycle thru the ports to make sure they work.. 

//     for pin in check_pins {

//     println!("Blinking LED {} on a {}.", pin, DeviceInfo::new()?.model());
//     let mut pin = Gpio::new()?.get(dc.port_3)?.into_output();
//     // Blink the LED by setting the pin's logic level high for 500 ms.
//     pin.set_high();
//     thread::sleep(Duration::from_millis(500));
//     pin.set_low();

//     }
        
    Ok(())
}
//////////////////////////////////////////////////////////////////////////////////

fn delay(millisec: u64) {
    thread::sleep(Duration::from_millis(millisec));
}