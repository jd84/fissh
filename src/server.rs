use slot::Slot;
use std::collections::HashMap;
use std::net::IpAddr;

pub trait Manager {
    type Item;

    /// Add new item
    fn add(&mut self, item: Self::Item);

    /// Find an item
    fn find(&self, name: &str) -> Option<&Self::Item>;
}

pub struct Server {
    pub name: String,
    pub host: String,
    pub port: u32,
    pub users: Vec<String>,
    pub group: String,
    pub ip: Option<IpAddr>,
    pub checked: bool,
}

pub enum Auth {
    PublicKey(String),
    Password,
}

pub struct Account {
    pub name: String,
    pub auth: Auth,
}

pub struct ServerManager {
    groups: HashMap<String, Vec<usize>>,
    servers: Slot<Server>,
}

pub struct CredentialManager {
    accounts: Vec<Account>,
}

impl Account {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            auth: Auth::Password,
        }
    }

    pub fn with_key(name: &str, key: String) -> Self {
        Self {
            name: String::from(name),
            auth: Auth::PublicKey(key),
        }
    }
}

impl Server {
    pub fn with(name: &str, host: &str, port: u32, users: Vec<String>, group: &str) -> Self {
        Self {
            name: name.to_owned(),
            host: host.to_owned(),
            port,
            users,
            group: group.to_owned(),
            ip: None,
            checked: false,
        }
    }
}

impl Default for ServerManager {
    fn default() -> Self {
        ServerManager {
            servers: Slot::new(),
            groups: HashMap::new(),
        }
    }
}

impl ServerManager {
    pub fn get_groups(&self) -> Vec<&String> {
        self.groups.iter().map(|(g, _)| g).collect::<Vec<_>>()
    }

    pub fn get_servers(&self) -> Vec<(usize, &Server)> {
        self.servers.iter().collect()
    }

    pub fn get_servers_mut(&mut self) -> Vec<(usize, &mut Server)> {
        self.servers.iter_mut().collect()
    }

    pub fn get_server_group(&self, name: &str) -> Vec<(usize, &Server)> {
        let keys = self.groups.get(name).unwrap();
        self.servers
            .iter()
            .filter(|(k, _)| keys.contains(k))
            .collect()
    }
}

impl Manager for ServerManager {
    type Item = Server;

    fn add(&mut self, s: Server) {
        if let Some(group) = self.groups.get_mut(&s.group) {
            let key = self.servers.insert(s);
            group.push(key);
        } else {
            let group = s.group.clone();
            let key = self.servers.insert(s);
            self.groups.insert(group, vec![key]);
        }
    }

    fn find(&self, name: &str) -> Option<&Self::Item> {
        if let Some(key) = self.servers.iter().position(|(_, s)| s.name == name) {
            return Some(self.servers.get(key));
        }
        None
    }
}

impl Default for CredentialManager {
    fn default() -> Self {
        Self {
            accounts: Vec::new(),
        }
    }
}

impl Manager for CredentialManager {
    type Item = Account;

    fn add(&mut self, a: Account) {
        self.accounts.push(a);
    }

    fn find(&self, name: &str) -> Option<&Self::Item> {
        if let Some(idx) = self.accounts.iter().position(|r| r.name == name) {
            return self.accounts.get(idx);
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn server_manager_integrity_check() {
        let mut s_manager = ServerManager::default();
        let s = Server::with(
            "test",
            "test.localhost.local",
            22,
            vec![String::from("root")],
            "default",
        );
        s_manager.add(s);

        assert_eq!(s_manager.find("test").unwrap().name, "test");
    }

    #[test]
    fn credential_manager_check() {
        let mut c_manager = CredentialManager::default();
        let account = Account::new("root");
        c_manager.add(account);

        assert_eq!(c_manager.find("root").unwrap().name, "root");
    }
}
