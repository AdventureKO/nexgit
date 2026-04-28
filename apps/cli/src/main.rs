mod args;
mod commands;
mod server;
mod tui;

use anyhow::Result;
use args::{AppServerCommand, Cli, Command, RepoCommand, StackCommand};
use clap::Parser;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();

    let cli = Cli::parse();

    match cli.command.unwrap_or(Command::Tui) {
        Command::Tui => tui::run(),
        Command::Version(args) => {
            let payload = commands::version_payload();
            if args.json {
                println!("{}", serde_json::to_string_pretty(&payload)?);
            } else {
                println!("nexgit {}", env!("CARGO_PKG_VERSION"));
            }
            Ok(())
        }
        Command::AppServer(args) => match args.command {
            Some(AppServerCommand::GenerateTs) => {
                print!("{}", nexgit_protocol::TYPESCRIPT_DEFINITIONS);
                Ok(())
            }
            Some(AppServerCommand::GenerateJsonSchema) => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&commands::protocol_schema_placeholder())?
                );
                Ok(())
            }
            None => server::run(&args.listen).await,
        },
        Command::Repo(args) => match args.command {
            RepoCommand::Status(output) => {
                let status = nexgit_core::Core::new().repo_status()?;
                if output.json {
                    println!("{}", serde_json::to_string_pretty(&status)?);
                } else {
                    println!(
                        "repository: {}",
                        status.root.as_deref().unwrap_or("not selected")
                    );
                    println!(
                        "branch: {}",
                        status.current_branch.as_deref().unwrap_or("none")
                    );
                    println!("clean: {}", status.is_clean);
                    println!("pending changes: {}", status.pending_changes);
                }
                Ok(())
            }
        },
        Command::Stack(args) => match args.command {
            StackCommand::List(output) => {
                let stacks = nexgit_core::Core::new().stacks()?;
                if output.json {
                    println!("{}", serde_json::to_string_pretty(&stacks)?);
                } else if stacks.is_empty() {
                    println!("no stacks found");
                } else {
                    for stack in stacks {
                        println!("{}", stack.name);
                    }
                }
                Ok(())
            }
        },
    }
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .init();
}
