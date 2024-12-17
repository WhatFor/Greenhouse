use std::{os::unix::net::UnixStream, time::Duration};
use std::path::Path;
use std::process::Command;
use anyhow::{Result, Context};
use std::thread::sleep;

const DOCKER_CONNECTION_RETRIES: u8 = 10;

fn main() -> Result<()> {
    println!("Starting Greenhouse...");
    let socket_path = "/var/run/docker.sock";
    
    for i in 0..DOCKER_CONNECTION_RETRIES {
        println!("Attempting to connect to Docker socket (attempt {})", i + 1);
        
        match UnixStream::connect(socket_path) {
            Ok(mut stream) => {
                return Ok(());
            }
            Err(e) => {
                println!("Connection attempt failed: {}", e);
                sleep(Duration::from_secs(1));
            }
        }
    }

    anyhow::bail!("Failed to connect to Docker socket after 30 attempts")
}
