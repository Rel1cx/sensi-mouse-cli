use clap::Parser;
use helper::{read_mouse_cfg, write_mouse_cfg};

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
    let args = Args::parse();

    write_mouse_cfg(args.sen as i32, args.acc as i32).unwrap();

    let (sen, acc) = read_mouse_cfg().unwrap();

    println!("Settings applied: sen = {}, acc = {}", sen, acc);

    while args.daemon {
        let (sen, acc) = read_mouse_cfg().unwrap();

        if sen != args.sen as i32 || acc != args.acc as i32 {
            write_mouse_cfg(args.sen as i32, args.acc as i32).unwrap();

            println!("system-wide mouse settings changed, reapplying...");
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
