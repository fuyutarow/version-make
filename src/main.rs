// use semver;
use regex;
use semver::Version;
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use toml::Value as Toml;

#[derive(StructOpt, Debug)]
enum Opt {
    #[structopt(name = "up")]
    Up {
        #[structopt(parse(from_os_str))]
        fpath: Option<PathBuf>,
        #[structopt(short = "x", long = "major")]
        major: bool,
        #[structopt(short = "y", long = "minor")]
        minor: bool,
        #[structopt(short = "z", long = "pathc")]
        patch: bool,
    },
}

trait Semver {
    fn up_major(self, n: u64) -> Self;
    fn up_minor(self, n: u64) -> Self;
    fn up_patch(self, n: u64) -> Self;
}

impl Semver for Version {
    fn up_major(self, n: u64) -> Self {
        Version {
            major: self.clone().major + n,
            ..self
        }
    }
    fn up_minor(self, n: u64) -> Self {
        Version {
            minor: self.clone().minor + n,
            ..self
        }
    }
    fn up_patch(self, n: u64) -> Self {
        Version {
            patch: self.clone().patch + n,
            ..self
        }
    }
}

#[derive(Debug, Deserialize)]
struct Cargo {
    package: CargoPackage,
}

impl Cargo {
    fn load(fpath: &Path) -> Self {
        let mut f = File::open(fpath).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        /// toml validation
        match contents.parse::<Toml>() {
            Ok(toml_string) => {}
            Err(error) => panic!("failed to parse TOML: {}", error),
        }

        let this: Self = toml::from_str(&contents).expect("failed to parse TOML");
        this
    }
}

#[derive(Debug, Deserialize)]
struct CargoPackage {
    version: String,
}

fn main() {
    match Opt::from_args() {
        Opt::Up {
            fpath,
            major,
            minor,
            patch,
        } => {
            let fpath = Path::new("Cargo.toml");
            let mut f = File::open(fpath).expect("file not found");
            let mut contents = String::new();
            f.read_to_string(&mut contents)
                .expect("something went wrong reading the file");
            let version_template = |version: &str| {
                format!(
                    r#"(\s*version\s*=\s*["|']){version}(["|']\n)"#,
                    version = version
                )
            };

            let re_version =
                regex::Regex::new(&version_template(r#"(?P<version>[a-zA-Z0-9-+.]+)"#)).unwrap();
            dbg!(&re_version);
            let caps = re_version.captures(&contents).unwrap();
            let ver_s = &caps["version"];

            let mut ver = Version::parse(&ver_s).unwrap();
            if major {
                ver = ver.up_major(1);
            }
            if minor {
                ver = ver.up_minor(1);
            }
            if patch {
                ver = ver.up_patch(1);
            }
            dbg!(&ver);
            let ver_t: String = format!("{}{}{}", &caps[1], &ver.to_string().as_str(), &caps[3]);
            dbg!(&ver_t);
            let result = re_version
                .replace_all(&contents, ver_t.as_str())
                .to_string();

            dbg!(&result);
            let mut file = File::create("new.toml").unwrap();
            writeln!(&mut file, "{}", &result).unwrap();
        }
        _ => {}
    }
}
