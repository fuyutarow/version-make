// use semver;
use semver::Version;
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

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

fn main() {
    match Opt::from_args() {
        Opt::Up {
            fpath,
            major,
            minor,
            patch,
        } => {
            let ver_s = "1.2.3";
            let mut ver = Version::parse(ver_s).unwrap();
            if major {
                ver = ver.up_major(1);
            }
            if minor {
                ver = ver.up_minor(1);
            }
            if patch {
                ver = ver.up_patch(1);
            }
            dbg!(ver);
        }
        _ => {}
    }
}
