pub mod commands;

use clap::Parser;

use crate::commands::{install::install, lsrem::lsrem};

#[derive(Parser)]
struct Cli {
    arg_type: String,
    arg: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    println!("pattern: {:?}, path: {:?}", args.arg_type, args.arg);

    if args.arg_type == "lsrem" {
        let _ = lsrem();
    } else if args.arg_type == "install" {
        match args.arg {
            Some(x) => {
                let _ = install(x);
            }
            None => {
                println!("please mention a jdk name while running install");
            }
        }
    }
}
