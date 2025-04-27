
use std::error::Error;
use std::{thread, u8};
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;
use rppal::pwm::{Channel, Polarity, Pwm};


fn main() -> Result<(), Box<dyn Error>> {

    // Gpio uses BCM pin numbering. Set pin numbers according to the BCM pinout.
    const DC_PORT_1: u8 = 22;
    const DC_PORT_2: u8 = 23;
    const DC_PORT_3: u8 = 24;
    const DC_PORT_X: u8 = 25;

    const DH_PORT_1: u8 = 12;
    const DH_PORT_2: u8 = 13;

    

    //cycle thru all the GPIO ports
    let mut pin = Gpio::new()?.get(DC_PORT_1)?.into_output();
    pin.set_high();
    println!("Blinking LED {:?} on a {}.", DC_PORT_1, DeviceInfo::new()?.model());
    delay(500);
    pin.set_low();

    let mut pin = Gpio::new()?.get(DC_PORT_2)?.into_output();
    pin.set_high();
    println!("Blinking LED {:?} on a {}.", DC_PORT_2, DeviceInfo::new()?.model());
    delay(500);
    pin.set_low();

    let mut pin = Gpio::new()?.get(DC_PORT_3)?.into_output();
    pin.set_high();
    println!("Blinking LED {:?} on a {}.", DC_PORT_3, DeviceInfo::new()?.model());
    delay(500);
    pin.set_low();

    let mut pin = Gpio::new()?.get(DC_PORT_X)?.into_output();
    pin.set_high();
    println!("Blinking LED {:?} on a {}.", DC_PORT_X, DeviceInfo::new()?.model());
    delay(500);
    pin.set_low();
   
    // PWM pin tests
    println!("Test PWMs");
    let pwm = Pwm::with_frequency(Channel::Pwm0, 2.0, 0.25, Polarity::Normal, true)?;
    // Enable PWM channel 0 (BCM GPIO 12, physical pin 32) at 2 Hz with a 25% duty cycle.
    //let dh1 = Pwm::with_frequency( Channel::Pwm0, 2.0, 0.25, Polarity::Normal, true)?;
    println!("PWM0");
    delay(500);
    //dh1.set_frequency(8.0, 0.5)?;
    pwm.set_frequency(8.0, 0.5)?;
    delay(500);

    // let dh2 = Pwm::with_frequency( dew_heaters.heater_2, 2.0, 0.25, Polarity::Normal, true)?;
    // delay(500);
    // dh2.set_frequency(8.0, 0.5)?;
    // delay(500);


        
    Ok(())          //Return value to OS
}


fn delay(millisec: u64) {
    thread::sleep(Duration::from_millis(millisec));
}