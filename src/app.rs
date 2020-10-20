use crate::Transfer;
use clap::{App, Arg, ArgMatches};
use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const HOME: &str = env!("HOME");

pub enum Format {
    Pretty,
    None,
}

pub enum RunMode<'a> {
    List(Option<&'a str>, Format),
    Ssh(&'a str),
    Scp(&'a str, Transfer<'a>),
    Unknown,
}

pub struct Application<'a> {
    args: ArgMatches<'a>,
}

impl<'a> Application<'a> {
    pub fn get_run_mode(&self) -> RunMode {
        // list run mode
        if self.args.is_present("list") {
            let group = self.args.value_of("host_or_group");
            let format;
            if self.args.value_of("format").unwrap() == "pretty" {
                format = Format::Pretty;
            } else {
                format = Format::None;
            }

            return RunMode::List(group, format);
        }
        // ssh run mode
        if self.args.is_present("host_or_group") && !self.args.is_present("to_or_from") {
            let host = self.args.value_of("host_or_group").unwrap();
            return RunMode::Ssh(host);
        }
        // scp run mode
        if self.args.is_present("host_or_group") && self.args.is_present("to_or_from") {
            let lhalf = self.args.value_of("host_or_group").unwrap();
            let rhalf = self.args.value_of("to_or_from").unwrap();

            if lhalf.contains(":") {
                let lparts: Vec<&str> = lhalf.split(":").collect();
                let host = lparts[0];
                let dest = lparts[1];
                let transfer = Transfer::FromHost {
                    from: dest,
                    to: rhalf,
                };
                return RunMode::Scp(host, transfer);
            } else if rhalf.contains(":") {
                let rparts: Vec<&str> = rhalf.split(":").collect();
                let host = rparts[0];
                let dest = rparts[1];
                let transfer = Transfer::ToHost {
                    from: lhalf,
                    to: dest,
                };
                return RunMode::Scp(host, transfer);
            } else {
                return RunMode::Unknown;
            }
        }

        RunMode::Unknown
    }

    pub fn get_config_file(&self) -> String {
        let mut config_file = self.args.value_of("config").unwrap().to_string();
        if config_file.contains("~") {
            config_file = config_file.replace("~", HOME);
        }
        config_file
    }
}

impl<'a> Default for Application<'a> {
    fn default() -> Application<'a> {
        let mut default_config = HOME.to_owned();
        default_config.push_str("/.ssh/russh.toml");

        let args = App::new("russh")
            .version(VERSION)
            .author("jd84 <jd84@protonmail.com>")
            .about("SSH and SCP connection manager")
            .arg(
                Arg::with_name("config")
                    .short("c")
                    .default_value("~/.ssh/russh.toml")
                    .help("The configuration file for russh"),
            )
            .arg(
                Arg::with_name("list")
                    .short("l")
                    .help("Prints available hosts"),
            )
            .arg(
                Arg::with_name("format")
                    .short("f")
                    .possible_values(&["pretty", "none"])
                    .required_if("list", "pretty")
                    .default_value("pretty")
                    .help("Specifies the output formatting"),
            )
            .arg(
                Arg::with_name("host_or_group")
                    .index(1)
                    .required_unless_one(&["list", "Version"])
                    .help("host for connection or group to list"),
            )
            .arg(
                Arg::with_name("to_or_from")
                    .index(2)
                    .help("The source or destination for scp"),
            )
            .get_matches();

        Application { args }
    }
}
