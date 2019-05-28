use getopts::Options;

pub fn build_args() -> getopts::Options {
    let mut opts = Options::new();
    opts.optflag("h", "help", "Print the help menu");
    opts.optopt("c", "command", "Root command to execute", "");
    opts.optopt("s", "sub", "Subcommand to execute", "");
    opts.optopt("i", "item", "Item, mostly for 'get item' calls", "");
    opts.optopt("f", "fields", "List of fields to retrieve", "*");
    opts
}

pub fn print_usage(opts: getopts::Options) {
    let brief = String::from("Usage: opcli [options]");
    print!("{}", opts.usage(&brief));
}
