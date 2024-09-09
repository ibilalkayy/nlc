use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "nlc")]
#[command(about = "Nativelink client application", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Upload a file
    Upload {
        #[arg(short, long)]
        file: String,
    },
    /// Hash a file
    Hash {
        #[arg(short, long)]
        file: String,
    },
    /// Compute a Remote Execution Protocol
    Compute {},
    /// Issue the command to Nativelink
    Issue {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Upload { file } => {
            println!("Uploading a file: {}", file);
        }
        Commands::Hash { file } => {
            println!("Hashing a file: {}", file);
        }
        Commands::Compute {} => {
            println!("Computing a remote execution protocol");
        }
        Commands::Issue {} => {
            println!("Issue the command to nativelink");
        }
    }
}
