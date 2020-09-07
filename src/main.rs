#![warn(rust_2018_idioms)]

pub mod driver;

use futures::prelude::*;
use futures::sink::SinkExt;
use tokio::io::{AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() {
    let t = driver::serial("/dev/ttyACM0", 115200).unwrap();
    let (mut writer, mut reader) = t.split();

    tokio::spawn(async move {
        while let Some(line_result) = reader.next().await {
            let line = line_result.expect("Failed to read line");
            print!("{}", line);
        }
    });

    let input = tokio::io::stdin();
    let mut input = BufReader::new(input).lines();
    loop {
        let r = input.next_line().await.unwrap();
        if let Some(line) = r {
            writer.send(line).await.unwrap();
        }
    }
}
