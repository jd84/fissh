#[macro_use]
extern crate prettytable;

mod app;
pub use app::{Application, Format, RunMode};
mod print;
pub use print::{print_server_group, print_servers};
mod server;
pub use server::{AuthMethod, Identity, Server, Servers};
mod process;
pub use process::{Process, ProcessMode, Transfer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let application = Application::default();
    let servers = Servers::from_file(application.get_config_file())?;

    match application.get_run_mode() {
        RunMode::List(group, format) => match group {
            Some(name) => print_server_group(name, servers.find_by_group(name), &format),
            None => print_servers(&servers, &format),
        },
        RunMode::Ssh(host) => {
            let (server, identity) = servers.server_with_identity(&host);
            let mut process = Process::new(ProcessMode::Ssh)
                .with_ssh_args(&server, &identity)
                .build();

            println!(
                "Start SSH for {} ({}) as {}",
                server.name, server.hostname, identity.user
            );
            process.run();
        }
        RunMode::Scp(host, transfer) => {
            let (server, identity) = servers.server_with_identity(&host);
            let mut process = Process::new(ProcessMode::Scp)
                .with_scp_args(&server, identity, transfer)
                .build();

            println!(
                "Start SCP for {} ({}) as {}",
                server.name, server.hostname, identity.user
            );
            process.run();
        }
        RunMode::Unknown => panic!("invalid parameter combination"),
    }

    Ok(())
}
