#[macro_use]
extern crate clap;

mod config;
mod tests;
mod versioning;

use crate::{
    config::{Config, Format},
    versioning::{semver::Semver, sequential::Sequential, Versioning},
};
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let prefix = matches.value_of("prefix").unwrap().trim();
    let dir = matches.value_of("directory").unwrap().trim();
    let typ = matches.value_of("type").unwrap().trim();
    let format = matches.value_of("format").unwrap().trim();

    let mut config = Config::new(dir, prefix, typ.into(), format.into());

    println!("{}", next_version(&mut config));
}

fn next_version(config: &mut Config) -> String {
    match config.format {
        Format::Semver => Semver::next_version(config),
        Format::Sequential => Sequential::next_version(config),
    }
}
