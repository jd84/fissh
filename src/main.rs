extern crate clap;

mod config;
mod server;
mod process;
mod args;

use std::env;
use std::path::Path;

use config::ConfigError;
use process::{Process, Mode, Transfer};
use server::{Account, Server, Manager};

fn main() -> Result<(), ConfigError> {
    let matches = args::get_matches();

    init();

    let mut default_file = env::var_os("HOME").unwrap();
    default_file.push("/.ssh/russh.yml");

    let config_file = matches
        .value_of("config")
        .unwrap_or(default_file.to_str().unwrap());
    let config = config::Config::from_file(config_file)?;

    if matches.is_present("HOST_OR_GROUP") && matches.is_present("TO_OR_FROM") {
    // if matches.is_present("transfer") {
        let src = matches.value_of("HOST_OR_GROUP").unwrap();
        let dest = matches.value_of("TO_OR_FROM").unwrap();

        // transfer from server
        let trans;
        let account;
        let server;

        if src.contains(":") {
            let src_parts: Vec<&str> = src.split(":").collect();
            server = config.server_manager().find(&src_parts[0]).unwrap();
            account = config.credential_manager().find(&server.users[0]).unwrap();
            trans = Transfer::FromHost(server, (&src_parts[1], dest));
        } else {
            let dest_parts: Vec<&str> = dest.split(":").collect();
            server = config.server_manager().find(&dest_parts[0]).unwrap();
            account = config.credential_manager().find(&server.users[0]).unwrap();
            trans = Transfer::ToHost(server, (src, &dest_parts[1]));
        }

        transfer(server, account, trans);

        return Ok(());
    }

    if matches.is_present("edit") {
        match config.editor {
            Some(ref prog) => {
                edit(prog, config_file);
            }
            _ => {
                println!("Specify an editor to use this feature!");
            }
        }
        
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
        let server = config.server_manager().find(host).unwrap();
        let account = config.credential_manager().find(&server.users[0]).unwrap();
        connect(server, account);
    }


    Ok(())
}

fn init() {
    let mut config_dir = env::var_os("HOME").unwrap();
    config_dir.push("/.russh");
    if !Path::new(&config_dir).exists() {
        std::fs::create_dir(&config_dir).unwrap();
    }
}

fn connect(server: &Server, account: &Account) {
    println!("Start SSH for {} as {}", server.host, account.name);

    let mut ssh = Process::new(Mode::SSH).with_ssh_args(server, account).build();
    ssh.run();

    println!("Thanks for using russh!");
}

fn transfer(server: &Server, account: &Account, trans: Transfer) {
    println!("Start SCP for {} as {}", server.host, account.name);

    let mut scp = Process::new(Mode::SCP).with_scp_args(account, trans).build();
    scp.run();

    println!("Thanks for using russh!");
}

fn edit(program: &str, path: &str) {
    println!("Start {} to edit russh.yml", program);

    let mut editor = Process::new(Mode::Editor(program.to_owned())).with_editor_args(path).build();
    editor.run();

    println!("Thanks for using russh!");
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
