use clap::Parser;
use duxcore::error::Error;

pub fn parse_cli_args_standard() -> Result<CliArgsStandard, Error> {
    Ok(CliArgsStandard::parse())
}

/// Dux standard use case : apply a task list on remote hosts
#[derive(Parser, Debug)]
#[command(arg_required_else_help(true))]
#[command(version)]
pub struct CliArgsStandard {
    /// Path to configuration file
    #[arg(short, long)]
    pub conf: Option<String>,

    /// Path to TaskList file
    #[arg(short, long)]
    pub tasklist: Option<String>,

    /// Path to HostList file
    #[arg(short = 'l', long)]
    pub hostlist: Option<String>,

    /// Username to use on remote hosts
    #[arg(short, long)]
    pub user: Option<String>,

    /// Password to use on remote hosts
    #[arg(short, long)]
    pub password: Option<String>,

    /// Path to private SSH2 key to use
    #[arg(short = 'k', long)]
    pub key: Option<String>,
}
