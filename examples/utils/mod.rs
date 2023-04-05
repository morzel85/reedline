use log::{info, LevelFilter};
use simplelog::{format_description, ConfigBuilder, WriteLogger};
use std::fs::OpenOptions;

pub fn logger_init() {
    let config = ConfigBuilder::new()
        .set_time_format_custom(format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
        ))
        .build();

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("reedline.log")
        .expect("Unable to create/open readline.log file.");

    WriteLogger::init(LevelFilter::Debug, config, file).expect("Unable to initialize logger.");

    info!("############################## START ##############################");
}
