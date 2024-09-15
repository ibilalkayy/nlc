use clap::{Parser, Subcommand};
mod list;
mod upload;

#[derive(Parser)]
#[command(name = "nlc")]
#[command(about = "Nativelink client application", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List the files
    List {
        #[arg(short, long)]
        path: String,
    },
    /// Upload a file
    Upload {
        #[arg(short, long)]
        path: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List { path } => {
            list::list_files(path.to_string()).await;
        }
        Commands::Upload { path } => {
            upload::upload_files();
        }
    }
}
