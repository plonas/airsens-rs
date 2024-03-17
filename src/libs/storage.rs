use std::fs::File;
use chrono::{DateTime, Utc};
use csv::Writer;

pub struct CsvWriter {
    writer: Writer<File>,
}

impl CsvWriter {
    pub fn new(file_path: &str, row: &[&str; 2]) -> Result<Self, csv::Error> {
        let file = File::create(file_path)?;
        let mut writer = Writer::from_writer(file);
        let row_w_time = ["Date", "Time", row[0], row[1]];
        let result = writer.write_record(row_w_time);
        match result {
            Ok(_) => {},
            Err(e) => {
                println!("Error writing header to CSV file: {:?}", e);
            }
        }
        Ok(Self { writer })
    }

    pub fn write(&mut self, time: &DateTime<Utc>, row: &[f32; 2]) -> Result<(), csv::Error> {
        let ds = time.format("%Y-%m-%d").to_string();
        let ts = time.format("%H:%M:%S").to_string();
        let t = format!("{:.1}", row[0]);
        let h = format!("{:.1}", row[1]);
        let res_write = self.writer.write_record(&[ds, ts, t, h]);
        match res_write {
            Ok(_) => {},
            Err(e) => {
                println!("Error writing data to CSV file: {:?}", e);
            }
        }
        let res_flush: Result<(), std::io::Error> = self.writer.flush();
        match res_flush {
            Ok(_) => {},
            Err(e) => {
                println!("Error flushing CSV file: {:?}", e);
            }
        }
        Ok(())
    }
}
