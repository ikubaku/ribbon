use embedded_hal::blocking::i2c::{Write, Read};

pub struct At8563<I2C> {
    i2c: I2C,
    addr: u8,
}

impl<I2C> At8563<I2C>
where I2C: Write + Read,
      <I2C as Write>::Error : core::fmt::Debug,
      <I2C as Read>::Error : core::fmt::Debug {
    pub fn new(i2c: I2C) -> At8563<I2C> {
        At8563 {
            i2c: i2c,
            addr: 0xA2,
        }
    }

    fn send_data(&mut self, payload: &[u8]) {
        self.i2c.write(self.addr, &payload).unwrap_or_else(|err| panic!("AT8563: Driver fatal error: {:#?}", err));
    }

    pub fn init(&mut self) {
        let payload = [0x02, 0x00] as [u8; 2];

        self.send_data(&payload);
    }

    pub fn enable_clkout(&mut self) {
        let payload = [0x0D, 0x80] as [u8; 2];

        self.send_data(&payload);
    }
}
