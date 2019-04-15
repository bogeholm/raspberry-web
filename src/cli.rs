use clap::{App, Arg, ArgMatches};

/// Parse command line arguments
pub fn get_cli_args() -> ArgMatches<'static> {
    App::new("Raspberry Web")
        .author("Troels Mikkelsen <troelsmikkelsen@gmail.com>")
        .about("Control GPIO ports on your Raspberry Pi over the network")
        .arg(
            Arg::with_name("config-file")
                .short("c")
                .long("config-file")
                .value_name("FILE")
                .help("Set a custom config file")
                .takes_value(true)
                .required(false),
        )
        .get_matches()
}
