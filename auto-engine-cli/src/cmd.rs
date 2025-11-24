use clap_derive::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The subcommand to execute.
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Convert the other config to auto-engine pipeline
    Convert {
        /// Config Type
        #[command(subcommand)]
        config_type: ConfigType,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum ConfigType {
    KeyMouseGo {
        /// KeyMouseGo config path
        #[arg(short, long, value_name = "config")]
        config: String,
    },
    QuickInput {
        /// QuickInput config path
        #[arg(short, long, value_name = "config")]
        config: String,
        #[arg(long, value_name = "with_exist")]
        with_exist: Option<String>,
        #[arg(long, value_name = "with_duration")]
        with_duration: Option<u32>,
    },
}
