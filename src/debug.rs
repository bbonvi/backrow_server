use chrono::prelude::Utc;
use env_logger::{Builder, WriteStyle};
use std::io::Write;

pub fn init() {
        let mut builder = Builder::from_default_env();

        builder.format(|buf, record| {
                writeln!(
                        buf,
                        "{:?}|{}|{}|{}",
                        Utc::now(),
                        record.level(),
                        record.target(),
                        record.args()
                )
        })
        .write_style(WriteStyle::Always)
        .init();
}
