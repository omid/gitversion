use crate::config::Config;
use git2::string_array::StringArray;

pub mod semver;
pub mod sequential;

fn git_tag(repo: &git2::Repository) -> Result<StringArray, git2::Error> {
    repo.tag_names(None)
}

pub trait Versioning {
    fn current_version(config: &mut Config) -> String;
    fn next_version(config: &mut Config) -> String;
}
