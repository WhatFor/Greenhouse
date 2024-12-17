use std::{os::unix::net::UnixStream, time::Duration};
use std::path::Path;
use std::process::Command;
use anyhow::{Result, Context};
use std::thread::sleep;

fn main() -> Result<()> {
    println!("Starting Greenhouse...");
    let socket_path = "/usr/bin/docker";
    
    // Retry connection up to 30 times (30 seconds)
    for i in 0..30 {
        println!("Attempting to connect to Docker socket (attempt {})", i + 1);
        
        match UnixStream::connect(socket_path) {
            Ok(mut stream) => {
                println!("Successfully connected to Docker socket");
                // Your Docker API code here
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
