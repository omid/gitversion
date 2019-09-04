use git2::Repository;
use semver::Version;
use git2::string_array::StringArray;
use regex::Regex;

pub struct GitVersion {
  pub dir: String,
  pub prefix: String,
  version: String,
}

impl GitVersion {
  pub fn new(dir: String, prefix: String) -> GitVersion {
    GitVersion {
      dir,
      prefix,
      version: String::from("0.0.0"),
    }
  }

  pub fn current_version(&mut self) -> String {
    let repo = match Repository::open(self.dir.clone()) {
      Ok(repo) => repo,
      Err(e) => panic!("failed to open: {}", e),
    };

    let all_tags = self.git_tag(&repo).unwrap();

    // filter by a specific regexp
    let pattern = format!("{}{}{}", '^', self.prefix, "\\d+\\.\\d+\\.\\d+$");

    let tag_regex = Regex::new(pattern.as_ref()).unwrap();

    // sort git versions
    let mut version_tags: Vec<String> = all_tags.iter().filter(|&tag| {
      tag_regex.is_match(tag.unwrap())
    }).map(|tag| tag.unwrap().to_owned()).collect();
    version_tags.sort_by(|a, b| natord::compare(&a, &b));

    // find the greatest version
    let tag_list_length = version_tags.len().clone();

    if tag_list_length != 0 {
      self.version = version_tags[tag_list_length - 1].clone();
    }

    // version
    self.version.to_string()
  }

  pub fn next_version(&mut self, _type: String) -> String {
    let mut version = Version::parse(self.current_version().as_str()).unwrap();

    match _type.as_str() {
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

    self.version = version.to_string();

    format!("{}{}", self.prefix, self.version)
  }

  fn git_tag(&mut self, repo: &git2::Repository) -> Result<StringArray, git2::Error> {
    repo.tag_names(None)
  }
}
