mod settings;

use flexi_logger::Logger;
use log::{debug, error, info, trace, warn};
use settings::Settings;

fn main() {
    println!("Hello, world!");

    let settings = Settings::new().unwrap();

    Logger::with_str(settings.miteimasu.log.level)
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));

    trace!("Test trace");
    debug!("Test debug");
    info!("Test info");
    warn!("Test warn");
    error!("Test error");

    info!("Welcome to 見て います");
}
