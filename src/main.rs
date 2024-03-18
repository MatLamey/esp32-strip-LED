

// fn main() {
//     // It is necessary to call this function once. Otherwise some patches to the runtime
//     // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
//     esp_idf_svc::sys::link_patches();

//     // Bind the log crate to the ESP Logging facilities
//     esp_idf_svc::log::EspLogger::initialize_default();

//     loop {
//         log::info!("Hello, world!");
//     }
    
// }


use esp_idf_hal::peripherals::Peripherals;
use ioreal::IoReal;
use smart_leds_trait::{SmartLedsWrite, White};


use std::thread::sleep;
use std::time::Duration;
use ws2812_esp32_rmt_driver::driver::color::LedPixelColorGrbw32;
use ws2812_esp32_rmt_driver::{LedPixelEsp32Rmt, RGBW8};
mod ioreal;


fn main() -> ! {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();


    // let peripherals = Peripherals::take().unwrap();
    // let led_pin = peripherals.pins.gpio1;
    // let channel = peripherals.rmt.channel0;
    // let mut ws2812= LedPixelEsp32Rmt::<RGBW8, LedPixelColorGrbw32>::new(channel, led_pin).unwrap();
    // const LENGTH_STRIP:usize=42;
    // const MAX_LUMINOSITY:u8=u8::MAX;
    // const UPDATE_PERIOD:u64=50;

//     loop {
//         for i in 0..LENGTH_STRIP{
//             let pixels= std::iter::repeat(RGBW8::from((MAX_LUMINOSITY, 0, 0, White(0)))).take(i);
//             ws2812.write(pixels).unwrap();
//             sleep(Duration::from_millis(UPDATE_PERIOD));
//         }
//         let pixels = std::iter::repeat(RGBW8::from((0, 0, 0, White(0)))).take(LENGTH_STRIP);
//         ws2812.write(pixels).unwrap();
//         sleep(Duration::from_millis(UPDATE_PERIOD));
//     }
// }
 let mut io = IoReal::new();
 loop {
     io.update();
     sleep(Duration::from_millis(250));
 }

}

