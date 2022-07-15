#![no_std]
#![no_main]

use embedded_graphics::Drawable;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::prelude::{Point, Size};
use embedded_graphics::primitives::{Rectangle, StyledDrawable, PrimitiveStyleBuilder};
use embedded_graphics::text::Text;

// PWM trait;
use embedded_hal::PwmPin;

// GPIO traits
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::digital::v2::InputPin;

use embedded_hal::spi::MODE_0;
// A few traits required for using the CountDown timer
use embedded_hal::timer::CountDown;
use embedded_time::duration::Extensions;
use embedded_time::rate::Extensions as RateExtensions;
use embedded_time::fixed_point::FixedPoint;

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::{Rgb565, RgbColor};

use hal::gpio::{FunctionSpi};
// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// The macro for our start-up function
use picosystem_rs::entry;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use picosystem_rs::hal;

// A few traits required for using the CountDown timer
use picosystem_rs::hal::pac;
use picosystem_rs::hal::Clock;
use picosystem_rs::hal::Timer;
use picosystem_rs::hal::spi::Spi;

use display_interface_spi::SPIInterface;
use st7789::ST7789;
use cortex_m::delay::Delay;

#[entry]
fn main() -> ! {

    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        picosystem_rs::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = picosystem_rs::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Configure the timer peripheral to be a CountDown timer for our blinky delay
    let timer = Timer::new(pac.TIMER, &mut pac.RESETS);
    let mut delay = timer.count_down();

    // Set the LED to be an output
    let mut led_green = pins.led_g.into_push_pull_output();
    let mut led_red = pins.led_r.into_push_pull_output();
    let mut led_blue = pins.led_b.into_push_pull_output();

    // Ensure red and blue are low.
    led_red.set_low().unwrap();
    led_blue.set_low().unwrap();

    
    // Configure ST7789
    let lcd_dc = pins.lcd_dc.into_push_pull_output();
    let lcd_cs = pins.lcd_cs.into_push_pull_output();
    let lcd_reset = pins.lcd_reset.into_push_pull_output();

    pins.lcd_sclk.into_mode::<FunctionSpi>();
    pins.lcd_mosi.into_mode::<FunctionSpi>();

    let mut lcd_backlight = pins.lcd_backlight.into_push_pull_output();
    lcd_backlight.set_high().unwrap();

    let spi_screen = Spi::<_, _, 8>::new(pac.SPI0).init(
        &mut pac.RESETS,
        125_000_000u32.Hz(),
        16_000_000u32.Hz(),
        &MODE_0,
    );

    let lcd_spi_interface = SPIInterface::new(spi_screen, lcd_dc, lcd_cs);

    let mut display = ST7789::new(
        lcd_spi_interface,
        lcd_reset,
        240,
        240
    );

    let mut lcd_delay = Delay::new(core.SYST, clocks.system_clock.freq().integer());
    display.init(&mut lcd_delay).unwrap();
    display.clear(Rgb565::RED).unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(Rgb565::WHITE)
        .build();
    let text = Text::new("Hello, World!", Point::new(80, 120), text_style);
    text.draw(&mut display).unwrap();

    let rect_style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::MAGENTA)
        .build();

    let rect = Rectangle::new(
        Point::new(0, 0),
        Size::new(10, 10)
    );

    // Setup buttons.
    let button_x = pins.button_x.into_pull_down_input();
    let button_y = pins.button_y.into_pull_down_input();
    let button_a = pins.button_a.into_pull_down_input();
    let button_b = pins.button_b.into_pull_down_input();

    // Buzzer
    let pwm_slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);
    let mut buzzer = pwm_slices.pwm5;
    buzzer.set_ph_correct();
    buzzer.enable();

    let channel = &mut buzzer.channel_b;
    channel.output_to(pins.audio);
    channel.set_duty(0);

    led_red.set_low().unwrap();
    led_green.set_low().unwrap();
    led_blue.set_low().unwrap();
 
    rect.draw_styled(&rect_style, &mut display).unwrap();
    loop {
        if button_x.is_high().unwrap() {
            led_green.set_low().unwrap();  
        } else {
            led_green.set_high().unwrap();
        }

        if button_y.is_high().unwrap() {
            led_red.set_low().unwrap();  
        } else {
            led_red.set_high().unwrap();
        }

        if button_a.is_high().unwrap() {
            led_blue.set_low().unwrap();  
        } else {
            led_blue.set_high().unwrap();
        }

        if button_b.is_low().unwrap() {
            channel.set_duty(440);
            delay.start(200.milliseconds());
            let _ = nb::block!(delay.wait());
            channel.set_duty(0);
        }

        delay.start(16.milliseconds());
        let _ = nb::block!(delay.wait());
    }
}
