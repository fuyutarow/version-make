use std::path::PathBuf;
use structopt::StructOpt;

mod lib;
use lib::version::Version;
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
        #[structopt(short = "v", long = "version")]
        maybe_version: String,

        /// Replace the target configuration file when this option is set
        #[structopt(short = "r", long)]
        replace: bool,
    },
    #[structopt(name = "up")]
    Up {
        /// target config file [possible values: Cargo.tomml, package.json, pyproject.toml, manifest.json]
        #[structopt(parse(from_os_str))]
        fpath: PathBuf,

        /// major version
        #[structopt(short = "x", long)]
        major: bool,

        /// minor version
        #[structopt(short = "y", long)]
        minor: bool,

        /// patch version
        #[structopt(short = "z", long)]
        patch: bool,

        #[structopt(short = "a", long)]
        pre: Option<String>,

        /// build version
        #[structopt(short = "b", long)]
        build: Option<String>,

        /// Replace the target configuration file when this option is set
        #[structopt(short = "r", long)]
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
        Opt::Set {
            fpath,
            maybe_version,
            replace,
        } => {
            let mut manager = Manager::load(&fpath);
            let current_version = manager.parse_version();
            if let Ok(version) = Version::parse(&maybe_version) {
                manager = manager.rewrite_version(version);
            } else {
                println!(r#""{}" is invalid as semver"#, maybe_version);
                return;
            }

            let new_version = manager.parse_version();
            if replace {
                manager.overwrite_file();
                println!(
                    "{} was updated {} -> {}",
                    fpath.as_path().to_str().unwrap().to_string(),
                    current_version,
                    new_version
                );
            } else {
                manager.print();
            }
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
            let current_version = manager.parse_version();
            manager = manager.update_version(
                if major { Some(1) } else { None },
                if minor { Some(1) } else { None },
                if patch { Some(1) } else { None },
            );
            manager = manager.set_version(None, None, None, pre, build);
            let new_version = manager.parse_version();

            if replace {
                manager.overwrite_file();
                println!(
                    "{} was updated {} -> {}",
                    fpath.as_path().to_str().unwrap().to_string(),
                    current_version,
                    new_version
                );
            } else {
                manager.print();
            }
        }
    }
}
