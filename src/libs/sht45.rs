use embedded_hal as hal;
use hal::blocking::delay::DelayMs;
use linux_embedded_hal::I2cdev;
use sensor_temp_humidity_sht40::{SHT40Driver, I2CAddr, Precision, TempUnit};
use std::thread::sleep;



pub struct SimpleDelayMs;

impl DelayMs<u16> for SimpleDelayMs {
    fn delay_ms(&mut self, ms: u16) {
        sleep(std::time::Duration::from_millis(ms.into()));
    }
}

pub struct SHT45Driver {
    sht40_driver: SHT40Driver<I2cdev, SimpleDelayMs>,
    precision: Precision,
    temp_unit: TempUnit,
}

impl SHT45Driver {
    pub fn new(i2c_addr: I2CAddr, precision: Precision, temp_unit: TempUnit) -> Self {
        let i2c = I2cdev::new("/dev/i2c-2").unwrap();
        let d = SimpleDelayMs;
        let sht40_driver = SHT40Driver::new(i2c, i2c_addr, d);
        Self { sht40_driver, precision, temp_unit }
    }

    pub fn read_temperature_and_humidity(&mut self) -> Result<(f32, f32), ()> {
        let result = self.sht40_driver.get_temp_and_rh(self.precision, self.temp_unit);
        match result {
            Ok(m) => Ok(((m.temp as f32) / 1000.0, (m.rel_hum_pcm as f32) / 1000.0)),
            // print error message and return error
            Err(e) => {
                println!("Error reading data from SHT45: {:?}", e);
                Err(())
            }
        }
    }
}
