extern crate clap;
extern crate slot;

#[macro_use]
extern crate prettytable;

mod args;
mod auth;
mod parser;
mod print;
mod process;

use parser::parse_config_file;
use process::{Mode, Process, Transfer};

use std::env;
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = args::get_matches();

    init();

    let mut default_file = env::var_os("HOME").unwrap();
    default_file.push("/.ssh/russh.yml");
    let sm = parse_config_file(&default_file)?;

    if matches.is_present("list") {
        match matches.value_of("format").unwrap() {
            "table" => print::print_servers(&sm),
            "none" => print::print_servers_raw(&sm),
            _ => unimplemented!()
        }
    }

    if let Some(host) = matches.value_of("HOST_OR_GROUP") {
        let server = sm.get_server_by(&host).expect("host not found");
        let identity = sm.get_identity(server.user()).expect("user not found");

        println!("Start SSH for {} as {}", server.hostname(), identity.user());

        let mut ssh = Process::new_builder(Mode::SSH)
            .with_ssh_args(server, identity)
            .build();

        ssh.run();

        println!("Thanks for using russh!");
    }

    if matches.is_present("HOST_OR_GROUP") && matches.is_present("TO_OR_FROM") {
        let src = matches.value_of("HOST_OR_GROUP").unwrap();
        let dest = matches.value_of("TO_OR_FROM").unwrap();

        let trans;
        let account;
        let server;

        if src.contains(':') {
            let src_parts: Vec<&str> = src.split(':').collect();
            server = sm.get_server_by(&src_parts[0]).unwrap();
            account = sm.get_identity(&server.user()).unwrap();
            trans = Transfer::FromHost(server, (&src_parts[1], dest));
        } else {
            let dest_parts: Vec<&str> = dest.split(':').collect();
            server = sm.get_server_by(&dest_parts[0]).unwrap();
            account = sm.get_identity(&server.user()).unwrap();
            trans = Transfer::ToHost(server, (src, &dest_parts[1]));
        }

        println!("Start SCP for {} as {}", server.hostname(), account.user());

        let mut scp = Process::new_builder(Mode::SCP)
            .with_scp_args(account, trans)
            .build();
        scp.run();

        println!("Thanks for using russh!");

        return Ok(());
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
