#![no_std]
#![no_main]

use panic_semihosting as _;
use cortex_m_rt as rt;
use stm32f1xx_hal as hal;

use cortex_m_rt::{ExceptionFrame, exception, entry};

use embedded_graphics::prelude::*;
use embedded_graphics::fonts::Font6x8;

use embedded_hal::digital::v2::InputPin;

use hal::prelude::*;
use hal::stm32;
use hal::i2c::{BlockingI2c, DutyCycle, Mode};

use shared_bus::CortexMBusManager;

use arrayvec::ArrayString;

use sh1106::prelude::*;
use sh1106::Builder;

mod at8563;
use at8563::At8563;

mod bk1080;
use bk1080::Bk1080;


#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let btn0 = gpiob.pb10.into_pull_up_input(&mut gpiob.crh);
    let btn1 = gpiob.pb5.into_pull_up_input(&mut gpiob.crl);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 100_000.hz(),
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
    tuner.start_tuning(760);

    disp.draw(
        Font6x8::render_str("Hello, world!")
            .with_stroke(Some(1u8.into()))
            .translate(Coord::new(1, 1))
            .into_iter(),
    );

    disp.flush().unwrap();

    let mut freq = 760;
    loop {
        let mut is_freq_changed = false;

        if btn0.is_low().unwrap() {
            freq -= 1;
            is_freq_changed = true;
        }
        if btn1.is_low().unwrap() {
            freq += 1;
            is_freq_changed = true;
        }
        if freq < 760 {
            freq = 760;
        }
        if freq > 1080 {
            freq = 1080;
        }

        if is_freq_changed {
            tuner.start_tuning(freq);
        }

        let mut buf = ArrayString::<[_; 16]>::new();
        core::fmt::write(&mut buf, format_args!("Freq: {}", freq)).unwrap();
        disp.draw(
            Font6x8::render_str(buf.as_str())
                .with_stroke(Some(1u8.into()))
                .translate(Coord::new(1, 3))
                .into_iter(),
        );

        disp.flush().unwrap();

        delay.delay_ms(100 as u32);
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("FATAL: HardFault: {:#?}", ef);
}
