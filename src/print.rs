use crate::{Format, Server, Servers};
use prettytable::Table;

pub fn print_servers(servers: &Servers, format: &Format) {
    match format {
        Format::None => {
            for (_, sg) in &servers.groups {
                for server in &sg.servers {
                    println!("{}", server.name);
                }
            }
        }
        Format::Pretty => {
            let mut table = Table::new();
            table.add_row(row!["Server Group", "Server", "Description"]);

            for (group, server_group) in &servers.groups {
                let mut srv_str = String::new();
                let mut desc_str = String::new();
                for server in &server_group.servers {
                    srv_str += &format!("{} ({})\n", server.name, server.hostname);
                    desc_str += &format!("{}\n", server.description);
                }
                table.add_row(row![&group, srv_str, desc_str]);
            }

            table.printstd();
        }
    }
}

pub fn print_server_group(group: &str, servers: &[Server], format: &Format) {
    match format {
        Format::None => {
            for server in servers {
                println!("{}", server.name);
            }
        }
        Format::Pretty => {
            let mut table = Table::new();
            table.add_row(row!["Server Group", "Server", "Description"]);

            let mut srv_str = String::new();
            let mut desc_str = String::new();
            for server in servers {
                srv_str += &format!("{} ({})\n", server.name, server.hostname);
                desc_str += &format!("{}\n", server.description);
            }
            table.add_row(row![&group, srv_str, desc_str]);
            table.printstd();
        }
    }
}
