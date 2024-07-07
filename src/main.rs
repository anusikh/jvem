pub mod commands;
pub mod constants;
pub mod tests;
pub mod utils;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use clap::{Parser, Subcommand, ValueEnum};
use commands::{
    java::{
        clean::clean, current::current, deactivate::deactivate, install::install, ls::ls,
        lsrem::lsrem, uninstall::uninstall, usev::usev,
    },
    maven, node,
};
use tokio::signal;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    cmd: Lang,
}

#[derive(Debug, Clone, Subcommand)]
enum Lang {
    #[clap(about = "java version management")]
    Java {
        // #[arg(short, long, value_enum, help = "the Java action to perform")]
        #[clap(help = "the java action to be performed")]
        action: Option<Command>,
        param: Option<String>,
    },
    #[clap(about = "maven management")]
    Maven {
        #[clap(help = "the maven action to be performed")]
        action: Option<MavenCommand>,
    },
    #[clap(about = "node version management")]
    Node {
        action: Option<NodeCommand>,
        param: Option<String>,
    },
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, ValueEnum)]
enum Command {
    #[clap(help = "install the specific JDK")]
    Install,
    #[clap(help = "uninstall the specified JDK version")]
    Uninstall,
    #[clap(help = "use a specific JDK version after installation")]
    Usev,
    #[clap(help = "clean empty folders in the .jvem/java_versions directory")]
    Clean,
    #[clap(help = "find the currently active JDK version")]
    Current,
    #[clap(help = "list all JDK versions available for install")]
    Lsrem,
    #[clap(help = "list locally installed JDK versions")]
    Ls,
    #[clap(help = "deactivate the currently active JDK")]
    Deactivate,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, ValueEnum)]
enum NodeCommand {
    #[clap(help = "install the specific node version")]
    Install,
    #[clap(help = "uninstall the specified node version")]
    Uninstall,
    #[clap(help = "use a specific node version after installation")]
    Usev,
    #[clap(help = "clean empty folders in the .jvem/node_versions directory")]
    Clean,
    #[clap(help = "find the currently active node version")]
    Current,
    #[clap(help = "list all node versions available for install")]
    Lsrem,
    #[clap(help = "list locally installed node versions")]
    Ls,
    #[clap(help = "deactivate the currently active node version")]
    Deactivate,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, ValueEnum)]
enum MavenCommand {
    #[clap(help = "install maven to system")]
    Install,
    #[clap(help = "uninstall maven from system")]
    Uninstall,
}

impl std::str::FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "install" => Ok(Command::Install),
            "lsrem" => Ok(Command::Lsrem),
            "ls" => Ok(Command::Ls),
            "current" => Ok(Command::Current),
            "uninstall" => Ok(Command::Uninstall),
            "usev" => Ok(Command::Usev),
            "deactivate" => Ok(Command::Deactivate),
            "clean" => Ok(Command::Clean),
            _ => Err(format!("invalid Java action: {}", s)),
        }
    }
}

impl std::str::FromStr for NodeCommand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "install" => Ok(NodeCommand::Install),
            "lsrem" => Ok(NodeCommand::Lsrem),
            "ls" => Ok(NodeCommand::Ls),
            "current" => Ok(NodeCommand::Current),
            "uninstall" => Ok(NodeCommand::Uninstall),
            "usev" => Ok(NodeCommand::Usev),
            "deactivate" => Ok(NodeCommand::Deactivate),
            "clean" => Ok(NodeCommand::Clean),
            _ => Err(format!("invalid Node action: {}", s)),
        }
    }
}

impl std::str::FromStr for MavenCommand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "install" => Ok(MavenCommand::Install),
            "uninstall" => Ok(MavenCommand::Uninstall),
            _ => Err(format!("invalid Maven action: {}", s)),
        }
    }
}

async fn handle_java_action(action: Option<Command>, param: Option<String>) {
    if let Some(action) = action {
        match action {
            Command::Install => {
                let jdk = param.unwrap();
                install(jdk);
            }
            Command::Current => current(),
            Command::Uninstall => {
                let jdk = param.unwrap();
                uninstall(jdk);
            }
            Command::Usev => {
                let jdk = param.unwrap();
                usev(jdk).await;
            }
            Command::Lsrem => lsrem(),
            Command::Ls => ls(),
            Command::Deactivate => deactivate(),
            Command::Clean => clean(),
        }
    } else {
        println!("enter valid action. for more details use --help or -h");
    }
}

async fn handle_maven_action(action: Option<MavenCommand>) {
    if let Some(action) = action {
        match action {
            MavenCommand::Install => maven::install::install(),
            MavenCommand::Uninstall => maven::uninstall::uninstall(),
        }
    } else {
        println!("enter valid action, for more details use --help or -h");
    }
}

async fn handle_nodejs_action(action: Option<NodeCommand>, param: Option<String>) {
    if let Some(action) = action {
        match action {
            NodeCommand::Lsrem => node::lsrem::lsrem(),
            NodeCommand::Ls => node::ls::ls(),
            NodeCommand::Clean => node::clean::clean(),
            NodeCommand::Current => node::current::current().await,
            NodeCommand::Deactivate => node::deactivate::deactivate(),
            _ => println!("not implemented yet"),
        }
    } else {
        println!("enter valid action, for more details use --help or -h");
    }
}

async fn logic(running: Arc<AtomicBool>) {
    while running.load(Ordering::Relaxed) {
        match Cli::parse().cmd {
            Lang::Java { action, param } => handle_java_action(action, param).await,
            Lang::Maven { action } => handle_maven_action(action).await,
            Lang::Node { action, param } => handle_nodejs_action(action, param).await,
        }

        // the below step is important to prevent infinite loop on failure
        running.store(false, Ordering::Relaxed);
    }
}

#[tokio::main]
async fn main() {
    // NOTE: the below logic is used for ctrl+c handling
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
