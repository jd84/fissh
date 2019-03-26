extern crate clap;

mod server;
mod config;
mod command;

use clap::{Arg, App};
use std::env;

use server::{ServerManager, Server, CredentialManager, Account};

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
        .arg(Arg::with_name("HOST")
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
        for group in config.server_manager().groups().iter() {
            println!("{}\n", group);
            let mut loops = 0;
            for server in config.server_manager().iter(group) {
                loops += 1;
                if loops % 4 == 0 {
                    println!("\t{} ({})", server.name, server.host);
                } else {
                    print!("\t{} ({})", server.name, server.host);
                }
            }
            println!("");
            println!("");
        }
        
        std::process::exit(0);
    }

    if let Some(host) = matches.value_of("HOST") {
        connect(config.server_manager(), config.credential_manager(), host);
    }
    
    println!("Thanks for using fissh!");
}

fn connect(servers: &ServerManager, accounts: &CredentialManager, name: &str) {
    let server: &Server = servers.find(String::from(name));
    let account: &Account = accounts.find(&server.users[0]);

    println!("Start SSH for {} as {}", server.host, account.name);
    
    let mut ssh = command::Ssh::with(server, account);
    ssh.run();
}
