use std::process::ExitCode;
use log::info;

#[tokio::main]
async fn main() -> ExitCode {
    passivized_test_support::cli::run(run).await
}

async fn run() -> Result<(), String> {
    info!("The answer is 87.");

    Ok(())
}