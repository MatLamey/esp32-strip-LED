
use std::f32::consts::PI;


use ws2812_esp32_rmt_driver::driver::color::LedPixelColorGrbw32;
use ws2812_esp32_rmt_driver::{LedPixelEsp32Rmt, RGBW8};
use smart_leds_trait::{SmartLedsWrite, White};
use libm::{expf,powf};

const LED_STRIDE:f32=1000.0/60.0; //m

pub struct StripLED<'a>{
    pub length:usize,
    pub counter:usize,
    pub ws2812: LedPixelEsp32Rmt<'a, smart_leds_trait::RGBA<u8, White<u8>>, ws2812_esp32_rmt_driver::driver::color::LedPixelColorImpl<4, 1, 0, 2, 3>>,
}

impl<'a> StripLED<'a> {
    

    pub const LENGTH_STRIP:usize=42;
    pub const MAX_LUMINOSITY:u8=u8::MAX;
    // pub const UPDATE_PERIOD:u64=50;

    pub fn update(&mut self){

        self.counter=self.counter.wrapping_add(1);
        let pixels: std::iter::Take<std::iter::Repeat<smart_leds_trait::RGBA<u8, White<u8>>>>= std::iter::repeat(RGBW8::from((Self::MAX_LUMINOSITY, 0, 0, White(0)))).take(self.counter);
        self.ws2812.write(pixels).unwrap();
        println!("{}",self.counter);
    }
fn luminosity(){

}
fn color(){}
}
fn gaussian(mean:f32,sigma:f32,input:f32)->f32{
    (1.0/(sigma*powf(2.0*PI, 0.5)))*expf(-powf(input-mean, 2.0)/2.0*powf(sigma, 2.0))
}