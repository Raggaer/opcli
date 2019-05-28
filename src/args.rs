use getopts::Options;

pub fn build_args() -> getopts::Options {
    let mut opts = Options::new();
    opts.optflag("h", "help", "Print the help menu");
    opts
}

pub fn print_usage(opts: getopts::Options) {
    let brief = String::from("Usage: opcli [options]");
    print!("{}", opts.usage(&brief));
}
