use env_logger::Env;
use log::warn;
use std::{io::Write, os::unix::net::UnixStream, time::Duration};
use anyhow::Result;
use std::thread::sleep;

const DOCKER_CONNECTION_RETRIES: u8 = 10;

fn main() -> Result<()> {
    init_logger();

    log::info!("Starting Greenhouse...");
    let socket_path = "/var/run/docker.sock";
    
    for i in 0..DOCKER_CONNECTION_RETRIES {
        log::info!("Attempting to connect to Docker socket (attempt {})", i + 1);
        
        match UnixStream::connect(socket_path) {
            Ok(mut stream) => {
                stream.write_all(b"Hello?")?;
                return Ok(());
            }
            Err(e) => {
                log::warn!("Connection attempt failed: {}", e);
                sleep(Duration::from_secs(1));
            }
        }
    }

    anyhow::bail!("Failed to connect to Docker socket after 30 attempts")
}

fn init_logger() {
    let env = Env::default().default_filter_or("TRACE");

    env_logger::Builder::from_env(env)
        .format_target(false)
        .init();
}