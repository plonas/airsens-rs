use chrono::{DateTime, Utc};

use crate::libs::sht45::SHT45Driver;
use crate::libs::storage::CsvWriter;
use sensor_temp_humidity_sht40::I2CAddr;
use sensor_temp_humidity_sht40::Precision;
use sensor_temp_humidity_sht40::TempUnit;

// constant value of one minute
const DATA_READ_INTERVAL: u64 = 60;

// struct containing the processed data from the sensor
struct AirsenseData {
    time: DateTime<Utc>,
    temperature: f32,
    humidity: f32,
}

pub fn run() {
    println!("Airsense is running");

    let mut writer = CsvWriter::new("airsens.csv", &["Temperature", "Humidity"]).unwrap();
    let mut sht45 = SHT45Driver::new(
        I2CAddr::SHT4x_A,
        Precision::High,
        TempUnit::MilliDegreesCelsius,
    );

    loop {
        let data = read_data(&mut sht45);
        store_data(&mut writer, data);

        std::thread::sleep(std::time::Duration::from_secs(DATA_READ_INTERVAL));
    }
}

fn read_data(sth45: &mut SHT45Driver) -> AirsenseData {
    let (t, h) = sth45.read_temperature_and_humidity().unwrap_or_default();
    AirsenseData {
        time: Utc::now(),
        temperature: t,
        humidity: h,
    }
}

fn store_data(writer: &mut CsvWriter, data: AirsenseData) {
    writer
        .write(&data.time, &[data.temperature, data.humidity])
        .unwrap();
    // println!("Date: {} Time: {} Temperature: {:.1}°C, Humidity: {:.1}%", data.time.format("%Y-%m-%d").to_string(), data.time.format("%H:%M:%S").to_string(), data.temperature, data.humidity);
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_read_data() {
    //     // set what values will be returned by the read_data function
    //     let data = read_data();

    //     assert_eq!(data.temperature, 0);
    //     assert_eq!(data.humidity, 0);
    // }

    #[test]
    fn test_store_data() {
        let mut writer = CsvWriter::new("airsens.csv", &["T", "H"]).unwrap();
        let data = AirsenseData {
            time: Utc::now(),
            temperature: 0.0,
            humidity: 0.0,
        };

        store_data(&mut writer, data);
    }

    #[test]
    fn test_run() {
        run();
    }
}
