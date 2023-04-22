use clap::Parser;
use helper::{read_mouse_cfg, write_mouse_cfg};
use std::io::{self, BufRead};
use std::thread;
use std::time::Duration;

mod helper;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        default_value_t = 1800,
        help = "Mouse Sensitivity, between 10 and 1990, default 1800"
    )]
    sen: u32,

    #[arg(
        short,
        long,
        default_value_t = 57344,
        help = "Mouse Acceleration, default 57344"
    )]
    acc: u32,

    #[arg(short, long, help = "Run as daemon")]
    daemon: bool,
}

fn main() {
    let argc = std::env::args().count();

    if argc == 1 {
        let (sen, acc) = read_mouse_cfg().unwrap();

        println!("Current Settings: sen = {}, acc = {}", sen, acc);

        return;
    }

    let args = Args::parse();

    write_mouse_cfg(args.sen as i32, args.acc as i32).unwrap();

    let (sen, acc) = read_mouse_cfg().unwrap();

    println!("Settings applied: sen = {}, acc = {}", sen, acc);

    if args.daemon {
        thread::spawn(move || loop {
            let (sen, acc) = read_mouse_cfg().unwrap();

            if sen != args.sen as i32 || acc != args.acc as i32 {
                write_mouse_cfg(args.sen as i32, args.acc as i32).unwrap();

                println!("system-wide mouse settings changed, reapplying...");
            }

            thread::sleep(Duration::from_secs(15));
        });

        println!("Running as daemon, press Ctrl+C to exit");

        let stdin = io::stdin();
        for _ in stdin.lock().lines() {}
    }
}
