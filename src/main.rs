#[macro_use]
extern crate log;
extern crate log4rs;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

use neon_core;

fn main() {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d}:{l} - {m}\n")))
        .build("log/log.log").unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
                .appender("logfile")
                .build(LevelFilter::Info)).unwrap();
    log4rs::init_config(config).unwrap();

    info!("Welcome to the neon! :)");

    let mut core = neon_core::core::Core::new();
    core.run();

    info!("Goodbye!");
}
