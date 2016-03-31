extern crate moka_conf;
extern crate rustc_serialize;
extern crate docopt;
extern crate regex;
extern crate core;

use moka_conf::util;

use docopt::Docopt;
use core::ops::Add;

const MOKA_CONF_VERSION : &'static str = env!("CARGO_PKG_VERSION");
const USAGE : &'static str = "
Create a new moka module configuration file

Usage:
    mokaconf new
    mokaconf --help
    mokaconf --version

Options:
    -h, --help      Show this text
    -v, --version       Show the installed Moka version
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_version: bool,
    flag_help: bool,

    cmd_new: bool
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|opts| opts.decode())
                            .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("v{}", MOKA_CONF_VERSION);
        std::process::exit(0);
    }

    if args.flag_help {
        println!("{}", USAGE);
        std::process::exit(0);
    }

    let name = util::ask_with_default("Module name", util::get_pwd_name());
    let author = util::ask_with_default("Author", util::get_default_user());
    let license = util::ask_with_default("License", String::from("MIT"));

    let proposed = format!(r#"[meta]
name = "{0}"
version = "0.0.1"
author = "{1}"
license = "{2}"

[options]
"#, name, author, license);

    println!("Proposed configuration file:\n'{}'", proposed);
    let answer = util::ask_with_custom_default("Is this correct?", "Y/n", &String::from("y")).to_lowercase();
    let result = match &(*answer) {
        "y" => util::write_toml_out(util::get_pwd().add("/module.toml"), proposed),
        _ =>  {
            println!("Aborting config write");
            std::process::exit(0);
            // Exit due to a no condition is a regular status, no program error occured
        }
    };
    result.unwrap();
}
