use super::server::{Account, Auth, Server};
use std::process::Command;

/// Struct to build a Process
pub struct ProcessBuilder {
    process: Process,
}

/// Encapsulate a file transfer 0: from; 1: to
type Targets<'a> = (&'a str, &'a str);

pub enum Transfer<'a> {
    FromHost(&'a Server, Targets<'a>),
    ToHost(&'a Server, Targets<'a>),
}

/// Process Modes
pub enum Mode {
    SSH,
    SCP,
    Editor(String),
}

/// The process itself
pub struct Process {
    cmd: Command,
}

// ===== ProcessBuilder =====

/// Process builder to customize the process
impl ProcessBuilder {
    pub fn with_ssh_args(mut self, server: &Server, account: &Account) -> ProcessBuilder {
        let mut args: Vec<&str> = Vec::new();

        match account.auth {
            Auth::PublicKey(ref path) => {
                args.push("-i");
                args.push(path);
            }
            _ => {}
        }

        let conn = format!("{}@{}", account.name, server.host);
        args.push(&conn);

        self.process.cmd.args(args);
        self
    }

    pub fn with_scp_args(mut self, account: &Account, trans: Transfer) -> ProcessBuilder {
        let mut args: Vec<&str> = Vec::new();

        match account.auth {
            Auth::PublicKey(ref path) => {
                args.push("-i");
                args.push(path);
            }
            _ => {}
        }

        let conn;

        match trans {
            Transfer::FromHost(server, targets) => {
                let host_file = format!("{}:{}", server.host, targets.0);
                conn = format!("{}@{}", account.name, host_file);
                args.push(&conn);
                args.push(targets.1);
            }
            Transfer::ToHost(server, targets) => {
                let host_file = format!("{}:{}", server.host, targets.1);
                args.push(targets.0);
                conn = format!("{}@{}", account.name, host_file);
                args.push(&conn);
            }
        }

        self.process.cmd.args(args);
        self
    }

    pub fn with_editor_args(mut self, path: &str) -> ProcessBuilder {
        let mut args: Vec<&str> = Vec::new();

        args.push(path);
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
    pub fn new(mode: Mode) -> ProcessBuilder {
        let cmd;

        match mode {
            Mode::SCP => {
                cmd = Command::new("scp");
            }
            Mode::SSH => {
                cmd = Command::new("ssh");
            }
            Mode::Editor(prog) => cmd = Command::new(prog),
        }

        let p = Process { cmd: cmd };

        ProcessBuilder { process: p }
    }

    /// Execute the process and wait
    pub fn run(&mut self) {
        let mut process = self.cmd.spawn().expect("process failed");
        process.wait().unwrap();
    }
}
