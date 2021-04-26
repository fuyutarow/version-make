use std::path::PathBuf;
use structopt::StructOpt;

mod lib;
use lib::Manager;

#[derive(StructOpt, Debug)]
#[structopt(after_help = r##"EXAMPLES
    $ version-make up -zr Cargo.toml
        ... Increment Patch version: x.y.Z+1 and replace new file
    $ version-make up --pre alpha --build beta Cargo.toml
        ... Set pre and build: x.y.z -> x.y.z-alpha+beta
"##)]
enum Opt {
    #[structopt(name = "show")]
    Show {
        /// target config file [possible values: Cargo.tomml, package.json, pyproject.toml, manifest.json]
        #[structopt(parse(from_os_str))]
        fpath: PathBuf,

        /// Show only X.Y.Z without -pre+build if version=X.Y.Z-pre+build
        #[structopt(long = "core")]
        core: bool,
    },
    #[structopt(name = "set")]
    Set {
        /// target config file [possible values: Cargo.tomml, package.json, pyproject.toml, manifest.json]
        #[structopt(parse(from_os_str))]
        fpath: PathBuf,

        /// The version to be set in the configuration file
        #[structopt()]
        version: String,
    },
    #[structopt(name = "up")]
    Up {
        /// target config file [possible values: Cargo.tomml, package.json, pyproject.toml, manifest.json]
        #[structopt(parse(from_os_str))]
        fpath: PathBuf,

        /// major version
        #[structopt(short = "x", long = "major")]
        major: Option<u64>,

        /// minor version
        #[structopt(short = "y", long = "minor")]
        minor: Option<u64>,

        /// patch version
        #[structopt(short = "z", long = "patch")]
        patch: Option<u64>,

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
            let manager = Manager::load(&fpath);

            let ver = if core {
                manager.show_version_core()
            } else {
                manager.show_version()
            };

            println!("{}", &ver)
        }
        Opt::Set { fpath, version } => {
            let manager = Manager::load(&fpath);

            let v = manager.parse_version();
            dbg!(v);
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
            manager = manager.update_version(major, minor, patch);

            if replace {
                manager.overwrite();
            } else {
                manager.print();
            }
        }
    }
}
