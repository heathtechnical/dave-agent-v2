//! dave-agent: hygiene checks for Linux servers.

use std::process::ExitCode;

use clap::{Parser, Subcommand};

/// DAVE agent: hygiene checks for Linux servers.
#[derive(Debug, Parser)]
#[command(name = "dave-agent", version, about)]
struct Cli {
    /// Print debug output to stderr.
    #[arg(long, global = true)]
    debug: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Run the hygiene checks.
    Run,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    if cli.debug {
        eprintln!("debug: {cli:?}");
    }
    match cli.command {
        Command::Run => run(),
    }
}

/// Stub: the checks are not implemented yet.
fn run() -> ExitCode {
    eprintln!("dave-agent: run is not implemented yet");
    ExitCode::FAILURE
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    #[test]
    fn parses_run() {
        let cli = Cli::try_parse_from(["dave-agent", "run"]).unwrap();
        assert!(matches!(cli.command, Command::Run));
        assert!(!cli.debug);
    }

    #[test]
    fn debug_is_global() {
        let before = Cli::try_parse_from(["dave-agent", "--debug", "run"]).unwrap();
        let after = Cli::try_parse_from(["dave-agent", "run", "--debug"]).unwrap();
        assert!(before.debug);
        assert!(after.debug);
    }

    #[test]
    fn subcommand_is_required() {
        assert!(Cli::try_parse_from(["dave-agent"]).is_err());
    }

    #[test]
    fn unknown_subcommand_is_rejected() {
        assert!(Cli::try_parse_from(["dave-agent", "frobnicate"]).is_err());
    }
}
