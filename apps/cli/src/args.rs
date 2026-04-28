use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "nexgit",
    version,
    about = "A Git stacking CLI, TUI, and local app server"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(about = "Launch the terminal UI")]
    Tui,

    #[command(about = "Run the local app server used by desktop and integrations")]
    AppServer(AppServerArgs),

    #[command(about = "Repository-oriented headless commands")]
    Repo(RepoArgs),

    #[command(about = "Stack-oriented headless commands")]
    Stack(StackArgs),

    #[command(about = "Print version information")]
    Version(VersionArgs),
}

#[derive(Debug, Args)]
pub struct VersionArgs {
    #[arg(long, help = "Print machine-readable JSON")]
    pub json: bool,
}

#[derive(Debug, Args)]
pub struct JsonOutputArgs {
    #[arg(long, help = "Print machine-readable JSON")]
    pub json: bool,
}

#[derive(Debug, Args)]
pub struct RepoArgs {
    #[command(subcommand)]
    pub command: RepoCommand,
}

#[derive(Debug, Subcommand)]
pub enum RepoCommand {
    #[command(about = "Print repository status")]
    Status(JsonOutputArgs),
}

#[derive(Debug, Args)]
pub struct StackArgs {
    #[command(subcommand)]
    pub command: StackCommand,
}

#[derive(Debug, Subcommand)]
pub enum StackCommand {
    #[command(about = "List known stacks")]
    List(JsonOutputArgs),
}

#[derive(Debug, Args)]
pub struct AppServerArgs {
    #[arg(
        long,
        default_value = "stdio://",
        help = "Transport endpoint: stdio://, unix://, unix://PATH, ws://IP:PORT, or off"
    )]
    pub listen: String,

    #[command(subcommand)]
    pub command: Option<AppServerCommand>,
}

#[derive(Debug, Subcommand)]
pub enum AppServerCommand {
    #[command(about = "Generate TypeScript protocol definitions")]
    GenerateTs,

    #[command(about = "Generate a placeholder JSON Schema for the app-server protocol")]
    GenerateJsonSchema,
}
