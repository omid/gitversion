#[macro_use]
extern crate clap;
extern crate semver;
extern crate regex;
extern crate git2;

mod git_version;

use clap::App;
use git_version::git_version::GitVersion;

fn main() {
  let yaml = load_yaml!("cli.yml");
  let matches = App::from_yaml(yaml).get_matches();
  let prefix = matches.value_of("prefix").unwrap().trim().to_string();
  let dir = matches.value_of("directory").unwrap().trim().to_string();
  let _type = matches.value_of("type").unwrap().trim().to_string();

  let mut gitversion = GitVersion::new(dir, prefix);

  println!("{}", gitversion.next_version(_type).clone());
}
