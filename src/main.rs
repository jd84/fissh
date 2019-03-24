extern crate clap;
extern crate yaml_rust;

use clap::{Arg, App};
use yaml_rust::{Yaml, YamlLoader};

use std::fs::File;
use std::process::{Command};
use std::io::{Read};

#[derive(Debug)]
struct Server {
    name: String,
    host: String,
    port: u32,
    users: Vec<String>,
}

#[derive(Debug)]
enum Auth {
    Key(String),
    Password,
}

#[derive(Debug)]
struct Credential {
    user: String,
    auth: Auth,
}

type Servers = Vec<Server>;
type Credentials = Vec<Credential>;

fn main() {
    let matches = App::new("fissh")
        .version("1.0")
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

    let config_file = matches.value_of("config").unwrap_or("config.yml");
    let config = YamlLoader::load_from_str(&config_to_string(config_file)).unwrap();

    let mut servers = Servers::new();
    let mut credientials = Credentials::new();
    
    parse_config_hash(config[0]["hosts"].as_vec().unwrap(), &mut servers, |s| {
        let users: Vec<String> = s["users"].as_vec().unwrap()
            .into_iter()
            .map(|u| u.as_str().unwrap().to_owned())
            .collect();

        Server {
            name: s["Name"].as_str().unwrap().to_owned(),
            host: s["HostName"].as_str().unwrap().to_owned(),
            port: s["Port"].as_i64().unwrap() as u32,
            users: users,
        }
    });

    parse_config_hash(config[0]["credentials"].as_vec().unwrap(), &mut credientials, |c| {
        let mut auth = Auth::Password;
        if let Some(file) = c["IdentityFile"].as_str() {
            auth = Auth::Key(file.to_owned());
        } 
        Credential {
            user: c["User"].as_str().unwrap().to_owned(),
            auth: auth,
        }
    });

    if matches.is_present("list") {
        for server in &servers {
            println!("{} ({})", server.name, server.host);
        }
        std::process::exit(0);
    }

    if let Some(host) = matches.value_of("HOST") {
        connect(&servers, &credientials, host);
    }
    
    println!("Thanks for using fissh!");
}

fn config_to_string(path: &str) -> String {
    let mut content = String::new();
    let mut file = File::open(path).expect("Unable to open file");
    file.read_to_string(&mut content).expect("Unable to read file");
    content
}

fn parse_config_hash<F, T>(data: &Vec<Yaml>, storage: &mut Vec<T>, f: F)
    where F: Fn(&Yaml) -> T
{
    for item in data {
        storage.push(f(item));
    }
}

fn connect(servers: &Servers, accounts: &Credentials, name: &str) {
    let server: &Server = servers.into_iter().filter(|s| s.name == name).collect::<Vec<_>>()[0];
    let account: &Credential = accounts.into_iter().filter(|a| a.user == server.users[0]).collect::<Vec<_>>()[0];

    println!("Start SSH for {} as {}", server.host, account.user);
    
    let mut args: Vec<&str> = Vec::new();
    if let Auth::Key(ref file) = account.auth {
        args.push("-i");
        args.push(file);
    }

    let conn = format!("{}@{}", account.user, server.host);
    args.push(&conn);

    let mut ssh = Command::new("ssh")
        .args(args)
        .spawn()
        .expect("ssh failed");
    ssh.wait().unwrap();
}
