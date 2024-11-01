use std::io::{stdin, Write};
use std::{io, time};
use colored::Colorize;

pub(crate) fn read_console() -> String {
    print(">>> ");
    let mut buffer = String::new();
    let handle = stdin().read_line(&mut buffer);
    if let Err(_) = handle {
        eprint!("Oops... Error while reading from console.\n>>>");
    }
    buffer.pop();
    buffer.to_lowercase()
}

fn print(content: &str) {
    let buffer: Box<[u8]> = content.bytes().collect();
    io::stdout().write(&buffer).unwrap();
    io::stdout().flush().unwrap();
}

pub(crate) async fn animator(time_seconds: u64) {
    print("[");
    for _ in 1..=15 {
        print(&format!("{}", "#".blue()));
        tokio::time::sleep(time::Duration::from_secs(time_seconds)).await;
    }
    print("]\n");
}