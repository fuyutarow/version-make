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
    #[structopt(name = "up")]
    Up {
        #[structopt(parse(from_os_str))]
        fpath: PathBuf,
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
            _ => format!(
                r#"(\s*version\s*=\s*["|']){version}(["|']\n)"#,
                version = version
            ),
        }
    }

    fn update_version(self, (major, minor, patch): (bool, bool, bool)) -> Self {
        let re_version = regex::Regex::new(
            &self
                .clone()
                .version_template(r#"(?P<version>[a-zA-Z0-9-+.]+)"#),
        )
        .unwrap();
        let caps = re_version.captures(&self.contents).unwrap();
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

    fn save(self, out_path: &str) {
        let mut file = File::create(out_path).unwrap();
        writeln!(&mut file, "{}", &self.contents).unwrap();
    }

    fn print(self) {
        print!("{}", &self.contents);
    }
}

fn main() {
    match Opt::from_args() {
        Opt::Up {
            fpath,
            major,
            minor,
            patch,
        } => {
            let mut manager = Manager::load(&fpath);
            manager = manager.update_version((major, minor, patch));
            // manager.save("new.toml");
            manager.print();
        }
        _ => {}
    }
}
