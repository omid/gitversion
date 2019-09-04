mod tests {
  use super::super::git_version::GitVersion;

  #[test]
  #[should_panic]
  fn wrong_dir() {
    let prefix = String::from("v");
    let dir = String::from("not_existing_dir");

    let mut gitversion = GitVersion::new(dir, prefix);

    gitversion.next_version(String::from("patch"));
  }

  #[test]
  #[should_panic]
  fn wrong_type() {
    let prefix = String::from("v");
    let dir = String::from(".");

    let mut gitversion = GitVersion::new(dir, prefix);

    gitversion.next_version(String::from("not_existing_type"));
  }

  #[test]
  fn generated_version_patch() {
    let prefix = String::from("v");
    let dir = String::from(".");

    let mut gitversion = GitVersion::new(dir, prefix);

    assert_eq!(gitversion.next_version(String::from("patch")), "v0.0.1");
  }

  #[test]
  fn generated_version_minor() {
    let prefix = String::from("");
    let dir = String::from(".");

    let mut gitversion = GitVersion::new(dir, prefix);

    assert_eq!(gitversion.next_version(String::from("minor")), "0.1.0");
  }

  #[test]
  fn generated_version_major() {
    let prefix = String::from("");
    let dir = String::from(".");

    let mut gitversion = GitVersion::new(dir, prefix);

    assert_eq!(gitversion.next_version(String::from("major")), "1.0.0");
  }

  #[test]
  fn generated_version() {
    let prefix = String::from("v");
    let dir = String::from(".");

    let mut gitversion = GitVersion::new(dir, prefix);

    gitversion.next_version(String::from("major"));
    gitversion.next_version(String::from("major"));
    gitversion.next_version(String::from("minor"));
    let last_version = gitversion.next_version(String::from("patch"));

    assert_eq!(last_version, "v2.1.1");
  }
}
