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
            addr: 0x51,
        }
    }

    fn send_data(&mut self, payload: &[u8]) {
        self.i2c.write(self.addr, payload).unwrap_or_else(|err| panic!("AT8563: Driver fatal error: {:#?}", err));
    }

    fn write_register(&mut self, addr: u8, data: u8) {
        let payload = [addr, data];

        self.send_data(&payload);
    }

    pub fn init(&mut self) {
        self.write_register(0x02, 0x00);
    }

    pub fn enable_clkout(&mut self) {
        self.write_register(0x0D, 0x80);
    }
}
