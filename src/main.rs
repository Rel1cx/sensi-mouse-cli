use clap::Parser;
use helper::{read_mouse_cfg, write_mouse_cfg};
use std::io::{self, BufRead};
use std::thread;
use std::time::Duration;

mod helper;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 200, help = "Mouse sensitivity")]
    sen: u32,

    #[arg(short, long, default_value_t = 57344, help = "Mouse acceleration")]
    acc: u32,

    #[arg(short, long, help = "Run as daemon")]
    daemon: bool,
}

fn main() {
    let argc = std::env::args().count();

    if argc == 1 {
        let (sen, acc) = read_mouse_cfg().unwrap();

        println!("Current settings: sen = {}, acc = {}", sen, acc);

        return;
    }

    let args = Args::parse();

    write_mouse_cfg(args.sen as i32, args.acc as i32).unwrap();

    let (sen, acc) = read_mouse_cfg().unwrap();

    println!("Settings applied: sen = {}, acc = {}", sen, acc);

    let daemon = args.daemon;

    if daemon {
        thread::spawn(move || loop {
            let (sen, acc) = read_mouse_cfg().unwrap();

            if sen != args.sen as i32 || acc != args.acc as i32 {
                write_mouse_cfg(args.sen as i32, args.acc as i32).unwrap();

                println!("system-wide mouse settings changed, reapplying...");
            }

            thread::sleep(Duration::from_secs(15));
        });

        println!("Running as daemon, press 'q' then 'Enter' to quit");
        let stdin = io::stdin();
        for line in stdin.lock().lines().flatten() {
            if line == "q" {
                break;
            }
        }
    }
}
