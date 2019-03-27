use super::server::{Server, Account, Auth};

use std::process::Command;

pub struct Ssh {
    command: Command,    
}

pub struct Scp {
    command: Command,
}

impl Ssh {
    pub fn with(server: &Server, account: &Account) -> Self {
        let mut args: Vec<&str> = Vec::new();
        if let Auth::PublicKey(ref file) = account.auth {
            args.push("-i");
            args.push(file);
        }

        let conn = format!("{}@{}", account.name, server.host);
        args.push(&conn);

        let mut ssh = Self {
            command: Command::new("ssh"),
        };

        ssh.command.args(args);
        ssh
    }

    pub fn run(&mut self) {
        let mut ssh = self.command.spawn().expect("ssh failed");
        ssh.wait().unwrap();
    }
}

impl Scp {
    pub fn with(_server: &Server, account: &Account, src: &str, dest: &str) -> Self {
        let mut args: Vec<&str> = Vec::new();
        
        if let Auth::PublicKey(ref file) = account.auth {
            args.push("-i");
            args.push(file);
        }

        let mut _conn = String::new();

        if src.contains(":") {
            _conn = format!("{}@{}", account.name, src);
            args.push(&_conn);
            args.push(dest);
        } else {
            args.push(src);
            _conn = format!("{}@{}", account.name, dest);
            args.push(&_conn);
        }
        
        let mut scp = Self {
            command: Command::new("scp"),
        };

        scp.command.args(args);
        scp
    }

    pub fn run(&mut self) {
        let mut scp = self.command.spawn().expect("scp failed");
        scp.wait().unwrap();
    }
}
