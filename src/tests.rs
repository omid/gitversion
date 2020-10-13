#[cfg(test)]
mod tests {
    use super::super::{config::Config, next_version};
    use crate::config::{Format, Type};

    // GENERAL TESTS
    #[test]
    #[should_panic]
    fn general_wrong_dir() {
        let prefix = "v";
        let dir = "not_existing_dir";

        let mut config = Config::new(dir, prefix, Type::Patch, Format::Semver);

        next_version(&mut config);
    }

    // SEMVER TESTS
    #[test]
    fn semver_wrong_type() {
        let prefix = "v";
        let dir = ".";

        let mut config = Config::new(dir, prefix, "not_existing_type".into(), Format::Semver);

        assert_eq!(next_version(&mut config), "v0.0.1");
    }

    #[test]
    fn semver_generated_version_patch() {
        let prefix = "v";
        let dir = ".";

        let mut config = Config::new(dir, prefix, Type::Patch, Format::Semver);

        assert_eq!(next_version(&mut config), "v0.0.1");
    }

    #[test]
    fn semver_generated_version_minor() {
        let prefix = "";
        let dir = ".";

        let mut config = Config::new(dir, prefix, Type::Minor, Format::Semver);

        assert_eq!(next_version(&mut config), "0.1.0");
    }

    #[test]
    fn semver_generated_version_major() {
        let prefix = "";
        let dir = ".";

        let mut config = Config::new(dir, prefix, Type::Major, Format::Semver);

        assert_eq!(next_version(&mut config), "1.0.0");
    }

    // SEQUENTIAL TESTS
    #[test]
    fn sequential_wrong_type() {
        let prefix = "v";
        let dir = ".";

        let mut config = Config::new(dir, prefix, "not_existing_type".into(), Format::Sequential);

        assert_eq!(next_version(&mut config), "v1");
    }

    #[test]
    fn sequential_generated_version() {
        let prefix = "v";
        let dir = ".";

        let mut config = Config::new(dir, prefix, Type::Patch, Format::Sequential);

        assert_eq!(next_version(&mut config), "v1");
    }
}
