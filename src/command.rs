use super::server::{Server, Account, Auth};

use std::process::Command;

pub struct Ssh {
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
