use noaa_weather_cli::run;

#[tokio::main]
async fn main() -> std::process::ExitCode {
    run().await.into()
}
