extern crate clap;

mod server;
mod config;
mod command;

use clap::{Arg, App};
use std::env;

use server::{Server, Account};

fn main() {
    let matches = App::new("fissh")
        .version("0.0.0")
        .author("Jan D. <jd84@protonmail.com>")
        .about("fissh is a ssh wrapper and connection manager.")
        .arg(Arg::with_name("config")
            .short("c")
            .help("The configuration file for fissh")
            .takes_value(true)
        )
        .arg(Arg::with_name("list")
            .short("l")
            .help("List all configured hosts")
        )
        .arg(Arg::with_name("HOST_OR_GROUP")
            .help("The host used for the next connection")
            .index(1)
            .required_unless_one(&["list", "Version"])
        )
        .get_matches();

    let mut default_file = env::var_os("HOME").unwrap();
    default_file.push("/.ssh/fissh.yml");

    let config_file = matches.value_of("config").unwrap_or(default_file.to_str().unwrap());
    let config = config::Config::from_file(config_file);

    if matches.is_present("list") {
        match matches.value_of("HOST_OR_GROUP") {
            Some(group) => print_servers(group, config.server_manager().get_servers(group)),
            None => {
                for group in config.server_manager().groups() {
                    print_servers(group, config.server_manager().get_servers(group));
                }
            }
        }
        std::process::exit(0);
    }

    if let Some(host) = matches.value_of("HOST_OR_GROUP") {
        let server = config.server_manager().find(host);
        let account = config.credential_manager().find(&server.users[0]);
        connect(server, account);
    }
    
    println!("Thanks for using fissh!");
}

fn connect(server: &Server, account: &Account) {
    println!("Start SSH for {} as {}", server.host, account.name);
    
    let mut ssh = command::Ssh::with(server, account);
    ssh.run();
}

fn print_servers(group: &str, servers: &Vec<Server>) {
    println!("{}\n", group);

    let mut i = 0;
    for server in servers {
        i += 1;
        if i % 4 == 0 {
            println!("\t{0: <10} {1: <25}", server.name, server.host);
        } else {
            print!("\t{0: <10} {1: <25}", server.name, server.host);
        }
    }
    println!("\n");
}
