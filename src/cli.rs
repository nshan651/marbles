use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "marbles")]
#[command(about = "Distributed consensus ranked choice voting system")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Cast a ballot in an election (online)
    Vote(VoteArgs),
    /// Tally ballots locally (standalone, no server)
    Tally(TallyArgs),
    /// Start a polling station node
    Serve(ServeArgs),
    /// Query and manage elections
    #[command(subcommand)]
    Election(ElectionArgs),
}

#[derive(Parser)]
pub struct VoteArgs {
    /// Election ID to vote in
    pub election_id: String,
    /// Path to ballot JSON file (omit to pipe stdin)
    #[arg(short, long)]
    pub ballot: Option<PathBuf>,
    /// Path to ed25519 key for signing
    #[arg(short, long)]
    pub identity: Option<PathBuf>,
    /// Server URL
    #[arg(long, default_value = "http://localhost:8080")]
    pub server: String,
}

#[derive(Parser)]
pub struct TallyArgs {
    /// Voting method: rcv, star, borda, schulze, minimax, baldwin
    #[arg(short, long, default_value = "rcv")]
    pub method: String,
    /// Path to ballots collection JSON file (omit to pipe stdin)
    #[arg(short, long)]
    pub ballots: Option<PathBuf>,
    /// Maximum score per ballot (required for star/score)
    #[arg(long)]
    pub max_score: Option<u32>,
}

#[derive(Parser)]
pub struct ServeArgs {
    /// Port to listen on
    #[arg(short, long, default_value = "8080")]
    pub port: u16,
    /// Peer node URLs
    #[arg(long)]
    pub peers: Vec<String>,
}

#[derive(Subcommand)]
pub enum ElectionArgs {
    /// Get election status and details
    Info(ElectionInfoArgs),
    /// Create a new election on a server
    Create(ElectionCreateArgs),
}

#[derive(Parser)]
pub struct ElectionInfoArgs {
    /// Election ID
    pub election_id: String,
    /// Server URL
    #[arg(long, default_value = "http://localhost:8080")]
    pub server: String,
}

#[derive(Parser)]
pub struct ElectionCreateArgs {
    /// Server URL
    #[arg(long, default_value = "http://localhost:8080")]
    pub server: String,
}
