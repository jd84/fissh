use clap::{App, Arg};

pub fn get_matches<'a>() -> clap::ArgMatches<'a> {
    App::new("russh")
        .version("0.1.0")
        .author("Jan D. <jd84@protonmail.com>")
        .about("russh is a ssh wrapper and connection manager.")
        .arg(
            Arg::with_name("CONFIG")
                .short("c")
                .help("The configuration file for russh. Default is `~/.russh/russh.yml`")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list")
                .help("List all configured hosts"),
        )
        .arg(
            Arg::with_name("HOST_OR_GROUP")
                .help("The host used for the next connection")
                .index(1)
                .required_unless_one(&["list", "Version"]),
        )
        .arg(
            Arg::with_name("TO_OR_FROM")
                .help("The source or destination for scp")
                .index(2),
        )
        .arg(
            Arg::with_name("format")
                .short("f")
                .help("Output formatting")
                .possible_values(&["table", "none"])
                .default_value("table")
                .takes_value(true),
        )
        .get_matches()
}
