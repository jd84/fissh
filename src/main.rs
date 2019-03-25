extern crate clap;

mod server;
mod config;
mod command;

use clap::{Arg, App};

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
        )
        .get_matches();

    let config_file = matches.value_of("config").unwrap_or("~/.ssh/fissh.yml");
    let config = config::Config::from_file(config_file);

    if matches.is_present("list") {
        for group in config.server_manager().groups().iter() {
            println!("{}\n", group);
            for server in config.server_manager().iter(group) {
                println!("\t{} ({})", server.name, server.host);
            }
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
