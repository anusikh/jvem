pub mod commands;

use clap::Parser;

use crate::commands::{
    current::current, deactivate::deactivate, install::install, ls::ls, lsrem::lsrem,
    uninstall::uninstall, usev::usev,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Parser, Debug)]
enum Command {
    #[clap(about)]
    Install(Arg),
    Uninstall(Arg),
    Usev(Arg),
    Current,
    Lsrem,
    Ls,
    Deactivate,
}

#[derive(Parser, Debug)]
struct Arg {
    #[clap(index = 1)]
    arg: String,
}

#[tokio::main]
async fn main() {
    match Cli::parse().cmd {
        Command::Install(arg) => {
            let jdk = arg.arg;
            install(jdk);
        }
        Command::Current => current(),
        Command::Uninstall(arg) => {
            let jdk = arg.arg;
            uninstall(jdk);
        }
        Command::Usev(arg) => {
            let jdk = arg.arg;
            usev(jdk).await;
        }
        Command::Lsrem => {
            let _ = lsrem();
        }
        Command::Ls => ls(),
        Command::Deactivate => deactivate(),
    }
}
