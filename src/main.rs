mod commands;
mod constants;
mod runtime;
#[cfg(test)]
mod tests;
mod utils;

use std::error::Error;

use clap::{Parser, Subcommand, ValueEnum};
use commands::{java, maven, node};

type AppResult = Result<(), Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    cmd: Lang,
}

#[derive(Debug, Clone, Subcommand)]
enum Lang {
    /// Java version management
    Java {
        /// The action to perform
        action: Option<JavaAction>,
        /// The JDK name, e.g. `zulu17` (required by install/uninstall/usev)
        param: Option<String>,
    },
    /// Maven management
    Maven {
        /// The action to perform
        action: Option<MavenAction>,
    },
    /// Node version management
    Node {
        /// The action to perform
        action: Option<NodeAction>,
        /// The node version, e.g. `22.11.0` (required by install/uninstall/usev)
        param: Option<String>,
    },
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, ValueEnum)]
enum JavaAction {
    /// Install a JDK, e.g. `jvem java install zulu17`
    Install,
    /// Uninstall an installed JDK, e.g. `jvem java uninstall zulu17`
    Uninstall,
    /// Activate an installed JDK, e.g. `jvem java usev zulu17`
    Usev,
    /// Remove empty JDK version directories
    Clean,
    /// Show the currently active JDK version
    Current,
    /// List JDK versions available for install
    Lsrem,
    /// List locally installed JDK versions
    Ls,
    /// Deactivate the currently active JDK
    Deactivate,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, ValueEnum)]
enum NodeAction {
    /// Install a node version, e.g. `jvem node install 22.11.0`
    Install,
    /// Uninstall a node version, e.g. `jvem node uninstall 22.11.0`
    Uninstall,
    /// Activate a node version, e.g. `jvem node usev 22.11.0`
    Usev,
    /// Remove empty node version directories
    Clean,
    /// Show the currently active node and npm versions
    Current,
    /// List node versions available for install
    Lsrem,
    /// List locally installed node versions
    Ls,
    /// Deactivate the currently active node version
    Deactivate,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, ValueEnum)]
enum MavenAction {
    /// Install maven to the system
    Install,
    /// Uninstall maven from the system
    Uninstall,
}

/// Extract the version argument for actions that need one, failing with a
/// usage hint when it is missing.
fn require_param(
    tool: &str,
    action: &str,
    param: Option<String>,
) -> Result<String, Box<dyn Error>> {
    param.ok_or_else(|| {
        format!("`jvem {tool} {action}` requires a version argument, e.g. `jvem {tool} {action} <version>`").into()
    })
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    if let Err(error) = run(cli).await {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

async fn run(cli: Cli) -> AppResult {
    match cli.cmd {
        Lang::Java { action: None, .. } => {
            Err("no action given; run `jvem java --help` for the available actions".into())
        }
        Lang::Java {
            action: Some(action),
            param,
        } => match action {
            JavaAction::Install => java::install::install(require_param("java", "install", param)?),
            JavaAction::Uninstall => {
                java::uninstall::uninstall(require_param("java", "uninstall", param)?)
            }
            JavaAction::Usev => java::usev::usev(require_param("java", "usev", param)?).await,
            JavaAction::Clean => java::clean::clean(),
            JavaAction::Current => java::current::current().await,
            JavaAction::Lsrem => java::lsrem::lsrem(),
            JavaAction::Ls => java::ls::ls(),
            JavaAction::Deactivate => java::deactivate::deactivate(),
        },
        Lang::Maven { action: None } => {
            Err("no action given; run `jvem maven --help` for the available actions".into())
        }
        Lang::Maven {
            action: Some(action),
        } => match action {
            MavenAction::Install => maven::install::install(),
            MavenAction::Uninstall => maven::uninstall::uninstall(),
        },
        Lang::Node { action: None, .. } => {
            Err("no action given; run `jvem node --help` for the available actions".into())
        }
        Lang::Node {
            action: Some(action),
            param,
        } => match action {
            NodeAction::Install => node::install::install(require_param("node", "install", param)?),
            NodeAction::Uninstall => {
                node::uninstall::uninstall(require_param("node", "uninstall", param)?)
            }
            NodeAction::Usev => node::usev::usev(require_param("node", "usev", param)?).await,
            NodeAction::Clean => node::clean::clean(),
            NodeAction::Current => node::current::current().await,
            NodeAction::Lsrem => node::lsrem::lsrem(),
            NodeAction::Ls => node::ls::ls(),
            NodeAction::Deactivate => node::deactivate::deactivate(),
        },
    }
}
