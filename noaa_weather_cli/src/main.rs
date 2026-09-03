use noaa_weather_cli::try_main;

#[tokio::main]
async fn main() {
    if let Err(error) = try_main().await {
        eprintln!("noaa-weather: {error:#}");
        std::process::exit(1);
    }
}
