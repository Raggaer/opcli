mod args;
mod clipboard;
mod command;

use std::env;
use std::process;

fn main() {
    // Build args vector removing first element (application name)
    let args: Vec<String> = env::args()
        .enumerate()
        .filter(|&(i, _)| i > 0)
        .map(|(_, e)| e)
        .collect();

    let opts = args::build_args();
    let opts_matches = match opts.parse(args) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    run(opts, opts_matches);
}

fn run(opts: getopts::Options, matches: getopts::Matches) {
    if matches.opt_present("h") {
        args::print_usage(opts);
        process::exit(1);
    }
    match matches.opt_str("c") {
        Some(command) => match command.as_str() {
            "get" => command::execute_command(
                matches.opt_str("s"),
                matches.opt_str("i"),
                matches.opt_str("f"),
                matches.opt_present("pw"),
            ),
            _ => (),
        },
        None => process::exit(1),
    }
}
