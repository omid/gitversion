#[macro_use]
extern crate clap;

mod git_version;

use clap::App;
use git_version::git_version::GitVersion;

fn main() {
  let yaml = load_yaml!("cli.yml");
  let matches = App::from_yaml(yaml).get_matches();
  let prefix = matches.value_of("prefix").unwrap().trim();
  let dir = matches.value_of("directory").unwrap().trim();
  let typ = matches.value_of("type").unwrap().trim();

  let mut gitversion = GitVersion::new(dir, prefix);

  println!("{}", gitversion.next_version(typ));
}
