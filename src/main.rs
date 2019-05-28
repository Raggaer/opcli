mod args;

use std::env;
use std::process;

use getopts::Matches;
use getopts::Options;

fn main() {
    // Build args vector removing first element (application name)
    let args: Vec<String> = env::args()
        .enumerate()
        .filter(|&(i, _)| i > 0)
        .map(|(_, e)| e)
        .collect();

    let opts = args::build_args();
    let mut opts_matches = match opts.parse(args) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Unable to parse application arguments: {}", e);
            process::exit(1);
        }
    };
    run(opts, opts_matches);
}

fn run(opts: getopts::Options, matches: getopts::Matches) {
    if matches.opt_present("h") {
        args::print_usage(opts);
    }
}
