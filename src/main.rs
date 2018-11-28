#[macro_use]
extern crate clap;
extern crate semver;
extern crate regex;
extern crate git2;

use clap::App;
use git2::Repository;
use git2::string_array::StringArray;
use semver::Version;
use regex::Regex;

fn main() {
  let yaml = load_yaml!("cli.yml");
  let matches = App::from_yaml(yaml).get_matches();
  let version_prefix = matches.value_of("prefix").unwrap().trim();
  let git_dir = matches.value_of("directory").unwrap().trim();

  let repo = match Repository::open(git_dir) {
    Ok(repo) => repo,
    Err(e) => panic!("failed to open: {}", e),
  };

//  git_fetch(&repo).unwrap();
  let all_tags = git_tag(&repo).unwrap();

  // filter by a specific regexp
  let pattern = format!("{}{}{}", '^', version_prefix, "\\d+\\.\\d+\\.\\d+$");

  let tag_regex = Regex::new(pattern.as_ref()).unwrap();

  // sort git versions
  let mut version_tags = Vec::new();
  for tag in all_tags.iter() {
    if tag_regex.is_match(tag.unwrap()) {
      version_tags.push(tag.unwrap().clone());
    }
  }
  version_tags.sort_by(|&a, &b| natord::compare(a, b));

  // find the greatest version
  let tag_list_length = version_tags.len().clone();
  let last_version;

  if tag_list_length == 0 {
    last_version = "0.0.0";
  } else {
    last_version = version_tags[tag_list_length - 1];
  }

  // version
  let mut version = Version::parse(last_version).unwrap();

  match matches.value_of("type").unwrap() {
    "major" => {
      version.increment_major();
    }
    "minor" => {
      version.increment_minor();
    }
    "patch" => {
      version.increment_patch();
    }
    _ => unreachable!()
  }

  println!("{}{}", version_prefix, version.to_string());
}

//fn git_fetch(repo: &git2::Repository) -> Result<(), git2::Error> {
//  repo.find_remote("origin")?.fetch(&["master"], None, None)
//}

//fn git_checkout(repo: &git2::Repository) -> Result<(), git2::Error> {
//  repo.set_head("master")
//}

fn git_tag(repo: &git2::Repository) -> Result<StringArray, git2::Error> {
  repo.tag_names(None)
}
