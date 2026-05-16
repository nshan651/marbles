use marbles::cli;

use clap::Parser;

#[tokio::main]
async fn main() {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Command::Vote(args) => {
            eprintln!("vote not implemented");
            std::process::exit(1);
        }
        cli::Command::Tally(args) => {
            eprintln!("tally not implemented");
            std::process::exit(1);
        }
        cli::Command::Serve(args) => {
            eprintln!("serve subcommand not yet implemented (port={})", args.port);
            std::process::exit(1);
        }
        cli::Command::Election(args) => {
            eprintln!("tally not implemented");
            std::process::exit(1);
        }
    }
}
