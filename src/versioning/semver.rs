use crate::{
    config::{Config, Type},
    versioning::Versioning,
};
use git2::Repository;
use regex::Regex;
use semver::Version;

pub struct Semver {}

impl Versioning for Semver {
    fn current_version(config: &mut Config) -> String {
        let repo = Repository::open(config.dir)
            .map_err(|e| format!("Failed to open: {}", e))
            .unwrap();

        let all_tags = super::git_tag(&repo).unwrap();

        // filter by a specific regexp
        let pattern = format!("{}{}{}", '^', config.prefix, "\\d+\\.\\d+\\.\\d+$");
        let tag_regex = Regex::new(pattern.as_ref()).unwrap();

        // sort git versions
        let mut version_tags: Vec<&str> = all_tags
            .iter()
            .filter(|tag| tag_regex.is_match(tag.unwrap()))
            .map(|tag| tag.unwrap())
            .collect();
        version_tags.sort_by(|a, b| natord::compare(&a, &b));

        // find the greatest version
        let tag_list_length = version_tags.len();

        if tag_list_length != 0 {
            config.version = String::from(version_tags[tag_list_length - 1]);
            return config.version.to_string();
        }

        "0.0.0".to_string()
    }

    fn next_version(config: &mut Config) -> String {
        let mut version = Version::parse(&Self::current_version(config)).unwrap();

        match config.typ {
            Type::Major => version.increment_major(),
            Type::Minor => version.increment_minor(),
            Type::Patch => version.increment_patch(),
        }

        config.version = version.to_string();

        format!("{}{}", config.prefix, config.version)
    }
}
