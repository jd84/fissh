extern crate clap;

mod server;
mod config;
mod command;

use clap::{Arg, App};
use std::env;

use server::{Server, Account};
use config::ConfigError;

fn main() -> Result<(), ConfigError> {
    let matches = App::new("fissh")
        .version("0.0.2")
        .author("Jan D. <jd84@protonmail.com>")
        .about("fissh is a ssh wrapper and connection manager.")
        .arg(Arg::with_name("config")
            .short("c")
            .help("The configuration file for fissh. Default is `~/.ssh/fissh.yml`")
            .takes_value(true)
        )
        .arg(Arg::with_name("list")
            .short("l")
            .long("list")
            .help("List all configured hosts")
        )
        .arg(Arg::with_name("transfer")
            .short("t")
            .long("transfer")
            .help("Transfer a file from or to a server")
            .requires("TO_OR_FROM")
        )
        .arg(Arg::with_name("HOST_OR_GROUP")
            .help("The host used for the next connection")
            .index(1)
            .required_unless_one(&["list", "Version", "edit"])
        )
        .arg(Arg::with_name("TO_OR_FROM")
            .help("The source or destination for scp")
            .index(2)
        )
        .arg(Arg::with_name("edit")
            .help("Open fissh.yml in your favorite editor.")
            .short("e")
            .long("edit")
        )
        .get_matches();

    let mut default_file = env::var_os("HOME").unwrap();
    default_file.push("/.ssh/fissh.yml");

    let config_file = matches.value_of("config").unwrap_or(default_file.to_str().unwrap());
    let config = config::Config::from_file(config_file)?;

    if matches.is_present("transfer") {
        let src = matches.value_of("HOST_OR_GROUP").unwrap();
        let dest = matches.value_of("TO_OR_FROM").unwrap();

        // transfer from server 
        if src.contains(":") {
            let src_parts: Vec<&str> = src.split(":").collect();
            let server = config.server_manager().find(&src_parts[0]);
            let account = config.credential_manager().find(&server.users[0]);
            
            let file_src = format!("{}:{}", server.host, &src_parts[1]);
            transfer(server, account, &file_src, dest);

        // transfer to server
        } else {
            let dest_parts: Vec<&str> = dest.split(":").collect();
            let server = config.server_manager().find(&dest_parts[0]);
            let account = config.credential_manager().find(&server.users[0]);

            let file_dest = format!("{}:{}", server.host, &dest_parts[1]);
            transfer(server, account, src, &file_dest);
        }
        println!("Thanks for using fissh!");
        return Ok(());
    }

    if matches.is_present("edit") {
        match config.editor {
            Some(ref prog) => {
                println!("Start {} to edit fissh.yml", prog);
                let mut editor = command::Editor::new(prog, config_file);
                editor.run();
            }
            _ => {
                println!("Specify an editor to use this feature!");
            }
        }
        println!("Thanks for using fissh!");
        return Ok(());
    }

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
    Ok(())
}

fn connect(server: &Server, account: &Account) {
    println!("Start SSH for {} as {}", server.host, account.name);
    
    let mut ssh = command::Ssh::with(server, account);
    ssh.run();
}

fn transfer(server: &Server, account: &Account, from: &str, to: &str) {
    println!("Start SCP for {} as {}", server.host, account.name);

    let mut scp = command::Scp::with(server, account, from, to);
    scp.run();
}

fn print_servers(group: &str, servers: &Vec<Server>) {
    println!("{}\n", group);

    let mut i = 0;
    for server in servers {
        i += 1;
        let server_name = format!("{} ({})", server.name, server.host);
        if i % 4 == 0 {
            println!("\t{0: <40}", server_name);
        } else {
            print!("\t{0: <40}", server_name);
        }
    }
    println!("\n");
}
