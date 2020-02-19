mod tests {
  use super::super::git_version::GitVersion;

  #[test]
  #[should_panic]
  fn wrong_dir() {
    let prefix = "v";
    let dir = "not_existing_dir";

    let mut gitversion = GitVersion::new(dir, prefix);

    gitversion.next_version("patch");
  }

  #[test]
  #[should_panic]
  fn wrong_type() {
    let prefix = "v";
    let dir = ".";

    let mut gitversion = GitVersion::new(dir, prefix);

    gitversion.next_version("not_existing_type");
  }

  #[test]
  fn generated_version_none() {
    let prefix = "v";
    let dir = ".";

    let mut gitversion = GitVersion::new(dir, prefix);

    assert_eq!(gitversion.next_version("none"), "v0.0.0");
  }

  #[test]
  fn generated_version_patch() {
    let prefix = "v";
    let dir = ".";

    let mut gitversion = GitVersion::new(dir, prefix);

    assert_eq!(gitversion.next_version("patch"), "v0.0.1");
  }

  #[test]
  fn generated_version_minor() {
    let prefix = "";
    let dir = ".";

    let mut gitversion = GitVersion::new(dir, prefix);

    assert_eq!(gitversion.next_version("minor"), "0.1.0");
  }

  #[test]
  fn generated_version_major() {
    let prefix = "";
    let dir = ".";

    let mut gitversion = GitVersion::new(dir, prefix);

    assert_eq!(gitversion.next_version("major"), "1.0.0");
  }

  #[test]
  fn generated_version() {
    let prefix = "v";
    let dir = ".";

    let mut gitversion = GitVersion::new(dir, prefix);

    gitversion.next_version("major");
    gitversion.next_version("major");
    gitversion.next_version("minor");
    let last_version = gitversion.next_version("patch");

    assert_eq!(last_version, "v2.1.1");
  }
}
