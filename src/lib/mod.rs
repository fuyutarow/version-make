use regex;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub mod version;
use version::{Semver, Version};

#[derive(Debug, Deserialize)]
struct CargoPackage {
    version: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Manager {
    fpath: PathBuf,
    contents: String,
}

enum Operation {
    set,
    update,
}

impl Manager {
    pub fn load(fpath: &PathBuf) -> Self {
        let mut f = File::open(fpath).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        Self {
            fpath: fpath.to_owned(),
            contents,
        }
    }

    fn version_template(self, version: &str) -> String {
        let fpath: Option<String> = match self.fpath.file_name() {
            Some(os_str) => Some(os_str.to_str().unwrap().to_owned()),
            None => None,
        };
        match fpath.unwrap_or("".to_owned()).as_str() {
            "Cargo.toml" | "pyproject.toml" => format!(
                r#"(\s*version\s*=\s*["|']){version}(["|']\n)"#,
                version = version
            ),
            "package.json" | "manifest.json" => {
                format!(r#"("version":\s*"){version}(")"#, version = version)
            }
            "vue.config.js" => format!(
                r#"(process\.env\.VUE_APP_VERSION\s*=\s*["|']){version}(["|'];*)"#,
                version = version
            ),
            _ => format!(
                r#"(\s*version\s*=\s*["|']){version}(["|']\n)"#,
                version = version
            ),
        }
    }

    pub fn parse_version(&self) -> Version {
        let re_version = regex::Regex::new(
            &self
                .clone()
                .version_template(r#"(?P<version>[a-zA-Z0-9-+.]+)"#),
        )
        .unwrap();
        let caps = re_version.captures(&self.contents).unwrap();
        let ver_s = &caps["version"];
        let current_version = Version::parse(&ver_s).unwrap();
        current_version
    }

    pub fn rewrite_version(self, version: Version) -> Self {
        let re_version = regex::Regex::new(
            &self
                .clone()
                .version_template(r#"(?P<version>[a-zA-Z0-9-+.]+)"#),
        )
        .unwrap();
        let caps = re_version.captures(&self.contents).unwrap();
        let _ver_s = &caps["version"];

        let ver_t: String = format!("{}{}{}", &caps[1], &version.to_string().as_str(), &caps[3]);
        let re_version = regex::Regex::new(
            &self
                .clone()
                .version_template(r#"(?P<version>[a-zA-Z0-9-+.]+)"#),
        )
        .unwrap();
        let contents = re_version
            .replace_all(&self.contents, ver_t.as_str())
            .to_string();

        Self { contents, ..self }
    }

    pub fn set_version(
        self,
        major: Option<u64>,
        minor: Option<u64>,
        patch: Option<u64>,
        pre: Option<String>,
        build: Option<String>,
    ) -> Self {
        let mut version = self.parse_version();
        version = version.set(major, minor, patch, pre, build);
        self.rewrite_version(version)
    }

    pub fn update_version(
        self,
        major: Option<u64>,
        minor: Option<u64>,
        patch: Option<u64>,
    ) -> Self {
        let mut version = self.parse_version();
        version = version.update(major, minor, patch);
        self.rewrite_version(version)
    }

    fn save(self, out_path: &PathBuf) {
        let mut file = File::create(out_path).unwrap();
        write!(&mut file, "{}", &self.contents).unwrap();
    }

    pub fn overwrite_file(self) {
        self.clone().save(&self.fpath)
    }

    pub fn print(self) {
        print!("{}", &self.contents);
    }

    pub fn show_version_core(self) -> String {
        let mut ver = self.parse_version();
        ver = ver.set(None, None, None, Some("".to_string()), Some("".to_string()));
        ver.to_string()
    }

    pub fn show_version(self) -> String {
        self.parse_version().to_string()
    }
}
