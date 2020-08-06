extern crate json;
use json::parse;

#[macro_use] extern crate log;
extern crate loggerv;

extern crate structopt;
// #[macro_use] extern crate structopt_derive;
use structopt::StructOpt;

use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use std::process::exit;

mod frame;

/// Do fancy things
#[derive(StructOpt, Debug)]
#[structopt(name = "fireframe")]
struct Cli {
    /// The source, possibly unfancy
    #[structopt()]
    app: Option<String>,

    /// Enable logging, use multiple `v`s to increase verbosity
    #[structopt(short = "v", long = "verbose")]
    verbosity: Option<u64>,

    /// Output file
    #[structopt(long = "debug", short = "d")]
    output: bool,
}

#[macro_export]
macro_rules! err {
    ($( $x:expr ),*) => {
        error!($($x,)*);
        exit(1);
    }
}

fn main() {
    let args = Cli::from_args();

    loggerv::init_with_verbosity(args.verbosity.unwrap_or(0)).expect("yada");

    let mut example = env::current_exe().expect("failure to get cwd").parent().expect("failure to get cwd parent").to_path_buf().clone();
    example.push("example");

    let app = args.app.and_then(|p: String| Some(PathBuf::from(p))).or(Some(example)).expect("failure to unwrap app");

    info!("launching {}", app.to_str().expect("yada"));

    let mut package_json = app.clone();

    package_json.push("package.json");

    if !package_json.is_file() {
        err!("couldn't find {}, not a launchable app", package_json.to_str().expect("yada"));
    }

    debug!("package.json {}", package_json.to_str().expect("yada"));

    let mut file = File::open(package_json).expect("failed to load package.json");
    let mut data = String::new();

    file.read_to_string(&mut data).expect("failure to read package.json");

    let meta = parse(&data).expect("package.json is formatted wrongly");
    let launcher = &meta["launcher"];
    if !launcher.is_object() {
        err!("launcher settings not found, got {:?}", meta);
    }

    debug!("launching with {:?}", launcher);

    if !launcher["main"].is_string() {
        err!("launcher.main not found");
    }

    let mut main_file = app.clone();

    main_file.push(launcher["main"].as_str().expect("iyada"));

    info!("main {}", main_file.to_str().expect("yada"));

    frame::main();
}
