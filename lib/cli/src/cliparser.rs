use clap::Parser;

/// Run Dux as a all-in-one tool
#[derive(Parser, Debug)]
#[command(arg_required_else_help(true))]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// Path to configuration file
    #[arg(short, long)]
    pub conf: Option<String>,

    /// Path to TaskList file
    #[arg(short, long, default_value_t = String::from("tasklist.yml"))]
    pub tasklist: String,

    /// Path to HostList file
    #[arg(short = 'l', long, default_value_t = String::from("hostlist"))]
    pub hostlist: String,

    /// Username to use on remote hosts
    #[arg(short, long, default_value_t = String::from("root"))]
    pub user: String,

    /// Path to private SSH key to use
    #[arg(short = 'k', long)]
    pub key: Option<String>,

    /// Password to use on remote hosts
    #[arg(short, long)]
    pub password: Option<String>,

    /// Number of threads used by all-in-one tool (default = number of CPU of the local machine)
    #[arg(long)]
    pub threads: Option<usize>,
}

pub fn parse_cli_args() -> CliArgs {
    CliArgs::parse()
}