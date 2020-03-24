use super::auth::ServerManager;
use prettytable::Table;

pub fn print_servers(sm: &ServerManager) {
    let mut table = Table::new();
    table.add_row(row!["Server-Group", "Hosts"]);

    for group in sm.get_groups() {
        let mut cell_str = String::from("");
        for server in sm.get_servers_by(&group) {
            cell_str += &format!("{} ({})\n", server.name(), server.hostname());
        }
        table.add_row(row![group, cell_str]);
    }

    table.printstd();
}

pub fn print_servers_raw(sm: &ServerManager) {
    for server in sm.get_servers() {
        println!("{}", server.name());
    }
}
