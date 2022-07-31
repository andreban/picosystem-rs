#![no_std]

pub extern crate rp2040_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;
pub use hal::pac;

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
#[cfg(feature = "boot2")]
#[link_section = ".boot2"]
#[no_mangle]
#[used]
pub static BOOT2_FIRMWARE: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000; // 12 MHz

hal::bsp_pins!(
    Gpio0 {
        name: uart_tx,
        aliases: { FunctionUart: UartTx }
    },
    Gpio1 {
        name: uart_rx,
        aliases: { FunctionUart: UartRx }
    },
    Gpio2 { name: vbus_detect },
    Gpio4 { name: lcd_reset },
    Gpio5 { 
        name: lcd_cs
        aliases: { FunctionSpi: LcdCs }
    },
    Gpio6 { 
        name: lcd_sclk
        aliases: { FunctionSpi: Sclk }
    },
    Gpio7 { 
        name: lcd_mosi, // TX
        aliases: { FunctionSpi: Mosi}
    }, 
    Gpio8 { name: lcd_vsync },
    Gpio9 { name: lcd_dc},
    Gpio11 {
        name: audio,
        aliases: { FunctionPwm: Buzzer}
    }
    Gpio12 {
        name: lcd_backlight,
        aliases: { FunctionPwm: LcdBacklight }
    }
    Gpio13 { name: led_g },
    Gpio14 { name: led_r },
    Gpio15 { name: led_b },
    Gpio16 { name: button_y},
    Gpio17 { name: button_x},
    Gpio18 { name: button_a},
    Gpio19 { name: button_b},
    Gpio20 { name: button_down},
    Gpio21 { name: button_right},
    Gpio22 { name: button_left},
    Gpio23 { name: button_up},
);
