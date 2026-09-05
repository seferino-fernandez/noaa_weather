use noaa_weather_cli::run;

#[tokio::main(flavor = "current_thread")]
async fn main() -> std::process::ExitCode {
    run().await.into()
}
