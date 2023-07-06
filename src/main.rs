use anyhow::Error;
use clap::{arg, Parser};
use colored::Colorize;
use std::path::PathBuf;
use std::process::{exit, Command};

/// A cargo runner for Arduino boards
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

struct Args {
    /// The ELF firmware file
    #[arg(required = true)]
    elf: PathBuf,

    /// The Fully Qualified Board Name (FQBN)
    #[arg(long, short, required = true)]
    board: String,

    /// The serial port
    #[arg(
        long,
        short,
        required = true,
        env = "ARDUINO_RUN_PORT",
        hide_env_values = true
    )]
    port: String,

    /// Open serial monitor after flashing
    #[arg(long, short)]
    monitor: bool,

    /// The serial monitor baudrate
    #[arg(long, short = 'r', default_value_t = 9600)]
    baudrate: u32,
}

fn main() {
    if let Err(e) = arduino_run() {
        eprintln!("{} {}", "error:".bright_red().bold(), e);
        exit(1);
    }
}

fn arduino_run() -> Result<(), Error> {
    let args = Args::parse();
    let mut commands = [
        Command::new("rust-objcopy"),
        Command::new("rust-objcopy"),
        Command::new("arduino-cli"),
    ];

    commands[0]
        .arg("-O")
        .arg("binary")
        .arg(&args.elf)
        .arg("target/firmware.bin");
    commands[1]
        .arg("-O")
        .arg("ihex")
        .arg(&args.elf)
        .arg("target/firmware.hex");
    commands[2]
        .arg("upload")
        .arg("-i")
        .arg("target/firmware.bin")
        .arg("-b")
        .arg(args.board)
        .arg("-p")
        .arg(&args.port);

    for mut command in commands {
        let status = command.status()?;
        if !status.success() {
            exit(status.code().unwrap_or(1));
        }
    }

    if args.monitor {
        let status = Command::new("arduino-cli")
            .arg("monitor")
            .arg("-p")
            .arg(&args.port)
            .arg("-c")
            .arg(format!("baudrate={}", args.baudrate))
            .status()?;
        if !status.success() {
            exit(status.code().unwrap_or(1));
        }
    }
    Ok(())
}
