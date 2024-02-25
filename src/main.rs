pub mod commands;

use clap::Parser;

use crate::commands::{
    current::current, deactivate::deactivate, install::install, ls::ls, lsrem::lsrem,
    uninstall::uninstall, usev::usev,
};

#[derive(Parser)]
struct Cli {
    arg_type: String,
    arg: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    println!("pattern: {:?}, path: {:?}", args.arg_type, args.arg);

    let _ = match args.arg_type.as_str() {
        "lsrem" => lsrem(),
        "install" => Ok(match args.arg {
            Some(x) => {
                install(x);
            }
            None => {
                println!("please mention a jdk");
            }
        }),
        "ls" => Ok(ls()),
        "usev" => Ok(match args.arg {
            Some(x) => {
                let _ = usev(x).await;
            }
            None => {
                println!("please mention a jdk");
            }
        }),
        "deactivate" => Ok(deactivate()),
        "current" => Ok(current()),
        "uninstall" => Ok(match args.arg {
            Some(x) => {
                let _ = uninstall(x);
            }
            None => {
                println!("please mention a jdk");
            }
        }),
        _ => todo!(),
    };
}
