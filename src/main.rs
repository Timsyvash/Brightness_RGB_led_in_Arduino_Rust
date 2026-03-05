#![no_std]
#![no_main]

use arduino_hal::simple_pwm::IntoPwmPin;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut timer = arduino_hal::simple_pwm::Timer3Pwm::new(
        dp.TC3,
        arduino_hal::simple_pwm::Prescaler::Prescale64,
    );

    let mut red_led = pins.d5.into_output().into_pwm(&mut timer);
    let mut green_led = pins.d2.into_output().into_pwm(&mut timer);
    let mut blue_led = pins.d3.into_output().into_pwm(&mut timer);

    red_led.enable();
    green_led.enable();
    blue_led.enable();

    loop {
        for i in 0..=255 {
            red_led.set_duty(255 - i);
            green_led.set_duty(255 - i);
            blue_led.set_duty(255 - i);
            arduino_hal::delay_ms(10);
        }

        for i in (0..=255).rev() {
            red_led.set_duty(255 - i);
            green_led.set_duty(255 - i);
            blue_led.set_duty(255 - i);
            arduino_hal::delay_ms(10);
        }
    }
}
