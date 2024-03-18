mod strip_led;

use esp_idf_hal::peripherals::Peripherals;
use smart_leds_trait::{SmartLedsWrite, White};
use ws2812_esp32_rmt_driver::driver::color::LedPixelColorGrbw32;
use ws2812_esp32_rmt_driver::{LedPixelEsp32Rmt, RGBW8};

use self::strip_led::StripLED;

pub struct IoReal<'a>{
    strip_led: strip_led::StripLED<'a>,
} 

impl <'a> IoReal <'a> {
    pub fn new()->Self{
        let peripherals = Peripherals::take().unwrap();
        let led_pin = peripherals.pins.gpio1;
        let channel = peripherals.rmt.channel0;
        Self{
            strip_led: StripLED{
                counter : 0,
                length: StripLED::LENGTH_STRIP,
                ws2812: LedPixelEsp32Rmt::<RGBW8, LedPixelColorGrbw32>::new(channel, led_pin).unwrap(),
            } 
        }
        
    }

    pub fn update(&mut self){
        self.strip_led.update();

    }
    
}