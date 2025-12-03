use crate::cmd::{Cli, Commands, ConfigType};
use crate::converter::keymousego::{Converter, ConverterFrom};
use crate::converter::quickinput;
use clap::Parser;
use std::path::PathBuf;

mod cmd;
mod converter;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Convert { config_type } => match config_type {
            ConfigType::KeyMouseGo { config } => {
                let path = PathBuf::from(config);
                let result = Converter::new(&path)
                    .convert(ConverterFrom::KeyMouseGo)
                    .unwrap();

                let content = serde_yaml::to_string(&result).unwrap();

                println!("{}", content);
            }
            ConfigType::QuickInput {
                config,
                with_duration,
                with_exist,
            } => {
                let path = PathBuf::from(config);
                let result = quickinput::Converter::new(&path, with_exist, with_duration)
                    .convert()
                    .unwrap();
                let content = serde_yaml::to_string(&result).unwrap();
                println!("{}", content);
            }
        },
    }
}
