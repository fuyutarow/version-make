// use semver;
use regex;
use semver::Version;
use serde_derive::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use toml::Value as Toml;

#[derive(StructOpt, Debug)]
enum Opt {
    #[structopt(name = "show")]
    Show {
        #[structopt(parse(from_os_str))]
        fpath: PathBuf,
        #[structopt(long = "core")]
        core: bool,
    },
    #[structopt(name = "up")]
    Up {
        #[structopt(parse(from_os_str))]
        fpath: PathBuf,
        #[structopt(short = "x", long = "major")]
        major: bool,
        #[structopt(short = "y", long = "minor")]
        minor: bool,
        #[structopt(short = "z", long = "patch")]
        patch: bool,
        #[structopt(short = "p", long = "pre")]
        pre: Option<String>,
        #[structopt(short = "b", long = "build")]
        build: Option<String>,
        #[structopt(short = "r", long = "replace")]
        replace: bool,
    },
}

trait Semver {
    fn up_major(self, n: u64) -> Self;
    fn up_minor(self, n: u64) -> Self;
    fn up_patch(self, n: u64) -> Self;
    fn set_pre(self, pre: Option<String>) -> Self;
    fn set_build(self, build: Option<String>) -> Self;
    fn update(
        &mut self,
        major: bool,
        minor: bool,
        patch: bool,
        pre: Option<String>,
        build: Option<String>,
    ) -> Self;
}

impl Semver for Version {
    fn up_major(self, n: u64) -> Self {
        Version {
            major: self.clone().major + n,
            minor: 0,
            patch: 0,
            ..self
        }
    }

    fn up_minor(self, n: u64) -> Self {
        Version {
            minor: self.clone().minor + n,
            patch: 0,
            ..self
        }
    }

    fn up_patch(self, n: u64) -> Self {
        Version {
            patch: self.clone().patch + n,
            ..self
        }
    }

    fn set_pre(self, pre: Option<String>) -> Self {
        match pre {
            Some(pre_s) if &pre_s == "" => Version {
                pre: vec![],
                ..self
            },
            Some(pre_s) => Version {
                pre: vec![semver::Identifier::AlphaNumeric(pre_s.into())],
                ..self
            },
            None => self,
        }
    }

    fn set_build(self, build: Option<String>) -> Self {
        match build {
            Some(build_s) if &build_s == "" => Version {
                build: vec![],
                ..self
            },
            Some(build_s) => Version {
                build: vec![semver::Identifier::AlphaNumeric(build_s.into())],
                ..self
            },
            None => self,
        }
    }

    fn update(
        &mut self,
        major: bool,
        minor: bool,
        patch: bool,
        pre: Option<String>,
        build: Option<String>,
    ) -> Self {
        let mut ver = self.clone();
        if major {
            ver = ver.up_major(1);
        }
        if minor {
            ver = ver.up_minor(1);
        }
        if patch {
            ver = ver.up_patch(1);
        }
        ver = ver.set_pre(pre);
        ver = ver.set_build(build);
        return ver;
    }
}

#[derive(Debug, Deserialize)]
struct CargoPackage {
    version: String,
}

#[derive(Debug, Clone, Deserialize)]
struct Manager {
    fpath: PathBuf,
    contents: String,
}

impl Manager {
    fn load(fpath: &PathBuf) -> Self {
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
            "Cargo.toml" => format!(
                r#"(\s*version\s*=\s*["|']){version}(["|']\n)"#,
                version = version
            ),
            "package.json" => format!(r#"("version":\s*"){version}(")"#, version = version),
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

    fn parse_version(self) -> Version {
        let re_version = regex::Regex::new(
            &self
                .clone()
                .version_template(r#"(?P<version>[a-zA-Z0-9-+.]+)"#),
        )
        .unwrap();
        let caps = re_version.captures(&self.contents).unwrap();
        let ver_s = &caps["version"];
        return Version::parse(&ver_s).unwrap();
    }

    fn update_version(
        self,
        (major, minor, patch, pre, build): (bool, bool, bool, Option<String>, Option<String>),
    ) -> Self {
        let re_version = regex::Regex::new(
            &self
                .clone()
                .version_template(r#"(?P<version>[a-zA-Z0-9-+.]+)"#),
        )
        .unwrap();
        let caps = re_version.captures(&self.contents).unwrap();
        let ver_s = &caps["version"];
        let mut ver = Version::parse(&ver_s).unwrap();
        ver = ver.update(major, minor, patch, pre, build);

        let ver_t: String = format!("{}{}{}", &caps[1], &ver.to_string().as_str(), &caps[3]);
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

    fn save(self, out_path: &PathBuf) {
        let mut file = File::create(out_path).unwrap();
        write!(&mut file, "{}", &self.contents).unwrap();
    }

    fn overwrite(self) {
        self.clone().save(&self.fpath)
    }
    fn print(self) {
        print!("{}", &self.contents);
    }
}

fn main() {
    match Opt::from_args() {
        Opt::Show { fpath, core } => {
            let mut manager = Manager::load(&fpath);
            let mut ver = manager.parse_version();

            if (core) {
                ver = ver.update(false, false, false, Some("".into()), Some("".into()));
            }

            let s = ver.to_string();
            println!("{}", &s)
        }
        Opt::Up {
            fpath,
            major,
            minor,
            patch,
            pre,
            build,
            replace,
        } => {
            let mut manager = Manager::load(&fpath);
            manager = manager.update_version((major, minor, patch, pre, build));

            if replace {
                manager.overwrite();
            } else {
                manager.print();
            }
        }
        _ => {}
    }
}
