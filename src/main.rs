#![no_std]
#![no_main]
use epd_waveshare::{color::TriColor, epd7in5b_v2::*, prelude::*};

extern crate alloc;
use embedded_graphics::{
    image::Image,
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    prelude::*,
    primitives::{Line, PrimitiveStyle},
    text::{Alignment, Text},
};
use embedded_iconoir::prelude::*;
use esp_backtrace as _;
use esp_println::println;
use hal::{
    clock::ClockControl, delay, peripherals::Peripherals, prelude::*, spi, timer::TimerGroup, Rtc,
    IO,
};

mod allocator;

#[entry]
fn main() -> ! {
    println!("init_heap!");
    allocator::init_heap();
    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let mut clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();
    println!("Hello world!");

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut delay = delay::Delay::new(&clocks);
    println!("delay created");

    let mut spi = spi::Spi::new_no_cs(
        peripherals.SPI2,
        io.pins.gpio31,
        io.pins.gpio35,
        io.pins.gpio37,
        1u32.MHz(),
        spi::SpiMode::Mode0,
        &mut system.peripheral_clock_control,
        &mut clocks,
    );
    println!("spi created");

    let mut epd = Epd7in5::new(
        &mut spi,
        io.pins.gpio34.into_push_pull_output(),
        io.pins.gpio2.into_floating_input(),
        io.pins.gpio0.into_push_pull_output(),
        io.pins.gpio1.into_push_pull_output(),
        &mut delay,
        Some(100_000),
    )
    .unwrap();

    let mut display = Display7in5::default();
    println!("display created");

    let _ = Line::new(Point::new(0, 0), Point::new(200, 200))
        .into_styled(PrimitiveStyle::with_stroke(TriColor::White, 10))
        .draw(&mut display);
    let _ = Line::new(Point::new(200, 0), Point::new(200, 200))
        .into_styled(PrimitiveStyle::with_stroke(TriColor::White, 3))
        .draw(&mut display);
    let _ = Line::new(Point::new(200, 0), Point::new(200, 200))
        .into_styled(PrimitiveStyle::with_stroke(TriColor::Chromatic, 1))
        .draw(&mut display);

    let character_style = MonoTextStyle::new(&FONT_6X10, TriColor::White);
    let text = "Tainha";
    Text::with_alignment(
        text,
        display.bounding_box().center() + Point::new(250, 15),
        character_style,
        Alignment::Center,
    )
    .draw(&mut display)
    .unwrap();

    let icon = icons::size24px::actions::Download::new(TriColor::Chromatic);
    let image = Image::new(&icon, Point::new(250, 15));
    image.draw(&mut display).unwrap();

    // Display updated frame
    epd.update_frame(&mut spi, &display.buffer(), &mut delay)
        .unwrap();
    println!("update frame!");
    epd.display_frame(&mut spi, &mut delay).unwrap();
    println!("display!!");

    // Set the EPD to sleep
    epd.sleep(&mut spi, &mut delay).unwrap();
    println!("done!!!!");
    loop {}
}
