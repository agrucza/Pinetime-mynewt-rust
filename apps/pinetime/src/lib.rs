#![no_std]

extern "C" {
    fn sysinit_start();
    fn sysinit_app();
    fn sysinit_end();
    fn hal_gpio_init_out(pin: i32, val: i32) -> i32;
    fn hal_gpio_toggle(pin: i32);
    fn os_time_delay(osticks: u32);
}

extern crate panic_halt;

const OS_TICKS_PER_SEC: u32 = 128;

const LCD_BACKLIGHT_LOW_PIN     : i32 = 14;
const LCD_BACKLIGHT_MED_PIN     : i32 = 22;
const LCD_BACKLIGHT_HIGH_PIN    : i32 = 23;
const VIBRATOR_PIN              : i32 = 16;

#[no_mangle]
pub extern "C" fn main() {
    /* Initialize all packages. */
    unsafe { sysinit_start(); }
    unsafe { sysinit_app(); }
    unsafe { sysinit_end(); }

    unsafe { hal_gpio_init_out(LCD_BACKLIGHT_LOW_PIN, 1); }
    unsafe { hal_gpio_init_out(LCD_BACKLIGHT_MED_PIN, 1); }
    unsafe { hal_gpio_init_out(LCD_BACKLIGHT_HIGH_PIN, 1); }

    unsafe { hal_gpio_init_out(VIBRATOR_PIN, 1); }

    loop {
        /* Wait one second */
        unsafe { os_time_delay(OS_TICKS_PER_SEC); }

        /* Toggle the LED */
        unsafe { hal_gpio_toggle(LCD_BACKLIGHT_LOW_PIN); }
        unsafe { hal_gpio_toggle(LCD_BACKLIGHT_MED_PIN); }
        unsafe { hal_gpio_toggle(LCD_BACKLIGHT_HIGH_PIN); }

        unsafe { hal_gpio_toggle(VIBRATOR_PIN); }
    }
}