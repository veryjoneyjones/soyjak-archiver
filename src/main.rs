use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::thread::{Board, Thread};

mod file;
mod thread;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Archive {
        #[arg(short, long)]
        board: Board,

        #[arg(short, long)]
        id: u64,

        #[arg(short, long)]
        path: PathBuf,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    match args.command {
        Command::Archive { board, id, path } => {
            let thread = Thread::fetch(board, id).await;

            for file in thread.get_images() {
                if let Err(err) = file.download_to_disk(&path).await {
                    eprintln!("An error occured while downloading {}. {err}", file.url);
                }
            }
        }
    }

    Ok(())
}
