use std::env;
use goldberg::goldberg_string;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use log::LevelFilter;

fn get_log_level() -> LevelFilter {
  let args: Vec<String> = env::args().collect();
  if args.contains(&goldberg_string!("-v").to_string()) {
    LevelFilter::Debug
  } else {
    LevelFilter::Warn
  }
}

pub fn init_logger() {
  let stdout = ConsoleAppender::builder()
    .encoder(Box::new(PatternEncoder::new(goldberg_string!("{d} - {h({l})} {m}{n}"))))
    .build();
  
  let config = Config::builder()
    .appender(Appender::builder().build(goldberg_string!("stdout"), Box::new(stdout) ))
    .build(Root::builder().appender(goldberg_string!("stdout")).build(get_log_level()))
    .unwrap();

  log4rs::init_config(config).unwrap();
}
