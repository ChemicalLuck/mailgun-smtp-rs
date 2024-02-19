use std::path::PathBuf;

use clap::{Parser, Subcommand};
use lettre::message::Mailbox;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
    pub subject: String,
    pub message: PathBuf,
    #[arg(long)]
    pub recipients: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Command {
    Send {
        #[arg(long, env = "SMTP_USERNAME")]
        username: Option<String>,
        #[arg(long, env = "SMTP_PASSWORD", hide_env_values = true)]
        password: Option<String>,
        #[arg(long, env = "SMTP_RELAY")]
        smtp_relay: Option<String>,
        #[arg(long, env = "SMTP_FROM")]
        from: Option<Mailbox>,
        #[arg(long, env = "SMTP_REPLY_TO")]
        reply_to: Option<Mailbox>,
    },
    Preview {},
}
