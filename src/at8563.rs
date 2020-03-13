pub struct At8563<I2C> {
    i2c: I2C,
    addr: u8,
}

impl At8563<I2C> {
    pub fn new(i2c: I2C) -> At8563<I2C>
    where I2C: embedded_hal::blocking::i2c::Write {
        At8563 {
            i2c: i2c,
            addr: 0xA2,
        }
    }
}
