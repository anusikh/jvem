pub mod commands;
pub mod constants;
pub mod tests;
pub mod utils;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use clap::Parser;
use tokio::signal;

use crate::commands::{
    clean::clean, current::current, deactivate::deactivate, install::install, ls::ls, lsrem::lsrem,
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
    #[clap(about = "Install the specific JDK")]
    Install(Arg),
    #[clap(about = "Uninstall the specified JDK version")]
    Uninstall(Arg),
    #[clap(about = "Use a specific JDK version after installation")]
    Usev(Arg),
    #[clap(about = "Clean empty folders in the .jvem directory")]
    Clean,
    #[clap(about = "Find the currently active JDK version")]
    Current,
    #[clap(about = "List all JDK versions available for install")]
    Lsrem,
    #[clap(about = "List locally installed JDK versions")]
    Ls,
    #[clap(about = "Deactivate the currently active JDK")]
    Deactivate,
}

#[derive(Parser, Debug)]
struct Arg {
    #[clap(index = 1)]
    arg: String,
}

async fn logic(running: Arc<AtomicBool>) {
    while running.load(Ordering::Relaxed) {
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
            Command::Clean => clean(),
        }
        // the below step is important to prevent infinite loop on failure
        running.store(false, Ordering::Relaxed);
    }
}

#[tokio::main]
async fn main() {
    // the below logic is used for ctrl+c handling
    // the idea here is to set a boolean atomic value, when program is running
    // on ctrl+c, the value is set to false and it stops the process

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);
    let sigint = signal::ctrl_c();

    tokio::spawn(async move {
        let _ = sigint.await;
        println!("stopping program...");
        running_clone.store(false, Ordering::Relaxed);
    });

    logic(running.clone()).await;
}
