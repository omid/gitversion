pub struct Config<'a> {
    pub dir: &'a str,
    pub prefix: &'a str,
    pub typ: Type,
    pub format: Format,
    pub version: String,
}

pub enum Type {
    Major,
    Minor,
    Patch,
}

impl From<&str> for Type {
    fn from(input: &str) -> Self {
        match input {
            "major" => Self::Major,
            "minor" => Self::Minor,
            "patch" => Self::Patch,
            _ => Self::default(),
        }
    }
}

impl Default for Type {
    fn default() -> Self {
        Self::Patch
    }
}

pub enum Format {
    Semver,
    Sequential,
}

impl From<&str> for Format {
    fn from(input: &str) -> Self {
        match input {
            "semver" => Self::Semver,
            "sequential" => Self::Sequential,
            _ => Self::default(),
        }
    }
}

impl Default for Format {
    fn default() -> Self {
        Self::Semver
    }
}

impl Config<'_> {
    pub fn new<'a>(dir: &'a str, prefix: &'a str, typ: Type, format: Format) -> Config<'a> {
        Config {
            dir,
            prefix,
            typ,
            format,
            version: "".to_string(),
        }
    }
}
