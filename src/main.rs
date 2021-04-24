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

mod lib;
use lib::Manager;

#[derive(StructOpt, Debug)]
#[structopt(after_help = r##"EXAMPLES
    $ version-make up -z -r Cargo.toml
        ... Increment Patch version: x.y.Z+1 and replace new file
    $ version-make up --pre alpha --build beta Cargo.toml
        ... Set pre and build: x.y.z -> x.y.z-alpha+beta
"##)]
enum Opt {
    #[structopt(name = "show")]
    Show {
        /// target config file [possible values: Cargo.tomml, package.json, pyproject.toml]
        #[structopt(parse(from_os_str))]
        fpath: PathBuf,

        /// Show only X.Y.Z without -pre+build if version=X.Y.Z-pre+build
        #[structopt(long = "core")]
        core: bool,
    },
    #[structopt(name = "up")]
    Up {
        /// target config file [possible values: Cargo.tomml, package.json, pyproject.toml]
        #[structopt(parse(from_os_str))]
        fpath: PathBuf,

        /// major version
        #[structopt(short = "x", long = "major")]
        major: bool,

        /// minor version
        #[structopt(short = "y", long = "minor")]
        minor: bool,

        /// patch version
        #[structopt(short = "z", long = "patch")]
        patch: bool,

        /// pre version
        #[structopt(short = "p", long = "pre")]
        pre: Option<String>,

        /// build version
        #[structopt(short = "b", long = "build")]
        build: Option<String>,

        /// Replace the target configuration file when this option is set
        #[structopt(short = "r", long = "replace")]
        replace: bool,
    },
}

fn main() {
    match Opt::from_args() {
        Opt::Show { fpath, core } => {
            let mut manager = Manager::load(&fpath);

            let ver = if (core) {
                manager.show_version_core()
            } else {
                manager.show_version()
            };

            println!("{}", &ver)
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
