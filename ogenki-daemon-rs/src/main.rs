mod cli;
mod sender;
mod format;

use std::io::{BufRead, BufReader};

use clap::Parser;
use serialport::{DataBits, FlowControl, Parity, StopBits};

use cli::Cli;
use sender::*;
use format::*;
use twelite_serial::*;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let cli = Cli::parse();
    let sender: &'static Sender = Box::leak(Box::new(Sender::new(&cli.backend)));

    let serial = serialport::new(&cli.serial.serial_port, cli.serial.baudrate)
        .flow_control(FlowControl::None)
        .data_bits(DataBits::Eight)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .open()
        .expect("Failed to open serial port");

    let serial = BufReader::new(serial);

    for line in serial.lines() {
        let line = line.unwrap();

        let Ok(status) = StatusNotify::decode_str(&line) else {
            continue;
        };

        if let Err(v) = status.validate() {
            eprintln!("{v}");
            continue;
        }

        println!("{}", status.format());

        tokio::spawn(async move { sender.send(&status).await.unwrap() });
    }
}
