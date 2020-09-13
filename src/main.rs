use flexi_logger::Logger;

use settings::Settings;

mod rest;
mod settings;

#[tokio::main]
async fn main() {
    let settings = Settings::new().unwrap();

    Logger::with_str(&settings.miteimasu.log.level)
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));

    log::info!("Welcome to 見ています");
    log::debug!("Configuration: {:?}", settings);

    let address = (
        settings.miteimasu.server.address,
        settings.miteimasu.server.port,
    );

    let shutdown = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install CTRL+C signal handler");
    };

    let (_, server) =
        warp::serve(rest::routes(settings)).bind_with_graceful_shutdown(address, shutdown);

    tokio::select! {
            _ = server => {},
    }
}
