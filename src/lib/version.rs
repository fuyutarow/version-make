pub use semver::Version;
use serde_derive::{Deserialize, Serialize};

pub trait Semver {
    fn set(
        &mut self,
        major: Option<u64>,
        minor: Option<u64>,
        patch: Option<u64>,
        pre: Option<String>,
        build: Option<String>,
    ) -> Self;
    fn update(&mut self, major: Option<u64>, minor: Option<u64>, patch: Option<u64>) -> Self;
}

impl Semver for Version {
    fn set(
        &mut self,
        major: Option<u64>,
        minor: Option<u64>,
        patch: Option<u64>,
        pre: Option<String>,
        build: Option<String>,
    ) -> Self {
        Self {
            major: if let Some(n) = major { n } else { self.major },
            minor: if let Some(n) = minor { n } else { self.minor },
            patch: if let Some(n) = patch { n } else { self.patch },
            pre: match pre {
                Some(s) if &s == "" => vec![],
                Some(s) => vec![semver::Identifier::AlphaNumeric(s.into())],
                _ => self.pre.clone(),
            },
            build: match build {
                Some(s) if &s == "" => vec![],
                Some(s) => vec![semver::Identifier::AlphaNumeric(s.into())],
                _ => self.build.clone(),
            },
        }
    }

    fn update(&mut self, major: Option<u64>, minor: Option<u64>, patch: Option<u64>) -> Self {
        if let Some(n) = major {
            for _ in 0..n {
                self.increment_major();
            }
        }
        if let Some(n) = minor {
            for _ in 0..n {
                self.increment_minor();
            }
        }
        if let Some(n) = patch {
            for _ in 0..n {
                self.increment_patch();
            }
        }
        self.to_owned()
    }
}

#[derive(Debug, Deserialize)]
struct CargoPackage {
    version: String,
}
