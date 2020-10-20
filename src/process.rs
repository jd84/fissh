use crate::{AuthMethod, Identity, Server};
use std::process::Command;

pub enum ProcessMode {
    Ssh,
    Scp,
}

pub enum Transfer<'a> {
    FromHost { from: &'a str, to: &'a str },
    ToHost { from: &'a str, to: &'a str },
}

pub struct Process {
    cmd: Command,
}

pub struct ProcessBuilder {
    process: Process,
}

impl Process {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(mode: ProcessMode) -> ProcessBuilder {
        let cmd;
        match mode {
            ProcessMode::Ssh => cmd = Command::new("ssh"),
            ProcessMode::Scp => cmd = Command::new("scp"),
        }

        let process = Process { cmd };
        ProcessBuilder { process }
    }

    pub fn run(&mut self) {
        let mut process = self.cmd.spawn().expect("failed to spawn process");
        process.wait().unwrap();
    }
}

impl ProcessBuilder {
    pub fn with_ssh_args(mut self, server: &Server, identity: &Identity) -> ProcessBuilder {
        let mut args: Vec<&str> = Vec::new();

        match identity.method() {
            AuthMethod::Key(key) => {
                args.push("-i");
                args.push(key);
            }
            AuthMethod::Password => {}
        }

        let dsn = format!("{}@{}", identity.user, server.hostname);
        args.push(&dsn);

        self.process.cmd.args(args);
        self
    }

    pub fn with_scp_args(
        mut self,
        server: &Server,
        identity: &Identity,
        transfer: Transfer,
    ) -> ProcessBuilder {
        let mut args: Vec<&str> = Vec::new();

        match identity.method() {
            AuthMethod::Key(key) => {
                args.push("-i");
                args.push(key);
            }
            AuthMethod::Password => {}
        }

        let dsn;
        match transfer {
            Transfer::FromHost { from, to } => {
                let host_file = format!("{}:{}", server.hostname, from);
                dsn = format!("{}@{}", identity.user, host_file);
                args.push(&dsn);
                args.push(to);
            }
            Transfer::ToHost { from, to } => {
                let host_file = format!("{}:{}", server.hostname, to);
                dsn = format!("{}@{}", identity.user, host_file);
                args.push(from);
                args.push(&dsn);
            }
        }

        self.process.cmd.args(args);
        self
    }

    pub fn build(self) -> Process {
        self.process
    }
}
