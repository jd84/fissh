extern crate clap;

mod command;
mod config;
mod server;

use clap::{App, Arg};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use config::ConfigError;
use server::{Account, Server, ServerManager};

fn main() -> Result<(), ConfigError> {
    let matches = App::new("russh")
        .version("0.0.3")
        .author("Jan D. <jd84@protonmail.com>")
        .about("russh is a ssh wrapper and connection manager.")
        .arg(
            Arg::with_name("config")
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
            Arg::with_name("transfer")
                .short("t")
                .long("transfer")
                .help("Transfer a file from or to a server")
                .requires("TO_OR_FROM"),
        )
        .arg(
            Arg::with_name("HOST_OR_GROUP")
                .help("The host used for the next connection")
                .index(1)
                .required_unless_one(&["list", "Version", "edit", "autocomplete"]),
        )
        .arg(
            Arg::with_name("TO_OR_FROM")
                .help("The source or destination for scp")
                .index(2),
        )
        .arg(
            Arg::with_name("edit")
                .help("Open russh.yml in your favorite editor.")
                .short("e")
                .long("edit"),
        )
        .arg(
            Arg::with_name("autocomplete")
                .help("Write the bash autocompletion.")
                .long("autocomplete"),
        )
        .get_matches();

    init();

    let mut default_file = env::var_os("HOME").unwrap();
    default_file.push("/.ssh/russh.yml");

    let config_file = matches
        .value_of("config")
        .unwrap_or(default_file.to_str().unwrap());
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
        println!("Thanks for using russh!");
        return Ok(());
    }

    if matches.is_present("autocomplete") {
        write_autocomplete(config.server_manager());
        return Ok(());
    }

    if matches.is_present("edit") {
        match config.editor {
            Some(ref prog) => {
                println!("Start {} to edit russh.yml", prog);
                let mut editor = command::Editor::new(prog, config_file);
                editor.run();
            }
            _ => {
                println!("Specify an editor to use this feature!");
            }
        }
        println!("Thanks for using russh!");
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

    println!("Thanks for using russh!");
    Ok(())
}

fn init() {
    let mut config_dir = env::var_os("HOME").unwrap();
    config_dir.push("/.russh");
    if !Path::new(&config_dir).exists() {
        std::fs::create_dir(&config_dir).unwrap();
        config_dir.push("/completion");
        std::fs::create_dir(&config_dir).unwrap();
    }
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

fn write_autocomplete(servers: &ServerManager) {
    let func_raw = r#"#compdef fissh
_arguments "1: :(%__SERVERS__%)"
"#;

    let mut compreply = String::new();
    for (_, ss) in servers.all() {
        for s in ss {
            compreply.push_str(&format!("{} ", s.name));
        }
    }

    let func = func_raw.replace("%__SERVERS__%", &compreply);
    print!("{}", func);

    let mut default_file = env::var_os("HOME").unwrap();
    default_file.push("/.russh/completion/_russh");
    let mut file = File::create(default_file).unwrap();
    file.write_all(func.as_bytes()).unwrap();

    println!("Add the following lines to you .zshrc file.\n");
    println!("# COMPLETION SETTINGS");
    println!("# add custom completion scripts");
    println!("fpath=(~/.russh/completion $fpath)\n");
    println!("# compsys initialization");
    println!("autoload -U compinit");
    println!("compinit\n");
    println!("# show completion menu when number of options is at least 2");
    println!("zstyle ':completion:*' menu select=2");
}
