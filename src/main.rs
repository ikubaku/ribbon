#![no_std]
#![no_main]

use panic_semihosting as _;
use cortex_m_rt as rt;
use stm32f1xx_hal as hal;

use cortex_m_rt::{ExceptionFrame, exception, entry};

use embedded_graphics::prelude::*;
use embedded_graphics::fonts::Font6x8;

use hal::prelude::*;
use hal::stm32;
use hal::i2c::{BlockingI2c, DutyCycle, Mode};

use shared_bus::CortexMBusManager;

use sh1106::prelude::*;
use sh1106::Builder;

mod at8563;
use at8563::At8563;

mod bk1080;
use bk1080::Bk1080;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 300_000,
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        &mut rcc.apb1,
        1000,
        10,
        1000,
        1000,
    );

    let manager = CortexMBusManager::new(i2c);

    let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(manager.acquire()).into();
    let mut rtc: At8563<_> = At8563::new(manager.acquire()).into();
    let mut tuner: Bk1080<_> = Bk1080::new(manager.acquire()).into();

    disp.init().unwrap();
    disp.flush().unwrap();

    rtc.init();
    rtc.enable_clkout();

    tuner.init();
    tuner.start_tuning(835);

    disp.draw(
        Font6x8::render_str("Hello, world!")
            .with_stroke(Some(1u8.into()))
            .translate(Coord::new(1, 1))
            .into_iter(),
    );

    disp.flush().unwrap();

    loop {}
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("FATAL: HardFault: {:#?}", ef);

    loop {}
}
