extern crate clap;
extern crate yaml_rust;

mod server;

use clap::{Arg, App};
use yaml_rust::{Yaml, YamlLoader};

use std::fs::File;
use std::process::{Command};
use std::io::{Read};

use server::{ServerManager, Server, CredentialManager, Account, Auth, Manager};

fn main() {
    let matches = App::new("fissh")
        .version("0.0.0")
        .author("Jan D. <jan@stdpixel.com>")
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
    let config = YamlLoader::load_from_str(&config_to_string(config_file)).unwrap();

    let mut server_manager = ServerManager::default();
    let mut credential_manager = CredentialManager::default();

    for group in config[0]["groups"].as_vec().unwrap() {
        parse_config_hash(group["Hosts"].as_vec().unwrap(), &mut server_manager, |s| {
            let users: Vec<String> = s["Users"].as_vec().unwrap()
                .into_iter()
                .map(|u| u.as_str().unwrap().to_owned())
                .collect();

            Server::with(
                s["Name"].as_str().unwrap().to_owned(), 
                s["HostName"].as_str().unwrap().to_owned(),
                s["Port"].as_i64().unwrap() as u32,
                users,
                group["Name"].as_str().unwrap().to_owned(),
            )
        });
    }
    
    parse_config_hash(config[0]["credentials"].as_vec().unwrap(), &mut credential_manager, |c| {
        let user = c["User"].as_str().unwrap().to_owned();

        if let Some(file) = c["IdentityFile"].as_str() {
            return Account::with_key(user, file.to_owned());
        } else {
            return Account::new(user);
        }
    });

    if matches.is_present("list") {
        for group in server_manager.groups().iter() {
            println!("{}\n", group);
            for server in server_manager.iter(group) {
                println!("\t{} ({})", server.name, server.host);
            }
            println!("");
        }
        
        std::process::exit(0);
    }

    if let Some(host) = matches.value_of("HOST") {
        connect(&server_manager, &credential_manager, host);
    }
    
    println!("Thanks for using fissh!");
}

fn config_to_string(path: &str) -> String {
    let mut content = String::new();
    let mut file = File::open(path).expect("Unable to open file");
    file.read_to_string(&mut content).expect("Unable to read file");
    content
}

fn parse_config_hash<F, M, T>(data: &Vec<Yaml>, manager: &mut M, f: F)
    where F: Fn(&Yaml) -> T,
        M: Manager<Item=T>
{
    for item in data {
        manager.add(f(item));
    }
}

fn connect(servers: &ServerManager, accounts: &CredentialManager, name: &str) {
    let server: &Server = servers.find(String::from(name));
    let account: &Account = accounts.find(&server.users[0]);

    println!("Start SSH for {} as {}", server.host, account.name);
    
    let mut args: Vec<&str> = Vec::new();
    if let Auth::PublicKey(ref file) = account.auth {
        args.push("-i");
        args.push(file);
    }

    let conn = format!("{}@{}", account.name, server.host);
    args.push(&conn);

    let mut ssh = Command::new("ssh")
        .args(args)
        .spawn()
        .expect("ssh failed");
    ssh.wait().unwrap();
}
