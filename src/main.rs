use clap::Parser;
use std::{env, fs, path::PathBuf};

use cli::Args;
use figment::{
    providers::{Format, Yaml},
    Figment,
};
mod cli;
mod hypr;
mod ui;
mod types;
mod utils;

fn parse_config() -> Args {
    // there is probably a way better way to do this...
    let mut args = Args::default();

    let config_dir = env::var("HOME").expect("HOME environment variable not set");
    let config_dir = PathBuf::from(config_dir).join(".config/easyfocus-hyprland");
    // create a base directory if it doesn't exist, and a config file if it doesn't exist
    let config_path = config_dir.join("config.yaml");
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).expect("failed to create config directory");
    }
    if !config_path.exists() {
        fs::File::create(&config_path).expect("failed to create config file");
    }

    if let Ok(config_args) = Figment::new()
        .merge(Yaml::file(&config_path))
        .extract::<Args>()
    {
        args.merge(&config_args);
    }

    let cli_args = Args::parse();
    args.merge(&cli_args);

    args
}

fn main() {
    let args = parse_config();
    ui::run_ui(args);
}
