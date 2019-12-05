use super::auth::{self, AuthType, Identity};
use std::process::Command;

/// Struct to build a Process
pub struct ProcessBuilder {
    process: Process,
}

/// Encapsulate a file transfer 0: from; 1: to
type Targets<'a> = (&'a str, &'a str);

pub enum Transfer<'a> {
    FromHost(&'a auth::Server, Targets<'a>),
    ToHost(&'a auth::Server, Targets<'a>),
}

/// Process Modes
pub enum Mode {
    SSH,
    SCP,
}

/// The process itself
pub struct Process {
    cmd: Command,
}

// ===== ProcessBuilder =====

/// Process builder to customize the process
impl ProcessBuilder {
    pub fn with_ssh_args(mut self, server: &auth::Server, identity: &Identity) -> ProcessBuilder {
        let mut args: Vec<&str> = Vec::new();

        match identity.auth() {
            AuthType::Key(ref path) => {
                args.push("-i");
                args.push(path);
            }
            AuthType::Password => {}
        }

        let conn = format!("{}@{}", identity.user(), server.hostname());
        args.push(&conn);

        self.process.cmd.args(args);
        self
    }

    pub fn with_scp_args(mut self, identity: &Identity, trans: Transfer) -> ProcessBuilder {
        let mut args: Vec<&str> = Vec::new();

        match identity.auth() {
            AuthType::Key(ref path) => {
                args.push("-i");
                args.push(path);
            }
            AuthType::Password => {}
        }

        let conn;

        match trans {
            Transfer::FromHost(server, targets) => {
                let host_file = format!("{}:{}", server.hostname(), targets.0);
                conn = format!("{}@{}", identity.user(), host_file);
                args.push(&conn);
                args.push(targets.1);
            }
            Transfer::ToHost(server, targets) => {
                let host_file = format!("{}:{}", server.hostname(), targets.1);
                args.push(targets.0);
                conn = format!("{}@{}", identity.user(), host_file);
                args.push(&conn);
            }
        }

        self.process.cmd.args(args);
        self
    }

    pub fn build(self) -> Process {
        self.process
    }
}

// ===== Process =====

impl Process {
    /// Returns a ProcessBuilder to construct the Process
    pub fn new_builder(mode: Mode) -> ProcessBuilder {
        let cmd;

        match mode {
            Mode::SCP => {
                cmd = Command::new("scp");
            }
            Mode::SSH => {
                cmd = Command::new("ssh");
            }
        }

        let p = Process { cmd };

        ProcessBuilder { process: p }
    }

    /// Execute the process and wait
    pub fn run(&mut self) {
        let mut process = self.cmd.spawn().expect("process failed");
        process.wait().unwrap();
    }
}
