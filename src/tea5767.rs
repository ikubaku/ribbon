use embedded_hal::blocking::i2c::{Read, Write, WriteRead};

pub struct Tea5767<I2C> {
    i2c: I2C,
    addr: u8,
}

const INIT_DATA: [u8; 5] = [
    0x00,
    0x00,
    0x90,
    0x3E,
    0x40,
];

impl<I2C> Tea5767<I2C>
where I2C: Read + Write,
      <I2C as Read>::Error: core::fmt::Debug,
      <I2C as Write>::Error: core::fmt::Debug, {
    pub fn new(i2c: I2C) -> Tea5767<I2C> {
        Tea5767 {
            i2c: i2c,
            addr: 0x60,
        }
    }

    fn read_all(&mut self, buf: &mut [u8; 5]) {
        self.i2c.read(self.addr, buf).unwrap_or_else(|err| panic!("TEA5767: Driver fatal error: {:#?}", err));
    }

    fn write_all(&mut self, buf: &[u8; 5]) {
        self.i2c.write(self.addr, buf).unwrap_or_else(|err| panic!("TEA5767: Driver fatal error: {:#?}", err));
    }

    pub fn init(&mut self) {
        self.write_all(&INIT_DATA);
    }

    pub fn start_tuning(&mut self, freq: u16) {
        let pll_val = 4 * (freq as i32 * 100 - 225) / 32768 * 1000;
        let pll_val = pll_val as u16;
        let payload: [u8; 5] = [(0x3F & (pll_val >> 8)) as u8, (0xFF & pll_val) as u8, INIT_DATA[2], INIT_DATA[3], INIT_DATA[4]];

        self.write_all(&payload);
    }
}
